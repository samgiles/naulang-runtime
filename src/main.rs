mod naulang;

use naulang::interpreter::frame::{Frame, FrameInfo};


fn main() {
    let frame = Frame::new(&FrameInfo {
        stack_depth: 1,
        local_count: 1,
        literal_count: 0
    });
}
