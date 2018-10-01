// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


pub(crate) trait OpcodeEncoding
{
	#[inline(always)]
	fn emit_2(self, instruction_stream: &mut InstructionStream, opcode1: u8);
	
	#[inline(always)]
	fn emit_3(self, instruction_stream: &mut InstructionStream, opcode1: u8, opcode2: u8);
}

impl<R: Register> OpcodeEncoding for R
{
	#[inline(always)]
	fn emit_2(self, instruction_stream: &mut InstructionStream, opcode1: u8)
	{
		let rcode = self;
		let delta = rcode.index_truncated_to_lowest_3_bits();
		instruction_stream.opcode_1(opcode1 + delta);
	}
	
	#[inline(always)]
	fn emit_3(self, instruction_stream: &mut InstructionStream, opcode1: u8, opcode2: u8)
	{
		let rcode = self;
		let delta = rcode.index_truncated_to_lowest_3_bits();
		instruction_stream.opcode_1(opcode1);
		instruction_stream.opcode_1(opcode2 + delta);
	}
}

impl OpcodeEncoding for u8
{
	#[inline(always)]
	fn emit_2(self, instruction_stream: &mut InstructionStream, opcode1: u8)
	{
		instruction_stream.opcode_1(opcode1);
		instruction_stream.opcode_1(self);
	}
	
	#[inline(always)]
	fn emit_3(self, instruction_stream: &mut InstructionStream, opcode1: u8, opcode2: u8)
	{
		instruction_stream.opcode_1(opcode1);
		instruction_stream.opcode_1(opcode2);
		instruction_stream.opcode_1(self);
	}
}
