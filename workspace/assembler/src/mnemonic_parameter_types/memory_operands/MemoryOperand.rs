// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A memory operand.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryOperand(u64);

impl MemoryOrBranchHint for MemoryOperand
{
	#[inline(always)]
	fn emit_prefix_group2(self, byte_emitter: &mut ByteEmitter)
	{
		if self.has_segment_register()
		{
			let segment_register_byte = match self.get_segment_register_index()
			{
				0 => 0x26,
				1 => 0x2E,
				2 => 0x36,
				3 => 0x3E,
				4 => 0x64,
				5 => 0x65,
				_ => unreachable!(),
			};
			byte_emitter.emit_u8(segment_register_byte)
		}
	}
}

impl MemoryOrRegister for MemoryOperand
{
	#[inline(always)]
	fn emit_mod_rm_sib(self, byte_emitter: &mut ByteEmitter, reg: impl Register)
	{
		const ScaleBitsMask: u8 = 0b1100_0000;
		const ScaleShift: u8 = 6;
		const MidBitsMask: u8 = 0b0011_1000;
		const MidBitsShift: u8 = 3;
		const Mod40: u8 = 0x40;
		const Mod80: u8 = 0x80;
		
		// Every path we take needs these bits for the Mod.r/m byte.
		let rrr = (reg.index() << MidBitsShift) & 0b0011_1000;
		
		// Special case for `RIP+disp32` (`disp32` is a 32-bit signed displacement).
		if self.has_relative_instruction_pointer_offset()
		{
			let mod_byte = 0x00 | rrr | 0x05;
			byte_emitter.emit_u8(mod_byte);
			
			self.get_displacement().displacement().emit(byte_emitter);
			return
		}
		
		// Special case if there is no base register.
		if !self.has_base_register()
		{
			let mod_byte = 0x00 | rrr | 0x04;
			byte_emitter.emit_u8(mod_byte);
			
			let scaled_index_byte = if self.has_index_register()
			{
				((self.get_index_scale() << ScaleShift) & ScaleBitsMask)
				| ((self.get_index_register_index() << MidBitsShift) & MidBitsMask)
				| 0x05
			}
			else
			{
				0x00 | 0x20 | 0x05
			};
			byte_emitter.emit_u8(scaled_index_byte);
			
			self.get_displacement().displacement().emit(byte_emitter);
			return
		}
		
		let bbb = self.get_base_register_index() & 0x07;
		
		// This logic determines what the value of the mod bits will be.
		// It also controls how many immediate bytes we emit later.
		let displacement = self.get_displacement().0;
		let mod_ = if displacement < -128 || displacement >= 128
		{
			Mod80
		}
		else if displacement == 0 && bbb != 0x05
		{
			0x00
		}
		else
		{
			Mod40
		};
		
		if self.has_index_register()
		{
			let mod_byte = mod_ | rrr | 0x04;
			byte_emitter.emit_u8(mod_byte);
			
			let scaled_index_byte = ((self.get_index_scale() << ScaleShift) & ScaleBitsMask) | ((self.get_index_register_index() << MidBitsShift) & MidBitsMask) | bbb;
			byte_emitter.emit_u8(scaled_index_byte);
		}
		// Is the base register sitting in the `EIP+disp32` or `RIP+disp32` (where `disp32` is a 32-bit displacement) 'row' of Intel's encoding table?
		else if bbb == 0x04
		{
			let mod_byte = mod_ | rrr | 0x04;
			byte_emitter.emit_u8(mod_byte);
			
			let scaled_index_byte = ((self.get_index_scale() << ScaleShift) & ScaleBitsMask) | 0x20 | 0x04;
			byte_emitter.emit_u8(scaled_index_byte);
		}
		else
		{
			let mod_byte = mod_ | rrr | bbb;
			byte_emitter.emit_u8(mod_byte);
		}
		
		// Write displacement if it is not zero.
		if mod_ == Mod40
		{
			Immediate8Bit(displacement as i8).displacement().emit(byte_emitter)
		}
		else if mod_ == Mod80
		{
			Immediate32Bit(displacement).displacement().emit(byte_emitter)
		}
	}
	
	#[inline(always)]
	fn emit_rex_3(self, byte_emitter: &mut ByteEmitter, r: impl Register, mut byte: u8)
	{
		byte |= if r.requires_rex_byte()
		{
			OrdinaryInstructionStream::REX
		}
		else
		{
			0x00
		};
		
		byte |= if r.requires_rex_bit()
		{
			OrdinaryInstructionStream::REX_R
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
			OrdinaryInstructionStream::REX_B
		}
		else
		{
			0x00
		};
		
		byte |= if rm.has_index_register() && rm.get_index_register().requires_rex_bit()
		{
			OrdinaryInstructionStream::REX_X
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

impl MemoryOperand
{
	const DisplacementMask: u64 = 0x00000000FFFFFFFF;
	
	const BaseRegisterMask: u64 = 0x0000001F00000000;
	
	const IndexRegisterMask: u64 = 0x00001F0000000000;
	
	const IndexScaleMask: u64 = 0x0003000000000000;
	
	const SegmentRegisterMask: u64 = 0x0700000000000000;
	
	const AddressOverrideFor32BitMask: u64 = 0x1000000000000000;
	
	const RelativeInstructionPointerOffsetMask: u64 = 0x2000000000000000;
	
	const DisplacementShift: u64 = 0;
	
	const BaseRegisterShift: u64 = 32;
	
	const IndexRegisterShift: u64 = 40;
	
	const IndexScaleShift: u64 = 48;
	
	const SegmentRegisterShift: u64 = 56;
	
	const AddressOverrideFor32BitShift: u64 = 61;
	
	const RelativeInstructionPointerOffsetShift: u64 = 61;
	
	const NullGeneralPurposeRegister: u64 = 0x10;
	
	const NullSegmentRegister: u64 = 0x07;
	
	#[inline(always)]
	fn has_segment_register(self) -> bool
	{
		(self.0 & Self::SegmentRegisterMask) != (Self::NullSegmentRegister << Self::SegmentRegisterShift)
	}
	
	#[inline(always)]
	fn has_base_register(self) -> bool
	{
		(self.0 & Self::BaseRegisterMask) != (Self::NullGeneralPurposeRegister << Self::BaseRegisterShift)
	}
	
	#[inline(always)]
	fn has_index_register(self) -> bool
	{
		(self.0 & Self::IndexRegisterMask) != (Self::NullGeneralPurposeRegister << Self::IndexRegisterShift)
	}
	
	#[inline(always)]
	fn has_address_override_for_32_bit(self) -> bool
	{
		(self.0 & Self::AddressOverrideFor32BitMask) != 0
	}
	
	#[inline(always)]
	fn has_relative_instruction_pointer_offset(self) -> bool
	{
		(self.0 & Self::RelativeInstructionPointerOffsetMask) != 0
	}
	
	#[inline(always)]
	fn get_segment_register_index(self) -> u8
	{
		((self.0 & Self::SegmentRegisterMask) >> Self::SegmentRegisterShift) as u8
	}
	
	#[inline(always)]
	fn get_base_register_index(self) -> u8
	{
		((self.0 & Self::BaseRegisterMask) >> Self::BaseRegisterShift) as u8
	}
	
	#[inline(always)]
	fn get_base_register(self) -> Register64Bit
	{
		unsafe { transmute(self.get_base_register_index()) }
	}
	
	#[inline(always)]
	fn get_index_register_index(self) -> u8
	{
		((self.0 & Self::IndexRegisterMask) >> Self::IndexRegisterShift) as u8
	}
	
	#[inline(always)]
	fn get_index_register(self) -> Register64Bit
	{
		unsafe { transmute(self.get_index_register_index()) }
	}
	
	#[inline(always)]
	fn get_index_scale(self) -> u8
	{
		(unsafe { transmute::<u64, IndexScale>((self.0 & Self::IndexScaleMask) >> Self::IndexScaleShift) }).into()
	}
	
	#[inline(always)]
	fn get_displacement(self) -> Immediate32Bit
	{
		unsafe { transmute((self.0 & Self::DisplacementMask) >> Self::DisplacementShift) }
	}
	
	#[inline(always)]
	pub(crate) fn emit_prefix_group4(self, byte_emitter: &mut ByteEmitter)
	{
		if self.has_address_override_for_32_bit()
		{
			byte_emitter.emit_u8(0x67)
		}
	}
	
	#[inline(always)]
	pub fn new(displacement: Immediate32Bit, base: Option<impl GeneralPurposeRegister>, index: Option<impl GeneralPurposeRegister>, scale: IndexScale, segment_register: Option<impl AnySegmentRegister>, address_override_for_32_bit: bool, relative_instruction_pointer_offset: bool) -> Self
	{
		debug_assert_eq!(address_override_for_32_bit && relative_instruction_pointer_offset, false, "address_override_for_32_bit and relative_instruction_pointer_offset can not both be specified");
		
		MemoryOperand
		(
			(displacement.to_u64() | Self::DisplacementMask)
			| match base
			{
				None => Self::NullGeneralPurposeRegister << Self::BaseShift,
				Some(base) => (base.index() as u64) << Self::BaseShift,
			}
			| match index
			{
				None => Self::NullGeneralPurposeRegister << Self::IndexShift,
				Some(index) => (base.index() as u64) << Self::IndexShift,
			}
			| scale.to_u64() << Self::IndexScaleShift
			| match segment_register
			{
				None => Self::NullSegmentRegister << Self::SegmentRegisterShift,
				Some(segment_register) => (segment_register.index() as u64) << Self::SegmentRegisterShift,
			}
			| (address_override_for_32_bit as u64) << Self::AddressOverrideFor32BitShift
			| (relative_instruction_pointer_offset as u64) << Self::RelativeInstructionPointerOffsetShift
		)
	}
	
	/// Create a new memory operand using the `displacement` form.
	#[inline(always)]
	pub fn displacement(displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, None, IndexScale::x1, None, false, false)
	}
	
	/// Create a new memory operand using the `segment:displacement` form.
	#[inline(always)]
	pub fn segment_displacement(segment_register: impl AnySegmentRegister, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, None, IndexScale::x1, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `RIP` (relative instruction pointer) form.
	#[inline(always)]
	pub fn relative_instruction_pointer_relative() -> Self
	{
		Self::new(Immediate32Bit(0), None, None, IndexScale::x1, None, false, true)
	}
	
	/// Create a new memory operand using the `segment:RIP` (relative instruction pointer) form.
	#[inline(always)]
	pub fn segment_relative_instruction_pointer_relative(segment_register: impl AnySegmentRegister) -> Self
	{
		Self::new(Immediate32Bit(0), None, None, IndexScale::x1, Some(segment_register), false, true)
	}
	
	/// Create a new memory operand using the `RIP+displacement` (relative instruction pointer) form.
	#[inline(always)]
	pub fn relative_instruction_pointer_relative_displacement(displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, None, IndexScale::x1, None, false, true)
	}
	
	/// Create a new memory operand using the `segment:RIP+displacement` (relative instruction pointer) form.
	#[inline(always)]
	pub fn segment_relative_instruction_pointer_relative_displacement(segment_register: impl AnySegmentRegister, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, None, IndexScale::x1, Some(segment_register), false, true)
	}
	
	/// Create a new memory operand using the `base64` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	pub fn base_64(base_64: Register64Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_64), None, IndexScale::x1, None, false, false)
	}
	
	/// Create a new memory operand using the `base32` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	pub fn base_32(base_32: Register32Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_32), None, IndexScale::x1, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:base64` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	pub fn segment_base_64(segment_register: impl AnySegmentRegister, base_64: Register64Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_64), None, IndexScale::x1, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:base32` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	pub fn segment_base_32(segment_register: impl AnySegmentRegister, base_32: Register32Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_32), None, IndexScale::x1, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `displacement(base64)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	pub fn displacement_base_64(base_64: Register64Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_64), None, IndexScale::x1, None, false, false)
	}
	
	/// Create a new memory operand using the `displacement(base32)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	pub fn displacement_base_32(base_32: Register32Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_32), None, IndexScale::x1, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:displacement(base64)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	pub fn segment_displacement_base_64(segment_register: impl AnySegmentRegister, base_64: Register64Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_64), None, IndexScale::x1, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:displacement(base32)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	pub fn segment_displacement_base_32(segment_register: impl AnySegmentRegister, base_32: Register32Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_32), None, IndexScale::x1, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `(index64,scale)` form.
	#[inline(always)]
	pub fn index_64_scale(index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), None, Some(index_64), scale, None, false, false)
	}
	
	/// Create a new memory operand using the `(index32,scale)` form.
	#[inline(always)]
	pub fn index_32_scale(index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), None, Some(index_32), scale, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:(index64,scale)` form.
	#[inline(always)]
	pub fn segment_index_64_scale(segment_register: impl AnySegmentRegister, index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), None, Some(index_64), scale, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:(index32,scale)` form.
	#[inline(always)]
	pub fn segment_index_32_scale(segment_register: impl AnySegmentRegister, index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), None, Some(index_32), scale, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `displacement:(index64,scale)` form.
	#[inline(always)]
	pub fn displacement_index_64_scale(index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, Some(index_64), scale, None, false, false)
	}
	
	/// Create a new memory operand using the `displacement:(index32,scale)` form.
	#[inline(always)]
	pub fn displacement_index_32_scale(index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, Some(index_32), scale, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:displacement:(index64,scale)` form.
	#[inline(always)]
	pub fn segment_displacement_index_64_scale(segment_register: impl AnySegmentRegister, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, Some(index_64), scale, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:displacement:(index32,scale)` form.
	#[inline(always)]
	pub fn segment_displacement_index_32_scale(segment_register: impl AnySegmentRegister, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, Some(index_32), scale, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `(base64,index64,scale)` form.
	#[inline(always)]
	pub fn base_64_index_64_scale(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_64), Some(index_64), scale, None, false, false)
	}
	
	/// Create a new memory operand using the `(base32,index32,scale)` form.
	#[inline(always)]
	pub fn base_32_index_32_scale(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_32), Some(index_32), scale, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:(base64,index64,scale)` form.
	#[inline(always)]
	pub fn segment_base_64_index_64_scale(segment_register: impl AnySegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_64), Some(index_64), scale, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:(base32,index32,scale)` form.
	#[inline(always)]
	pub fn segment_base_32_index_32_scale(segment_register: impl AnySegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(base_32), Some(index_32), scale, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `displacement(base64,index64,scale)` form.
	#[inline(always)]
	pub fn displacement_base_64_index_64_scale(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_64), Some(index_64), scale, None, false, false)
	}
	
	/// Create a new memory operand using the `displacement(base32,index32,scale)` form.
	#[inline(always)]
	pub fn displacement_base_32_index_32_scale(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_32), Some(index_32), scale, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:displacement(base64,index64,scale)` form.
	#[inline(always)]
	pub fn segment_displacement_base_64_index_64_scale(segment_register: impl AnySegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_64), Some(index_64), scale, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:displacement(base32,index32,scale)` form.
	#[inline(always)]
	pub fn segment_displacement_base_32_index_32_scale(segment_register: impl AnySegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(base_32), Some(index_32), scale, Some(segment_register), true, false)
	}
}
