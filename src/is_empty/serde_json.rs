extern crate alloc;
use alloc::string::String;

use serde_json::{Map, Value};

use super::IsEmpty;

impl IsEmpty for Map<String, Value> {
    fn is_empty(&self) -> bool {
        Map::is_empty(self)
    }
}
