
const _stack_effect_depends_on_args: i32 = -9999;

pub struct Code {
	pub	HALT: u32,
	pub	LOAD_CONST: u32,
	pub	LOAD      : u32,
	pub	STORE     : u32,
	pub	OR        : u32,
	pub	AND       : u32,
	pub	EQUAL     : u32,
	pub	NOT_EQUAL : u32,
	pub	LESS_THAN : u32,
	pub	LESS_THAN_EQ   : u32,
	pub	GREATER_THAN   : u32,
	pub	GREATER_THAN_EQ: u32,
	pub	ADD: u32,
	pub	SUB: u32,
	pub	MUL: u32,
	pub	DIV: u32,
	pub	NOT: u32,
	pub	NEG: u32,
	pub	JUMP_IF_FALSE: u32,
	pub	JUMP: u32,
	pub	PRINT: u32,
	pub	INVOKE: u32,
	pub	RETURN: u32,
	pub	ARRAY_LOAD: u32,
	pub	ARRAY_STORE: u32,
	pub	STORE_DYNAMIC: u32,
	pub	LOAD_DYNAMIC: u32,
	pub	INVOKE_GLOBAL: u32,
	pub	MOD: u32,
	pub	COPY_LOCAL: u32,
	pub	DUP: u32,
	pub	INVOKE_ASYNC: u32,
	pub	CHAN_OUT: u32,
	pub	CHAN_IN: u32,
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
