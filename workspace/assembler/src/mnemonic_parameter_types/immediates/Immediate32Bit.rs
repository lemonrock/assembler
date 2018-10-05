// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


impl_immediate!(Immediate32Bit, i32, u32);

impl From<Immediate8Bit> for Immediate32Bit
{
	#[inline(always)]
	fn from(immediate: Immediate8Bit) -> Self
	{
		Self::from(immediate.0)
	}
}

impl From<Immediate16Bit> for Immediate32Bit
{
	#[inline(always)]
	fn from(immediate: Immediate16Bit) -> Self
	{
		Self::from(immediate.0)
	}
}

impl From<u8> for Immediate32Bit
{
	#[inline(always)]
	fn from(immediate: u8) -> Self
	{
		Immediate32Bit(immediate as i32)
	}
}

impl From<i8> for Immediate32Bit
{
	#[inline(always)]
	fn from(immediate: i8) -> Self
	{
		Immediate32Bit(immediate as i32)
	}
}

impl From<u16> for Immediate32Bit
{
	#[inline(always)]
	fn from(immediate: u16) -> Self
	{
		Immediate32Bit(immediate as i32)
	}
}

impl From<i16> for Immediate32Bit
{
	#[inline(always)]
	fn from(immediate: i16) -> Self
	{
		Immediate32Bit(immediate as i32)
	}
}
