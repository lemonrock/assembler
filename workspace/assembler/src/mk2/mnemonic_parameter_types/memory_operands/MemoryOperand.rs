// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A memory operand.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryOperand(u64);

impl MemoryOperand
{
	const DisplacementMask: u64 = 0x00000000FFFFFFFF;
	
	const BaseMask: u64 = 0x0000001F00000000;
	
	const IndexMask: u64 = 0x00001F0000000000;
	
	const ScaleMask: u64 = 0x0003000000000000;
	
	const SegmentRegisterMask: u64 = 0x0700000000000000;
	
	const AddressOrMask: u64 = 0x1000000000000000;
	
	const RelativeInstructionPointerMask: u64 = 0x2000000000000000;
	
	const BaseShift: u64 = x;
	
	const IndexShift: u64 = x;
	
	const ScaleShift: u64 = x;
	
	const SegmentRegisterShift: u64 = x;
	
	const AddressOrShift: u64 = x;
	
	const RelativeInstructionPointerShift: u64 = x;
	
	const NullRegister: u64 = 0x10;
	
	const NullSegmentRegister: u64 = 0x10;


// TODO: GeneralPurposeRegister.to_u64()
// TODO: AnySegmentRegister.to_u64()
	
	
	// TODO: const fn new won't work as it uses 'match'
	#[inline(always)]
	pub const fn new(displacement: Immediate32Bit, base: Option<GeneralPurposeRegister>, index: Option<GeneralPurposeRegister>, scale: IndexScale, segment_register: Option<AnySegmentRegister>, addr_or: bool, relative_instruction_pointer: bool) -> Self
	{
		let result = (displacement.to_u64() | Self::DisplacementMask)
		| match base
		{
			None => Self::NullRegister << Self::BaseShift,
			Some(base) => base.to_u64() << Self::BaseShift,
		}
		| match index
		{
			None => Self::NullRegister << Self::IndexShift,
			Some(index) => index.to_u64() << Self::IndexShift,
		}
		| scale.to_u64() << Self::ScaleShift
		| match segment_register
		{
			None => Self::NullSegmentRegister << Self::SegmentRegisterShift,
			Some(segment_register) => segment_register.to_u64() << Self::SegmentRegisterShift,
		}
		| (addr_or as u64) << Self::AddressOrShift
		| (relative_instruction_pointer as u64) << Self::RelativeInstructionPointerShift;
		
		MemoryOperand(result)
	}
	
	/// Create a new memory operand using the `displacement` form.
	#[inline(always)]
	pub const fn displacement(displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, None, IndexScale::x1, None, false, false)
	}
	
	/// Create a new memory operand using the `segment:displacement` form.
	#[inline(always)]
	pub const fn segment_displacement(segment_register: AnySegmentRegister, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, None, IndexScale::x1, Some(AnySegmentRegister), false, false)
	}
	
	/// Create a new memory operand using the `RIP` (relative instruction pointer) form.
	#[inline(always)]
	pub const fn rip() -> Self
	{
		Self::new(Immediate32Bit(0), None, None, IndexScale::x1, None, false, true)
	}
	
	/// Create a new memory operand using the `segment:RIP` (relative instruction pointer) form.
	#[inline(always)]
	pub const fn segment_rip(segment_register: AnySegmentRegister) -> Self
	{
		Self::new(Immediate32Bit(0), None, None, IndexScale::x1, Some(segment_register), false, true)
	}
	
	/// Create a new memory operand using the `RIP+displacement` (relative instruction pointer) form.
	#[inline(always)]
	pub const fn rip_displacement(displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, None, IndexScale::x1, None, false, true)
	}
	
	/// Create a new memory operand using the `segment:RIP+displacement` (relative instruction pointer) form.
	#[inline(always)]
	pub const fn segment_rip_displacement(segment_register: AnySegmentRegister, displacement: Immediate32) -> Self
	{
		Self::new(displacement, None, None, IndexScale::x1, Some(segment_register), false, true)
	}
	
	/// Create a new memory operand using the `base64` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	pub const fn base_64(base_64: Register64Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(GeneralPurposeRegister::_64(base_64)), None, IndexScale::x1, None, false, false)
	}
	
	/// Create a new memory operand using the `base32` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	pub const fn base_32(base_32: Register32Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(GeneralPurposeRegister::_32(base_32)), None, IndexScale::x1, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:base64` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	pub const fn segment_base_64(segment_register: AnySegmentRegister, base_64: Register64Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(GeneralPurposeRegister::_64(base_64)), None, IndexScale::x1, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:base32` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	pub const fn segment_base_32(segment_register: AnySegmentRegister, base_32: Register32Bit) -> Self
	{
		Self::new(Immediate32Bit(0), Some(GeneralPurposeRegister::_32(base_32)), None, IndexScale::x1, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `displacement(base64)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	pub const fn displacement_base_64(base_64: Register64Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(GeneralPurposeRegister::_64(base_64)), None, IndexScale::x1, None, false, false)
	}
	
	/// Create a new memory operand using the `displacement(base32)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	pub const fn displacement_base_32(base_32: Register32Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(GeneralPurposeRegister::_32(base_32)), None, IndexScale::x1, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:displacement(base64)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	pub const fn segment_displacement_base_64(segment_register: AnySegmentRegister, base_64: Register64Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(GeneralPurposeRegister::_64(base_64)), None, IndexScale::x1, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:displacement(base32)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	pub const fn segment_displacement_base_32(segment_register: AnySegmentRegister, base_32: Register32Bit, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(GeneralPurposeRegister::_32(base_32)), None, IndexScale::x1, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `(index64,scale)` form.
	#[inline(always)]
	pub const fn index_64_scale(index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), None, Some(GeneralPurposeRegister::_64(index_64)), scale, None, false, false)
	}
	
	/// Create a new memory operand using the `(index32,scale)` form.
	#[inline(always)]
	pub const fn index_32_scale(index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), None, Some(GeneralPurposeRegister::_32(index_32)), scale, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:(index64,scale)` form.
	#[inline(always)]
	pub const fn segment_index_64_scale(segment_register: AnySegmentRegister, index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), None, Some(GeneralPurposeRegister::_64(index_64)), scale, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:(index32,scale)` form.
	#[inline(always)]
	pub const fn segment_index_32_scale(segment_register: AnySegmentRegister, index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), None, Some(GeneralPurposeRegister::_32(index_32)), scale, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `displacement:(index64,scale)` form.
	#[inline(always)]
	pub const fn displacement_index_64_scale(index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, Some(GeneralPurposeRegister::_64(index_64)), scale, None, false, false)
	}
	
	/// Create a new memory operand using the `displacement:(index32,scale)` form.
	#[inline(always)]
	pub const fn displacement_index_32_scale(index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, Some(GeneralPurposeRegister::_32(index_32)), scale, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:displacement:(index64,scale)` form.
	#[inline(always)]
	pub const fn segment_displacement_index_64_scale(segment_register: AnySegmentRegister, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, Some(GeneralPurposeRegister::_64(index_64)), scale, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:displacement:(index32,scale)` form.
	#[inline(always)]
	pub const fn segment_displacement_index_32_scale(segment_register: AnySegmentRegister, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, None, Some(GeneralPurposeRegister::_32(index_32)), scale, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `(base64,index64,scale)` form.
	#[inline(always)]
	pub const fn base_64_index_64_scale(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(GeneralPurposeRegister::_64(base_64)), Some(GeneralPurposeRegister::_64(index_64)), scale, None, false, false)
	}
	
	/// Create a new memory operand using the `(base32,index32,scale)` form.
	#[inline(always)]
	pub const fn base_32_index_32_scale(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(GeneralPurposeRegister::_32(base_32)), Some(GeneralPurposeRegister::_32(index_32)), scale, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:(base64,index64,scale)` form.
	#[inline(always)]
	pub const fn segment_base_64_index_64_scale(segment_register: AnySegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(GeneralPurposeRegister::_64(base_64)), Some(GeneralPurposeRegister::_64(index_64)), scale, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:(base32,index32,scale)` form.
	#[inline(always)]
	pub const fn segment_base_32_index_32_scale(segment_register: AnySegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self
	{
		Self::new(Immediate32Bit(0), Some(GeneralPurposeRegister::_32(base_32)), Some(GeneralPurposeRegister::_32(index_32)), scale, Some(segment_register), true, false)
	}
	
	/// Create a new memory operand using the `displacement(base64,index64,scale)` form.
	#[inline(always)]
	pub const fn displacement_base_64_index_64_scale(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(GeneralPurposeRegister::_64(base_64)), Some(GeneralPurposeRegister::_64(index_64)), scale, None, false, false)
	}
	
	/// Create a new memory operand using the `displacement(base32,index32,scale)` form.
	#[inline(always)]
	pub const fn displacement_base_32_index_32_scale(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(GeneralPurposeRegister::_32(base_32)), Some(GeneralPurposeRegister::_32(index_32)), scale, None, true, false)
	}
	
	/// Create a new memory operand using the `segment:displacement(base64,index64,scale)` form.
	#[inline(always)]
	pub const fn segment_displacement_base_64_index_64_scale(segment_register: AnySegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(GeneralPurposeRegister::_64(base_64)), Some(GeneralPurposeRegister::_64(index_64)), scale, Some(segment_register), false, false)
	}
	
	/// Create a new memory operand using the `segment:displacement(base32,index32,scale)` form.
	#[inline(always)]
	pub const fn segment_displacement_base_32_index_32_scale(segment_register: AnySegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self
	{
		Self::new(displacement, Some(GeneralPurposeRegister::_32(base_32)), Some(GeneralPurposeRegister::_32(index_32)), scale, Some(segment_register), true, false)
	}
}
