// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A parsed register.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register
{
	/// Size of the register.
	size: Size,
	
	/// Kind of the register.
	register_identifier: RegisterIdentifier,
}

impl Register
{
	#[inline(always)]
	pub(crate) fn to_family_and_size(&self) -> (RegisterFamily, Size)
	{
		(self.family(), self.size())
	}
	
	#[inline(always)]
	pub(crate) fn addressing_mode_is_rip_relative(base: Option<Self>) -> bool
	{
		base.map_or(false, |register| register.is_rip())
	}
	
	#[inline(always)]
	pub(crate) fn addressing_uses_rbp_base(base: Option<Self>) -> bool
	{
		base.map_or(false, |register| register.is_rbp_or_r13())
	}
	
	#[inline(always)]
	pub(crate) fn is_rbp_or_r13(&self) -> bool
	{
		use self::RegisterIdentifier::*;
		
		self.is(RBP) || self.is(R13)
	}
	
	#[inline(always)]
	pub(crate) fn is(&self, register_identifier: RegisterIdentifier) -> bool
	{
		self.register_identifier == register_identifier
	}
	
	#[inline(always)]
	pub(crate) fn is_not(&self, register_identifier: RegisterIdentifier) -> bool
	{
		self.register_identifier != register_identifier
	}
	
	#[inline(always)]
	pub(crate) fn has_family(&self, register_family: RegisterFamily) -> bool
	{
		self.family() == register_family
	}
	
	#[inline(always)]
	pub(crate) fn is_legacy(&self) -> bool
	{
		self.has_family(RegisterFamily::LEGACY)
	}
	
	#[inline(always)]
	pub(crate) fn is_rip(&self) -> bool
	{
		self.has_family(RegisterFamily::RIP)
	}
	
	#[inline(always)]
	pub(crate) fn is_xmm(&self) -> bool
	{
		self.has_family(RegisterFamily::XMM)
	}
	
	#[inline(always)]
	pub(crate) fn is_high_byte(&self) -> bool
	{
		self.has_family(RegisterFamily::HIGH_BYTE)
	}
	
	#[inline(always)]
	pub(crate) fn is_extended(&self) -> bool
	{
		use self::RegisterFamily::*;
		
		match self.family()
		{
			LEGACY | XMM | CONTROL | DEBUG => self.code() > 7,
			
			_ => false
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_byte_sized_and_rsp_rbp_rsi_or_rdi(&self) -> bool
	{
		if self.size == Size::BYTE
		{
			use self::RegisterIdentifier::*;
			
			match self.register_identifier
			{
				RSP | RBP | RSI | RDI => true,
				_ => false,
			}
		}
		else
		{
			false
		}
	}
	#[inline(always)]
	pub(crate) fn size(&self) -> Size
	{
		self.size
	}
	
	#[inline(always)]
	pub(crate) fn identifier(&self) -> RegisterIdentifier
	{
		self.register_identifier
	}
	
	#[inline(always)]
	fn code(&self) -> u8
	{
		self.register_identifier.code()
	}
	
	#[inline(always)]
	fn family(&self) -> RegisterFamily
	{
		self.register_identifier.family()
	}
}
