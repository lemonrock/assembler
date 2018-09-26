// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An immediate argument.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Immediate16Bit(pub i16);

impl AsDisplacement for Immediate16Bit
{
	type D = u16;
	
	#[inline(always)]
	fn displacement(self) -> Self::D
	{
		self.0 as u16
	}
}

impl Immediate for Immediate16Bit
{
	type SignedInteger = i16;
	
	#[inline(always)]
	fn value(self) -> Self::SignedInteger
	{
		self.0
	}
}

impl Into<i16> for Immediate16Bit
{
	#[inline(always)]
	fn into(self) -> i16
	{
		self.0
	}
}

impl Into<u16> for Immediate16Bit
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0 as u16
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

impl From<u16> for Immediate16Bit
{
	#[inline(always)]
	fn from(immediate: u16) -> Self
	{
		Immediate16Bit(immediate as i16)
	}
}

impl From<i16> for Immediate16Bit
{
	#[inline(always)]
	fn from(immediate: i16) -> Self
	{
		Immediate16Bit(immediate)
	}
}
