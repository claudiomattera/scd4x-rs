// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use self::r#async::Scd4x as AsyncScd4x;

#[cfg(feature = "blocking")]
mod blocking;
#[cfg(feature = "blocking")]
pub use self::blocking::Scd4x;

#[cfg(any(feature = "async", feature = "blocking"))]
mod checksum;
#[cfg(any(feature = "async", feature = "blocking"))]
use self::checksum::compute as compute_checksum;
#[cfg(any(feature = "async", feature = "blocking"))]
use self::checksum::verify as verify_checksum;

#[cfg(any(feature = "async", feature = "blocking"))]
mod constants;
#[cfg(any(feature = "async", feature = "blocking"))]
pub use self::constants::DEFAULT_ADDRESS;

#[cfg(any(feature = "async", feature = "blocking"))]
mod conversion;

#[cfg(any(feature = "async", feature = "blocking"))]
mod error;
#[cfg(any(feature = "async", feature = "blocking"))]
pub use self::error::Error;

#[cfg(any(feature = "async", feature = "blocking"))]
mod sample;
#[cfg(any(feature = "async", feature = "blocking"))]
pub use self::sample::{Altitude, Co2, Humidity, Pressure, Sample, Temperature};

#[cfg(any(feature = "async", feature = "blocking"))]
mod util;

#[cfg(any(feature = "async", feature = "blocking"))]
mod state;
#[cfg(any(feature = "async", feature = "blocking"))]
pub use self::state::{Idle, Measuring, State};
