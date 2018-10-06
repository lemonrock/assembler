// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A memory operand.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryOperand
{
	displacement: Immediate32Bit,
	base_register: Option<Register64Bit>,
	index_register: Option<Register64Bit>,
	index_scale: IndexScale,
	segment_register: Option<SegmentRegister>,
	pub(crate) address_override_for_32_bit: bool,
	relative_instruction_pointer_offset: bool
}

impl PrefixGroup2 for MemoryOperand
{
	#[inline(always)]
	fn emit_prefix_group2(self, byte_emitter: &mut ByteEmitter)
	{
		if let Some(segment_register) = self.segment_register
		{
			byte_emitter.emit_prefix_group2_for_segment_register(segment_register)
		}
	}
}

impl MemoryOrRegister for MemoryOperand
{
	#[inline(always)]
	fn emit_mod_rm_sib(self, byte_emitter: &mut ByteEmitter, reg: impl Register)
	{
		let rrr = Self::rrr(reg);
		
		if self.relative_instruction_pointer_offset
		{
			self.emit_mod_rm_sib_for_relative_instruction_pointer_addressing(byte_emitter, rrr)
		}
		else if self.has_base_register()
		{
			self.emit_mod_rm_sib_for_all_other_addressing_modes(byte_emitter, rrr)
		}
		else
		{
			self.emit_mod_rm_sib_if_no_base_register(byte_emitter, rrr)
		}
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
		
		self.emit_rex_2(byte_emitter, byte)
	}
	
	#[inline(always)]
	fn emit_rex_2(self, byte_emitter: &mut ByteEmitter, mut byte: u8)
	{
		let rm = self;
		
		byte |= if rm.has_base_register() && rm.get_base_register().requires_rex_bit()
		{
			InstructionStream::REX_B
		}
		else
		{
			0x00
		};
		
		byte |= if rm.has_index_register() && rm.get_index_register().requires_rex_bit()
		{
			InstructionStream::REX_X
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
		let x_bit = if rm.has_index_register()
		{
			(!rm.get_index_register_index() << 3) & 0x40
		}
		else
		{
			0x40
		};
		
		let b_bit = if rm.has_base_register()
		{
			(!rm.get_base_register_index() << 2) & 0x20
		}
		else
		{
			0x20
		};
		
		if x_bit == 0x40 && b_bit == 0x20 && mmmmm == 0x01 && w == 0
		{
			byte_emitter.emit_2_byte_vex_prefix(r_bit, vvvv, L, pp)
		}
		else
		{
			byte_emitter.emit_3_byte_vex_prefix(r_bit, x_bit, b_bit, mmmmm, w, vvvv, L, pp)
		}
	}
}

impl Memory for MemoryOperand
{
	#[inline(always)]
	fn relative_instruction_pointer_relative() -> Self
	{
		Self::new(Immediate32Bit(0), Self::NoBaseOrIndexRegister, Self::NoBaseOrIndexRegister, IndexScale::x1, Self::NoSegmentRegister, false, true)
	}
	
	#[inline(always)]
	fn relative_instruction_pointer_relative_displacement(displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Self::NoBaseOrIndexRegister, Self::NoBaseOrIndexRegister, IndexScale::x1, Self::NoSegmentRegister, false, true)
	}
	
	#[inline(always)]
	fn displacement(displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Self::NoBaseOrIndexRegister, Self::NoBaseOrIndexRegister, IndexScale::x1, Self::NoSegmentRegister, false, false)
	}
	
	#[inline(always)]
	fn base_64(base_64: Register64Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_64), Self::NoBaseOrIndexRegister, IndexScale::x1, Self::NoSegmentRegister, false, false)
	}
	
	#[inline(always)]
	fn base_64_displacement(base_64: Register64Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_64), Self::NoBaseOrIndexRegister, IndexScale::x1, Self::NoSegmentRegister, false, false)
	}
	
	#[inline(always)]
	fn index_64(index_64: Register64Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Self::NoBaseOrIndexRegister, Some(index_64), IndexScale::x1, Self::NoSegmentRegister, false, false)
	}
	
	#[inline(always)]
	fn index_64_scale(index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Self::NoBaseOrIndexRegister, Some(index_64), scale, Self::NoSegmentRegister, false, false)
	}
	
	#[inline(always)]
	fn index_64_scale_displacement(index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Self::NoBaseOrIndexRegister, Some(index_64), scale, Self::NoSegmentRegister, false, false)
	}
	
	/// If `base_64` is `RBP`, then the SIB byte is followed by four-byte `disp32` field and the addressing mode is absolute.
	///
	/// If `index_64` is `RSP`, then the index is ignored.
	#[inline(always)]
	fn base_64_index_64(base_64: Register64Bit, index_64: Register64Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_64), Some(index_64), IndexScale::x1, Self::NoSegmentRegister, false, false)
	}
	
	/// If `base_64` is `RBP`, then the SIB byte is followed by four-byte `disp32` field and the addressing mode is absolute.
	///
	/// If `index_64` is `RSP`, then the index is ignored.
	#[inline(always)]
	fn base_64_index_64_scale(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_64), Some(index_64), scale, Self::NoSegmentRegister, false, false)
	}
	
	/// If `base_64` is `RBP`, then the SIB byte is followed by four-byte `disp32` field and the addressing mode is absolute.
	///
	/// If `index_64` is `RSP`, then the index is ignored.
	#[inline(always)]
	fn base_64_index_64_scale_displacement(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_64), Some(index_64), scale, Self::NoSegmentRegister, false, false)
	}
	
	#[inline(always)]
	fn base_32(base_32: Register32Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_32), Self::NoBaseOrIndexRegister, IndexScale::x1, Self::NoSegmentRegister, true, false)
	}
	
	#[inline(always)]
	fn base_32_displacement(base_32: Register32Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_32), Self::NoBaseOrIndexRegister, IndexScale::x1, Self::NoSegmentRegister, true, false)
	}
	
	#[inline(always)]
	fn index_32(index_32: Register32Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Self::NoBaseOrIndexRegister, Some(index_32), IndexScale::x1, Self::NoSegmentRegister, true, false)
	}
	
	#[inline(always)]
	fn index_32_scale(index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Self::NoBaseOrIndexRegister, Some(index_32), scale, Self::NoSegmentRegister, true, false)
	}
	
	#[inline(always)]
	fn index_32_scale_displacement(index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Self::NoBaseOrIndexRegister, Some(index_32), scale, Self::NoSegmentRegister, true, false)
	}
	
	#[inline(always)]
	fn base_32_index_32(base_32: Register32Bit, index_32: Register32Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_32), Some(index_32), IndexScale::x1, Self::NoSegmentRegister, true, false)
	}
	
	#[inline(always)]
	fn base_32_index_32_scale(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_32), Some(index_32), scale, Self::NoSegmentRegister, true, false)
	}
	
	#[inline(always)]
	fn base_32_index_32_scale_displacement(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_32), Some(index_32), scale, Self::NoSegmentRegister, true, false)
	}
	
	#[inline(always)]
	fn segment_relative_instruction_pointer_relative(segment_register: SegmentRegister) -> Self
	{
		Self::new(Immediate32Bit(0), Self::NoBaseOrIndexRegister, Self::NoBaseOrIndexRegister, IndexScale::x1, Some(segment_register), false, true)
	}
	
	#[inline(always)]
	fn segment_relative_instruction_pointer_relative_displacement(segment_register: SegmentRegister, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Self::NoBaseOrIndexRegister, Self::NoBaseOrIndexRegister, IndexScale::x1, Some(segment_register), false, true)
	}
	
	#[inline(always)]
	fn segment_displacement(segment_register: SegmentRegister, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Self::NoBaseOrIndexRegister, Self::NoBaseOrIndexRegister, IndexScale::x1, Some(segment_register), false, false)
	}
	
	#[inline(always)]
	fn segment_base_64(segment_register: SegmentRegister, base_64: Register64Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_64), Self::NoBaseOrIndexRegister, IndexScale::x1, Some(segment_register), false, false)
	}
	
	#[inline(always)]
	fn segment_base_64_displacement(segment_register: SegmentRegister, base_64: Register64Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_64), Self::NoBaseOrIndexRegister, IndexScale::x1, Some(segment_register), false, false)
	}
	
	#[inline(always)]
	fn segment_index_64_scale(segment_register: SegmentRegister, index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Self::NoBaseOrIndexRegister, Some(index_64), scale, Some(segment_register), false, false)
	}
	
	#[inline(always)]
	fn segment_index_64_scale_displacement(segment_register: SegmentRegister, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Self::NoBaseOrIndexRegister, Some(index_64), scale, Some(segment_register), false, false)
	}
	
	#[inline(always)]
	fn segment_base_64_index_64_scale_displacement(segment_register: SegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_64), Some(index_64), scale, Some(segment_register), false, false)
	}
	
	#[inline(always)]
	fn segment_base_64_index_64_scale(segment_register: SegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_64), Some(index_64), scale, Some(segment_register), false, false)
	}
	
	#[inline(always)]
	fn segment_base_32(segment_register: SegmentRegister, base_32: Register32Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_32), Self::NoBaseOrIndexRegister, IndexScale::x1, Some(segment_register), true, false)
	}
	
	#[inline(always)]
	fn segment_base_32_displacement(segment_register: SegmentRegister, base_32: Register32Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_32), Self::NoBaseOrIndexRegister, IndexScale::x1, Some(segment_register), true, false)
	}
	
	#[inline(always)]
	fn segment_index_32_scale(segment_register: SegmentRegister, index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Self::NoBaseOrIndexRegister, Some(index_32), scale, Some(segment_register), true, false)
	}
	
	#[inline(always)]
	fn segment_index_32_scale_displacement(segment_register: SegmentRegister, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Self::NoBaseOrIndexRegister, Some(index_32), scale, Some(segment_register), true, false)
	}
	
	#[inline(always)]
	fn segment_base_32_index_32_scale_displacement(segment_register: SegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_32), Some(index_32), scale, Some(segment_register), true, false)
	}
	
	#[inline(always)]
	fn segment_base_32_index_32_scale(segment_register: SegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_32), Some(index_32), scale, Some(segment_register), true, false)
	}
}

impl MemoryOperand
{
	const NoBaseOrIndexRegister: Option<Register64Bit> = None;
	
	const NoSegmentRegister: Option<SegmentRegister> = None;
	
	#[inline(always)]
	fn has_base_register(self) -> bool
	{
		self.base_register.is_some()
	}
	
	#[inline(always)]
	fn get_base_register(self) -> Register64Bit
	{
		self.base_register.unwrap()
	}
	
	#[inline(always)]
	fn get_base_register_index(self) -> u8
	{
		self.get_base_register().index()
	}
	
	#[inline(always)]
	fn has_index_register(self) -> bool
	{
		self.index_register.is_some()
	}
	
	#[inline(always)]
	fn get_index_register(self) -> Register64Bit
	{
		self.index_register.unwrap()
	}
	
	#[inline(always)]
	fn get_index_register_index(self) -> u8
	{
		self.get_index_register().index()
	}
	
	#[inline(always)]
	fn get_displacement(self) -> i32
	{
		self.displacement.into()
	}
	
	#[inline(always)]
	fn new(displacement: Immediate32Bit, base_register: Option<impl GeneralPurposeRegister>, index_register: Option<impl GeneralPurposeRegister>, index_scale: IndexScale, segment_register: Option<SegmentRegister>, address_override_for_32_bit: bool, relative_instruction_pointer_offset: bool) -> Self
	{
		debug_assert_eq!(address_override_for_32_bit && relative_instruction_pointer_offset, false, "address_override_for_32_bit and relative_instruction_pointer_offset can not both be specified");
		
		Self
		{
			displacement,
			base_register: base_register.map(|register| unsafe { transmute(register.index()) }),
			index_register: index_register.map(|register| unsafe { transmute(register.index()) }),
			index_scale,
			segment_register,
			address_override_for_32_bit,
			relative_instruction_pointer_offset,
		}
	}
}

// Implementation details for emitting the Mod.R/M byte, scaled index byte (SIB) and displacement.
impl MemoryOperand
{
	const ModBitsShift: u8 = 6;
	
	const Mod_0b00: u8 = 0b00 << Self::ModBitsShift;
	const Mod_0b01: u8 = 0b01 << Self::ModBitsShift;
	const Mod_0b10: u8 = 0b10 << Self::ModBitsShift;
	pub(crate) const ModRegisterAddressingMode: u8 = 0b11 << Self::ModBitsShift;
	
	const NoIndex: u8 = Self::index_register_shifted_(0b100);
	
	const RegisterRspOrR12: u8 = 0b100;
	
	const RegisterRbpOrR13: u8 = 0b101;
	
	/// Special case for `RIP+disp32` (`disp32` is a 32-bit signed displacement).
	#[inline(always)]
	fn emit_mod_rm_sib_for_relative_instruction_pointer_addressing(self, byte_emitter: &mut ByteEmitter, reg: u8)
	{
		// ModR/M byte.
		byte_emitter.emit_mod_r_m_byte(Self::Mod_0b00, reg, Self::RegisterRbpOrR13);
		
		// No scaled index byte (SIB).
		
		// Displacement.
		Self::emit_displacement_32bit(byte_emitter, self.get_displacement())
	}
	
	/// Special case if there is no base register (uses `RBP` as base implicitly) BUT displacement is always 32-bit - there is no 8-bit optimal encoding of it.
	#[inline(always)]
	fn emit_mod_rm_sib_if_no_base_register(self, byte_emitter: &mut ByteEmitter, reg: u8)
	{
		// ModR/M byte.
		byte_emitter.emit_mod_r_m_byte(Self::Mod_0b00, reg, Self::RegisterRspOrR12);
		
		// Scaled index byte (SIB).
		{
			let (scale, index) = if self.has_index_register()
			{
				(self.index_scale_shifted(), self.index_register_shifted())
			}
			else
			{
				const NoScale: u8 = MemoryOperand::index_scale_shifted_(0b00);
				
				(NoScale, Self::NoIndex)
			};
			
			byte_emitter.emit_scaled_index_byte(scale, index, Self::RegisterRbpOrR13)
		}
		
		// Displacement.
		Self::emit_displacement_32bit(byte_emitter, self.get_displacement())
	}
	
	#[inline(always)]
	fn emit_mod_rm_sib_for_all_other_addressing_modes(self, byte_emitter: &mut ByteEmitter, reg: u8)
	{
		let bbb = self.get_base_register().index_truncated_to_lowest_3_bits();
		
		// Is the base register sitting in the `EIP+disp32` or `RIP+disp32` 'row' of Intel's encoding table (note: said table should be in their architecture manuals but keeps moving around).
		#[inline(always)]
		fn base_register_is_relative_instruction_pointer_plus_displacement_32bit(bbb: u8) -> bool
		{
			bbb == MemoryOperand::RegisterRspOrR12
		}
		
		let (displacement, mod_) = self.displacement_and_mod(bbb);
		
		if self.has_index_register()
		{
			// ModR/M byte.
			byte_emitter.emit_mod_r_m_byte(mod_, reg, Self::RegisterRspOrR12);
			
			// Scaled index byte (SIB).
			{
				let scale = self.index_scale_shifted();
				let index = self.index_register_shifted();
				byte_emitter.emit_scaled_index_byte(scale, index, bbb)
			}
			
			// Displacement.
			Self::emit_displacement_0_8bit_or_32bit(byte_emitter, mod_, displacement)
		}
		else if base_register_is_relative_instruction_pointer_plus_displacement_32bit(bbb)
		{
			// ModR/M byte.
			byte_emitter.emit_mod_r_m_byte(mod_, reg, Self::RegisterRspOrR12);
			
			// Scaled index byte (SIB).
			{
				let scale = self.index_scale_shifted();
				byte_emitter.emit_scaled_index_byte(scale, Self::NoIndex, Self::RegisterRspOrR12)
			}
			
			// Displacement.
			Self::emit_displacement_0_8bit_or_32bit(byte_emitter, mod_, displacement)
		}
		else
		{
			// ModR/M byte.
			byte_emitter.emit_mod_r_m_byte(mod_, reg, bbb);
			
			// No scaled index byte (SIB).
			
			// Displacement.
			Self::emit_displacement_0_8bit_or_32bit(byte_emitter, mod_, displacement)
		}
	}
	
	#[inline(always)]
	fn emit_displacement_8bit(byte_emitter: &mut ByteEmitter, displacement: i32)
	{
		Immediate8Bit(displacement as i8).displacement().emit(byte_emitter)
	}
	
	#[inline(always)]
	fn emit_displacement_32bit(byte_emitter: &mut ByteEmitter, displacement: i32)
	{
		Immediate32Bit(displacement).displacement().emit(byte_emitter)
	}
	
	#[inline(always)]
	fn emit_displacement_0_8bit_or_32bit(byte_emitter: &mut ByteEmitter, mod_: u8, displacement: i32)
	{
		if mod_ == Self::Mod_0b01
		{
			Self::emit_displacement_8bit(byte_emitter, displacement)
		}
		else if mod_ == Self::Mod_0b10
		{
			Self::emit_displacement_32bit(byte_emitter, displacement)
		}
	}
	
	#[inline(always)]
	fn index_register_shifted(self) -> u8
	{
		Self::index_register_shifted_(self.get_index_register_index())
	}
	
	#[inline(always)]
	const fn index_register_shifted_(index_register_index: u8) -> u8
	{
		const ScaleRegisterBitsMask: u8 = 0b0011_1000;
		const ScaleRegisterBitsShift: u8 = 3;
		
		(index_register_index << ScaleRegisterBitsShift) & ScaleRegisterBitsMask
	}
	
	#[inline(always)]
	fn index_scale_shifted(self) -> u8
	{
		Self::index_scale_shifted_(self.index_scale.into())
	}
	
	#[inline(always)]
	const fn index_scale_shifted_(index_scale: u8) -> u8
	{
		const ScaleBitsMask: u8 = 0b1100_0000;
		const ScaleShift: u8 = 6;
		
		(index_scale << ScaleShift) & ScaleBitsMask
	}
	
	#[inline(always)]
	fn displacement_and_mod(self, bbb: u8) -> (i32, u8)
	{
		#[inline(always)]
		fn bbb_is_not_RBP_or_R13(bbb: u8) -> bool
		{
			bbb != MemoryOperand::RegisterRbpOrR13
		}
		
		// This logic determines what the value of the mod bits will be.
		// It also controls how many immediate bytes we emit later.
		let displacement = self.get_displacement();
		let mod_ = if displacement < -128 || displacement > 127
		{
			Self::Mod_0b10
		}
		else if displacement == 0 && bbb_is_not_RBP_or_R13(bbb)
		{
			Self::Mod_0b00
		}
		else
		{
			Self::Mod_0b01
		};
		(displacement, mod_)
	}
	
	#[inline(always)]
	pub(crate) fn rrr(reg: impl Register) -> u8
	{
		const ModRMRegisterBitsMask: u8 = 0b0011_1000;
		const ModRMRegisterBitsShift: u8 = 3;
		
		(reg.index() << ModRMRegisterBitsShift) & ModRMRegisterBitsMask
	}
}
