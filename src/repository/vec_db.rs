// SPDX-License-Identifier: MIT
// Copyright © 2025 Venkatesh V.K.

#![allow(dead_code)]

use crate::repository::BaseRepo;

/// Simple in memory database which is simple `Vec<T>`
#[derive(Debug)]
pub struct VecDB<T> {
    store: Vec<T>,
}

impl<T> VecDB<T> {
    /// Create a simple in memory database
    pub fn new(data: Option<Vec<T>>) -> Self {
        let d = data.unwrap_or_else(|| Vec::<T>::new());
        Self { store: d }
    }

    /// return `Iterator` wih ref to `Item`
    fn ref_list(&self) -> impl Iterator<Item = &T> {
        self.store.iter()
    }
}

impl<T> BaseRepo for VecDB<T>
where
    T: Clone,
{
    type Item = T;
    type Output = Result<(), Box<dyn std::error::Error>>;

    fn save(&mut self, data: Self::Item) -> Self::Output {
        self.store.push(data);
        Ok(())
    }

    fn list(&self) -> impl Iterator<Item = Self::Item> {
        self.store.iter().cloned()
    }
}
