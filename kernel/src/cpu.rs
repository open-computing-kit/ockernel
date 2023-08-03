use crate::{
    mm::{PageDirTracker, PageManager},
    process::ProcessTable,
    sched::Scheduler,
    timer::Timer,
};
use alloc::{sync::Arc, vec::Vec};
use log::debug;
use spin::{Mutex, RwLock};

pub struct CPU {
    pub timer: Arc<Timer>,
    pub stack_manager: crate::arch::StackManager,
    pub scheduler: Arc<Scheduler>,
}

impl CPU {
    pub fn start_context_switching(&self) -> ! {
        debug!("starting context switching");
        let scheduler = self.scheduler.clone();
        self.scheduler.force_next_context_switch();
        self.timer.timeout_at(0, move |registers| scheduler.context_switch(registers, scheduler.clone(), true));

        crate::sched::wait_around();
    }
}

/// the global state that is stored by all CPUs
pub struct GlobalState {
    pub cpus: RwLock<Vec<CPU>>,
    pub page_directory: Arc<Mutex<PageDirTracker<crate::arch::PageDirectory>>>,
    pub page_manager: Arc<Mutex<PageManager>>,
    pub process_table: RwLock<ProcessTable>,
}

static mut GLOBAL_STATE: Option<GlobalState> = None;

/// gets the global shared state
pub fn get_global_state() -> &'static GlobalState {
    unsafe { GLOBAL_STATE.as_ref().unwrap() }
}

/// initializes the global shared state. must be ran only once, before interrupts are enabled and other CPUs are brought up
///
/// # Safety
///
/// this is unsafe because it changes the state of a global static containing a non thread safe value (the `Option`, not the `GlobalState`)
pub unsafe fn init_global_state(state: GlobalState) {
    if GLOBAL_STATE.is_some() {
        panic!("can't init global state more than once");
    }

    GLOBAL_STATE = Some(state);
}