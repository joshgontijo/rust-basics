use std::sync::{
    atomic::{AtomicPtr, Ordering},
    Arc,
};
use std::{cmp::PartialEq, ops::Deref};

//A wrapper around AtomicPtr, useful for swapping heap allocated values in concurrent environment
//This is more or less like the behaviour a GCed language provides.
//Allows multiple immutable access to the inner data and at the same time swapping it with some other value
#[derive(Clone, Debug)]
pub struct AtomPtr<T> {
    inner: Arc<AtomicPtr<Arc<T>>>,
}


impl<T: Default> Default for AtomPtr<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> PartialEq for AtomPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.get_ref().inner, &other.get_ref().inner)
    }
}

impl<T> Drop for AtomPtr<T> {
    fn drop(&mut self) {
        if Arc::strong_count(&self.inner) == 1 {
            let ptr = self.inner.load(Ordering::Acquire);
            let _b = unsafe { Box::from_raw(ptr) };
            // Let _b go out of scope to clean up memory
        }
    }
}

impl<T> AtomPtr<T> {
    fn make_raw_ptr(t: T) -> *mut Arc<T> {
        Box::into_raw(Box::new(Arc::new(t)))
    }

    /// Create a new atomic pointer for a type
    pub fn new(t: T) -> Self {
        let ptr = Self::make_raw_ptr(t);
        let inner = Arc::new(AtomicPtr::from(ptr));
        Self { inner }
    }

    /// Get an immutable reference to the current value
    pub fn get_ref(&self) -> Ref<T> {
        let ptr = self.inner.load(Ordering::Relaxed);
        let b = unsafe { Box::from_raw(ptr) };

        let arc = Arc::clone(&*b);
        std::mem::forget(b);

        Ref {
            inner: Box::new(arc),
            ptr,
        }
    }

    /// Swap the data entry with a new value, returning the old
    pub fn swap(&self, new: T) -> Ref<T> {
        let new = Self::make_raw_ptr(new);
        let prev = self.inner.swap(new, Ordering::AcqRel);

        let inner = unsafe { Box::from_raw(prev) };
        Ref { inner, ptr: prev }
    }

    /// Compare and swap this pointer for a new one
    pub fn compare_exchange(&self, prev: Ref<T>, new: T) -> CasResult<T> {
        let new = Self::make_raw_ptr(new);
        let prev: *const Arc<T> = prev.as_ptr();
        let prev_mut = prev as *mut Arc<T>;

        match self
            .inner
            .compare_exchange(prev_mut, new, Ordering::SeqCst, Ordering::Acquire)
        {
            Ok(t) => CasResult::Success(Ref {
                inner: unsafe { Box::from_raw(t) },
                ptr: t,
            }),
            Err(t) => CasResult::Failure(Ref {
                inner: unsafe { Box::from_raw(t) },
                ptr: t,
            }),
        }
    }

    /// Compare and swap this pointer for a new one
    ///
    /// Use this variant when called in a loop
    pub fn compare_exchange_weak(&self, prev: Ref<T>, new: T) -> CasResult<T> {
        let new = Self::make_raw_ptr(new);
        let prev: *const Arc<T> = prev.as_ptr();
        let prev_mut = prev as *mut Arc<T>;

        match self
            .inner
            .compare_exchange_weak(prev_mut, new, Ordering::SeqCst, Ordering::Acquire)
        {
            Ok(t) => CasResult::Success(Ref {
                inner: unsafe { Box::from_raw(t) },
                ptr: t,
            }),
            Err(t) => CasResult::Failure(Ref {
                inner: unsafe { Box::from_raw(t) },
                ptr: t,
            }),
        }
    }
}

/// An alias for a referenced pointer
pub struct Ref<T> {
    inner: Box<Arc<T>>,
    ptr: *const Arc<T>,
}

impl<T> Ref<T> {
    /// Consume this Ref wrapper to yield the underlying `Arc<T>`
    ///
    /// If you want to take ownership of the underlying type data, and
    /// you can prove that only one strong-reference Arc exists to
    /// this type, you can use `std::arc::Arc::try_unwrap()` to peel
    /// the reference counter and take exclusive ownership.
    pub fn consume(self) -> Arc<T> {
        *self.inner
    }

    fn as_ptr(&self) -> *const Arc<T> {
        self.ptr
    }
}

impl<T> Deref for Ref<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Result from a CAS operation
pub enum CasResult<T> {
    Success(Ref<T>),
    Failure(Ref<T>),
}

impl<T> CasResult<T> {
    pub fn success(&self) -> bool {
        match self {
            Self::Success(_) => true,
            Self::Failure(_) => false,
        }
    }

    pub fn inner(self) -> Ref<T> {
        match self {
            Self::Success(r) => r,
            Self::Failure(r) => r,
        }
    }
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
struct TestStruct {
    name: String,
}

#[test]
fn cloned() {
    let ts = TestStruct {
        name: "Hello".into(),
    };

    let ptr1 = AtomPtr::new(ts);
    let ptr2 = ptr1.clone();

    assert_eq!(ptr1, ptr2);
}

#[test]
fn swap() {
    let ts1 = TestStruct {
        name: "Hello 1".into(),
    };

    let ts2 = TestStruct {
        name: "Hello 2".into(),
    };

    // Make an AtomPtr with some data
    let ptr = AtomPtr::new(ts1.clone());
    assert_eq!(ptr.get_ref().name, "Hello 1".to_string());

    // Swap the data
    let still_ts1 = ptr.swap(ts2);
    assert_eq!(ptr.get_ref().name, "Hello 2".to_string());

    // But the old ref is still valid
    assert_eq!(ts1, *still_ts1.as_ref());
}

#[test]
fn compare_exchange() {
    let ts1 = TestStruct {
        name: "Hello 1".into(),
    };

    let ts2 = TestStruct {
        name: "Hello 2".into(),
    };

    let ts3 = TestStruct {
        name: "Hello 3".into(),
    };

    // Make an AtomPtr with some data
    let ptr = AtomPtr::new(ts1.clone());
    assert_eq!(ptr.get_ref().name, "Hello 1".to_string());

    // Swap the data
    let still_ts1 = ptr.compare_exchange(ptr.get_ref(), ts2.clone()).inner();
    assert_eq!(ptr.get_ref().name, "Hello 2".to_string());

    let still_ts2 = ptr.compare_exchange(ptr.get_ref(), ts3).inner();
    assert_eq!(ptr.get_ref().name, "Hello 3".to_string());

    // But the old ref is still valid
    assert_eq!(ts1, *still_ts1.as_ref());
    assert_eq!(ts2, *still_ts2.as_ref());
}

#[test]
fn take_from_swap() {
    let ts1 = TestStruct {
        name: "Hello 1".into(),
    };

    let ts2 = TestStruct {
        name: "Hello 2".into(),
    };

    // Make an AtomPtr with some data
    let ptr = AtomPtr::new(ts1.clone());
    assert_eq!(ptr.get_ref().name, "Hello 1".to_string());

    // Swap the data
    let still_ts1 = ptr.swap(ts2);
    assert_eq!(ptr.get_ref().name, "Hello 2".to_string());
    assert_eq!(Arc::strong_count(&still_ts1), 1);

    // We can now also take ownership of the Arc
    let ts1_again = Arc::try_unwrap(still_ts1.consume()).unwrap();
    assert_eq!(ts1_again, ts1);
}

#[test]
fn release() {
    let ts1 = TestStruct {
        name: "Hello world!".into(),
    };
    let ts2 = TestStruct {
        name: "Isn't it lovely outside?".into(),
    };

    let ptr = AtomPtr::new(ts1);

    let first = ptr.get_ref();
    println!("Pointer: {:?}", *first);

    let prev = ptr.compare_exchange(first, ts2);
    if prev.success() {
        println!("Successfully swapped pointer values!");

        let second = ptr.get_ref();
        println!("First: {:?}, Second: {:?}", *prev.inner(), *second);
    }
}
