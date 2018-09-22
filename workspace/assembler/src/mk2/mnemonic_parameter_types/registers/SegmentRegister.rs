// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 4 (`FS`) and Register 5 (`GS`) are not defined on this enumeration.
///
/// Whilst technically valid to use, they are better encoded via the specialist singleton registers `FS::O` and `GS::O` as these produce a higher code density.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SegmentRegister
{
	/// Register 0.
	ES = 0,

	/// Register 1.
	CS = 1,

	/// Register 2.
	SS = 2,

	/// Register 3.
	DS = 3,
}

impl Default for SegmentRegister
{
	#[inline(always)]
	fn default() -> Self
	{
		SegmentRegister::ES
	}
}

impl AnySegmentRegister for SegmentRegister
{
	/// Zero-based index from 0 to 5.
	#[inline(always)]
	fn index(self) -> u8
	{
		self as u8
	}
}
