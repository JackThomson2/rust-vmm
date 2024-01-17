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
#[derive(Clone)]
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
    pub ring: *mut UsedCell
}

#[derive(Debug)]
pub struct VirtQueue<const S: usize> {
    pub descriptor_cell: *mut DescriptorCell,
    pub available: *mut Available,
    pub used: *mut Used,
    pub size: u16,
}

type MemoryRange      = *mut UsedCell;
type MemoryDescriptor = *mut DescriptorCell;
type MemoryAvailable  = *mut u16;

impl Available {
    pub unsafe fn get_ring_from_idx(&mut self, idx: u16) -> *mut u16 {
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

impl Used {
    pub unsafe fn get_ring_from_idx(&mut self, idx: u16) -> *mut UsedCell {
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
        memory_loc = used_list_loc.add(S + 1) as usize;

        let available_list_loc = memory_loc as MemoryAvailable;
        memory_loc = available_list_loc.add(S + 1) as usize;

        let descript_list_loc = memory_loc as MemoryDescriptor;
        memory_loc = descript_list_loc.add(S + 1) as usize;

        let used_loc = memory_loc as *mut Used;
        memory_loc = used_loc.add(1) as usize;

        let available_loc = memory_loc as *mut Available;
        available_loc.add(1) as usize;

        Self {
            descriptor_cell: descript_list_loc,
            available: available_loc,
            used: used_loc,
            size: S as u16,
        }
    }

    pub unsafe fn get_descriptor_from_idx(&self, idx: u16) -> &mut DescriptorCell {
        self.descriptor_cell.add(idx as usize).as_mut().unwrap()
    }
}

unsafe impl<const S: usize> Send for VirtQueue<S> {}
unsafe impl<const S: usize> Sync for VirtQueue<S> {}

