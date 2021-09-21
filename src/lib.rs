//! Helpers for dealing with [`Option`]s of collections or other things (like
//! `String`s) that may be empty.
//!
//! See usage examples at [`EmptyIntoNone`] and [`IsEmpty`].
//!
//! [Option]: std::option::Option
//! [EmptyIntoNone]: crate::empty_into_none::EmptyIntoNone
//! [IsEmpty]: crate::is_empty::IsEmpty

pub mod empty_into_none;
pub mod is_empty;

pub use empty_into_none::EmptyIntoNone;
pub use is_empty::IsEmpty;
