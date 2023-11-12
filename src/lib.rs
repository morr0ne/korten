#![no_std]

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use rustix::mm::{MapFlags, ProtFlags};

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let res = unsafe {
            rustix::mm::mmap_anonymous(
                null_mut(),
                layout.size(),
                ProtFlags::WRITE | ProtFlags::READ,
                MapFlags::PRIVATE,
            )
        };

        if let Ok(addr) = res {
            addr.cast()
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            let _ = rustix::mm::munmap(ptr.cast(), layout.size());
        };
    }
}
