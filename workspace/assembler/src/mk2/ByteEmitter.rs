// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ByteEmitter
{
	start_instruction_point: usize,
	instruction_pointer: usize,
	end_instruction_pointer: usize,
}

impl ByteEmitter
{
	#[inline(always)]
	pub(crate) fn emit_u8(&mut self, emit: u8)
	{
		const Size: usize = 1;
		debug_assert!(self.instruction_pointer + Size <= self.end_instruction_pointer, "Not enough space to emit an u8");
		unsafe { *(self.instruction_pointer as *mut u8) } = emit;
		self.instruction_pointer += Size;
	}
	
	#[inline(always)]
	pub(crate) fn emit_u16(&mut self, emit: u16)
	{
		const Size: usize = 2;
		debug_assert!(self.instruction_pointer + Size <= self.end_instruction_pointer, "Not enough space to emit an u16");
		unsafe { *(self.instruction_pointer as *mut u16) } = emit;
		self.instruction_pointer += Size;
	}
	
	#[inline(always)]
	pub(crate) fn emit_u32(&mut self, emit: u32)
	{
		const Size: usize = 4;
		debug_assert!(self.instruction_pointer + Size <= self.end_instruction_pointer, "Not enough space to emit an u32");
		unsafe { *(self.instruction_pointer as *mut u32) } = emit;
		self.instruction_pointer += Size;
	}
	
	#[inline(always)]
	pub(crate) fn emit_u64(&mut self, emit: u64)
	{
		const Size: usize = 8;
		debug_assert!(self.instruction_pointer + Size <= self.end_instruction_pointer, "Not enough space to emit an u64");
		unsafe { *(self.instruction_pointer as *mut u64) } = emit;
		self.instruction_pointer += Size;
	}
}
