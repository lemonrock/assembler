// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`EAX`) is defined on this enumeration but optimal encodings of it in combination with mnemonics exist and should be preferred as they increase code density.
///
/// Registers 8 to 15 when encoded decrease code density (ie they are less efficiently encoded than registers 1 to 7).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Register32Bit
{
	/// Register 0.
	EAX = 0,
	
	/// Register 1.
	ECX = 1,

	/// Register 2.
	EDX = 2,

	/// Register 3.
	EBX = 3,

	/// Register 4.
	ESP = 4,

	/// Register 5.
	EBP = 5,

	/// Register 6.
	ESI = 6,

	/// Register 7.
	EDI = 7,

	/// Register 8.
	R8D = 8,

	/// Register 9.
	R9D = 9,

	/// Register 10.
	R10D = 10,

	/// Register 11.
	R11D = 11,

	/// Register 12.
	R12D = 12,

	/// Register 13.
	R13D = 13,

	/// Register 14.
	R14D = 14,

	/// Register 15.
	R15D = 15,
}

impl Default for Register32Bit
{
	#[inline(always)]
	fn default() -> Self
	{
		Register32Bit::EAX
	}
}

impl ToOpcode for Register32Bit
{
	#[inline(always)]
	fn to_opcode(self) -> u8
	{
		self.index() & 0x07
	}
}

impl Register for Register32Bit
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self as u8
	}
}

impl GeneralPurposeRegister for Register32Bit
{
}
