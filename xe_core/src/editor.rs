use libloading::{Library, Symbol};
use crate::traits::EditorTrait;

type Constructor = unsafe fn() -> *mut dyn EditorTrait;

pub fn load_editor(path: &str) -> (Library, Box<dyn EditorTrait>) {
    let lib = Library::new(path).expect("Failed to load editor dll!");
    debug!("Editor DLL read...");

    unsafe {
        let constructor: Symbol<Constructor> = lib.get(b"new_editor").expect("Failed to get constructor from dll!");
        debug!("Editor constructor found!");
        let boxed_raw = constructor();
        debug!("Raw editor created!");
        let editor = Box::from_raw(boxed_raw);
        debug!("Loaded editor! Version: ({:?})", editor.version());

        (lib, editor)
    }
}
