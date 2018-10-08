// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


use self::Register64Bit::*;
use super::ExecutableAnonymousMemoryMap;
use super::InstructionStreamHints;
use super::mnemonic_parameter_types::memory::*;
use super::mnemonic_parameter_types::registers::*;
use ::std::io::Write;


#[test]
pub fn lifecycle()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096, false).expect("Could not anonymously mmap");
	let instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
	
	instruction_stream.finish();
}

#[test]
pub fn labelling()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096, false).expect("Could not anonymously mmap");
	let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
	
	let label1 = instruction_stream.create_label();
	instruction_stream.attach_label(label1);
	
	let _label2 = instruction_stream.create_and_attach_label();
	
	instruction_stream.finish();
}

#[test]
pub fn simple_function()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096, false).expect("Could not anonymously mmap");
	
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
	let mut map = ExecutableAnonymousMemoryMap::new(4096, false).expect("Could not anonymously mmap");
	
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
	
	let mut map = ExecutableAnonymousMemoryMap::new(4096, false).expect("Could not anonymously mmap");
	
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
pub fn validate_that_rust_follows_the_system_v_abi_for_u128()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096, false).expect("Could not anonymously mmap");
	
	let u128_function_pointer: unsafe extern "C" fn() -> u128 =
	{
		let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
		
		let function_pointer = instruction_stream.nullary_function_pointer();
		
		instruction_stream.mov_Register64Bit_Immediate64Bit(RAX, 0xFFFFFFFF_FFFFFFFFu64.into());
		instruction_stream.mov_Register64Bit_Immediate64Bit(RDX, 0xAAAAAAAA_AAAAAAAAu64.into());
		instruction_stream.ret();
		
		let _ = instruction_stream.finish();
		
		function_pointer
	};
	
	assert_eq!(unsafe { u128_function_pointer() }, 0xAAAAAAAA_AAAAAAAA_FFFFFFFF_FFFFFFFF, "function result was not as expected for u128");
	
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
	#[repr(C)]
	struct TupleWrapper
	{
		rax: u64,
		rdx: u64,
	}
	
	let u64_u64_function_pointer: unsafe extern "C" fn() -> TupleWrapper =
	{
		let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
		
		let function_pointer = instruction_stream.nullary_function_pointer();
		
		instruction_stream.mov_Register64Bit_Immediate64Bit(RAX, 0xFFFFFFFF_FFFFFFFFu64.into());
		instruction_stream.mov_Register64Bit_Immediate64Bit(RDX, 0xAAAAAAAA_AAAAAAAAu64.into());
		instruction_stream.ret();
		
		let _ = instruction_stream.finish();
		
		function_pointer
	};
	
	assert_eq!(unsafe { u64_u64_function_pointer() }, TupleWrapper { rax: 0xFFFFFFFF_FFFFFFFF, rdx: 0xAAAAAAAA_AAAAAAAA }, "function result was not as expected for (u64, u64)");
	
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
	#[repr(C)]
	struct BoolU64
	{
		rax: bool,
		rdx: usize,
	}
	
	let bool_u64_function_pointer: unsafe extern "C" fn() -> BoolU64 =
	{
		let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
		
		let function_pointer = instruction_stream.nullary_function_pointer();
		
		instruction_stream.mov_Register64Bit_Immediate64Bit(RAX, 0xFFFFFFFF_FFFFFFFFu64.into());
		instruction_stream.set_RAX_to_c_bool_true();
		instruction_stream.mov_Register64Bit_Immediate64Bit(RDX, 0xAAAAAAAA_AAAAAAAAu64.into());
		instruction_stream.ret();
		
		let _ = instruction_stream.finish();
		
		function_pointer
	};
	
	assert_eq!(unsafe { bool_u64_function_pointer() }, BoolU64 { rax: true, rdx: 0xAAAAAAAA_AAAAAAAA }, "function result was not as expected for (bool, u64)");
}

#[test]
pub fn emit()
{
	let mut map = ExecutableAnonymousMemoryMap::new(4096, false).unwrap();
	let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
	
	let offset: usize = 64;
	let displacement = (offset as i32).into();
	let memory = Any8BitMemory::base_64_displacement(Register64Bit::SystemVApplicationBinaryInterface64IntegerFunctionArgument0, displacement);
	instruction_stream.prefetcht0_Any8BitMemory(memory);
	
	instruction_stream.nop();
	instruction_stream.nop();
	let forward_label = instruction_stream.create_label();
	instruction_stream.nop();
	
	instruction_stream.prefetcht0_Any8BitMemory(memory);
	
	instruction_stream.jmp_Label_1(forward_label);
	
	instruction_stream.nop();
	instruction_stream.nop();
	instruction_stream.nop();
	instruction_stream.nop();
	instruction_stream.nop();
	
	let backward_label = instruction_stream.create_and_attach_label();
	instruction_stream.jmp_Label_1(backward_label);
	
	instruction_stream.nop();
	instruction_stream.nop();
	instruction_stream.nop();
	instruction_stream.nop();
	instruction_stream.nop();
	instruction_stream.nop();
	instruction_stream.nop();
	instruction_stream.attach_label(forward_label);
	instruction_stream.nop();
	
	let (encoded_bytes, _) = instruction_stream.finish();
	println!("{}", bytes_to_string(encoded_bytes))
}

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
