// Copyright 2023 Martin Pool.

//! A generic interface for running a tool such as Cargo that can determine the tree
//! shape, build it, and run tests.
//!
//! At present only Cargo is supported, but this interface aims to leave a place to
//! support for example Bazel in future.

use std::fmt;

use camino::{Utf8Path, Utf8PathBuf};

use crate::Result;

pub trait Tool: fmt::Debug {
    /// Find the root of the package enclosing a given path.
    ///
    /// The root is the enclosing directory that needs to be copied to make a self-contained
    /// scratch directory, and from where source discovery begins.
    fn find_root(&self, path: &Utf8Path) -> Result<Utf8PathBuf>;
}
