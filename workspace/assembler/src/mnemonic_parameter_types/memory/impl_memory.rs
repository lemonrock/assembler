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
			#[inline(always)]
			fn displacement(displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::displacement(displacement))
			}
			
			#[inline(always)]
			fn segment_displacement(segment_register: SegmentRegister, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_displacement(segment_register, displacement))
			}
			
			#[inline(always)]
			fn relative_instruction_pointer_relative() -> Self
			{
				$struct_name(MemoryOperand::relative_instruction_pointer_relative())
			}
			
			#[inline(always)]
			fn segment_relative_instruction_pointer_relative(segment_register: SegmentRegister) -> Self
			{
				$struct_name(MemoryOperand::segment_relative_instruction_pointer_relative(segment_register))
			}
			
			#[inline(always)]
			fn relative_instruction_pointer_relative_displacement(displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::relative_instruction_pointer_relative_displacement(displacement))
			}
			
			#[inline(always)]
			fn segment_relative_instruction_pointer_relative_displacement(segment_register: SegmentRegister, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_relative_instruction_pointer_relative_displacement(segment_register, displacement))
			}
			
			#[inline(always)]
			fn base_64(base_64: Register64Bit) -> Self
			{
				$struct_name(MemoryOperand::base_64(base_64))
			}
			
			#[inline(always)]
			fn base_32(base_32: Register32Bit) -> Self
			{
				$struct_name(MemoryOperand::base_32(base_32))
			}
			
			#[inline(always)]
			fn segment_base_64(segment_register: SegmentRegister, base_64: Register64Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_base_64(segment_register, base_64))
			}
			
			#[inline(always)]
			fn segment_base_32(segment_register: SegmentRegister, base_32: Register32Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_base_32(segment_register, base_32))
			}
			
			#[inline(always)]
			fn base_64_displacement(base_64: Register64Bit, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::base_64_displacement(base_64, displacement))
			}
			
			#[inline(always)]
			fn base_32_displacement(base_32: Register32Bit, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::base_32_displacement(base_32, displacement))
			}
			
			#[inline(always)]
			fn segment_base_64_displacement(segment_register: SegmentRegister, base_64: Register64Bit, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_base_64_displacement(segment_register, base_64, displacement))
			}
			
			#[inline(always)]
			fn segment_base_32_displacement(segment_register: SegmentRegister, base_32: Register32Bit, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_base_32_displacement(segment_register, base_32, displacement))
			}
			
			#[inline(always)]
			fn index_64_scale(index_64: Register64Bit, scale: IndexScale) -> Self
			{
				$struct_name(MemoryOperand::index_64_scale(index_64, scale))
			}
			
			#[inline(always)]
			fn index_32_scale(index_32: Register32Bit, scale: IndexScale) -> Self
			{
				$struct_name(MemoryOperand::index_32_scale(index_32, scale))
			}
			
			#[inline(always)]
			fn segment_index_64_scale(segment_register: SegmentRegister, index_64: Register64Bit, scale: IndexScale) -> Self
			{
				$struct_name(MemoryOperand::segment_index_64_scale(segment_register, index_64, scale))
			}
			
			#[inline(always)]
			fn segment_index_32_scale(segment_register: SegmentRegister, index_32: Register32Bit, scale: IndexScale) -> Self
			{
				$struct_name(MemoryOperand::segment_index_32_scale(segment_register, index_32, scale))
			}
			
			#[inline(always)]
			fn index_64_scale_displacement(index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::index_64_scale_displacement(index_64, scale, displacement))
			}
			
			#[inline(always)]
			fn index_32_scale_displacement(index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::index_32_scale_displacement(index_32, scale, displacement))
			}
			
			#[inline(always)]
			fn segment_index_64_scale_displacement(segment_register: SegmentRegister, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_index_64_scale_displacement(segment_register, index_64, scale, displacement))
			}
			
			#[inline(always)]
			fn segment_index_32_scale_displacement(segment_register: SegmentRegister, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_index_32_scale_displacement(segment_register, index_32, scale, displacement))
			}
			
			#[inline(always)]
			fn base_64_index_64_scale(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self
			{
				$struct_name(MemoryOperand::base_64_index_64_scale(base_64, index_64, scale))
			}
			
			#[inline(always)]
			fn base_32_index_32_scale(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self
			{
				$struct_name(MemoryOperand::base_32_index_32_scale(base_32, index_32, scale))
			}
			
			#[inline(always)]
			fn segment_base_64_index_64_scale(segment_register: SegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self
			{
				$struct_name(MemoryOperand::segment_base_64_index_64_scale(segment_register, base_64, index_64, scale))
			}
			
			#[inline(always)]
			fn segment_base_32_index_32_scale(segment_register: SegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self
			{
				$struct_name(MemoryOperand::segment_base_32_index_32_scale(segment_register, base_32, index_32, scale))
			}
			
			#[inline(always)]
			fn base_64_index_64_scale_displacement(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::base_64_index_64_scale_displacement(base_64, index_64, scale, displacement))
			}
			
			#[inline(always)]
			fn base_32_index_32_scale_displacement(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::base_32_index_32_scale_displacement(base_32, index_32, scale, displacement))
			}
			
			#[inline(always)]
			fn segment_base_64_index_64_scale_displacement(segment_register: SegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_base_64_index_64_scale_displacement(segment_register, base_64, index_64, scale, displacement))
			}
			
			#[inline(always)]
			fn segment_base_32_index_32_scale_displacement(segment_register: SegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
			{
				$struct_name(MemoryOperand::segment_base_32_index_32_scale_displacement(segment_register, base_32, index_32, scale, displacement))
			}
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
