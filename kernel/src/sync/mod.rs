#![feature(const_fn)]
use core::sync::atomic::{AtomicBool, Ordering};

pub struct OnceGuard(AtomicBool);

impl OnceGuard {
    pub const fn new() -> OnceGuard {
        OnceGuard(AtomicBool::new(false))
    }

    pub fn can_call(&self) -> bool {
        self.0.compare_and_swap(false, true, Ordering::Relaxed) == false
    }
}

pub fn call_once<F>(guard: &OnceGuard, f: F)
    where F: Fn()
{
    if guard.can_call() { f(); }
}
