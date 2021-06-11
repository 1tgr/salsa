#[cfg(feature = "std")]
mod threads {
    pub use parking_lot::lock_api::RawRwLock;
    pub use parking_lot::{Condvar, Mutex, RwLock, RwLockUpgradableReadGuard};
}

#[cfg(not(feature = "std"))]
mod no_threads {
    pub struct Condvar;

    impl Condvar {
        pub fn new() -> Self {
            Self
        }

        pub fn wait<T>(&self, _guard: &mut MutexGuard<'_, T>) {}

        pub fn notify_one(&self) -> bool {
            false
        }
    }

    pub use spinning::{Mutex, MutexGuard, RawRwLock, RwLock, RwLockUpgradableReadGuard};
}

#[cfg(feature = "std")]
pub use threads::*;

#[cfg(not(feature = "std"))]
pub use no_threads::*;
