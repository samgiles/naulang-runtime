mod naulang;

use naulang::objectspace::primitives::{Object, IntegerObject};
use naulang::interpreter::bytecode::Code;


fn main() {
	println!("String: {:?}", Object::Integer(IntegerObject { value: 42 }));
	println!("HALT: {}", Code::LOAD as i32);
    println!("Hello, world!");
}
