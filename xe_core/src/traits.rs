use std::any::Any;

use std::str;

pub trait EditorTrait: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> (usize, usize, usize);
}
