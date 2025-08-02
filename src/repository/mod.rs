// SPDX-License-Identifier: MIT
// Copyright © 2025 Venkatesh V.K.

pub mod vec_db;

/// Base Repo provides with structure and methods which can be implemented by the concreate type
pub trait BaseRepo {
    type Item;
    type Iter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;
    type Output;

    /// Save the given `item` in the repository
    fn save(&mut self, item: Self::Item) -> Self::Output;

    /// Return an `Iterator` of all the `Item` stored
    fn list<'a>(&'a self) -> Self::Iter<'a>;
}
