#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/main/rustsec-logo-lg.png")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[macro_use]
mod error;

pub mod advisory;
mod collection;
pub mod database;
mod fixer;
pub mod osv;
pub mod report;
pub mod repository;
mod vulnerability;
mod warning;

// enabled unconditionally due to being able to name the type being useful
// and this type alone not pulling in any additional dependencies
pub mod binary_format;

#[cfg(feature = "binary-scanning")]
pub mod binary_deps;
#[cfg(feature = "binary-scanning")]
pub mod binary_type_filter;

#[cfg(feature = "git")]
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
mod cached_index;

#[cfg(feature = "git")]
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
pub mod registry {
    //! Support for interacting with the local crates.io registry index
    pub use super::cached_index::{CachedIndex, ClientBuilder};
}

pub use cargo_lock::{self, package, Lockfile, SourceId};
pub use fs_err as fs;
pub use platforms;
pub use semver::{self, Version, VersionReq};

pub use crate::{
    advisory::Advisory,
    collection::Collection,
    database::Database,
    error::{Error, ErrorKind, Result},
    report::Report,
    vulnerability::Vulnerability,
    warning::{Warning, WarningKind},
};

pub use crate::fixer::Fixer;

#[cfg(feature = "git")]
pub use crate::repository::git::Repository;

// Use BTreeMap and BTreeSet as our map and set types
use std::collections::{btree_map as map, btree_set as set, BTreeMap as Map, BTreeSet as Set};

/// Current version of the RustSec crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
