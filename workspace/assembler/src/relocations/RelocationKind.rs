// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Kind of relocation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum RelocationKind
{
	/// A RIP-relative relocation.
	/// No need to keep track of.
	Relative = 0,
	
	/// An absolute offset to a RIP-relative location.
	Absolute = 1,
	
	/// A relative offset to an absolute location.
	///
	/// Not supported in x64 Long mode.
	Extern = 2,
}

impl RelocationKind
{
	#[inline(always)]
	pub(crate) fn to_id(self) -> u8
	{
		self as u8
	}
}
