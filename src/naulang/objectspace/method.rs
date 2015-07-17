use std::vec;

use naulang::interpreter::frame::{Frame, FrameInfo};
use naulang::objectspace::primitives::Object;
use naulang::interpreter::bytecode::Bytecode;

#[derive(Clone)]
pub struct MethodObject<'m> {
    pub literals: &'m vec::Vec<Object<'m>>,
    pub stack_depth: usize,
    bytecodes: vec::Vec<u32>,
    arg_count: usize,
    // TODO: SourceMaps
}

impl<'m> MethodObject<'m> {

    pub fn new(
        bytecodes: vec::Vec<u32>, literals: &'m vec::Vec<Object<'m>>,  arg_count: usize)
    -> MethodObject<'m> {

        let stack_depth = Bytecode::calculate_stack_depth(&bytecodes);
        MethodObject {
            bytecodes: bytecodes.clone(),
            literals: literals,
            arg_count: arg_count,
            stack_depth: stack_depth,
        }
    }

    pub fn get_bytecode(&self, at_point: usize) -> u32 {
        self.bytecodes[at_point]
    }


    fn create_new_frame<'a>(&'a self, mut previous_frame: Frame<'a>, is_async: bool) -> Box<Frame> {
        let mut new_frame = Frame::new(FrameInfo {
            stack_depth: self.stack_depth,
            local_count: 0,
            literal_count: 0,
            method: self,
        });

        for index in (self.arg_count - 1)..0 {
            match previous_frame.pop() {
                Some(object) => new_frame.set_local_at(index, object),
                None => ()
            }
        }

        return new_frame;
    }
}

#[cfg(test)]
mod tests {
    use super::MethodObject;
    use naulang::interpreter::bytecode::Bytecode;

    #[test]
    fn test_get_bytecode() {
        let literals = vec!();
        let method: MethodObject = MethodObject::new(vec![
            Bytecode::LOAD_CONST, 0,
        ], &literals, 0);

        assert!(method.get_bytecode(0) == Bytecode::LOAD_CONST);
    }
}
