// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A general purpose register suitable for a memory operand.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GeneralPurposeRegister
{
	#[allow(missing_docs)]
	_8(Register8Bit),
	
	#[allow(missing_docs)]
	_16(Register16Bit),
	
	#[allow(missing_docs)]
	_32(Register32Bit),
	
	#[allow(missing_docs)]
	_64(Register64Bit),
	
	#[allow(missing_docs)]
	_8High(RegisterHigh8BitsOf16Bits),
	
	#[allow(missing_docs)]
	_AL(AL),
	
	#[allow(missing_docs)]
	_AX(AX),
	
	#[allow(missing_docs)]
	_CL(CL),
	
	#[allow(missing_docs)]
	_DX(DX),
	
	#[allow(missing_docs)]
	_EAX(EAX),
	
	#[allow(missing_docs)]
	_RAX(RAX),
}

impl Default for GeneralPurposeRegister
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::_8(Register8Bit::default())
	}
}
