use naulang::objectspace::primitives::{Object, BooleanObject};
use std::vec;

pub struct Frame {
	stack: vec::Vec<Object>,
	stack_pointer: u32,
	previous_frame: Option<Box<Frame>>,
	access_link: Option<Box<Frame>>,
	pc: u32,
}

impl Frame {
	pub fn new(stack_size: u32) -> Box<Frame> {
		Box::new(Frame {
			stack: vec::Vec::new(),
			stack_pointer: 0,
			pc: 0,
			access_link: Option::None,
			previous_frame: Option::None
		})
	}
}
