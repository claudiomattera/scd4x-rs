// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Logging macros
//!
//! If the feature `log` is enabled, then logging is done with macros from
//! crate [`log`](https://crates.io/crates/log).
//!
//! If the feature `defmt` is enabled, then logging is done with macros from
//! crate [`defmt`](https://crates.io/crates/defmt).
//!
//! If neither features are enabled, logging is done with no-op macros,
//! effectively disabling logging.
//!
//! If both features are enabled, logging is done with macros from both crates.
//! This is probably undesired, but it makes features additive.

// Just log

#[cfg(all(feature = "log", not(feature = "defmt")))]
pub(crate) use log::debug;

#[cfg(all(feature = "log", not(feature = "defmt")))]
pub(crate) use log::trace;

// Just defmt

#[cfg(all(feature = "defmt", not(feature = "log")))]
pub(crate) use defmt::debug;

#[cfg(all(feature = "defmt", not(feature = "log")))]
pub(crate) use defmt::trace;

// Neither log not defmt, logging is a no-op

#[cfg(not(any(feature = "log", feature = "defmt")))]
/// No-op debug macro
macro_rules! debug {
    ($($arg:tt)+) => {};
}

#[cfg(not(any(feature = "log", feature = "defmt")))]
pub(crate) use debug;

#[cfg(not(any(feature = "log", feature = "defmt")))]
/// No-op trace macro
macro_rules! trace {
    ($($arg:tt)+) => {};
}

#[cfg(not(any(feature = "log", feature = "defmt")))]
pub(crate) use trace;

// Both log and defmt, use both

#[cfg(all(feature = "log", feature = "defmt"))]
/// Both debug macro
macro_rules! debug {
    ($($arg:tt)+) => {
        ::log::debug!($($arg)+);
        ::defmt::debug!($($arg)+)
    };
}

#[cfg(all(feature = "log", feature = "defmt"))]
pub(crate) use debug;

#[cfg(all(feature = "log", feature = "defmt"))]
/// Both trace macro
macro_rules! trace {
    ($($arg:tt)+) => {
        ::log::trace!($($arg)+);
        ::defmt::trace!($($arg)+)
    };
}

#[cfg(all(feature = "log", feature = "defmt"))]
pub(crate) use trace;
