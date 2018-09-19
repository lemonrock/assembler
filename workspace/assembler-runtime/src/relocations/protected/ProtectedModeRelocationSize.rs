// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A size suitable for a relocation in protected (32-bit) mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum ProtectedModeRelocationSize
{
	/// A byte.
	BYTE = 1,
	
	/// A word, 2 bytes.
	WORD = 2,
	
	/// A doubleword, 4 bytes.
	DWORD = 4,
}

impl ProtectedModeRelocationSize
{
	#[inline(always)]
	pub(crate) fn to_bytes(self) -> usize
	{
		self as usize
	}
}
