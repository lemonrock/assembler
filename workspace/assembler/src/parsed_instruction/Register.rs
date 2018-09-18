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
	pub(crate) fn base_and_index_to_family_size_and_vector_scaled_index_byte_mode(base: Option<Self>, index: Option<Self>) -> Result<Option<(RegisterFamily, Size, bool)>, InstructionEncodingError>
	{
		let base_register_family_and_size = base.map(|base| base.to_family_and_size());
		let index_register_family_and_size = index.map(|index| index.to_family_and_size());
		
		const DisableVectorScaledIndexByteAddressingMode: bool = false;
		const EnableVectorScaledIndexByteAddressingMode: bool = true;
		
		let ok = match (base_register_family_and_size, index_register_family_and_size)
		{
			(None, None) => return Ok(None),
		
			(Some((register_family, size)), None) | (None, Some((register_family, size))) => (register_family, size, DisableVectorScaledIndexByteAddressingMode),
			
			(Some((base_register_family, base_size)), Some((index_register_family, index_size))) => if base_register_family == index_register_family
			{
				if base_size != index_size
				{
					return Err(InstructionEncodingError("Base and index registers are of differing sizes"))
				}
				(base_register_family, base_size, DisableVectorScaledIndexByteAddressingMode)
			}
			else
			{
				use self::RegisterFamily::*;
				
				// Vector Scaled Index Byte addressing.
				match (base_register_family, index_register_family)
				{
					(XMM, _) => (index_register_family, index_size, EnableVectorScaledIndexByteAddressingMode),
					(_, XMM) => (base_register_family, base_size, EnableVectorScaledIndexByteAddressingMode),
					_ => return Err(InstructionEncodingError("Register type combination not supported")),
				}
			}
		};
		Ok(Some(ok))
	}
	
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
	pub(crate) fn some_size(&self) -> Option<Size>
	{
		Some(self.size)
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
