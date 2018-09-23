// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A 32-bit relative address.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelativeAddress32Bit(pub i32);

impl AsDisplacement for RelativeAddress32Bit
{
	type D = u32;
	
	#[inline(always)]
	fn displacement(self) -> Self::D
	{
		self.0 as u32
	}
}

impl RelativeAddress<u64> for RelativeAddress32Bit
{
}
