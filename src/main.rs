mod naulang;

use naulang::interpreter::frame::{Frame, FrameInfo};
use naulang::objectspace::method::{MethodObject};


fn main() {
    let root_method = MethodObject::new_stub();
    let frame = Frame::new(FrameInfo {
        stack_depth: 1,
        local_count: 1,
        literal_count: 0,
        method: &root_method
    });
}
