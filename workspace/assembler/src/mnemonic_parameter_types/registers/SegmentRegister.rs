// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 4 (`FS`) and Register 5 (`GS`)  are defined on this enumeration but optimal encodings of them in combination with mnemonics exist and should be preferred as they increase code density.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SegmentRegister
{
	/// Register 0.
	///
	/// One of four `Data Segment` registers, typically used for the destination of string instructions.
	ES = 0,

	/// Register 1.
	///
	/// Known as `Code Segment`, used to fetch instructions from.
	CS = 1,

	/// Register 2.
	///
	/// Known as `Stack Segment`, used for stack memory.
	SS = 2,

	/// Register 3.
	///
	/// One of four `Data Segment` registers, this one is typically used for local data.
	DS = 3,

	/// Register 4.
	///
	/// One of four `Data Segment` registers.
	FS = 4,

	/// Register 5.
	///
	/// One of four `Data Segment` registers.
	GS = 5,
}

impl Default for SegmentRegister
{
	#[inline(always)]
	fn default() -> Self
	{
		SegmentRegister::ES
	}
}

impl Register for SegmentRegister
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self as u8
	}
}
