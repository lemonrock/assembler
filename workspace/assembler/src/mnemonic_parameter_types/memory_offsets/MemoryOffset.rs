// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Memory Offset.
pub(crate) trait MemoryOffset: AsDisplacement<D=u64>
{
	/// Get the segment register, if any.
	#[inline(always)]
	fn get_segment_register(&self) -> Option<SegmentRegister>;
	
	/// Get the offset displacement.
	#[inline(always)]
	fn get_offset(&self) -> Immediate64Bit;
}
