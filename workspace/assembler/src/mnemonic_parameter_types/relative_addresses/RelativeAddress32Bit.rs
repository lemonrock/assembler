// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


impl_relative_address!(RelativeAddress32Bit, i32, u32, Immediate32Bit);

impl From<Immediate8Bit> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: Immediate8Bit) -> Self
	{
		let into: i8 = value.into();
		RelativeAddress32Bit(into as i32)
	}
}

impl From<Immediate16Bit> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: Immediate16Bit) -> Self
	{
		let into: i16 = value.into();
		RelativeAddress32Bit(into as i32)
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

impl From<RelativeAddress8Bit> for RelativeAddress32Bit
{
	#[inline(always)]
	fn from(value: RelativeAddress8Bit) -> Self
	{
		RelativeAddress32Bit(value.0 as i32)
	}
}
