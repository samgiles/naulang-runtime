use std::vec;
use naulang::objectspace::method::MethodObject;

#[derive(Clone)]
pub enum Object<'o> {
    String(StringObject),
    Integer(IntegerObject),
    Float(FloatObject),
    Boolean(BooleanObject),
    Array(ArrayObject<'o>),
    Method(MethodObject<'o>),
    None,
}

#[derive(Clone)]
pub struct ArrayObject<'a> {
    pub value: vec::Vec<Object<'a>>,
}

#[derive(Clone)]
pub struct BooleanObject {
    pub value: bool,
}

#[derive(Clone)]
pub struct StringObject {
    pub value: String,
}

#[derive(Clone)]
pub struct IntegerObject {
    pub value: i32,
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
    pub value: f32,
}
