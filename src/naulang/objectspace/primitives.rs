use std::vec;
use naulang::interpreter::frame::Frame;

pub enum Object {
	String(StringObject),
	Integer(IntegerObject),
	Float(FloatObject),
	Boolean(BooleanObject),
	Array(ArrayObject),
	Method(MethodObject),
}

pub struct MethodObject {
	literals: vec::Vec<Object>,
	bytecodes: vec::Vec<u32>,
	arg_count: u32,
	enclosing_frame: Option<Frame>
	stack_depth: u32,
	// TODO: SourceMaps
}

pub struct ArrayObject {
	value: vec::Vec<Object>,
}

pub struct BooleanObject {
	value: bool,
}

pub struct StringObject {
	value: String,
}

pub struct IntegerObject {
	value: i32,
}

pub struct FloatObject {
	value: f32,
}
