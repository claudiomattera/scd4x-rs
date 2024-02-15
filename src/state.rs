// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Data types for type-state pattern

/// State for type-state pattern
pub trait State {}

/// Idle state for type-state pattern
pub struct Idle;

/// Measuring state for type-state pattern
pub struct Measuring;

impl State for Idle {}
impl State for Measuring {}
