// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


pub(crate) trait ToOpcode
{
	#[inline(always)]
	fn to_opcode(self) -> u8;
}

impl ToOpcode for u8
{
	#[inline(always)]
	fn to_opcode(self) -> u8
	{
		self
	}
}
