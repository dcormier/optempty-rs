//! Helpers for dealing with [`Option`]s of collections.
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
