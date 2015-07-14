
const _stack_effect_depends_on_args: i32 = -9999;

pub enum Code {
	HALT =       0,
	LOAD_CONST = 1,
	LOAD       = 2,
	STORE      = 3,
	OR         = 4,
	AND        = 5,
	EQUAL      = 6,
	NOT_EQUAL  = 7,
	LESS_THAN  = 8,
	LESS_THAN_EQ    = 9,
	GREATER_THAN    = 10,
	GREATER_THAN_EQ = 11,
	ADD = 12,
	SUB = 13,
	MUL = 14,
	DIV = 15,
	NOT = 16,
	NEG = 17,
	JUMP_IF_FALSE = 18,
	JUMP = 19,
	PRINT = 20,
	INVOKE = 21,
	RETURN = 22,
	ARRAY_LOAD = 23,
	ARRAY_STORE = 24,
	STORE_DYNAMIC = 25,
	LOAD_DYNAMIC = 26,
	INVOKE_GLOBAL = 27,
	MOD = 28,
	COPY_LOCAL = 29,
	DUP = 30,
	INVOKE_ASYNC = 31,
	CHAN_OUT = 32,
	CHAN_IN = 33,
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

const lengths: [i32; 34] = [
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
