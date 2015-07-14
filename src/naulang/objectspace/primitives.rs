use std::vec;
use naulang::interpreter::bytecode::ByteCode;

use naulang::objectspace::method::MethodObject;

pub enum Object {
	String(StringObject),
	Integer(IntegerObject),
	Float(FloatObject),
	Boolean(BooleanObject),
	Array(ArrayObject),
	Method(MethodObject),
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
