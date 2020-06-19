//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! Tranquility is an asynchronous Rust library for the Reddit API.
//!
//! It is currently under heavy development, and as such it is not recommended for production or any other use.
//!
//!
//! [ci]: https://github.com/Elinvynia/tranquility/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/tranquility/Rust/master?style=flat-square
//! [docs]: https://docs.rs/tranquility
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/tranquility
//! [crate-version]: https://img.shields.io/crates/v/tranquility.svg?style=flat-square

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![forbid(unsafe_code)]
#![doc(html_favicon_url = "https://i.imgur.com/weLfw63.png")]
#![doc(html_logo_url = "https://i.imgur.com/weLfw63.png")]

pub mod auth;
pub mod client;
pub mod error;
pub mod model;
pub mod prelude;
