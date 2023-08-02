//! simple multi-level feedback queue scheduler based on the 4.4BSD scheduler as described in https://www.scs.stanford.edu/23wi-cs212/pintos/pintos_7.html
//! because it seems to work and i don't care enough to reinvent the wheel here

use crate::{arch::bsp::RegisterContext, timer::Timer};
use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use core::{
    fmt::Display,
    pin::Pin,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use crossbeam::queue::SegQueue;
use log::debug;
use spin::Mutex;

type Registers = <crate::arch::InterruptManager as crate::arch::bsp::InterruptManager>::Registers;

const WAIT_STACK_SIZE: usize = 0x1000;
const TIME_SLICE: u64 = 6;
const MAX_PRIORITY: usize = 63;

/// formats a fixed point number properly with 4 decimal places
struct FixedPoint<T>(T);

impl<T: Display + Copy + TryFrom<usize> + core::ops::Shr<T, Output = T> + core::ops::BitAnd<T, Output = T> + core::ops::Mul<T, Output = T> + core::ops::Div<T, Output = T>> core::fmt::Display
    for FixedPoint<T>
where <T as TryFrom<usize>>::Error: core::fmt::Debug
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}.{:04}",
            self.0 >> 14_usize.try_into().unwrap(),
            ((self.0 & ((1_usize << 14) - 1).try_into().unwrap()) * 10000_usize.try_into().unwrap()) / (1_usize << 14).try_into().unwrap()
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExecMode {
    Running,
    Blocked,
    Exited,
}

/// a schedulable task, which can be a process, a thread, or something else entirely
pub struct Task {
    /// whether this task is valid or not.
    /// used when a task is queued for execution but needs to be removed from the queue before executing
    pub is_valid: bool,

    /// the register context of this task
    pub registers: Registers,

    /// whether this task is running, blocked, etc.
    pub exec_mode: ExecMode,

    /// the niceness value of this task, -20..=20
    pub niceness: isize,

    /// estimate of how much CPU time this task has used recently in 17.14 fixed point
    pub cpu_time: isize,
}

/// scheduler for a single CPU
pub struct Scheduler {
    /// the queues of tasks to run in the future
    run_queues: [SegQueue<Arc<Mutex<Task>>>; MAX_PRIORITY + 1],

    /// the task that's currently running
    current_task: Mutex<Option<Arc<Mutex<Task>>>>,

    /// the timer used for scheduling
    timer: Arc<Timer>,

    /// the stack used when waiting around for a task to be queued
    wait_around_stack: Mutex<Pin<Box<[u8]>>>,

    /// how many tasks are ready for execution
    ready_tasks: AtomicUsize,

    /// average of how many tasks have been ready over the past minute
    load_avg: AtomicUsize,

    /// whether or not this scheduler has been dropped
    is_dropped: Arc<AtomicBool>,

    /// temporary until a full on process list is created
    tasks: Mutex<Vec<Arc<Mutex<Task>>>>,
}

impl Scheduler {
    pub fn new(timer: Arc<Timer>) -> Arc<Self> {
        let arc_self = Arc::new(Self {
            run_queues: {
                let mut v = Vec::with_capacity(MAX_PRIORITY + 1);
                for _i in 0..=MAX_PRIORITY {
                    v.push(SegQueue::new());
                }
                v.try_into().unwrap()
            },
            current_task: Mutex::new(None),
            timer,
            wait_around_stack: Mutex::new(Box::into_pin(vec![0_u8; WAIT_STACK_SIZE].into_boxed_slice())),
            ready_tasks: AtomicUsize::new(0),
            load_avg: AtomicUsize::new(0),
            is_dropped: Arc::new(AtomicBool::new(false)),
            tasks: Mutex::new(vec![]),
        });

        Self::every_second(arc_self.clone());

        arc_self
    }

    /// calculates the scheduler's load average every second
    fn every_second(arc_self: Arc<Self>) {
        if arc_self.is_dropped.load(Ordering::SeqCst) {
            return;
        }

        let cur_load_avg = arc_self.load_avg.load(Ordering::SeqCst) as u64;
        let cur_ready_tasks = arc_self.ready_tasks.load(Ordering::SeqCst) as u64;

        // new_load_avg = (59.0 / 60.0) * cur_load_avg + (1.0 / 60.0) * cur_ready_tasks
        let new_load_avg = ((((59 << 14) / 60) * cur_load_avg) >> 14) + ((1 << 14) / 60) * cur_ready_tasks;

        arc_self.load_avg.store(new_load_avg.try_into().unwrap(), Ordering::SeqCst);

        debug!("load_avg is {}", FixedPoint(new_load_avg));

        // temporary, calculates the average CPU time for all tasks
        let new_load_avg: i64 = new_load_avg.try_into().unwrap();
        for task in arc_self.tasks.lock().iter() {
            let mut task = task.lock();

            // cpu_time = (new_load_avg * 2) / (new_load_avg * 2 + 1) * cpu_time + niceness
            let cpu_time = ((new_load_avg * 2 * (1 << 14)) / (new_load_avg * 2 + (1 << 14)) * task.cpu_time as i64) / (1 << 14) + (task.niceness as i64 * (1 << 14));
            task.cpu_time = cpu_time.try_into().unwrap();
        }

        // schedule this function to run again in another second
        arc_self.timer.clone().timeout_in(arc_self.timer.hz(), move |_| Self::every_second(arc_self.clone()));
    }

    /// pushes a task onto the proper runqueue
    pub fn push_task(&self, task: Arc<Mutex<Task>>) {
        let priority = {
            let task = task.lock();

            // MAX_PRIORITY - (cpu_time / 4) - (niceness * 2)
            let raw_prio = MAX_PRIORITY as isize - (((task.cpu_time / 4) - (task.niceness * 2 * (1 << 14))) >> 14);

            // clamp priority to 0..=MAX_PRIORITY
            raw_prio.max(0).min(MAX_PRIORITY as isize) as usize
        };

        self.run_queues[priority].push(task);
        self.ready_tasks.fetch_add(1, Ordering::SeqCst);
    }

    /// temporary
    pub fn add_task(&self, task: Task) {
        let task = Arc::new(Mutex::new(task));
        self.tasks.lock().push(task.clone());
        self.push_task(task);
    }

    /// iterates thru all the runqueues from highest to lowest priority to find an available task
    fn pop_task(&self) -> Option<Arc<Mutex<Task>>> {
        for i in (0..=MAX_PRIORITY).rev() {
            if let Some(task) = self.run_queues[i].pop() {
                if !task.lock().is_valid {
                    continue;
                }

                self.ready_tasks.fetch_sub(1, Ordering::SeqCst);
                return Some(task);
            }
        }

        None
    }

    /// performs a context switch,
    pub fn context_switch(&self, registers: &mut Registers, arc_self: Arc<Self>) {
        // save state of current task and re-queue it if necessary
        {
            let mut current_task = self.current_task.lock();

            if let Some(task) = current_task.take() {
                let exec_mode;

                #[allow(clippy::clone_on_copy)]
                {
                    let mut task = task.lock();
                    task.registers = registers.clone();
                    exec_mode = task.exec_mode;
                }

                if exec_mode == ExecMode::Running {
                    self.push_task(task);
                }
            }
        }

        // load state of new task from the queue, or just wait around if there are no tasks
        if let Some(task) = self.pop_task() {
            #[allow(clippy::clone_on_copy)]
            {
                let mut task = task.lock();

                *registers = task.registers.clone();
                task.cpu_time += 1 << 14;

                self.timer
                    .timeout_in(TIME_SLICE * self.timer.millis(), move |registers| arc_self.context_switch(registers, arc_self.clone()));
            }

            *self.current_task.lock() = Some(task);
        } else {
            // technically not safe or correct because the lock isn't held while waiting, but also i don't care
            let stack = {
                let mut stack = self.wait_around_stack.lock();
                let i = stack.len() - 1;
                &mut stack[i] as *mut _
            };
            *registers = Registers::from_fn(wait_around as *const _, stack);
        }
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.is_dropped.store(true, Ordering::SeqCst);
    }
}

pub extern "C" fn wait_around() -> ! {
    loop {
        (crate::arch::PROPERTIES.wait_for_interrupt)();
    }
}
