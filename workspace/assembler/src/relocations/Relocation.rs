// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Relocation
{
	target: JumpVariant,
	offset: u8,
	size: Size,
	protected_mode_relocation_kind: RelocationKind,
}

impl Relocation
{
	#[inline(always)]
	pub(crate) fn new(target: JumpVariant, size: Size, protected_mode_relocation_kind: RelocationKind) -> Self
	{
		Self
		{
			target,
			offset: 0,
			size,
			protected_mode_relocation_kind,
		}
	}
	
	#[inline(always)]
	pub(crate) fn bump(&mut self, size: Size)
	{
		self.offset += size.to_bytes()
	}
}
