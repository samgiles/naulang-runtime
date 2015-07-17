mod naulang;

use naulang::interpreter::frame::{Frame, FrameInfo};
use naulang::interpreter::bytecode::Bytecode;
use naulang::objectspace::method::{MethodObject};


fn main() {
    let literals = vec!();
    let root_method = MethodObject::new(vec!(Bytecode::HALT), &literals, 0);
    let frame = Frame::new(FrameInfo {
        stack_depth: 1,
        local_count: 1,
        literal_count: 0,
        method: &root_method
    });
}
