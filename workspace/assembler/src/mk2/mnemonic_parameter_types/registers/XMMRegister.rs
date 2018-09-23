// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`XMM0`) is defined on this enumeration but optimal encodings of it in combination with mnemonics exist and should be preferred as they increase code density.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum XMMRegister
{
	/// Register 0.
	XMM0 = 0,
	
	/// Register 1.
	XMM1 = 1,
	
	/// Register 2.
	XMM2 = 2,
	
	/// Register 3.
	XMM3 = 3,
	
	/// Register 7.
	XMM4 = 4,
	
	/// Register 5.
	XMM5 = 5,
	
	/// Register 6.
	XMM6 = 6,
	
	/// Register 7.
	XMM7 = 7,
	
	/// Register 8.
	XMM8 = 8,
	
	/// Register 9.
	XMM9 = 9,
	
	/// Register 10.
	XMM10 = 10,
	
	/// Register 11.
	XMM11 = 11,
	
	/// Register 12.
	XMM12 = 12,
	
	/// Register 13.
	XMM13 = 13,
	
	/// Register 14.
	XMM14 = 14,
	
	/// Register 15.
	XMM15 = 15,
}

impl Default for XMMRegister
{
	#[inline(always)]
	fn default() -> Self
	{
		XMMRegister::XMM0
	}
}

impl AsDisplacement for XMMRegister
{
	type D = u8;
	
	#[inline(always)]
	fn displacement(self) -> Self::D
	{
		(self as u8) << 4
	}
}

impl Register for XMMRegister
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self as u8
	}
}
