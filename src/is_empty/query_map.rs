use query_map::QueryMap;

use super::IsEmpty;

impl IsEmpty for QueryMap {
    fn is_empty(&self) -> bool {
        QueryMap::is_empty(self)
    }
}
