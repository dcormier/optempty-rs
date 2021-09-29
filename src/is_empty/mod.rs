mod no_std;
#[cfg(feature = "serdejson")]
mod serde_json;
#[cfg(feature = "std")]
mod std;

pub use self::no_std::*;
#[cfg(feature = "serdejson")]
pub use self::serde_json::*;
#[cfg(feature = "std")]
pub use self::std::*;

/// Used to determine if a collection, or `Option<T>` or `Result<T, E>` (where
/// `T` implements `IsEmpty`) is empty.
///
/// `IsEmpty` is implemented for the standard collections, and more.
pub trait IsEmpty {
    /// Returns `true` if it is empty.
    fn is_empty(&self) -> bool;
}
