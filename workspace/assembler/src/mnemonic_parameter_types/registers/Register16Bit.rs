// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`AL`) and Register 2 (`DX`) are defined on this enumeration but optimal encodings of them in combination with mnemonics exist and should be preferred as they increase code density.
///
/// Registers 8 to 15 when encoded decrease code density (ie they are less efficiently encoded than registers 1 to 7).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Register16Bit
{
	/// Register 0.
	AX = 0,
	
	/// Register 1.
	///
	/// The fourth function argument in the System V Application Binary Interface for x86-64.
	CX = 1,
	
	/// Register 2.
	///
	/// The third function argument in the System V Application Binary Interface for x86-64.
	DX = 2,

	/// Register 3.
	BX = 3,

	/// Register 4.
	SP = 4,

	/// Register 5.
	BP = 5,

	/// Register 6.
	///
	/// The 'source' operand in string instructions.
	///
	/// The second function argument in the System V Application Binary Interface for x86-64.
	SI = 6,

	/// Register 7.
	///
	/// The 'destination' operand in string instructions.
	///
	/// The first function argument in the System V Application Binary Interface for x86-64.
	DI = 7,

	/// Register 8.
	///
	/// The fifth function argument in the System V Application Binary Interface for x86-64.
	R8W = 8,

	/// Register 9.
	///
	/// The sixth function argument in the System V Application Binary Interface for x86-64.
	R9W = 9,

	/// Register 10.
	R10W = 10,

	/// Register 11.
	R11W = 11,

	/// Register 12.
	R12W = 12,

	/// Register 13.
	R13W = 13,

	/// Register 14.
	R14W = 14,

	/// Register 15.
	R15W = 15,
}

impl Default for Register16Bit
{
	#[inline(always)]
	fn default() -> Self
	{
		Register16Bit::AX
	}
}

impl Register for Register16Bit
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self as u8
	}
}

impl GeneralPurposeRegister for Register16Bit
{
}
