// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A relocation suitable for long (64-bit) mode.
///
/// A relocations in long mode are relative, hence there is no `relocation_kind` field as there is for `ProtectedModeRelocation`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LongModeRelocation
{
	pub(crate) relocation_offset: RelocationOffset,
	pub(crate) relocation_size: LongModeRelocationSize,
}

impl LongModeRelocation
{
	#[inline(always)]
	pub(crate) fn subtract_to_get_starting_pointer_put_relocation_at(&self) -> usize
	{
		self.relocation_offset as usize + self.relocation_size.to_bytes()
	}
	
	#[inline(always)]
	pub(crate) fn overwrite_relative_address_at(&self, pointer_to_location_to_patch: *mut u8, relative_address: u64)
	{
		unsafe
		{
			use self::LongModeRelocationSize::*;
			
			match self.relocation_size
			{
				BYTE => *pointer_to_location_to_patch = relative_address as u8,
				WORD => *(pointer_to_location_to_patch as *mut u16) = (relative_address as u16).to_le(),
				DWORD => *(pointer_to_location_to_patch as *mut u32) = (relative_address as u32).to_le(),
				QWORD => *(pointer_to_location_to_patch as *mut u64) = relative_address.to_le(),
			}
		}
	}
}
