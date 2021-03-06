//! low level i586-specific task switching

use alloc::{
    alloc::{Layout, alloc, dealloc},
    vec::Vec,
};
use core::arch::asm;
use crate::{
    tasks::{
        IN_TASK, CURRENT_TERMINATED,
        Task, BlockKind,
        add_task, remove_task, 
        get_task, get_task_mut, get_current_task, get_current_task_mut,
        add_page_reference, remove_page_reference, get_page_references,
        switch_tasks,
    },
    types::Errno,
};
use super::{
    PAGE_SIZE, LINKED_BASE,
    ints::SyscallRegisters,
    paging::{
        PAGE_DIR,
        PageDirectory, PageTableFlags,
        alloc_pages_at, free_page_phys,
    },
    syscalls::{read_handler, write_handler},
};
use x86::tlb::flush;

pub struct TaskState {
    pub registers: SyscallRegisters,
    pub pages: PageDirectory,
    pub page_updates: usize,
}

const PAGE_SIZE_U64: u64 = PAGE_SIZE as u64;

struct MappedMem {
    data: &'static mut [u8],
    ptr: *mut u8,
    layout: Layout,
    buf_len: usize,
    existing_phys: Vec<u64>,
}

impl TaskState {
    /// creates a new task state, copying pages from kernel directory
    pub fn new() -> Self {
        let global_dir = unsafe { PAGE_DIR.as_mut().expect("paging not initialized") };

        let mut state = Self {
            registers: Default::default(),
            pages: PageDirectory::new(),
            page_updates: global_dir.page_updates,
        };

        state.copy_pages_from(global_dir, 0, 1024);

        state
    }

    /// copies registers to task state
    pub fn save(&mut self, regs: &SyscallRegisters) {
        self.registers = *regs;
    }

    /// replaces registers with task state
    pub fn load(&self, regs: &mut SyscallRegisters) {
        *regs = self.registers; // replace all registers with our own (:
    }

    /// copy pages from existing page directory, in range start..end (start is inclusive, end is not)
    pub fn copy_pages_from(&mut self, dir: &mut PageDirectory, start: usize, end: usize) {
        assert!(start <= end);
        assert!(end <= 1024);

        for i in start..end {
            self.pages.tables[i] = dir.tables[i];

            unsafe {
                (*self.pages.tables_physical)[i] = (*dir.tables_physical)[i];
            }
        }
    }

    /// copy pages from existing page directory, in range start..end (start is inclusive, end is not)
    /// 
    /// all pages copied have the read/write flag unset, and if it was previously set, the copy on write flag
    /// 
    /// writing to any copied page will cause it to copy itself and all its data, and all writes will go to a new page
    pub fn copy_on_write_from(&mut self, dir: &mut PageDirectory, start: usize, end: usize, owner: usize) {
        assert!(start <= end);
        assert!(end <= 1024);

        for i in start..end {
            if dir.tables[i].is_null() {
                self.pages.tables[i] = core::ptr::null_mut();

                unsafe {
                    (*self.pages.tables_physical)[i] = 0;
                }
            } else {
                for addr in ((i << 22)..((i + 1) << 22)).step_by(PAGE_SIZE) {
                    let orig_page = unsafe { &mut *dir.get_page(addr as u32, false).expect("couldn't get page table") };

                    if !orig_page.is_unused() {
                        let page = unsafe { &mut *self.pages.get_page(addr as u32, true).expect("couldn't create page table") };

                        // disable write flag, enable copy on write
                        let mut flags: PageTableFlags = orig_page.get_flags().into();
                        
                        if flags & PageTableFlags::ReadWrite != 0 {
                            flags &= !PageTableFlags::ReadWrite;
                            flags |= PageTableFlags::CopyOnWrite;
                        }

                        page.set_flags(flags);
                        page.set_address(orig_page.get_address());

                        add_page_reference(orig_page.get_address() as u64, owner);
                    }
                }
            }
        }
    }

    /// frees pages used by this task, and decreases the reference count on any partially copied pages
    pub fn free_pages(&mut self) {
        for i in 0..(LINKED_BASE >> 22) {
            if !self.pages.tables[i].is_null() {
                for addr in ((i << 22)..((i + 1) << 22)).step_by(PAGE_SIZE) {
                    if let Some(page) = self.pages.get_page(addr as u32, false) {
                        let page = unsafe { &mut *page };

                        if (page.get_flags() & u16::from(PageTableFlags::CopyOnWrite)) > 0 {
                            remove_page_reference(page.get_address() as u64);
                        } else {
                            // free the page if there aren't any references to it
                            let phys = page.get_address() as u64;
                            if !get_page_references().contains_key(&phys) {
                                free_page_phys(phys);
                            }

                            page.set_unused();
                        }
                    }
                }
            }
        }
    }

    /// allocate a page at the specified address
    /// 
    /// we can't use the page directory's alloc_frame function, since it'll overwrite data
    pub fn alloc_page(&mut self, addr: u32, is_kernel: bool, is_writeable: bool, invalidate: bool) -> usize {
        assert!(addr % PAGE_SIZE as u32 == 0, "address is not page aligned");

        let page = self.pages.get_page(addr, true).unwrap();

        unsafe {
            let dir = PAGE_DIR.as_mut().unwrap();

            match dir.alloc_frame(page, is_kernel, is_writeable) {
                Ok(phys) => {
                    if invalidate {
                        flush(addr as usize); // invalidate this page in the TLB
                    }

                    phys as usize
                },
                Err(msg) => panic!("couldn't allocate page: {}", msg),
            }
        }
    }

    /// free a page at the specified address
    pub fn free_page(&mut self, addr: u32, invalidate: bool) {
        assert!(addr % PAGE_SIZE as u32 == 0, "address is not page aligned");

        if let Some(page) = self.pages.get_page(addr, false) {
            unsafe {
                let dir = PAGE_DIR.as_mut().unwrap();

                match dir.free_frame(page) {
                    Ok(_) =>
                        if invalidate {
                            flush(addr as usize); // invalidate this page in the TLB
                        },
                    Err(msg) => panic!("couldn't free page: {}", msg),
                }
            }
        }
    }

    pub fn virt_to_phys(&mut self, addr: u32) -> Option<u32> {
        self.pages.virt_to_phys(addr)
    }

    pub fn check_ptr(&mut self, addr: *const u8) -> bool {
        self.pages.virt_to_phys(addr as u32).is_some()
    }

    fn map_task_in(&mut self, addr: u64, len: u64, is_writable: bool) -> Result<MappedMem, Errno> {
        // get starting and ending addresses
        let mut start = addr;
        let mut end = addr + len;

        debug!("mapping task mem");
        debug!("start @ {:#x}, end @ {:#x}", start, end);

        // offset into memory we've paged in
        let mut offset = 0;

        // align start and end addresses to page boundaries
        if start % PAGE_SIZE_U64 != 0 {
            start &= !(PAGE_SIZE_U64 - 1);
            offset = addr - start;
        }

        if end % PAGE_SIZE_U64 != 0 {
            end = (end & !(PAGE_SIZE_U64 - 1)) + PAGE_SIZE_U64;
        }
        
        debug!("buf size {:#x}, aligned to {:#x}, offset {:#x}", len, end - start, offset);

        let buf_len = (end - start).try_into().map_err(|_| Errno::NotEnoughSpace)?;

        let layout = Layout::from_size_align(buf_len, PAGE_SIZE).unwrap();
        let ptr = unsafe { alloc(layout) };

        assert!(ptr as usize % PAGE_SIZE == 0); // make absolutely sure pointer is page aligned

        debug!("mapping {} pages from {:#x} (task mem) to {:#x} (kernel mem)", (end - start) / PAGE_SIZE_U64, start, ptr as usize);

        let dir = unsafe { PAGE_DIR.as_mut().unwrap() };

        // get addresses of pages we're gonna remap so we can map them back later
        let mut existing_phys: Vec<u64> = Vec::with_capacity(((end - start) / PAGE_SIZE_U64) as usize);

        for i in (ptr as usize..ptr as usize + buf_len).step_by(PAGE_SIZE) {
            existing_phys.push(dir.virt_to_phys(i.try_into().unwrap()).unwrap().into());
        }

        debug!("existing_phys: {:x?}", existing_phys);

        // loop over pages, get physical address of each page and map it in or create new page and alloc mem
        for i in (start..end).step_by(PAGE_SIZE) {
            // get the physical address of the page at the given address, or allocate a new one if there isn't one mapped
            let phys_addr = match self.virt_to_phys(i.try_into().map_err(|_| Errno::NotEnoughSpace)?) {
                Some(phys) => phys,
                None => self.alloc_page(i.try_into().map_err(|_| Errno::NotEnoughSpace)?, false, is_writable, false) as u32,
            };

            debug!("{:x} @ phys addr: {:x}", i, phys_addr);

            // todo: maybe change this to debug_assert at some point? its prolly hella slow
            assert!(!existing_phys.contains(&(phys_addr as u64)), "kernel trampling on process memory");

            let virt = ptr as usize + (i - start) as usize;

            // remap memory
            alloc_pages_at(virt, 1, phys_addr as u64, true, true, true);
        }

        // get slice to copy to
        let data = unsafe { core::slice::from_raw_parts_mut((ptr as usize + offset as usize) as *mut u8, len.try_into().map_err(|_| Errno::NotEnoughSpace)?) };

        Ok(MappedMem { data, ptr, layout, buf_len, existing_phys })
    }

    fn map_task_out(&self, mem: MappedMem) {
        debug!("mapping task mem out");

        // map memory back
        for (j, i) in (mem.ptr as usize..mem.ptr as usize + mem.buf_len).step_by(PAGE_SIZE).enumerate() {
            debug!("virt @ {:x}, phys @ {:x}", i, mem.existing_phys[j]);
            alloc_pages_at(i, 1, mem.existing_phys[j], true, true, true);
        }

        // free memory back to heap
        unsafe { dealloc(mem.ptr, mem.layout); }
    }

    /// writes data into task at provided address, allocating memory if required. is_writable controls whether pages are writable for task when allocated
    pub fn write_mem(&mut self, addr: u64, data: &[u8], is_writable: bool) -> Result<(), Errno> {
        let mapped = self.map_task_in(addr, data.len() as u64, is_writable)?;
        
        // copy memory
        debug!("writing {} bytes from slice @ {:#x}", data.len(), addr);
        mapped.data.clone_from_slice(data);

        self.map_task_out(mapped);

        Ok(())
    }

    /// reads data from task at provided address
    pub fn read_mem(&mut self, addr: u64, len: usize, is_writable: bool) -> Result<Vec<u8>, Errno> {
        let mapped = self.map_task_in(addr, len as u64, is_writable)?;
        
        // copy memory
        let res = mapped.data.to_vec();
        debug!("read {} bytes", res.len());

        self.map_task_out(mapped);

        Ok(res)
    }

    /// finds available area in task's memory of given size
    /// 
    /// start is optional, and provides an offset to start searching at (if you want to keep null pointers null, for example)
    pub fn find_hole(&mut self, start: usize, size: usize) -> Option<usize> {
        let mut hole_start: Option<usize> = None;

        for i in 0..(LINKED_BASE >> 22) {
            if self.pages.tables[i].is_null() {
                let addr = i << 22;

                if addr < start && addr + (1 << 22) > start {
                    if addr + (1 << 22) - start >= size {
                        return Some(start);
                    } else {
                        hole_start = Some(start);
                    }
                } else if if let Some(start) = hole_start { addr - start <= size } else { false } {
                    return hole_start;
                } else if size <= (1 << 22) && addr >= start {
                    return Some(addr);
                } else if hole_start.is_none() {
                    hole_start = Some(addr);
                }
            } else {
                for addr in ((i << 22)..((i + 1) << 22)).step_by(PAGE_SIZE) {
                    let orig_page = unsafe { &mut *self.pages.get_page(addr as u32, false).expect("couldn't get page table") };

                    if orig_page.is_unused() {
                        if if let Some(start) = hole_start { addr - start <= size } else { false } {
                            return hole_start;
                        } else if size <= PAGE_SIZE && addr >= start {
                            return Some(addr);
                        } else if hole_start.is_none() && addr >= start {
                            hole_start = Some(addr);
                        }
                    } else {
                        hole_start = None;
                    }
                }
            }
        }

        None
    }
}

impl Default for TaskState {
    fn default() -> Self {
        Self::new()
    }
}

/// idle the cpu until the next task switch
pub fn idle_until_switch() -> ! {
    debug!("idling until next context switch");

    unsafe {
        IN_TASK = true;
        CURRENT_TERMINATED = true;
    }

    loop {
        unsafe { asm!("sti; hlt"); }
    }
}

/// exits current task, cpu idles until next task switch
pub fn exit_current_task() -> ! {
    if let Err(msg) = kill_task(get_current_task().unwrap().id) {
        panic!("couldn't kill task: {}", msg);
    }

    idle_until_switch();
}

/// kills specified task
pub fn kill_task(id: usize) -> Result<(), &'static str> {
    // TODO: signals, etc

    if let Some(task) = get_task(id) {
        remove_task(task.id);

        debug!("process {} exited", id);

        Ok(())
    } else {
        Err("couldn't get task")
    }
}

/// forks task, creating another identical task
pub fn fork_task(id: usize) -> Result<&'static mut Task, &'static str> {
    let current =
        if let Some(task) = get_task_mut(id) {
            task
        } else {
            return Err("couldn't get task")
        };

    // create new task state
    let mut state = TaskState {
        registers: current.state.registers,
        pages: PageDirectory::new(),
        page_updates: current.state.page_updates,
    };

    // copy kernel pages, copy parent task's pages as copy on write
    let kernel_start = LINKED_BASE >> 22;
    let dir = unsafe { PAGE_DIR.as_mut().expect("no paging?") };
    state.copy_on_write_from(&mut current.state.pages, 0, kernel_start, current.id);
    state.copy_pages_from(dir, kernel_start, 1024);

    // create new task with provided state
    let mut task = Task::from_state(state);
    let id = task.id;

    // set new task's parent to current task's id
    task.parent = Some(current.id);

    // add child pid to parent's list of children
    current.children.push(task.id);

    add_task(task);

    // return reference to new task
    Ok(get_task_mut(id).unwrap())
}

/// perform a context switch, saving the state of the current task and switching to the next one in line
pub fn context_switch(regs: &mut SyscallRegisters) -> bool {
    // has the current task been terminated?
    if unsafe { CURRENT_TERMINATED } {
        // it no longer exists, so all we need to do is clear the flag
        unsafe { CURRENT_TERMINATED = false; }
    } else {
        // save state of current task
        get_current_task_mut().expect("no tasks?").state.save(regs);
    }

    // do we have a task to switch to?
    if switch_tasks() {
        // load state of new current task
        let current = get_current_task_mut().expect("no tasks?");

        current.state.load(regs);

        // get reference to global page directory
        let dir = unsafe { PAGE_DIR.as_mut().expect("paging not initialized") };

        // has the kernel page directory been updated?
        if current.state.page_updates != dir.page_updates {
            // get page directory index of the start of the kernel's address space
            let idx = LINKED_BASE >> 22;

            // copy from the kernel's page directory to the task's
            current.state.copy_pages_from(dir, idx, 1024);

            // the task's page directory is now up to date (at least for our purposes)
            current.state.page_updates = dir.page_updates;
        }

        // switch to task's page directory
        current.state.pages.switch_to();

        // was the current task just unblocked?
        if !current.blocked && current.just_unblocked {
            current.just_unblocked = false;

            if current.blocked_err != Errno::None {
                // yes, and we encountered an error
                debug!("unblocked to error {}", current.blocked_err);
                regs.eax = current.blocked_err as u32;
            } else {
                // yes, finish handling what the task was blocked for
                debug!("task just unblocked");

                match current.block_kind {
                    BlockKind::None => (),
                    BlockKind::Read(_) => {
                        if let Err(err) = read_handler(regs) {
                            debug!("syscall error: {}", err);
                            regs.eax = err as u32;
                        } else {
                            regs.eax = 0; // make sure we don't throw an error
                        }
                    },
                    BlockKind::Write(_) => {
                        if let Err(err) = write_handler(regs) {
                            debug!("syscall error: {}", err);
                            regs.eax = err as u32;
                        } else {
                            regs.eax = 0; // make sure we don't throw an error
                        }
                    },
                }
            }
        }

        true
    } else {
        // no, idle until next context switch
        false
    }
}
