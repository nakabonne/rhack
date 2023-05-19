#![warn(
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications,
    clippy::pedantic,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::nursery
)]
// TODO: Remove if you feel like documenting stuff
#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub mod cmd;
pub mod error;
