use std::collections::HashMap;
use std::ops;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_HANDLE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag(u64);

impl Tag {
    fn new() -> Self {
        let val = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
        // Prevent overflow:
        if val == u64::MAX {
            panic!("max valet tag reached")
        }
        Self(val)
    }
}

pub struct Valet<T> {
    items: HashMap<Tag, T>,
}

impl<T> Valet<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, item: T) -> Tag {
        let handle = Tag::new();
        self.items.insert(handle.clone(), item);
        handle
    }

    pub fn get(&self, handle: &Tag) -> &T {
        self.items
            .get(handle)
            .expect("handle is invalid for this depot")
    }

    pub fn get_mut(&mut self, handle: &Tag) -> &mut T {
        self.items
            .get_mut(handle)
            .expect("handle is invalid for this depot")
    }

    pub fn remove(&mut self, handle: &Tag) -> T {
        self.items
            .remove(handle)
            .expect("handle is invalid for this depot")
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<'a, T> ops::Index<&'a Tag> for Valet<T> {
    type Output = T;

    fn index(&self, idx: &'a Tag) -> &Self::Output {
        self.get(idx)
    }
}

impl<'a, T> ops::IndexMut<&'a Tag> for Valet<T> {
    fn index_mut(&mut self, idx: &'a Tag) -> &mut Self::Output {
        self.get_mut(idx)
    }
}
