#![no_std]

use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

pub struct ArrayVec<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    size: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    pub fn new() -> Self {
        let array: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];

        ArrayVec { array, size: 0 }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // On success, return Ok(()).
    // If this arrayvec is full, return Err(obj).
    pub fn push(&mut self, obj: T) -> Result<(), T> {
        if self.len() == self.capacity() {
            return Err(obj);
        }

        self.array[self.size].write(obj);
        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.size -= 1;
        let elem = unsafe { self.array[self.size].assume_init_read() };
        self.array[self.size] = MaybeUninit::uninit();

        Some(elem)
    }
}

impl<T, const N: usize> Default for ArrayVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Index<usize> for ArrayVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            panic!("ArrayVec index out of range")
        }

        unsafe { &*self.array[index].as_ptr() }
    }
}

impl<T, const N: usize> IndexMut<usize> for ArrayVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len() {
            panic!("ArrayVec index out of range")
        }

        unsafe { &mut *self.array[index].as_mut_ptr() }
    }
}

impl<T, const N: usize> Drop for ArrayVec<T, N> {
    fn drop(&mut self) {
        for elem in &mut self.array[0..self.size] {
            unsafe { elem.assume_init_drop() };
        }
    }
}
