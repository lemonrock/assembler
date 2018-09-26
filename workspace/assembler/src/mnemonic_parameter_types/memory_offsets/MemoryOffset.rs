// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Memory Offset.
pub(crate) trait MemoryOffset: AsDisplacement<D=u64> + From<(SegmentRegister, Immediate64Bit)> + From<(Option<SegmentRegister>, Immediate64Bit)> + From<Immediate64Bit> + From<i8> + From<u8> + From<i16> + From<u16> + From<i32> + From<u32> + From<i64> + From<u64> + Into<(Option<SegmentRegister>, Immediate64Bit)> + Into<Immediate64Bit> + Into<i64> + Into<u64>
{
	/// Get the segment register, if any.
	#[inline(always)]
	fn get_segment_register(&self) -> Option<SegmentRegister>;
	
	/// Get the offset displacement.
	#[inline(always)]
	fn get_offset(&self) -> Immediate64Bit;
}
