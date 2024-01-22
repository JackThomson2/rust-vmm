use core::{mem::ManuallyDrop, sync::atomic::AtomicU16};

use crate::dumb_alloc;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct DescriptorCell {
    pub addr: u64,
    pub length: u32,
    pub flags: u16,
    pub next: u16,
}

impl Default for DescriptorCell {
    fn default() -> Self {
        Self { addr: 0, length: 0, flags: 0, next: 0 }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Available {
    flags: u16,
    idx: u16,
    ring: *mut u16
}

#[repr(C)]
pub struct UsedCell {
    pub id: u16,
    pub len: u32
}

impl Default for UsedCell {
    fn default() -> Self {
        Self { id: 0, len: 0 }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct Used {
    pub flags: u16,
    pub idx: u16,
    pub ring: *mut ManuallyDrop<UsedCell>
}

#[derive(Debug)]
pub struct VirtQueue<const S: usize> {
    pub descriptor_cell: *mut ManuallyDrop<DescriptorCell>,
    pub available: *mut ManuallyDrop<Available>,
    pub available_ring: *mut u16,
    pub used: *mut ManuallyDrop<Used>,
    pub size: u16,
}

type MemoryRange      = *mut ManuallyDrop<UsedCell>;
type MemoryDescriptor = *mut ManuallyDrop<DescriptorCell>;
type MemoryAvailable  = *mut u16;

impl Available {
    pub unsafe fn get_idx(&mut self) -> u16 {
        let atomic = AtomicU16::from_ptr((&mut self.idx) as *mut u16);

        atomic.load(core::sync::atomic::Ordering::SeqCst)
    }

    pub unsafe fn increment_idx(&mut self, max_size: u16) {
        let new_idx = (self.get_idx() + 1) & max_size - 1;
        let atomic = AtomicU16::from_ptr((&mut self.idx) as *mut u16);

        atomic.store(new_idx, core::sync::atomic::Ordering::SeqCst);
    }
}

impl Used {
    pub unsafe fn get_ring_from_idx(&mut self, idx: u16) -> *mut ManuallyDrop<UsedCell> {
        self.ring.add(idx as usize)
    }

    pub unsafe fn get_idx(&mut self) -> u16 {
        (&self.idx as *const u16).read_volatile()
    }

    pub unsafe fn increment_idx(&mut self, max_size: u16) {
        let new_idx = (self.get_idx() + 1) & max_size - 1;

        (&mut self.idx as *mut u16).write_volatile(new_idx);
    }
}

impl<const S: usize> VirtQueue<S> {
    pub unsafe fn new_with_size(mut memory_loc: usize) -> Self {
        let used_list_loc = memory_loc as MemoryRange;
        memory_loc = dumb_alloc::build_static_list(used_list_loc, (0..S).map(|_| Default::default()));

        let available_list_loc = memory_loc as MemoryAvailable;
        memory_loc = dumb_alloc::build_static_list(available_list_loc, 0..S as u16);

        let descript_list_loc = memory_loc as MemoryDescriptor;
        memory_loc = dumb_alloc::build_static_list(descript_list_loc, (0..S).map(|_| Default::default()));

        let used_loc = memory_loc as *mut ManuallyDrop<Used>;
        memory_loc = dumb_alloc::box_object_volatile(used_loc, ManuallyDrop::new(Used {
            flags: 0,
            idx: 0,
            ring: used_list_loc
        }));

        let available_loc = memory_loc as *mut ManuallyDrop<Available>;
        dumb_alloc::box_object_volatile(available_loc, ManuallyDrop::new(Available {
            flags: 0,
            idx: 0,
            ring: available_list_loc
        }));

        Self {
            descriptor_cell: descript_list_loc,
            available: available_loc,
            available_ring: available_list_loc,
            used: used_loc,
            size: S as u16,
        }
    }

    pub unsafe fn from_memory(mut memory_loc: usize) -> Self {
        let used_list_loc = memory_loc as MemoryRange;
        memory_loc = used_list_loc.add(S + 1) as usize;

        let available_list_loc = memory_loc as MemoryAvailable;
        memory_loc = available_list_loc.add(S + 1) as usize;

        let descript_list_loc = memory_loc as MemoryDescriptor;
        memory_loc = descript_list_loc.add(S + 1) as usize;

        let used_loc = memory_loc as *mut ManuallyDrop<Used>;
        memory_loc = used_loc.add(1) as usize;

        let available_loc = memory_loc as *mut ManuallyDrop<Available>;
        available_loc.add(1) as usize;

        Self {
            descriptor_cell: descript_list_loc,
            available: available_loc,
            available_ring: available_list_loc,
            used: used_loc,
            size: S as u16,
        }
    }

    pub unsafe fn get_descriptor_from_idx(&self, idx: u16) -> &mut ManuallyDrop<DescriptorCell> {
        self.descriptor_cell.add(idx as usize).as_mut().unwrap()
    }

    pub unsafe fn get_available_ring_from_idx(&mut self, idx: u16) -> *mut u16 {
        self.available_ring.add(idx as usize)
    }
}

unsafe impl<const S: usize> Send for VirtQueue<S> {}
unsafe impl<const S: usize> Sync for VirtQueue<S> {}

