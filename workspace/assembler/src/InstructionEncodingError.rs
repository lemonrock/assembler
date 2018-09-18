// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An error when encoding an instruction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstructionEncodingError(pub &'static str);

impl InstructionEncodingError
{
	#[inline(always)]
	pub(crate) fn error_when_writing_machine_code(_error: io::Error) -> Self
	{
		InstructionEncodingError("Input/Output error when writing machine code")
	}
}
