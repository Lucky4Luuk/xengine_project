use std::any::Any;

pub trait EditorTrait: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> (usize, usize, usize);
}
