// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Any potential size.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Size
{
	/// A byte.
	///
	/// Coded as `b`.
	BYTE = 1,
	
	/// A word, 2 bytes.
	///
	/// Coded as `w`.
	WORD = 2,
	
	/// A doubleword, 4 bytes.
	///
	/// Coded as `d`.
	DWORD = 4,
	
	/// 6 bytes.
	///
	/// Coded as `f`.
	FWORD = 6,
	
	/// A quadword, 8 bytes.
	///
	/// Coded as `q`.
	QWORD = 8,
	
	/// 10 bytes.
	///
	/// Coded as `p`.
	PWORD = 10,
	
	/// An octword, 16 bytes.
	///
	/// Coded as `o`.
	OWORD = 16,
	
	/// A hexadecword, 32 bytes.
	///
	/// Coded as `h`.
	HWORD = 32,
}

impl Size
{
	#[inline(always)]
	pub(crate) fn is_16_bit_addressing_option(this: Option<Self>) -> bool
	{
		this == Some(Size::WORD)
	}
	
	#[inline(always)]
	pub(crate) fn is_16_bit_addressing(self) -> bool
	{
		self == Size::WORD
	}
	
	#[inline(always)]
	pub(crate) fn is_64_bit(self) -> bool
	{
		self == Size::QWORD
	}
	
	#[inline(always)]
	pub(crate) fn is_none_or(this: Option<Self>, is: Self) -> bool
	{
		match this
		{
			None => true,
			Some(value) => value == is,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_byte_or_word(self) -> bool
	{
		use self::Size::*;
		
		match self
		{
			BYTE => true,
			WORD => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_byte_or_dword(self) -> bool
	{
		use self::Size::*;
		
		match self
		{
			BYTE => true,
			DWORD => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_word_or_dword_or_qword(self) -> bool
	{
		use self::Size::*;
		
		match self
		{
			WORD => true,
			DWORD => true,
			QWORD => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_oword_or_hword(self) -> bool
	{
		use self::Size::*;
		
		match self
		{
			OWORD => true,
			HWORD => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn to_bytes(self) -> u8
	{
		self as u8
	}
	
	#[inline(always)]
	fn maximum_immediate_size(self) -> Self
	{
		min(self, Size::DWORD)
	}
}
