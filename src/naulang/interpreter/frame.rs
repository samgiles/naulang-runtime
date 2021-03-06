use std::vec;
use naulang::objectspace::primitives::{Object};
use naulang::objectspace::method::MethodObject;

pub struct FrameInfo<'fi> {
    pub stack_depth: usize,
    pub local_count: usize,
    pub literal_count: usize,
    pub method: &'fi MethodObject<'fi>,
}

#[derive(Clone)]
pub struct Frame<'f> {
    pub previous_frame: Option<Box<Frame<'f>>>,
    pub method: &'f MethodObject<'f>,
    pub pc: usize,

    stack: vec::Vec<Object<'f>>,
    access_link: Option<Box<Frame<'f>>>,
    locals: vec::Vec<Object<'f>>,
}

impl<'f> Frame<'f> {
    pub fn new(frame_info: FrameInfo<'f>) -> Box<Frame<'f>> {
        let mut new_frame = Frame {
            previous_frame: Option::None,
            method: frame_info.method,
            stack: vec::Vec::with_capacity(frame_info.stack_depth),
            pc: 0,
            access_link: Option::None,
            locals: vec::Vec::with_capacity(frame_info.local_count),
        };

        // REVIEW: Populate the locals space with some 'None' objects - to
        // allow random access - maybe we could
        // do this at init time or use 'Vec::get(&self) -> Option<&Object>'
        // and match at retrieval time?
        for _ in 0..frame_info.local_count {
            new_frame.locals.push(Object::None);
        }

        return Box::new(new_frame);
    }

    pub fn pop(&mut self) -> Option<Object<'f>> {
        self.stack.pop()
    }

    pub fn push(&mut self, object: Object<'f>) -> () {
        self.stack.push(object);
    }

    pub fn peek(&self) -> Option<&Object> {
        let stack_top = self.stack.len() - 1;
        self.stack.get(stack_top)
    }

    pub fn stack_height(&self) -> usize {
        self.stack.len()
    }

    pub fn set_local_at(&mut self, index: usize, object: Object<'f>) -> () {
        self.locals[index] = object;
    }

    pub fn get_local_at(&self, index: usize) -> Option<&Object> {
        self.locals.get(index)
    }

    pub fn get_literal_at(&self, index: usize) -> Option<&Object> {
        self.method.literals.get(index)
    }
}


#[cfg(test)]
mod tests {
    use super::{Frame, FrameInfo};
    use naulang::objectspace::primitives::{Object,IntegerObject};
    use naulang::objectspace::method::{MethodObject};
    use naulang::interpreter::bytecode::Bytecode;

    fn extract_referenced_primitive_integer(o: Option<&Object>, default: i32) -> i32 {
        match o {
            Some(object) => match *object {
                Object::Integer(ref i_obj) => i_obj.get_value(),
                _ => default,
            },
            None => default
        }
    }

    fn extract_primitive_integer(o: Option<Object>, default: i32) -> i32 {
        match o {
            Some(object) => extract_referenced_primitive_integer(Some(&object), default),
            None => default
        }
    }

    #[test]
    fn test_set_get_local_at() {
        let literals = vec!();
        let method = MethodObject::new(vec!(Bytecode::HALT), &literals, 0);

        let mut frame = Frame::new(FrameInfo {
            stack_depth: 1,
            local_count: 1,
            literal_count: 1,
            method: &method,
        });
        let integer_object = IntegerObject::new(42);
        frame.set_local_at(0, Object::Integer(integer_object));
        let local = frame.get_local_at(0);
        let internal_value = extract_referenced_primitive_integer(local, 0);

        assert!(internal_value == 42);
    }

    #[test]
    fn test_push_peek_pop() {
        let literals = vec!();
        let method = MethodObject::new(vec!(Bytecode::HALT), &literals, 0);
        let mut frame = Frame::new(FrameInfo {
            stack_depth: 1,
            local_count: 1,
            literal_count: 1,
            method: &method,
        });

        let integer_object = IntegerObject::new(42);
        frame.push(Object::Integer(integer_object));

        assert!(frame.stack_height() == 1);

        // Scoped so the lifetime of borrowed peeked_object is controlled here
        {
            let peeked_object = frame.peek();
            let peeked_value = extract_referenced_primitive_integer(peeked_object, 0);
            assert!(peeked_value == 42);
            assert!(frame.stack_height() == 1);
        }

        let popped_object = frame.pop();
        let popped_value = extract_primitive_integer(popped_object, 0);

        assert!(popped_value == 42);
        assert!(frame.stack_height() == 0);

    }
}
