// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


use self::Register8Bit::*;
use self::Register16Bit::*;
use self::Register32Bit::*;
use self::Register64Bit::*;
use super::ExecutableAnonymousMemoryMap;
use super::InstructionStreamHints;
use super::mnemonic_parameter_types::immediates::*;
use super::mnemonic_parameter_types::registers::*;
use ::std::io::Write;


#[test]
pub fn lifecycle()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096).expect("Could not anonymously mmap");
	let instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
	
	instruction_stream.finish();
}

#[test]
pub fn labelling()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096).expect("Could not anonymously mmap");
	let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
	
	let label1 = instruction_stream.create_label();
	instruction_stream.attach_label(label1);
	
	let _label2 = instruction_stream.create_and_attach_label();
	
	instruction_stream.finish();
}


#[test]
pub fn simple_function()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096).expect("Could not anonymously mmap");
	
	let _function_pointer =
	{
		let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
		
		instruction_stream.emit_alignment(64);
		
		let function_pointer: unsafe extern "C" fn() -> i32 = instruction_stream.nullary_function_pointer();
		
		instruction_stream.push_stack_frame();
		
		instruction_stream.zero_RAX();
		
		instruction_stream.pop_stack_frame_and_return();
		
		let (encoded_bytes, _hints) = instruction_stream.finish();
		
		assert_eq!(&bytes_to_string(encoded_bytes), "55 48 8B EC 31 C0 48 8B E5 5D C3", "Encoding of a basic function was wrong");
		
		function_pointer
	};
	
	let result = unsafe { _function_pointer() };
	assert_eq!(result, 0, "function result was not zero")
}

#[test]
pub fn validate_that_rust_follows_the_system_v_abi_for_bool()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096).expect("Could not anonymously mmap");
	
	let false_function_pointer =
	{
		let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
		
		let function_pointer: unsafe extern "C" fn() -> bool = instruction_stream.nullary_function_pointer();
		
		instruction_stream.mov_Register64Bit_Immediate64Bit(RAX, 0xFFFFFFFF_FFFFFFFFu64.into());
		instruction_stream.set_RAX_to_c_bool_false();
		instruction_stream.ret();
		
		let _ = instruction_stream.finish();
		
		function_pointer
	};
	
	assert_eq!(unsafe { false_function_pointer() }, false, "function result was not false");
	
	// See AMD64 ABI 1.0 – August 13, 2018 – 8:25, page 22, third-to-last paragraph and footnote 16.
	// In essence, a _Bool should be interpreted only from the bottom 8 bits.
	
	let mut map = ExecutableAnonymousMemoryMap::new(4096).expect("Could not anonymously mmap");
	
	let false_function_pointer =
	{
		let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
		
		let function_pointer: unsafe extern "C" fn() -> bool = instruction_stream.nullary_function_pointer();
		
		instruction_stream.mov_Register64Bit_Immediate64Bit(RAX, 0xFFFFFFFF_FFFFFFFFu64.into());
		instruction_stream.set_RAX_to_c_bool_true();
		instruction_stream.ret();
		
		let _ = instruction_stream.finish();
		
		function_pointer
	};
	
	assert_eq!(unsafe { false_function_pointer() }, true, "function result was not true")
}

#[test]
pub fn bake_off()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096).unwrap();
	let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());

	instruction_stream.mov_Register8Bit_Immediate8Bit(AL, Immediate8Bit::One);
	instruction_stream.mov_Register8Bit_Immediate8Bit_1(AL, Immediate8Bit::One);
	
	instruction_stream.mov_Register16Bit_Immediate16Bit(AX, Immediate16Bit::One);
	instruction_stream.mov_Register16Bit_Immediate16Bit_1(AX, Immediate16Bit::One);
	
	instruction_stream.mov_Register32Bit_Immediate32Bit(EAX, Immediate32Bit::One);
	instruction_stream.mov_Register32Bit_Immediate32Bit_1(EAX, Immediate32Bit::One);
	
	instruction_stream.mov_Register64Bit_Immediate32Bit(RAX, Immediate32Bit::One);
	instruction_stream.mov_Register64Bit_Immediate64Bit(RAX, Immediate64Bit::One);
	
	instruction_stream.mov_Register8Bit_Immediate8Bit(AL, Immediate8Bit::Zero);
	instruction_stream.xor_Register32Bit_Register32Bit(EAX, EAX);

	let (encoded_bytes, _) = instruction_stream.finish();
	println!("{}", bytes_to_string(encoded_bytes))
}

// TODO: Work out how to use the redzone (128-bit; space for 2 registers) for local variable storage (eg for callee-persisted registers): The callee function may use the red zone for storing local variables without the extra overhead of modifying the stack pointer


// Suitable for https://onlinedisassembler.com/odaweb/ .
fn bytes_to_string(encoded_bytes: &[u8]) -> String
{
	let mut string = Vec::with_capacity(encoded_bytes.len() * 3);
	
	let mut after_first = false;
	for byte in encoded_bytes
	{
		if after_first
		{
			write!(string, " ");
		}
		else
		{
			after_first = true
		}
		write!(string, "{:02X}", *byte);
	}
	
	String::from_utf8(string).unwrap()
}
