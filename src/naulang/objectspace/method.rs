use std::vec;

use naulang::interpreter::frame::{Frame, FrameInfo};
use naulang::objectspace::primitives::Object;

#[derive(Clone)]
pub struct MethodObject {
	literals: vec::Vec<Object>,
	bytecodes: vec::Vec<u32>,
	enclosing_frame: Option<Frame>,
	arg_count: usize,
	pub stack_depth: usize,
	// TODO: SourceMaps
}

impl MethodObject {
	pub fn new(bytecodes: vec::Vec<u32>) -> MethodObject {
		MethodObject {
			bytecodes: bytecodes.clone(),
			literals: vec::Vec::new(),
			enclosing_frame: Option::None,
			arg_count: 0,
			stack_depth: 0,
		}
	}

	pub fn get_bytecode(&self, at_point: usize) -> u32 {
		self.bytecodes[at_point]
	}


	fn create_new_frame(&self, mut previous_frame: Frame, is_async: bool) -> Box<Frame> {
		let mut new_frame = Frame::new(&FrameInfo {
			stack_depth: self.stack_depth,
			local_count: 0,
			literal_count: 0,
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
	use naulang::interpreter::bytecode::ByteCode;

	#[test]
	fn test_get_bytecode() {
		let method: MethodObject = MethodObject::new(vec![
			ByteCode.LOAD_CONST, 0,
		]);

		assert!(method.get_bytecode(0) == ByteCode.LOAD_CONST);
	}
}
