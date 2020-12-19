// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Collection types.

#[doc(inline)]
pub use {
  arrayvec::{Array, ArrayString, ArrayVec},
  std::collections::{binary_heap, BinaryHeap},
  std::collections::{btree_map, BTreeMap},
  std::collections::{btree_set, BTreeSet},
  std::collections::{hash_map, HashMap},
  std::collections::{hash_set, HashSet},
  std::collections::{linked_list, LinkedList},
  std::collections::{vec_deque, VecDeque},
};

/// Fixed-capacity vectors and strings provided by the `arrayvec` crate.
///
#[doc(inline)]
pub use arrayvec;

/// Immutable collection types provided by the `im` crate.
///
#[doc(inline)]
pub use im as immutable;
