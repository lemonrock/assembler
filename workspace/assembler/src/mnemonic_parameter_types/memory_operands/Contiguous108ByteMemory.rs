// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An 'array' of 108 bytes in memory referenced by `MemoryOperand`.
///
/// Used only by x87 Floating Point Unit (FPU) instructions.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Contiguous108ByteMemory(pub MemoryOperand);

impl MemoryOrRegister for Contiguous108ByteMemory
{
	#[inline(always)]
	fn emit_mod_rm_sib(self, byte_emitter: &mut ByteEmitter, reg: impl Register)
	{
		self.0.emit_mod_rm_sib(byte_emitter, reg)
	}
	
	#[inline(always)]
	fn emit_rex_3(self, byte_emitter: &mut ByteEmitter, r: impl Register, byte: u8)
	{
		self.0.emit_rex_3(byte_emitter, r, byte)
	}
	
	#[inline(always)]
	fn emit_rex_2(self, byte_emitter: &mut ByteEmitter, byte: u8)
	{
		self.0.emit_rex_2(byte_emitter, byte)
	}
}

impl MemoryOrBranchHint for Contiguous108ByteMemory
{
	#[inline(always)]
	fn emit_prefix_group2(self, byte_emitter: &mut ByteEmitter)
	{
		self.0.emit_prefix_group2(byte_emitter)
	}
}
