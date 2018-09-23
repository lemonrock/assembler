// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Memory or a register.
pub trait MemoryOrRegister
{
	/// Emits Mod.R/B, Scaled Index Byte and related data.
	#[inline(always)]
	fn emit_mod_rm_sib(self, byte_emitter: &mut ByteEmitter, reg: impl Register);
	
	/// Emits REX prefix.
	#[inline(always)]
	fn emit_rex_3(self, byte_emitter: &mut ByteEmitter, r: impl Register, byte: u8);
	
	/// Emits REX prefix.
	#[inline(always)]
	fn emit_rex_2(self, byte_emitter: &mut ByteEmitter, byte: u8);
}
