//! Myex2
//!
//! Application based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI
#![feature(let_chains)]
#![feature(exit_status_error)]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]
#![feature(bool_to_option)]

pub mod application;
pub mod commands;
pub mod config;
pub mod error;
pub mod prelude;

#[macro_use(defer)]
extern crate scopeguard;
