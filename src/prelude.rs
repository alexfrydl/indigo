// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A “prelude” module containing common imports.

#[doc(no_inline)]
pub use {
  crate::collections::{btree_map, BTreeMap},
  crate::collections::{btree_set, BTreeSet},
  crate::collections::{hash_map, HashMap},
  crate::collections::{hash_set, HashSet},
  crate::collections::{vec_deque, VecDeque},
  crate::collections::{Array, ArrayString, ArrayVec},
  crate::derive::*,
  crate::fail::{self, fail, Result},
  crate::fmt::{self, Debug, Describe, Display, Write as _},
  crate::fs,
  crate::future::{self, Future},
  crate::iter::{self, Itertools},
  crate::log::{self, debug, error, info, trace, warn},
  crate::math::{one, zero, Number, One, Zero},
  crate::random::{self, Random, Rng},
  crate::stream::{self, Stream, StreamExt},
  crate::sync::blocking::unblock,
  crate::sync::pin,
  crate::task::{self, Task},
  crate::thread::{self, Thread},
  crate::time::{self, Date, Duration, Time},
  crate::uuid::{self, Uuid},
  crate::{self as indigo, attempt, attempt_async},
  std::any::Any,
  std::borrow::*,
  std::cmp::{self, Eq, Ord, PartialEq, PartialOrd},
  std::convert::{TryFrom, TryInto},
  std::hash::{self, Hash, Hasher},
  std::io::{BufRead as _, Read as _, Seek as _, Write as _},
  std::marker::PhantomData,
  std::mem::{self, ManuallyDrop},
  std::ops::*,
  std::pin::Pin,
  std::rc::{Rc, Weak as RcWeak},
  std::str::{self, FromStr},
  std::sync::{Arc, Weak as ArcWeak},
  std::{char, panic, slice},
  std::{f32, f64},
  std::{i128, i16, i32, i64, i8, isize},
  std::{u128, u16, u32, u64, u8, usize},
};

#[cfg(feature = "cli")]
#[doc(no_inline)]
pub use crate::cli::{structopt, StructOpt};

#[cfg(feature = "postgres")]
#[doc(no_inline)]
pub use crate::postgres;

#[cfg(feature = "runtime")]
pub(crate) use crate::runtime;

/// Returns the “default value” for a type.
pub fn default<T: Default>() -> T {
  T::default()
}
