// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A “prelude” module containing common imports.

pub use crate::collections::{btree_map, BTreeMap};
pub use crate::collections::{btree_set, BTreeSet};
pub use crate::collections::{hash_map, HashMap};
pub use crate::collections::{hash_set, HashSet};
pub use crate::collections::{Array, ArrayString, ArrayVec};
pub use crate::derive::*;
pub use crate::fail::{self, fail, Result};
pub use crate::fmt::{self, Debug, Describe, Display, Write as _};
pub use crate::fs;
pub use crate::future::{self, Future};
pub use crate::iter::{self, Itertools};
pub use crate::log::{self, debug, error, info, trace, warn};
pub use crate::math::{one, zero, Number, One, Zero};
pub use crate::random::{self, Random, Rng};
pub use crate::stream::{self, Stream, StreamExt};
pub use crate::sync::blocking::{block_on, unblock};
pub use crate::sync::pin;
pub use crate::task::{self, Task};
pub use crate::thread::{self, Thread};
pub use crate::time::{self, Date, Duration, Time};
pub use crate::uuid::{self, Uuid};
pub use crate::{self as indigo, attempt, attempt_async};

#[cfg(feature = "cli")]
pub use crate::cli::{structopt, StructOpt};

#[cfg(feature = "postgres")]
pub use crate::postgres;

pub use std::borrow::*;
pub use std::cmp::{self, Eq, Ord, PartialEq, PartialOrd};
pub use std::convert::{TryFrom, TryInto};
pub use std::hash::{self, Hash, Hasher};
pub use std::io::{BufRead as _, Read as _, Seek as _, Write as _};
pub use std::marker::PhantomData;
pub use std::mem::{self, ManuallyDrop};
pub use std::ops::*;
pub use std::pin::Pin;
pub use std::rc::{Rc, Weak as RcWeak};
pub use std::str::{self, FromStr};
pub use std::sync::{Arc, Weak as ArcWeak};
pub use std::{char, panic, slice};
pub use std::{f32, f64};
pub use std::{i128, i16, i32, i64, i8, isize};
pub use std::{u128, u16, u32, u64, u8, usize};

/// Returns the “default value” for a type.
pub fn default<T: Default>() -> T {
  T::default()
}
