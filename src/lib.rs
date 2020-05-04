#![feature(external_doc)]
#![doc(include = "../README.md")]

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
