// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Represents a hint.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BranchHint
{
	/// Hint taken.
	Taken = 0x3E,
	
	/// Hint not taken.
	NotTaken = 0x2E,
}

impl MemoryOrBranchHint for BranchHint
{
	#[inline(always)]
	fn emit_prefix_group2(self, byte_emitter: &mut ByteEmitter)
	{
		byte_emitter.emit_u8(self as u8)
	}
}
