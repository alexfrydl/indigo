// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! idn_err {
  ($span:expr, $err:expr) => {
    idn::Error::new($span, $err)
  };

  ($span:expr, $($args:tt)*) => {
    idn::err!($span, format!($($args)*))
  };
}

#[macro_export]
macro_rules! idn_abort {
  ($span:expr, $err:expr) => {
    return Err(idn::Error::new($span, $err).into())
  };

  ($span:expr, $($args:tt)*) => {
    idn::abort!($span, format!($($args)*));
  };
}
