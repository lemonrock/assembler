// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// 32-bit memory offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryOffset32Bit
{
	/// `segment:offset` form.
	///
	/// Segment register is ignored in 64-bit long mode.
	SegmentOffsetForm32(SegmentRegister, Immediate64Bit),

	/// `offset` form.
	OffsetForm32(Immediate64Bit),
}

impl Default for MemoryOffset32Bit
{
	#[inline(always)]
	fn default() -> Self
	{
		MemoryOffset8Bit::OffsetForm32(Immediate64Bit::default())
	}
}

impl AsDisplacement for MemoryOffset32Bit
{
	type D = u64;
	
	#[inline(always)]
	fn displacement(self) -> Self::D
	{
		self.get_offset().displacement()
	}
}

impl MemoryOffset for MemoryOffset32Bit
{
	#[inline(always)]
	fn get_segment_register(&self) -> Option<SegmentRegister>
	{
		use self::MemoryOffset32Bit::*;
		
		match self
		{
			SegmentOffsetForm32(segment_register, _) => Some(segment_register),
			OffsetForm32(_) => None,
		}
	}
	
	#[inline(always)]
	fn get_offset(&self) -> Immediate64Bit
	{
		use self::MemoryOffset32Bit::*;
		
		match self
		{
			SegmentOffsetForm32(_, immediate) => immediate,
			OffsetForm32(immediate) => immediate,
		}
	}
}
