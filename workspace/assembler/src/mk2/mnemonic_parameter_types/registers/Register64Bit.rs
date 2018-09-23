// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`RAX`) is defined on this enumeration but optimal encodings of it in combination with mnemonics exist and should be preferred as they increase code density.
///
/// Registers 8 to 15 when encoded decrease code density (ie they are less efficiently encoded than registers 1 to 7).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Register64Bit
{
	/// Register 0.
	RAX = 0,

	/// Register 1.
	RCX = 1,

	/// Register 2.
	RDX = 2,

	/// Register 3.
	RBX = 3,

	/// Register 4.
	RSP = 4,

	/// Register 5.
	RBP = 5,

	/// Register 6.
	RSI = 6,

	/// Register 7.
	RDI = 7,

	/// Register 8.
	R8 = 8,

	/// Register 9.
	R9 = 9,

	/// Register 10.
	R10 = 10,

	/// Register 11.
	R11 = 11,

	/// Register 12.
	R12 = 12,

	/// Register 13.
	R13 = 13,

	/// Register 14.
	R14 = 14,

	/// Register 15.
	R15 = 15,
}

impl Default for Register64Bit
{
	#[inline(always)]
	fn default() -> Self
	{
		Register64Bit::RAX
	}
}

impl ToOpcode for Register64Bit
{
	#[inline(always)]
	fn to_opcode(self) -> u8
	{
		self.index() & 0x07
	}
}

impl Register for Register64Bit
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self as u8
	}
}

impl GeneralPurposeRegister for Register64Bit
{
}
