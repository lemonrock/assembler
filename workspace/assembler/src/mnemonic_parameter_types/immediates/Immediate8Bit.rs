// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An immediate argument, typically used for a displacement.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Immediate8Bit(pub i8);

impl AsDisplacement for Immediate8Bit
{
	type D = u8;
	
	#[inline(always)]
	fn displacement(self) -> Self::D
	{
		self.0 as u8
	}
}

impl Into<i8> for Immediate8Bit
{
	#[inline(always)]
	fn into(self) -> i8
	{
		self.0
	}
}

impl Into<u8> for Immediate8Bit
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0 as u8
	}
}

impl From<u8> for Immediate8Bit
{
	#[inline(always)]
	fn from(immediate: u8) -> Self
	{
		Immediate8Bit(immediate as i8)
	}
}

impl From<i8> for Immediate8Bit
{
	#[inline(always)]
	fn from(immediate: i8) -> Self
	{
		Immediate8Bit(immediate)
	}
}

impl Immediate<u8> for Immediate8Bit
{
}
