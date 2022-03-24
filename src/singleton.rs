use crate::inner::Inner;
use crate::Frc;
use std::{
    cell::Cell,
    ptr::NonNull,
    sync::{atomic, atomic::AtomicPtr},
};

pub struct Singleton<T: Sized> {
    ptr: AtomicPtr<Inner<T>>,
}

impl<T> Singleton<T> {
    #[inline]
    pub fn new(data: T) -> Singleton<T> {
        // Allocate the ptr on the heap and set the weights of the values
        // to the default.
        let ptr = Box::new(Inner::new(data, 1));
        Singleton {
            ptr: AtomicPtr::new(Box::into_raw(ptr)),
        }
    }

    #[inline]
    pub fn replace(&self, data: T) -> Option<Frc<T>> {
        let ptr = Box::new(Inner::new(data, 1));
        let prev_ptr = self.ptr.swap(Box::into_raw(ptr), atomic::Ordering::Relaxed);
        if !prev_ptr.is_null() {
            let thread_no = unsafe { *crate::get_thread_number() };
            Some(Frc {
                weight: Cell::new(1),
                ptr: NonNull::new(prev_ptr).unwrap(),
                thread_no,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn get(&self) -> Frc<T> {
        // Get from inner directly
        let ptr = self.ptr.load(atomic::Ordering::Relaxed);
        unsafe {
            ptr.as_ref().unwrap().add_weight(crate::imp::DEFAULT_WEIGHT);
        }
        let thread_no = unsafe { *crate::get_thread_number() };

        Frc {
            weight: Cell::new(crate::imp::DEFAULT_WEIGHT),
            ptr: NonNull::new(ptr).unwrap(),
            thread_no,
        }
    }
    pub const fn default() -> Self {
        Singleton {
            ptr: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
}

impl<T: Sized> Drop for Singleton<T> {
    fn drop(&mut self) {
        let ptr = self.ptr.load(atomic::Ordering::Relaxed);
        if !ptr.is_null() {
            let data = unsafe { Box::from_raw(ptr) };
            if data.drop_weight(1) == 0 && data.get_weight(atomic::Ordering::Acquire) == 0 {
                drop(data);
            }
        }
    }
}
