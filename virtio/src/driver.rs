use core::{
    sync::atomic::{
        fence,
        Ordering::{Acquire, Release},
    },
};

use super::virtqueue::{DescriptorCell, VirtQueue};

#[repr(C)]
pub struct DeviceDriver<const S: usize> {
    pub queue: *mut VirtQueue<S>,

    available_index: u16,
    free_index: u16,

    descriptor_item_index: usize,
    free_descriptor_cells: [u16; S],
}

impl<const S: usize> DeviceDriver<S> {
    pub fn new(queue: *mut VirtQueue<S>) -> Self {
        let mut free_cells = [0; S];

        for (idx, cell) in free_cells.iter_mut().enumerate() {
            *cell = idx as u16;
        }

        Self {
            queue: queue as _,
            available_index: 0,

            descriptor_item_index: S,
            free_descriptor_cells: free_cells,
            free_index: 0,
        }
    }

    pub unsafe fn poll_available_queue(&mut self) -> Option<(*mut DescriptorCell, u16)> {
        fence(Acquire);

        let queue = self.queue.read_volatile();
        let available_ring = queue.available.read_volatile();

        let ring_idx = available_ring.get_idx();

        if self.available_index == ring_idx {
            return None;
        }

        let loading_idx = self.available_index;
        let available_ring_pos = queue
            .get_available_ring_from_idx(loading_idx)
            .read_volatile();

        self.available_index += 1;

        Some((
            queue.get_descriptor_from_idx(available_ring_pos),
            available_ring_pos,
        ))
    }

    pub unsafe fn submit_to_avail_queue(&mut self, idx: u16) {
        let queue = self.queue.read_volatile();
        let available_ring = queue.available.read_volatile();

        let ring_cell = queue.get_available_ring_from_idx(self.available_index);
        *ring_cell = idx;

        available_ring.increment_idx(S as u16);

        fence(Release);

        self.available_index += 1;
        self.available_index &= (S as u16) - 1;
    }

    pub unsafe fn get_descriptor_cell(&mut self) -> Option<(*mut DescriptorCell, u16)> {
        if self.descriptor_item_index == 0 {
            return None;
        }

        self.descriptor_item_index -= 1;

        let queue = self.queue.read_volatile();
        let desc_cell_idx = self.free_descriptor_cells[self.descriptor_item_index];

        Some((queue.get_descriptor_from_idx(desc_cell_idx), desc_cell_idx))
    }

    pub unsafe fn submit_to_used_queue(&mut self, cell_pos: u16) {
        let queue = self.queue.read_volatile();
        let used_ring = queue.used.read_volatile();

        let ring_cell = used_ring
            .get_ring_from_idx(self.available_index)
            .read_volatile();
        (&ring_cell.id as *const u16 as *mut u16).write_volatile(cell_pos);
        used_ring.increment_idx(S as u16);

        fence(Release);

        self.free_index += 1;
        self.free_index &= (S as u16) - 1;
    }

    pub unsafe fn check_used_queue(&mut self) -> Option<(*mut DescriptorCell, u16)> {
        let queue = self.queue.read_volatile();
        let used = queue.used.read_volatile();

        let current_idx = used.get_idx();

        // If this happens there have been no updates
        if current_idx == self.free_index {
            return None;
        }

        let freed_item = used.get_ring_from_idx(self.free_index).as_ref().unwrap();
        self.free_index += 1;

        Some((
            queue.get_descriptor_from_idx(freed_item.id) as *mut DescriptorCell,
            freed_item.id,
        ))
    }

    pub unsafe fn release_back_to_pool(&mut self, idx: u16) {
        self.free_descriptor_cells[self.descriptor_item_index] = idx;
        self.descriptor_item_index += 1;
    }

    pub unsafe fn write_pointer_to_queue(
        &mut self,
        message: *const u8,
        length: usize,
        flag: u16,
    ) -> bool {
        let (cell_ptr, idx) = match self.get_descriptor_cell() {
            Some(res) => res,
            None => return false,
        };

        let cell = cell_ptr.as_mut().unwrap();

        cell.addr = message as u64;
        cell.length = length as u32;
        cell.flags = flag;
        cell.next = 0;

        self.submit_to_avail_queue(idx);

        return true;
    }
}

unsafe impl<const S: usize> Send for DeviceDriver<S> {}
