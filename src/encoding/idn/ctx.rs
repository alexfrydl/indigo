// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A type for storing contextual data.

use super::*;
use std::any::{Any, TypeId};
use std::cell::{self, RefCell};

type Items = im::HashMap<(&'static str, TypeId), Rc<dyn Any>>;

#[derive(Clone)]
pub struct Context {
  items: Items,
}

#[derive(Deref)]
pub struct Ref<'a, T> {
  _item: &'a RefCell<T>,
  #[deref]
  borrowed: cell::Ref<'a, T>,
}

#[derive(Deref, DerefMut)]
pub struct RefMut<'a, T> {
  _item: &'a RefCell<T>,
  #[deref]
  #[deref_mut]
  borrowed: cell::RefMut<'a, T>,
}

impl Context {
  pub fn get<T: Any>(&self, key: &'static str) -> Option<Ref<T>> {
    let item = self.items.get(&(key, TypeId::of::<T>()))?.downcast_ref::<RefCell<T>>().unwrap();

    Some(Ref { _item: item, borrowed: item.borrow() })
  }

  pub fn get_mut<T: Any>(&mut self, key: &'static str) -> Option<RefMut<T>> {
    let item = self.items.get(&(key, TypeId::of::<T>()))?.downcast_ref::<RefCell<T>>().unwrap();

    Some(RefMut { _item: item, borrowed: item.borrow_mut() })
  }

  pub fn put<T: Any>(&mut self, key: &'static str, value: T) {
    self.items.insert((key, TypeId::of::<T>()), Rc::new(RefCell::new(value)));
  }
}

impl Default for Context {
  fn default() -> Self {
    let mut ctx = Self { items: default() };

    ctx.put("errors", ErrorList::new());
    ctx
  }
}

impl From<&'_ Self> for Context {
  fn from(ctx: &'_ Self) -> Self {
    ctx.clone()
  }
}

impl From<&'_ mut Self> for Context {
  fn from(ctx: &'_ mut Self) -> Self {
    ctx.clone()
  }
}
