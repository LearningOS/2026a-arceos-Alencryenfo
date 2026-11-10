#![no_std]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
    b_count: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            b_count: 0,
        }
    }

    const fn is_initialized(&self) -> bool {
        self.start != 0
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = self.end;
        self.b_count = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::InvalidParam)
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if !self.is_initialized() {
            return Err(AllocError::NotAllocated);
        }
        let mut pos = self.b_pos;
        if layout.align() > 1 {
            pos = (pos + layout.align() - 1) & !(layout.align() - 1);
        }
        if pos + layout.size() > self.p_pos {
            return Err(AllocError::NoMemory);
        }
        self.b_pos = pos + layout.size();
        self.b_count += 1;
        NonNull::new(pos as *mut u8).ok_or(AllocError::NoMemory)
    }

    fn dealloc(&mut self, _pos: NonNull<u8>, _layout: Layout) {
        if self.b_count > 0 {
            self.b_count -= 1;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if num_pages == 0 {
            return Err(AllocError::InvalidParam);
        }
        let mut align = SIZE;
        if align_pow2 > align {
            align = align_pow2;
        }
        if !align.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        let size = num_pages * SIZE;
        let mut start = (self.p_pos - size) & !(align - 1);
        if start < self.b_pos {
            return Err(AllocError::NoMemory);
        }
        self.p_pos = start;
        Ok(start)
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {}

    fn total_pages(&self) -> usize {
        (self.end - self.start) / SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.p_pos) / SIZE
    }

    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / SIZE
    }
}
