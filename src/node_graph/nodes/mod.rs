// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Built-in pure node library: terminal constants, precision-preserving
//! arithmetic, and the sanctioned conversion nodes.
//!
//! Pure kernel — no Leptos, no wasm. Construction ties each node *instance*
//! to fixed port types; the engine's transfer verdicts and output-coherence
//! checks then guarantee no implicit precision change at runtime. The only
//! nodes that ever change precision are the conversion nodes in [`convert`].

pub mod arithmetic;
pub mod constant;
pub mod convert;

pub use arithmetic::{
    AddDecimal, AddInteger, DivDecimal, DivInteger, ModInteger, MulDecimal, MulInteger, SubDecimal,
    SubInteger,
};
pub use constant::Constant;
pub use convert::{DecimalToInt, IntToDecimal, IntegerCast, RescaleDecimal};
