// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A relocation suitable for protected (32-bit) mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ProtectedModeRelocation
{
	pub(crate) relocation_offset: RelocationOffset,
	pub(crate) relocation_size: ProtectedModeRelocationSize,
	pub(crate) relocation_kind: ProtectedModeRelocationKind,
}

impl ProtectedModeRelocation
{
	#[inline(always)]
	pub(crate) fn subtract_to_get_starting_pointer_put_relocation_at(&self) -> usize
	{
		self.relocation_offset as usize + self.relocation_size.to_bytes()
	}
	
	#[inline(always)]
	pub(crate) fn overwrite_address_at(&self, pointer_to_location_to_patch: *mut u8, address: u32)
	{
		unsafe
		{
			use self::ProtectedModeRelocationSize::*;
			
			match self.relocation_size
			{
				BYTE => *pointer_to_location_to_patch = address as u8,
				WORD => *(pointer_to_location_to_patch as *mut u16) = (address as u16).to_le(),
				DWORD => *(pointer_to_location_to_patch as *mut u32) = (address as u32).to_le(),
			}
		}
	}
}
