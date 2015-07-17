use std::vec;
use naulang::objectspace::method::MethodObject;

#[derive(Clone)]
pub enum Object {
    String(StringObject),
    Integer(IntegerObject),
    Float(FloatObject),
    Boolean(BooleanObject),
    Array(ArrayObject),
    Method(MethodObject),
    None,
}

#[derive(Clone)]
pub struct ArrayObject {
    value: vec::Vec<Object>,
}

#[derive(Clone)]
pub struct BooleanObject {
    value: bool,
}

#[derive(Clone)]
pub struct StringObject {
    value: String,
}

#[derive(Clone)]
pub struct IntegerObject {
    value: i32,
}

impl IntegerObject {
    pub fn new(value: i32) -> IntegerObject {
        IntegerObject {
            value: value,
        }
    }

    pub fn get_value(&self) -> i32 { self.value }
}

#[derive(Clone)]
pub struct FloatObject {
    value: f32,
}
