// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


use super::ExecutableAnonymousMemoryMap;
use super::mnemonic_parameter_types::registers::Register32Bit;
use super::mnemonic_parameter_types::registers::Register64Bit;
use ::std::io::Write;


#[test]
pub fn lifecycle()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096).expect("Could not anonymously mmap");
	let instruction_stream = map.instruction_stream(128);
	
	instruction_stream.finish();
}

#[test]
pub fn labelling()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096).expect("Could not anonymously mmap");
	let mut instruction_stream = map.instruction_stream(128);
	
	let label1 = instruction_stream.create_label();
	instruction_stream.attach_label(label1);
	
	let _label2 = instruction_stream.create_and_attach_label();
	
	instruction_stream.finish();
}


#[test]
pub fn simple_function()
{
	use self::Register64Bit::*;
	use self::Register32Bit::*;
	
	let mut map = ExecutableAnonymousMemoryMap::new(4096).expect("Could not anonymously mmap");
	
	let _function_pointer =
	{
		let mut instruction_stream = map.instruction_stream(128);
		
		//instruction_stream.emit_alignment(64);
		
		let function_pointer: unsafe extern "C" fn() -> i32 = instruction_stream.nullary_function_pointer();
		
		// Push stack frame.
		instruction_stream.push_Register64Bit_r64(RBP);
		instruction_stream.mov_Register64Bit_Register64Bit_rm64_r64(RBP, RSP);
		
		// Return 0
		instruction_stream.xor_Register32Bit_Register32Bit(EAX, EAX);
		
		// Pop stack frame and return.
		instruction_stream.pop_Register64Bit_r64(RBP);
		instruction_stream.ret();
		
		let encoded_bytes = instruction_stream.finish();
		
		assert_eq!(&bytes_to_string(encoded_bytes), "55 48 8B EC 31 C0 5D C3", "Encoding of a basic function was wrong");
		
		function_pointer
	};
	
	let result = unsafe { _function_pointer() };
	assert_eq!(result, 0, "function result was not zero")
}

// Suitable for https://onlinedisassembler.com/odaweb/ .
fn bytes_to_string(encoded_bytes: &[u8]) -> String
{
	let mut string = Vec::with_capacity(128);
	
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
