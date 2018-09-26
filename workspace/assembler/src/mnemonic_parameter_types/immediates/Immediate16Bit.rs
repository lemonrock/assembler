// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


impl_immediate!(Immediate16Bit, i16, u16);

impl From<Immediate8Bit> for Immediate16Bit
{
	#[inline(always)]
	fn from(immediate: Immediate8Bit) -> Self
	{
		Self::from(immediate.0)
	}
}

impl From<u8> for Immediate16Bit
{
	#[inline(always)]
	fn from(immediate: u8) -> Self
	{
		Immediate16Bit(immediate as i16)
	}
}

impl From<i8> for Immediate16Bit
{
	#[inline(always)]
	fn from(immediate: i8) -> Self
	{
		Immediate16Bit(immediate as i16)
	}
}
