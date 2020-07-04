use std::any::Any;

pub type EngineConstructor = unsafe fn() -> *mut dyn EngineCoreTrait;

pub trait EngineCoreTrait: Any {
    fn name(&self) -> &str;
    fn version(&self) -> (usize, usize, usize);
}

/*
let library_path = "../target/debug/xe_core.dll";
let (engine, _lib) = unsafe { load_engine(library_path) };

pub unsafe fn load_engine(library_path: &str) -> (Box<dyn EngineCoreTrait>, Library) {
    let lib = Library::new(library_path).expect("Failed to load engine core!");

    let constructor: Symbol<EngineConstructor> = lib.get(b"get_trait_obj").expect("Failed to load engine constructor!");
    let boxed_raw = constructor();

    let engine = Box::from_raw(boxed_raw);

    (engine, lib)
}
*/
