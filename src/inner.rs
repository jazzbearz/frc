use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    sync::atomic,
};

// The Wrc `Inner` value uses `AtomicUsize` to store the total weight value,
// meaning that mutations of the weight are thread-safe.
pub struct Inner<T: ?Sized> {
    weight: atomic::AtomicUsize,
    data: T,
}

impl<T> Inner<T> {
    #[inline]
    pub fn new(data: T, starting_weight: usize) -> Inner<T> {
        Inner {
            weight: atomic::AtomicUsize::new(starting_weight),
            data,
        }
    }
}

impl<T: ?Sized> Inner<T> {
    pub fn add_weight(&self, weight: usize) -> Option<usize> {
        let fetched = self
            .weight
            .fetch_add(weight, std::sync::atomic::Ordering::Relaxed);
        Some(fetched + weight)
    }

    pub fn drop_weight(&self, weight: usize) -> Option<usize> {
        let _fetched = self
            .weight
            .fetch_sub(weight, std::sync::atomic::Ordering::Relaxed);
        Some(weight)
    }

    // pub fn get_weight(&self) -> usize {
    //     self.weight.load(std::sync::atomic::Ordering::Relaxed)
    // }
}

unsafe impl<T: ?Sized + Send> Send for Inner<T> {}
unsafe impl<T: ?Sized + Sync> Sync for Inner<T> {}

impl<T> Deref for Inner<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for Inner<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T> fmt::Display for Inner<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.data, f)
    }
}

impl<T> fmt::Debug for Inner<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.data, f)
    }
}

impl<T: ?Sized + PartialEq> PartialEq for Inner<T> {
    fn eq(&self, other: &Inner<T>) -> bool {
        self.data == other.data
    }
}

impl<T: PartialOrd> PartialOrd for Inner<T> {
    fn partial_cmp(&self, other: &Inner<T>) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }

    fn lt(&self, other: &Inner<T>) -> bool {
        self.data < other.data
    }

    fn le(&self, other: &Inner<T>) -> bool {
        self.data <= other.data
    }

    fn gt(&self, other: &Inner<T>) -> bool {
        self.data > other.data
    }

    fn ge(&self, other: &Inner<T>) -> bool {
        self.data >= other.data
    }
}

impl<T: Ord> Ord for Inner<T> {
    fn cmp(&self, other: &Inner<T>) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<T: Eq> Eq for Inner<T> {}

impl<T: Hash> Hash for Inner<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}
