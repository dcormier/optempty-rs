//! Helpers for dealing with `Option`s/`Result`s of collections or other things
//! (like `String`s) that may be empty.
//!
//! # Examples
//!
//! Always start by including the traits:
//!
//! ```
//! use optempty::*;
//! ```
//!
//! These examples only show `Vec<T>`, but they support any type that implements
//! [`IsEmpty`].
//!
//! ## `empty_into_none`
//!
//! `Some` with an empty `Vec` becomes `None`.
//! ```
//! use optempty::*;
//!
//! let some: Option<Vec<&str>> = Some(vec![]);
//! let none = some.empty_into_none();
//! assert_eq!(None, none);
//! ```
//!
//! `Some` with a non-empty `Vec` remains unchanged.
//! ```
//! # use optempty::*;
//! #
//! let some = Some(vec!["a", "b", "c"]);
//! let still_some = some.clone().empty_into_none();
//! assert_eq!(some, still_some);
//! ```
//!
//! `None` remains unchanged.
//! ```
//! # use optempty::*;
//! #
//! let none: Option<Vec<&str>> = None;
//! let still_none = none.empty_into_none();
//! assert_eq!(None, still_none);
//! ```
//!
//! ## `empty_into_err`
//!
//! `Ok` with an empty `Vec` becomes `Err`.
//! ```
//! use optempty::*;
//!
//! let ok: Result<Vec<&str>, &str> = Ok(vec![]);
//! let err = ok.empty_into_err(|| "was empty");
//! assert_eq!(Err("was empty"), err);
//! ```
//!
//! `Ok` with a non-empty `Vec` remains unchanged.
//! ```
//! # use optempty::*;
//! #
//! let ok = Ok(vec!["a", "b", "c"]);
//! let still_ok = ok.empty_into_err(|| "was empty");
//! assert_eq!(Ok(vec!["a", "b", "c"]), still_ok);
//! ```
//!
//! `Err` remains unchanged.
//! ```
//! # use optempty::*;
//! #
//! let err: Result<Vec<&str>, &str> = Err("failed");
//! let still_err = err.empty_into_err(|| "was empty");
//! assert_eq!(Err("failed"), still_err);
//! ```
//!
//! See more examples at:
//! * [`IsEmpty`]
//! * [`EmptyIntoNone`]
//! * [`EmptyIntoErr`]
//!
//! # Features
//!
//! Available features are:
//! * `std`
//!   * Adds support for types in `std::collections` in addition to types from `alloc`.
//! * `serdejson`
//!   * Adds support for [`serde_json::Map`]
//!
//! Default features:
//! * `std`
//!
//! [Option]: std::option::Option
//! [IsEmpty]: crate::is_empty::IsEmpty
//! [EmptyIntoNone]: crate::empty_into_none::EmptyIntoNone
//! [EmptyIntoErr]: crate::empty_into_err::EmptyIntoErr

#![deny(warnings)]
#![cfg_attr(not(feature = "use_std"), no_std)]

pub mod empty_into_err;
pub mod empty_into_none;
pub mod is_empty;

pub use empty_into_err::EmptyIntoErr;
pub use empty_into_none::EmptyIntoNone;
pub use is_empty::IsEmpty;
