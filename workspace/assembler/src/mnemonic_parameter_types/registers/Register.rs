// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A register.
pub trait Register: Copy + Sized + Into<u8>
{
	#[doc(hidden)]
	const IsRegister8Bit: bool = false;
	
	/// Zero-based index of this register.
	#[inline(always)]
	fn index(self) -> u8;
	
	#[doc(hidden)]
	#[inline(always)]
	fn requires_rex_byte(self) -> bool
	{
		Self::IsRegister8Bit && self.index() > 3
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn requires_rex_bit(self) -> bool
	{
		self.index() > 7
	}
}

impl<R: Register> MemoryOrRegister for R
{
	#[inline(always)]
	fn emit_mod_rm_sib(self, byte_emitter: &mut ByteEmitter, reg: impl Register)
	{
		const ModRegisterAddressingMode: u8 = 0b11;
		
		let rm = self;
		let mod_rm_and_sib = (ModRegisterAddressingMode << 6) | ((reg.index() << 3) & 0b0011_1000) | (rm.index() & 0x07);
		byte_emitter.emit_u8(mod_rm_and_sib)
	}
	
	#[inline(always)]
	fn emit_rex_3(self, byte_emitter: &mut ByteEmitter, r: impl Register, mut byte: u8)
	{
		byte |= if r.requires_rex_byte()
		{
			InstructionStream::REX
		}
		else
		{
			0x00
		};
		
		byte |= if r.requires_rex_bit()
		{
			InstructionStream::REX_R
		}
		else
		{
			0x00
		};
		
		self.emit_rex_2(byte_emitter, byte);
	}
	
	#[inline(always)]
	fn emit_rex_2(self, byte_emitter: &mut ByteEmitter, mut byte: u8)
	{
		let rm = self;
		
		byte |= if rm.requires_rex_byte()
		{
			InstructionStream::REX
		}
		else
		{
			0x00
		};
		
		byte |= if rm.requires_rex_bit()
		{
			InstructionStream::REX_B
		}
		else
		{
			0x00
		};
		
		byte_emitter.emit_u8_if_not_zero(byte);
	}
	
	#[inline(always)]
	fn emit_vex_prefix(self, byte_emitter: &mut ByteEmitter, mmmmm: u8, L: u8, pp: u8, w: u8, vvvv: impl Register, r: impl Register)
	{
		let rm = self;
		
		let r_bit = (!r.index() << 4) & 0x80;
		let b_bit = (!rm.index() << 2) & 0x20;
		
		if b_bit == 0x20 && mmmmm == 0x01 && w == 0
		{
			byte_emitter.emit_2_byte_vex_prefix(r_bit, vvvv, L, pp)
		}
		else
		{
			byte_emitter.emit_3_byte_vex_prefix(r_bit, 0x40, b_bit, mmmmm, w, vvvv, L, pp)
		}
	}
}
