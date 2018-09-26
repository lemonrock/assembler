// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An immediate argument.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Immediate64Bit(pub i64);

impl AsDisplacement for Immediate64Bit
{
	type D = u64;
	
	#[inline(always)]
	fn displacement(self) -> Self::D
	{
		self.0 as u64
	}
}

impl Immediate<u64> for Immediate64Bit
{
}

impl From<Immediate8Bit> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: Immediate8Bit) -> Self
	{
		Self::from(immediate.0)
	}
}

impl From<Immediate16Bit> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: Immediate16Bit) -> Self
	{
		Self::from(immediate.0)
	}
}

impl From<Immediate32Bit> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: Immediate32Bit) -> Self
	{
		Self::from(immediate.0)
	}
}

impl Into<i64> for Immediate64Bit
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self.0
	}
}

impl Into<u64> for Immediate64Bit
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl From<u8> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: u8) -> Self
	{
		Immediate64Bit(immediate as i64)
	}
}

impl From<i8> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: i8) -> Self
	{
		Immediate64Bit(immediate as i64)
	}
}

impl From<u16> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: u16) -> Self
	{
		Immediate64Bit(immediate as i64)
	}
}

impl From<i16> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: i16) -> Self
	{
		Immediate64Bit(immediate as i64)
	}
}

impl From<u32> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: u32) -> Self
	{
		Immediate64Bit(immediate as i64)
	}
}

impl From<i32> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: i32) -> Self
	{
		Immediate64Bit(immediate as i64)
	}
}

impl From<u64> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: u64) -> Self
	{
		Immediate64Bit(immediate as i64)
	}
}

impl From<i64> for Immediate64Bit
{
	#[inline(always)]
	fn from(immediate: i64) -> Self
	{
		Immediate64Bit(immediate)
	}
}
