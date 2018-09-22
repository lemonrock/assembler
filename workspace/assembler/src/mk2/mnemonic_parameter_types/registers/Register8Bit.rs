// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`AL`) and Register 1 (`CL`) are not defined on this enumeration.
///
/// Whilst technically valid to use, they are better encoded via the specialist singleton registers `AL::O` and `CL::O` as these produce a higher code density.
///
/// Registers 8 to 15 when encoded decrease code density (ie they are less efficiently encoded than register 2 to 7).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Register8Bit
{
	/// Register 2.
	DL = 2,

	/// Register 3.
	BL = 3,

	/// Register 4.
	SPL = 4,

	/// Register 5.
	BPL = 5,

	/// Register 6.
	SIL = 6,

	/// Register 7.
	DIL = 7,

	/// Register 8.
	R8B = 8,

	/// Register 9.
	R9B = 9,

	/// Register 10.
	R10B = 10,

	/// Register 11.
	R11B = 11,

	/// Register 12.
	R12B = 12,

	/// Register 13.
	R13B = 13,

	/// Register 14.
	R14B = 14,

	/// Register 15.
	R15B = 15,
}

impl Default for Register8Bit
{
	#[inline(always)]
	fn default() -> Self
	{
		Register8Bit::DL
	}
}

impl ToOpcode for Register8Bit
{
	#[inline(always)
	fn to_opcode(self) -> u8
	{
		(self as u8) & 0x07
	}
}
