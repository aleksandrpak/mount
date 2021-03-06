#![crate_name = "mount"]
#![deny(missing_docs)]
#![deny(warnings)]

//! `Mount` provides mounting middleware for the Iron framework.

extern crate iron;
extern crate url;
extern crate sequence_trie;

pub use mount::{Mount, OriginalUrl};

mod mount;
