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

impl From<i8> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: i8) -> Self
	{
		RelativeAddress32Bit(value as i32)
	}
}

impl From<u8> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		RelativeAddress32Bit(value as i32)
	}
}

impl From<i16> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: i16) -> Self
	{
		RelativeAddress32Bit(value as i32)
	}
}

impl From<u16> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		RelativeAddress32Bit(value as i32)
	}
}

impl From<i32> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: i32) -> Self
	{
		RelativeAddress32Bit(value as i32)
	}
}

impl From<u32> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		RelativeAddress32Bit(value as i32)
	}
}

impl Into<i32> for RelativeAddress32Bit
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.0
	}
}

impl Into<u32> for RelativeAddress32Bit
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl From<RelativeAddress8Bit> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: RelativeAddress8Bit) -> Self
	{
		RelativeAddress32Bit(value.0 as i32)
	}
}

impl RelativeAddress<u32> for RelativeAddress32Bit
{
}
