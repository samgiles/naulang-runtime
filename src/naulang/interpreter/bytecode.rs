use std::vec;
const _stack_effect_depends_on_args: i32 = -9999;

pub struct Code {
    pub HALT: u32,
    pub LOAD_CONST: u32,
    pub LOAD      : u32,
    pub STORE     : u32,
    pub OR        : u32,
    pub AND       : u32,
    pub EQUAL     : u32,
    pub NOT_EQUAL : u32,
    pub LESS_THAN : u32,
    pub LESS_THAN_EQ   : u32,
    pub GREATER_THAN   : u32,
    pub GREATER_THAN_EQ: u32,
    pub ADD: u32,
    pub SUB: u32,
    pub MUL: u32,
    pub DIV: u32,
    pub NOT: u32,
    pub NEG: u32,
    pub JUMP_IF_FALSE: u32,
    pub JUMP: u32,
    pub PRINT: u32,
    pub INVOKE: u32,
    pub RETURN: u32,
    pub ARRAY_LOAD: u32,
    pub ARRAY_STORE: u32,
    pub STORE_DYNAMIC: u32,
    pub LOAD_DYNAMIC: u32,
    pub INVOKE_GLOBAL: u32,
    pub MOD: u32,
    pub COPY_LOCAL: u32,
    pub DUP: u32,
    pub INVOKE_ASYNC: u32,
    pub CHAN_OUT: u32,
    pub CHAN_IN: u32,
}

impl Code {
    fn get_stack_effect(bytecode: &u32) -> i32 {
        stack_effects[*bytecode as usize]
    }

    fn stack_effect_depends_on_args(bytecode: &u32) -> bool {
        Code::get_stack_effect(bytecode) == _stack_effect_depends_on_args
    }

    fn get_bytecode_length(bytecode: &u32) -> usize {
        bytecode_lengths[*bytecode as usize]
    }

    pub fn calculate_stack_depth(bytecodes: &vec::Vec<u32>) -> usize {
        let mut max_depth: i32 = 0;
        let mut current_depth: i32 = 0;
        let mut index = 0;
        let length = bytecodes.len();

        while index < length {
            let bytecode = &bytecodes[index];

            if Code::stack_effect_depends_on_args(bytecode) {
                current_depth = current_depth + 1;
            } else {
                current_depth = current_depth + Code::get_stack_effect(bytecode);
            }

            if current_depth > max_depth {
                max_depth = current_depth;
            }

            index += Code::get_bytecode_length(bytecode);
        }

        max_depth as usize
    }
}

pub static ByteCode: Code = Code {
    HALT:        0,
    LOAD_CONST : 1,
    LOAD       : 2,
    STORE      : 3,
    OR         : 4,
    AND        : 5,
    EQUAL      : 6,
    NOT_EQUAL  : 7,
    LESS_THAN  : 8,
    LESS_THAN_EQ    : 9,
    GREATER_THAN    : 10,
    GREATER_THAN_EQ : 11,
    ADD : 12,
    SUB : 13,
    MUL : 14,
    DIV : 15,
    NOT : 16,
    NEG : 17,
    JUMP_IF_FALSE : 18,
    JUMP : 19,
    PRINT : 20,
    INVOKE : 21,
    RETURN : 22,
    ARRAY_LOAD : 23,
    ARRAY_STORE : 24,
    STORE_DYNAMIC : 25,
    LOAD_DYNAMIC : 26,
    INVOKE_GLOBAL : 27,
    MOD : 28,
    COPY_LOCAL : 29,
    DUP : 30,
    INVOKE_ASYNC : 31,
    CHAN_OUT : 32,
    CHAN_IN : 33,
};

const stack_effects: [i32; 34]  = [
    0,  // halt
    1,  // load_const
    1,  // load
    -1,  // store
    -1,  // or
    -1,  // and
    -1,  // equal
    -1,  // not_equal
    -1,  // less_than
    -1,  // less_than_eq
    -1,  // greater_than
    -1,  // greater_than_eq
    -1,  // add
    -1,  // sub
    -1,  // mul
    -1,  // div
    0,  // not
    0,  // neg
    -1,  // jump_if_false
    0,  // jump_back
    -1,  // print
    _stack_effect_depends_on_args,  // invoke
    -1,  // return
    -1,  // array_load
    -3,  // array_store
    -1,  // store_dynamic
    1,  // load_dynamic
    _stack_effect_depends_on_args,  // invoke_global
    -1,  // mod
    0,  // copy_local
    1,  // dup
    _stack_effect_depends_on_args,  // invoke async
    0,  // chan out
    -2,  // chan in
];

const bytecode_lengths: [usize; 34] = [
    1,  // halt
    2,  // load_const
    2,  // load
    2,  // store
    1,  // or
    1,  // and
    1,  // equal
    1,  // not_equal
    1,  // less_than
    1,  // less_than_eq
    1,  // greater_than
    1,  // greater_than_eq
    1,  // add
    1,  // sub
    1,  // mul
    1,  // div
    1,  // not
    1,  // neg
    2,  // jump_if_false
    2,  // jump_back
    1,  // print
    1,  // invoke
    1,  // return
    1,  // array_load
    1,  // array_store
    3,  // store_dynamic
    3,  // load_dynamic
    2,  // invoke_global
    1,  // mod
    1,  // copy_local
    1,  // dup
    1,  // invoke async
    1,  // chan out
    1,  // chan in
];

#[cfg(test)]
mod test {
    use super::{ByteCode, Code};

    #[test]
    fn test_calculate_stack_depth() {
        let stack_depth = Code::calculate_stack_depth(&vec!(
            ByteCode.LOAD, 0,         // 1
            ByteCode.LOAD_CONST, 1,   // 1
            ByteCode.ADD,             // -1
            ByteCode.LOAD, 1,         // 1
            ByteCode.ADD,             // -1
            ByteCode.STORE, 1,        // -1
            ByteCode.LOAD, 1,         // 1
            ByteCode.LOAD, 2,         // 1
            ByteCode.LOAD, 3          // 1
        ));

        assert!(stack_depth == 3);
    }
    #[test]
    fn test_calculate_stack_depth2() {
        let stack_depth = Code::calculate_stack_depth(&vec!(
            ByteCode.LOAD_CONST, 0,         // 1
            ByteCode.JUMP_IF_FALSE, 37,     // -1
            ByteCode.LOAD, 0,               // 1
            ByteCode.CHAN_OUT,              // 0
            ByteCode.STORE, 2,              // -1
            ByteCode.LOAD, 2,               // 1
            ByteCode.LOAD_CONST, 1,         // 1
            ByteCode.GREATER_THAN,          // -1
            ByteCode.JUMP_IF_FALSE, 27,     // -1
            ByteCode.LOAD, 1,               // 1
            ByteCode.LOAD_CONST, 2,         // 1
            ByteCode.LOAD, 2,               // 1
            ByteCode.CHAN_OUT,              // 0
            ByteCode.ADD,                   // -1
            ByteCode.CHAN_IN,               // -2
            ByteCode.JUMP, 0,               // 0
            ByteCode.LOAD, 1,               // 1
            ByteCode.LOAD, 2,               // 1
            ByteCode.CHAN_OUT,              // 0
            ByteCode.CHAN_IN,               // -2
            ByteCode.JUMP, 37,              // 0
            ByteCode.JUMP, 0,               // 0
            ByteCode.HALT                   // 0
        ));

        assert!(stack_depth == 3);
    }
}
