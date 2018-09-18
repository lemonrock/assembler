// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Family to which a register belongs.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum RegisterFamily
{
	LEGACY = 0,
	
	RIP = 1,
	
	HIGH_BYTE = 2,
	
	FP = 3,
	
	MMX = 4,
	
	XMM = 5,
	
	SEGMENT = 6,
	
	CONTROL = 7,
	
	DEBUG = 8,
	
	BOUND = 9
}
