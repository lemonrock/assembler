// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Index scale.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u64)]
pub enum IndexScale
{
	/// Scale by × 1.
	x1 = 0,
	
	/// Scale by × 2.
	x2 = 1,
	
	/// Scale by × 4.
	x4 = 2,
	
	/// Scale by × 8.
	x8 = 3,
}

impl Default for IndexScale
{
	#[inline(always)]
	fn default() -> Self
	{
		IndexScale::x1,
	}
}

impl IndexScale
{
	#[inline(always)]
	pub(crate) fn to_u64(self) -> u64
	{
		self as u64
	}
}
