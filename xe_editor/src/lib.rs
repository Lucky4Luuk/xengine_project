///Big idea: load editor as dll and load usercode as dll, don't fuck around with dll's outside of that lol

use xe_core::traits::EditorTrait;

use std::str;

#[repr(C)]
#[derive(Debug, Default)]
pub struct Editor;

impl EditorTrait for Editor {
    fn name(&self) -> &'static str {
        "xe_editor"
    }

    fn version(&self) -> (usize, usize, usize) {
        (0,0,1)
    }
}

#[no_mangle]
pub extern "C" fn new_editor() -> *mut dyn EditorTrait {
    let editor = Editor::default();

    let boxed: Box<dyn EditorTrait> = Box::new(editor);

    let raw = Box::into_raw(boxed);

    raw
}
