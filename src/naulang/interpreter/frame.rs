use naulang::objectspace::primitives::{Object, BooleanObject};
use std::vec;

pub struct Frame {
	stack: vec::Vec<Object>,
	stack_pointer: u32,
	previous_frame: &Frame,
	access_link: &Frame,
	pc: u32,
}

impl Frame {
	pub fn new(stack_size: u32) -> Frame {
		Frame {
			stack: vec::Vec::new(),
			stack_pointer: 0,
		}
	}
}
