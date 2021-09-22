//! Helpers for dealing with [`Option`]s of collections or other things (like
//! `String`s) that may be empty.
//!
//! See usage examples at:
//! * [`IsEmpty`]
//! * [`EmptyIntoNone`]
//! * [`EmptyIntoErr`]
//!
//! [Option]: std::option::Option
//! [IsEmpty]: crate::is_empty::IsEmpty
//! [EmptyIntoNone]: crate::empty_into_none::EmptyIntoNone
//! [EmptyIntoErr]: crate::empty_into_err::EmptyIntoErr

pub mod empty_into_err;
pub mod empty_into_none;
pub mod is_empty;

pub use empty_into_err::EmptyIntoErr;
pub use empty_into_none::EmptyIntoNone;
pub use is_empty::IsEmpty;
