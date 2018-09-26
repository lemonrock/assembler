// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


macro_rules! impl_memory
{
	($struct_name: tt) =>
	{
		/// Any memory referenced by `MemoryOperand`.
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct $struct_name(pub MemoryOperand);

		impl From<MemoryOperand> for $struct_name
		{
			#[inline(always)]
			fn from(memory_operand: MemoryOperand) -> Self
			{
				$struct_name(memory_operand)
			}
		}
		
		impl Into<MemoryOperand> for $struct_name
		{
			#[inline(always)]
			fn into(self) -> MemoryOperand
			{
				self.memory_operand()
			}
		}
		
		impl Memory for $struct_name
		{
		}
		
		impl HasMemoryOperand for $struct_name
		{
			#[inline(always)]
			fn memory_operand(self) -> MemoryOperand
			{
				self.0
			}
		}

		impl MemoryOrRegister for $struct_name
		{
			#[inline(always)]
			fn emit_mod_rm_sib(self, byte_emitter: &mut ByteEmitter, reg: impl Register)
			{
				self.memory_operand().emit_mod_rm_sib(byte_emitter, reg)
			}
			
			#[inline(always)]
			fn emit_rex_3(self, byte_emitter: &mut ByteEmitter, r: impl Register, byte: u8)
			{
				self.memory_operand().emit_rex_3(byte_emitter, r, byte)
			}
			
			#[inline(always)]
			fn emit_rex_2(self, byte_emitter: &mut ByteEmitter, byte: u8)
			{
				self.memory_operand().emit_rex_2(byte_emitter, byte)
			}
			
			#[inline(always)]
			fn emit_vex_prefix(self, byte_emitter: &mut ByteEmitter, mmmmm: u8, L: u8, pp: u8, w: u8, vvvv: impl Register, r: impl Register)
			{
				self.memory_operand().emit_vex_prefix(byte_emitter, mmmmm, L, pp, w, vvvv, r)
			}
		}
		
		impl PrefixGroup2 for $struct_name
		{
			#[inline(always)]
			fn emit_prefix_group2(self, byte_emitter: &mut ByteEmitter)
			{
				self.memory_operand().emit_prefix_group2(byte_emitter)
			}
		}

	}
}
