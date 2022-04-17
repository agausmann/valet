use std::any::type_name;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_HANDLE: AtomicU64 = AtomicU64::new(0);

//XXX Don't use derives, they generate unnecessary bounds on T.
//#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag<T>(u64, PhantomData<T>);

impl<T> Debug for Tag<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Tag")
            .field(&type_name::<T>())
            .field(&self.0)
            .finish()
    }
}

impl<T> Clone for Tag<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T> Copy for Tag<T> {}

impl<T> PartialEq for Tag<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T> Eq for Tag<T> {}

impl<T> Hash for Tag<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl<T> Tag<T> {
    fn new() -> Self {
        let val = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
        // Prevent overflow:
        if val == u64::MAX {
            panic!("max valet tag reached")
        }
        Self(val, PhantomData)
    }
}

pub struct Valet<T> {
    items: HashMap<Tag<T>, T>,
}

impl<T> Valet<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, item: T) -> Tag<T> {
        let handle = Tag::new();
        self.items.insert(handle.clone(), item);
        handle
    }

    pub fn get(&self, handle: &Tag<T>) -> Option<&T> {
        self.items.get(handle)
    }

    pub fn get_mut(&mut self, handle: &Tag<T>) -> Option<&mut T> {
        self.items.get_mut(handle)
    }

    pub fn remove(&mut self, handle: &Tag<T>) -> Option<T> {
        self.items.remove(handle)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<'a, T> ops::Index<&'a Tag<T>> for Valet<T> {
    type Output = T;

    fn index(&self, idx: &'a Tag<T>) -> &Self::Output {
        self.get(idx).expect("tag is not present in this valet")
    }
}

impl<'a, T> ops::IndexMut<&'a Tag<T>> for Valet<T> {
    fn index_mut(&mut self, idx: &'a Tag<T>) -> &mut Self::Output {
        self.get_mut(idx).expect("tag is not present in this valet")
    }
}
