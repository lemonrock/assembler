// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An address size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum AddressSize
{
	/// Sixteen-Bit bit (`WORD`) address sizes.
	WORD = 2,
	
	/// Thirty-Two bit (`DWORD`) address sizes.
	DWORD = 4,
	
	/// Sixty-four bit (`QWORD`) address sizes.
	QWORD = 8,
}

impl Default for AddressSize
{
	#[inline(always)]
	fn default() -> Self
	{
		AddressSize::QWORD
	}
}

impl AddressSize
{
	#[inline(always)]
	pub(crate) fn is_16_bit_addressing(self) -> bool
	{
		self == AddressSize::WORD
	}
}
