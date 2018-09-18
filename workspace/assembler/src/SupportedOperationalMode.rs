// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Supported Operational Mode.
///
/// These are affected by ring level, which on the x86 architecture, can vary between 0 and 3.
///
/// There is also Real mode (first defined for the 8086 Intel CPUs), Unreal mode (first defined for the Intel 80286 CPUs), Virtual 8086 mode (first defined for the
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum SupportedOperationalMode
{
	/// Long.
	Long,

	/// Protected.
	Protected,
}

impl Default for SupportedOperationalMode
{
	#[inline(always)]
	fn default() -> Self
	{
		SupportedOperationalMode::Long
	}
}

impl SupportedOperationalMode
{
	#[inline(always)]
	pub(crate) fn new_relocations(self) -> Relocations
	{
		Relocations::new(self)
	}
	
	#[inline(always)]
	pub(crate) fn default_address_size(self) -> AddressSize
	{
		use self::AddressSize::*;
		use self::SupportedOperationalMode::*;
		
		match self
		{
			Long => DWORD,
			Protected => QWORD,
		}
	}
	
	#[inline(always)]
	pub(crate) fn address_size_override_prefix_required(self, address_size: AddressSize) -> bool
	{
		use self::AddressSize::*;
		use self::SupportedOperationalMode::*;
		
		match (self, address_size)
		{
			(Long, QWORD) => false,
			(Long, DWORD) => true,
			(Long, WORD) => panic!("Long should never be used with WORD"),
			(Protected, QWORD) => panic!("Protected should never be used with QWORD"),
			(Protected, DWORD) => false,
			(Protected, WORD) => true,
		}
	}
}
