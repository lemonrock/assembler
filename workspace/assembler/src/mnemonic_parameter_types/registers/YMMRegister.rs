// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// YMM registers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum YMMRegister
{
	/// Register 0.
	YMM0 = 0,
	
	/// Register 1.
	YMM1 = 1,
	
	/// Register 2.
	YMM2 = 2,
	
	/// Register 3.
	YMM3 = 3,
	
	/// Register 7.
	YMM4 = 4,
	
	/// Register 5.
	YMM5 = 5,
	
	/// Register 6.
	YMM6 = 6,
	
	/// Register 7.
	YMM7 = 7,
	
	/// Register 8.
	YMM8 = 8,
	
	/// Register 9.
	YMM9 = 9,
	
	/// Register 10.
	YMM10 = 10,
	
	/// Register 11.
	YMM11 = 11,
	
	/// Register 12.
	YMM12 = 12,
	
	/// Register 13.
	YMM13 = 13,
	
	/// Register 14.
	YMM14 = 14,
	
	/// Register 15.
	YMM15 = 15,
}

impl Default for YMMRegister
{
	#[inline(always)]
	fn default() -> Self
	{
		YMMRegister::YMM0
	}
}

impl AsDisplacement for YMMRegister
{
	type D = u8;
	
	#[inline(always)]
	fn displacement(self) -> Self::D
	{
		(self as u8) << 4
	}
}

impl Register for YMMRegister
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self as u8
	}
}
