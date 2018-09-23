// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A register.
pub trait Register: MemoryOrRegister
{
	/// Index.
	#[inline(always)]
	fn index(self) -> u8;
}

impl<R: Register> MemoryOrRegister for R
{
	#[inline(always)]
	fn emit(self, byte_emitter: &mut ByteEmitter, reg: impl Register)
	{
		const ModRegisterAddressingMode: u8 = 0b11;
		
		let rm = self;
		let mod_rm_and_sib = (ModRegisterAddressingMode << 6) | ((reg.index() << 3) & 0b0011_1000) | (rm.index() & 0x07);
		byte_emitter.emit_u8(mod_rm_and_sib)
	}
}
