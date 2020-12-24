// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Initializes the log module and sets the default level of the current crate.
#[macro_export]
macro_rules! log_init {
  () => {
    log::init();

    log::set_level_of(
      option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")).replace("-", "_"),
      match cfg!(debug_assertions) {
        true => log::Debug,
        false => log::Info,
      },
    );
  };
}
