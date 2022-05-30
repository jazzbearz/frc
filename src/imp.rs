use crate::inner::Inner;
use std::{
    cell::Cell,
    clone::Clone,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub const DEFAULT_WEIGHT: usize = 1 << 16;
const DEFAULT_ADD_WEIGHT: usize = DEFAULT_WEIGHT << 1;

pub struct Frc<T: ?Sized> {
    pub(crate) weight: Cell<usize>,
    pub(crate) ptr: NonNull<Inner<T>>,
    pub(crate) thread_no: u32,
}

impl<T> Frc<T> {
    #[inline]
    pub fn new(data: T) -> Frc<T> {
        // Allocate the ptr on the heap and set the weights of the values
        // to the default.
        let ptr = Box::new(Inner::new(data, DEFAULT_WEIGHT));
        let thread_no = unsafe { *crate::get_thread_number() };
        Frc {
            weight: Cell::new(DEFAULT_WEIGHT),
            ptr: NonNull::new(Box::into_raw(ptr)).unwrap(),
            thread_no,
        }
    }

    #[inline]
    fn inner(&self) -> &Inner<T> {
        unsafe { self.ptr.as_ref() }
    }
}

unsafe impl<T: ?Sized + Send> Send for Frc<T> {}
unsafe impl<T: ?Sized + Sync> Sync for Frc<T> {}

impl<T> Clone for Frc<T> {
    fn clone(&self) -> Self {
        let tno = unsafe { *crate::get_thread_number() };
        if self.thread_no == tno {
            // Reduce Current Weight
            let existing_weight = self.weight.get();
            let new_weight = if existing_weight > 1 {
                let new_weight = existing_weight >> 1;
                self.weight.set(new_weight);
                new_weight
            } else {
                let add_wei = DEFAULT_ADD_WEIGHT - existing_weight;
                self.inner().add_weight(add_wei);
                self.weight.set(DEFAULT_WEIGHT);
                DEFAULT_WEIGHT
            };
            Frc {
                weight: Cell::new(new_weight),
                ptr: self.ptr,
                thread_no: tno,
            }
        } else {
            // Get from inner directly
            self.inner().add_weight(DEFAULT_WEIGHT);

            Frc {
                weight: Cell::new(DEFAULT_WEIGHT),
                ptr: self.ptr,
                thread_no: tno,
            }
        }
    }
}

impl<T: ?Sized> Drop for Frc<T> {
    fn drop(&mut self) {
        let ptr = unsafe { self.ptr.as_ref() };
        let existing_weight = self.weight.get();
        if ptr.drop_weight(existing_weight) > 0 {
            return;
        }

        let ptr = self.ptr.as_ptr();
        let data = unsafe { Box::from_raw(ptr) };
        drop(data);
    }
}

impl<T> Deref for Frc<T> {
    /// The resulting type after dereferencing
    type Target = T;

    /// The method called to dereference a value
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner().deref()
    }
}

impl<T> DerefMut for Frc<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }.deref_mut()
    }
}

impl<T: fmt::Display> fmt::Display for Frc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.inner(), f)
    }
}

impl<T: fmt::Debug> fmt::Debug for Frc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.inner(), f)
    }
}

impl<T: PartialEq> PartialEq for Frc<T> {
    #[inline]
    fn eq(&self, other: &Frc<T>) -> bool {
        self.inner() == other.inner()
    }
}

impl<T: PartialOrd> PartialOrd for Frc<T> {
    #[inline]
    fn partial_cmp(&self, other: &Frc<T>) -> Option<Ordering> {
        self.inner().partial_cmp(other.inner())
    }

    #[inline]
    fn lt(&self, other: &Frc<T>) -> bool {
        self.inner() < other.inner()
    }

    #[inline]
    fn le(&self, other: &Frc<T>) -> bool {
        self.inner() <= other.inner()
    }

    #[inline]
    fn gt(&self, other: &Frc<T>) -> bool {
        self.inner() > other.inner()
    }

    #[inline]
    fn ge(&self, other: &Frc<T>) -> bool {
        self.inner() >= other.inner()
    }
}

impl<T: Ord> Ord for Frc<T> {
    #[inline]
    fn cmp(&self, other: &Frc<T>) -> Ordering {
        self.inner().cmp(other.inner())
    }
}

impl<T: Eq> Eq for Frc<T> {}

impl<T: Default> Default for Frc<T> {
    #[inline]
    fn default() -> Frc<T> {
        Frc::new(Default::default())
    }
}

impl<T: Hash> Hash for Frc<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state)
    }
}

impl<T> AsRef<T> for Frc<T> {
    fn as_ref(&self) -> &T {
        &self.inner().data
    }
}
