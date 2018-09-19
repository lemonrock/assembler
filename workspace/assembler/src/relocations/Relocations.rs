// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Relocations
{
	mode: SupportedOperationalMode,
	relocations: ArrayVec<[Relocation; 16]>,
}

impl Relocations
{
	#[inline(always)]
	pub(crate) fn new(mode: SupportedOperationalMode) -> Self
	{
		Self
		{
			mode,
			relocations: ArrayVec::default(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_jump_target_reference_addressing(&mut self, target: JumpTarget, size: Size)
	{
		use self::SupportedOperationalMode::*;
		
		match self.mode
		{
			Long => self.push_relative(target, size),
			Protected => self.push(target, size, RelocationKind::Absolute),
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_extern(&mut self, target: JumpTarget, size: Size) -> Result<(), InstructionEncodingError>
	{
		use self::SupportedOperationalMode::*;
		
		match self.mode
		{
			Long => return Err(InstructionEncodingError("AMD64 long mode does not support extern relocations")),
			Protected => self.push(target, size, RelocationKind::Extern),
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn push_relative(&mut self, target: JumpTarget, size: Size)
	{
		self.push(target, size, RelocationKind::Relative)
	}
	
	#[inline(always)]
	fn push(&mut self, target: JumpTarget, size: Size, protected_mode_relocation_kind: RelocationKind)
	{
		self.relocations.push(Relocation::new(target, size, protected_mode_relocation_kind))
	}
	
	#[inline(always)]
	pub(crate) fn bump(&mut self, size: Size)
	{
		for relocation in self.relocations.iter_mut()
		{
			relocation.bump(size)
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_to_statements_buffer(self, statements_buffer: &mut StatementsBuffer<impl Write>) -> Result<(), InstructionEncodingError>
	{
		for Relocation { target, offset, size, protected_mode_relocation_kind } in self.relocations
		{
			statements_buffer.push_relocation(target, offset, size, protected_mode_relocation_kind, self.mode).map_err(InstructionEncodingError::error_when_writing)?
		}
		Ok(())
	}
}
