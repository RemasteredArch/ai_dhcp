// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2026 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

/// Instantiate a static cell and take its value, returning a mutable reference.
///
/// From <https://www.darrik.dev/writing/blinking-pico-w-onboard-led-rust/>.
macro_rules! singleton {
    ($val:expr, $ty:ty) => {{
        static SINGLETON_CELL: ::static_cell::StaticCell<$ty> = ::static_cell::StaticCell::new();
        SINGLETON_CELL.init_with(move || $val)
    }};
}
