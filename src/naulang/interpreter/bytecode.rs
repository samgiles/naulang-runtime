pub mod Bytecode {
    use std::vec;
    const _stack_effect_depends_on_args: i32 = -9999;

    pub const HALT       :u32 = 0;
    pub const LOAD_CONST :u32 = 1;
    pub const LOAD       :u32 = 2;
    pub const STORE      :u32 = 3;
    pub const OR         :u32 = 4;
    pub const AND        :u32 = 5;
    pub const EQUAL      :u32 = 6;
    pub const NOT_EQUAL  :u32 = 7;
    pub const LESS_THAN  :u32 = 8;
    pub const LESS_THAN_EQ    :u32 = 9;
    pub const GREATER_THAN    :u32 = 10;
    pub const GREATER_THAN_EQ :u32 = 11;
    pub const ADD :u32 = 12;
    pub const SUB :u32 = 13;
    pub const MUL :u32 = 14;
    pub const DIV :u32 = 15;
    pub const NOT :u32 = 16;
    pub const NEG :u32 = 17;
    pub const JUMP_IF_FALSE :u32 = 18;
    pub const JUMP :u32 = 19;
    pub const PRINT :u32 = 20;
    pub const INVOKE :u32 = 21;
    pub const RETURN :u32 = 22;
    pub const ARRAY_LOAD :u32 = 23;
    pub const ARRAY_STORE :u32 = 24;
    pub const STORE_DYNAMIC :u32 = 25;
    pub const LOAD_DYNAMIC :u32 = 26;
    pub const INVOKE_GLOBAL :u32 = 27;
    pub const MOD :u32 = 28;
    pub const COPY_LOCAL :u32 = 29;
    pub const DUP :u32 = 30;
    pub const INVOKE_ASYNC :u32 = 31;
    pub const CHAN_OUT :u32 = 32;
    pub const CHAN_IN :u32 = 33;

    fn get_stack_effect(bytecode: &u32) -> i32 {
        stack_effects[*bytecode as usize]
    }

    fn stack_effect_depends_on_args(bytecode: &u32) -> bool {
        get_stack_effect(bytecode) == _stack_effect_depends_on_args
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

            if stack_effect_depends_on_args(bytecode) {
                current_depth = current_depth + 1;
            } else {
                current_depth = current_depth + get_stack_effect(bytecode);
            }

            if current_depth > max_depth {
                max_depth = current_depth;
            }

            index += get_bytecode_length(bytecode);
        }

        max_depth as usize
    }


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
}

#[cfg(test)]
mod test {
    use super::Bytecode;

    #[test]
    fn test_calculate_stack_depth() {
        let stack_depth = Bytecode::calculate_stack_depth(&vec!(
            Bytecode::LOAD, 0,         // 1
            Bytecode::LOAD_CONST, 1,   // 1
            Bytecode::ADD,             // -1
            Bytecode::LOAD, 1,         // 1
            Bytecode::ADD,             // -1
            Bytecode::STORE, 1,        // -1
            Bytecode::LOAD, 1,         // 1
            Bytecode::LOAD, 2,         // 1
            Bytecode::LOAD, 3          // 1
        ));

        assert!(stack_depth == 3);
    }
    #[test]
    fn test_calculate_stack_depth2() {
        let stack_depth = Bytecode::calculate_stack_depth(&vec!(
            Bytecode::LOAD_CONST, 0,         // 1
            Bytecode::JUMP_IF_FALSE, 37,     // -1
            Bytecode::LOAD, 0,               // 1
            Bytecode::CHAN_OUT,              // 0
            Bytecode::STORE, 2,              // -1
            Bytecode::LOAD, 2,               // 1
            Bytecode::LOAD_CONST, 1,         // 1
            Bytecode::GREATER_THAN,          // -1
            Bytecode::JUMP_IF_FALSE, 27,     // -1
            Bytecode::LOAD, 1,               // 1
            Bytecode::LOAD_CONST, 2,         // 1
            Bytecode::LOAD, 2,               // 1
            Bytecode::CHAN_OUT,              // 0
            Bytecode::ADD,                   // -1
            Bytecode::CHAN_IN,               // -2
            Bytecode::JUMP, 0,               // 0
            Bytecode::LOAD, 1,               // 1
            Bytecode::LOAD, 2,               // 1
            Bytecode::CHAN_OUT,              // 0
            Bytecode::CHAN_IN,               // -2
            Bytecode::JUMP, 37,              // 0
            Bytecode::JUMP, 0,               // 0
            Bytecode::HALT                   // 0
        ));

        assert!(stack_depth == 3);
    }
}
