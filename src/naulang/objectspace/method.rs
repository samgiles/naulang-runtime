use std::vec;

use naulang::interpreter::frame::Frame;
use naulang::objectspace::primitives::Object;

pub struct MethodObject {
	literals: vec::Vec<Object>,
	bytecodes: vec::Vec<u32>,
	enclosing_frame: Option<Frame>,
	arg_count: u32,
	stack_depth: usize,
	// TODO: SourceMaps
}

impl MethodObject {
	pub fn new(bytecodes: vec::Vec<u32>) -> MethodObject {
		MethodObject {
			bytecodes: bytecodes.clone(),
			literals: vec::Vec::new(),
			enclosing_frame: Option::None,
			arg_count: 0,
			stack_depth: bytecodes.len(),
		}
	}

	pub fn get_bytecode(&self, at_point: usize) -> u32 {
		self.bytecodes[at_point] as u32
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

		assert!(method.get_bytecode(0) == (ByteCode.LOAD_CONST as u32));
	}
}
