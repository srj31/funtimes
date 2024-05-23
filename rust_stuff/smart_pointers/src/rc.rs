use std::{cell::Cell, ptr::NonNull};

struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}
pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            ref_count: Cell::new(1),
        });

        Rc {
            // SAFETY: Box doesnt give null ptr
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.ref_count.get();
        inner.ref_count.set(c + 1);
        Rc { inner: self.inner }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.ref_count.get();
        if c == 1 {
            // SAFETY: we are the only reference
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            inner.ref_count.set(c - 1);
        }
    }
}
