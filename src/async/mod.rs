// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Data types and functions for asynchronous SCD4x sensor interface

mod command;
use self::command::Command;

mod commands;

mod sensor;
pub use self::sensor::Scd4x;
