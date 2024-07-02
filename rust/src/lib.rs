//! set-git-hooks-dir is a deadly simple crate to manage your Git hooks automatically.
//! See the following document for the usage:
//!
//! https://github.com/rhysd/set-git-hooks-dir

#![forbid(unsafe_code)]

mod installer;

pub use installer::setup;
