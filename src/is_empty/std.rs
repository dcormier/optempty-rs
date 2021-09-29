extern crate std;
use std::collections::{HashMap, HashSet};

/// Just wraps the existing `is_empty(&self)` method on the type.
// Because you can't spell `simple` without `impl`.
macro_rules! simple_is_empty {
    ($type:ty) => {
        impl $crate::is_empty::IsEmpty for $type {
            fn is_empty(&self) -> bool {
                <$type>::is_empty(self)
            }
        }
    };
    ($type:ident; $($args:tt)*) => {
        impl<$($args)*> $crate::is_empty::IsEmpty for $type<$($args)*> {
            fn is_empty(&self) -> bool {
                $type::is_empty(self)
            }
        }
    };
}

// Implement `IsEmpty` for the std collections.

simple_is_empty!(HashMap; K, V);
simple_is_empty!(HashSet; T);

// Some other things from the stdlib.
// TODO: Implement more from here: https://doc.rust-lang.org/std/?search=is_empty
