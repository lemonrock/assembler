// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Memory.
pub trait Memory: From<MemoryOperand> + Into<MemoryOperand> + Sized
{
	/// Create a new memory operand using the `RIP` (relative instruction pointer) form.
	#[inline(always)]
	fn relative_instruction_pointer_relative() -> Self;
	
	/// Create a new memory operand using the `RIP+displacement` (relative instruction pointer) form.
	#[inline(always)]
	fn relative_instruction_pointer_relative_displacement(displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `displacement` form.
	#[inline(always)]
	fn displacement(displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `base64` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	fn base_64(base_64: Register64Bit) -> Self;
	
	/// Create a new memory operand using the `displacement(base64)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	fn base_64_displacement(base_64: Register64Bit, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `(index64,scale)` form where `scale` is `1`.
	#[inline(always)]
	fn index_64(index_64: Register64Bit) -> Self;
	
	/// Create a new memory operand using the `(index64,scale)` form.
	#[inline(always)]
	fn index_64_scale(index_64: Register64Bit, scale: IndexScale) -> Self;
	
	/// Create a new memory operand using the `displacement:(index64,scale)` form.
	#[inline(always)]
	fn index_64_scale_displacement(index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `(base64,index64,scale)` form where `scale` is 1.
	#[inline(always)]
	fn base_64_index_64(base_64: Register64Bit, index_64: Register64Bit) -> Self;
	
	/// Create a new memory operand using the `(base64,index64,scale)` form.
	#[inline(always)]
	fn base_64_index_64_scale(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self;
	
	/// Create a new memory operand using the `displacement(base64,index64,scale)` form.
	#[inline(always)]
	fn base_64_index_64_scale_displacement(base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `base32` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	fn base_32(base_32: Register32Bit) -> Self;
	
	/// Create a new memory operand using the `displacement(base32)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	fn base_32_displacement(base_32: Register32Bit, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `(index32,scale)` form where `scale` is `1`.
	#[inline(always)]
	fn index_32(index_32: Register32Bit) -> Self;
	
	/// Create a new memory operand using the `(index32,scale)` form.
	#[inline(always)]
	fn index_32_scale(index_32: Register32Bit, scale: IndexScale) -> Self;
	
	/// Create a new memory operand using the `displacement:(index32,scale)` form.
	#[inline(always)]
	fn index_32_scale_displacement(index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `(base32,index32,scale)` form where `scale` is 1.
	#[inline(always)]
	fn base_32_index_32(base_32: Register32Bit, index_32: Register32Bit) -> Self;
	
	/// Create a new memory operand using the `(base32,index32,scale)` form.
	#[inline(always)]
	fn base_32_index_32_scale(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self;
	
	/// Create a new memory operand using the `displacement(base32,index32,scale)` form.
	#[inline(always)]
	fn base_32_index_32_scale_displacement(base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `segment:RIP` (relative instruction pointer) form.
	#[inline(always)]
	fn segment_relative_instruction_pointer_relative(segment_register: SegmentRegister) -> Self;
	
	/// Create a new memory operand using the `segment:RIP+displacement` (relative instruction pointer) form.
	#[inline(always)]
	fn segment_relative_instruction_pointer_relative_displacement(segment_register: SegmentRegister, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `segment:displacement` form.
	#[inline(always)]
	fn segment_displacement(segment_register: SegmentRegister, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `segment:base64` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	fn segment_base_64(segment_register: SegmentRegister, base_64: Register64Bit) -> Self;
	
	/// Create a new memory operand using the `segment:displacement(base64)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base64`.
	#[inline(always)]
	fn segment_base_64_displacement(segment_register: SegmentRegister, base_64: Register64Bit, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `segment:(index64,scale)` form.
	#[inline(always)]
	fn segment_index_64_scale(segment_register: SegmentRegister, index_64: Register64Bit, scale: IndexScale) -> Self;
	
	/// Create a new memory operand using the `segment:displacement:(index64,scale)` form.
	#[inline(always)]
	fn segment_index_64_scale_displacement(segment_register: SegmentRegister, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `segment:(base64,index64,scale)` form.
	#[inline(always)]
	fn segment_base_64_index_64_scale(segment_register: SegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale) -> Self;
	
	/// Create a new memory operand using the `segment:displacement(base64,index64,scale)` form.
	#[inline(always)]
	fn segment_base_64_index_64_scale_displacement(segment_register: SegmentRegister, base_64: Register64Bit, index_64: Register64Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `segment:base32` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	fn segment_base_32(segment_register: SegmentRegister, base_32: Register32Bit) -> Self;
	
	/// Create a new memory operand using the `segment:displacement(base32)` form.
	///
	/// Note that this has nothing to do with the binary encoding scheme `Base32`.
	#[inline(always)]
	fn segment_base_32_displacement(segment_register: SegmentRegister, base_32: Register32Bit, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `segment:(index32,scale)` form.
	#[inline(always)]
	fn segment_index_32_scale(segment_register: SegmentRegister, index_32: Register32Bit, scale: IndexScale) -> Self;
	
	/// Create a new memory operand using the `segment:displacement:(index32,scale)` form.
	#[inline(always)]
	fn segment_index_32_scale_displacement(segment_register: SegmentRegister, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self;
	
	/// Create a new memory operand using the `segment:(base32,index32,scale)` form.
	#[inline(always)]
	fn segment_base_32_index_32_scale(segment_register: SegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale) -> Self;
	
	/// Create a new memory operand using the `segment:displacement(base32,index32,scale)` form.
	#[inline(always)]
	fn segment_base_32_index_32_scale_displacement(segment_register: SegmentRegister, base_32: Register32Bit, index_32: Register32Bit, scale: IndexScale, displacement: Immediate32Bit) -> Self;
}
