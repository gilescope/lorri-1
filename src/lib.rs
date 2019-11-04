//! # lorri
//! lorri is a wrapper over Nix to abstract project-specific build
//! configuration and patterns in to a declarative configuration.

#![warn(missing_docs)]
// We usually want to use matches for clarity
#![allow(clippy::match_bool)]
#![allow(clippy::single_match)]

#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;

pub mod bash;
pub mod build_loop;
pub mod builder;
pub mod cas;
pub mod changelog;
pub mod cli;
pub mod constants;
pub mod daemon;
pub mod error;
pub mod locate_file;
pub mod logging;
pub mod nix;
pub mod ops;
pub mod osstrlines;
pub mod pathreduction;
pub mod project;
pub mod run_async;
pub mod socket;
pub mod thread;
pub mod watch;

// This and the following module declaration together publicly export the
// contents of the generated module "org_nixos_lorri" as "proto", which is a
// much nicer module name.
#[allow(missing_docs, clippy::all, unused_imports)]
mod org_nixos_lorri;

#[allow(missing_docs)]
pub mod proto {
    // Code generated from org.nixos.lorri.varlink
    pub use super::org_nixos_lorri::*;
}

// This and the following module declaration together publicly export the
// contents of the generated module "org_nixos_lorri_internal" as
// "internal_proto", which is a much nicer module name.
#[allow(missing_docs, clippy::all, unused_imports)]
mod org_nixos_lorri_internal;

#[allow(missing_docs)]
pub(crate) mod internal_proto {
    // Code generated from org.nixos.lorri.internal.varlink
    pub use super::org_nixos_lorri_internal::*;
}

use std::path::{Path, PathBuf};
use std::ffi::{OsStr};

// OUT_DIR and build_rev.rs are generated by cargo, see ../build.rs
include!(concat!(env!("OUT_DIR"), "/build_rev.rs"));

/// A .nix file.
///
/// Is guaranteed to have an absolute path by construction.
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct NixFile(AbsPathBuf);

/// Path guaranteed to be absolute by construction.
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct AbsPathBuf(PathBuf);

impl AbsPathBuf {
    /// Convert from a known absolute path.
    ///
    /// Passing a relative path is a programming bug (unchecked).
    pub fn new_unchecked(path: PathBuf) -> Self {
        AbsPathBuf(path)
    }

    /// The absolute path, as `&Path`.
    pub fn as_absolute_path(&self) -> &Path {
        &self.0
    }

    /// Proxy through the `Display` class for `PathBuf`.
    pub fn display(&self) -> std::path::Display {
        self.0.display()
    }

    /// Joins a path to the end of this absolute path.
    /// If the path is absolute, it will replace this absolute path.
    pub fn join<P: AsRef<Path>>(&self, pb: P) -> Self {
        let mut new = self.0.to_owned();
        new.push(pb);
        Self::new_unchecked(new)
    }

    /// Proxy through `with_file_name` for `PathBuf`
    pub fn with_file_name<S: AsRef<OsStr>>(&self, file_name: S) -> Self {
        // replacing the file name will never make the path relative
        Self::new_unchecked(self.0.with_file_name(file_name))
    }
}

impl AsRef<Path> for AbsPathBuf {
    fn as_ref(&self) -> &Path {
        self.as_absolute_path()
    }
}

impl NixFile {
    /// Absolute path of this file.
    pub fn as_absolute_path(&self) -> &Path {
        &self.0.as_absolute_path()
    }
}

impl From<AbsPathBuf> for NixFile {
    fn from(abs_path: AbsPathBuf) -> Self {
        NixFile(abs_path)
    }
}

/// Proxy through the `Display` class for `AbsPathBuf`.
impl std::fmt::Display for NixFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}

impl slog::Value for NixFile {
    fn serialize(
        &self,
        _record: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        serializer.emit_arguments(key, &format_args!("{}", self.as_absolute_path().display()))
    }
}


/// A .drv file (generated by `nix-instantiate`).
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct DrvFile(PathBuf);

impl DrvFile {
    /// Underlying `Path`.
    pub fn as_path(&self) -> &Path {
        self.0.as_ref()
    }
}

impl From<PathBuf> for DrvFile {
    fn from(p: PathBuf) -> DrvFile {
        DrvFile(p)
    }
}
