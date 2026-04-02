use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct TrackingAllocator {
    // These must be Atomics because GlobalAlloc methods take &self (immutable)
    alloc_count: AtomicUsize,
    total_bytes: AtomicUsize,
}

impl TrackingAllocator {
    // A const constructor is required to initialize a static
    pub const fn new() -> Self {
        Self {
            alloc_count: AtomicUsize::new(0),
            total_bytes: AtomicUsize::new(0),
        }
    }
    pub fn reset(&self) {
        self.alloc_count.store(0, Ordering::Relaxed);
        self.total_bytes.store(0, Ordering::Relaxed);
    } 
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        self.total_bytes.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

// Implement Display to show a nice summary of memory usage
impl fmt::Display for TrackingAllocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = self.alloc_count.load(Ordering::Relaxed);
        let bytes = self.total_bytes.load(Ordering::Relaxed);
        write!(f, "Memory Stats: {{ allocations: {}, bytes: {} }}", count, bytes)
    }
}