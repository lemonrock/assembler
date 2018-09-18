// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Officially known as an `operand` rather than a parameter, but official terminology is very confusing.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct MnemonicParameter
{
	kind: MnemonicParameterKind,
	size: MnemonicParameterSize,
}

impl MnemonicParameter
{
	#[inline(always)]
	pub(crate) fn to_size(&self, operand_size: Option<Size>, immediate_size: Option<Size>) -> Size
	{
		use self::MnemonicParameterKind::*;
		use self::MnemonicParameterSize::*;
		use self::Size::*;
		
		match (self.size, self.kind)
		{
			(Fixed(BYTE), _) => BYTE,
			(Fixed(WORD), _) => WORD,
			(_, vector_scaled_index_byte_32) | (Fixed(DWORD), _) => DWORD,
			(_, vector_scaled_index_byte_64) | (Fixed(QWORD), _) => QWORD,
			(Fixed(FWORD), _) => FWORD,
			(Fixed(PWORD), _) => PWORD,
			(Fixed(OWORD), _) => OWORD,
			(Fixed(HWORD), _) => HWORD,
			
			// Computed by first pass.
			(AllPossible, immediate) => immediate_size.unwrap(),
			(AllPossible, _) => operand_size.unwrap(),
			
			// Should never be used; a placeholder.
			(NoSize, _) => BYTE,
			
			_ => unreachable!()
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_all_possible(&self) -> bool
	{
		self.size == MnemonicParameterSize::AllPossible
	}
}
