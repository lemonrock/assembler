// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Any 16-bit (two bytes) of memory referenced by `MemoryOperand`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Any16BitMemory(pub MemoryOperand);

impl MemoryOrRegister for Any16BitMemory
{
	#[inline(always)]
	fn emit(self, byte_emitter: &mut ByteEmitter, reg: impl Register)
	{
		self.0.emit(byte_emitter, reg)
	}
}
