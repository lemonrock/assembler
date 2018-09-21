// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`ST0`) is not defined on this enumeration.
///
/// Whilst technically valid to use, it is better encoded via the specialist singleton register `STO::O` as this produces a higher code density.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum X87Register
{
	/// Register 1.
	ST1 = 1,
	
	/// Register 2.
	ST2 = 2,
	
	/// Register 3.
	ST3 = 3,
	
	/// Register 7.
	ST4 = 4,
	
	/// Register 5.
	ST5 = 5,
	
	/// Register 6.
	ST6 = 6,
	
	/// Register 7.
	ST7 = 7,
}

impl Default for X87Register
{
	#[inline(always)]
	fn default() -> Self
	{
		X87Register::ST1
	}
}
