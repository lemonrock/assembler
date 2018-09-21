// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Officially known as `operands` rather than parameters, but official terminology is very confusing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct MnemonicDefinitionSignature
{
	parameters: &'static [MnemonicParameter],
	
	opcode_bytes: &'static [u8],
	
	register_identifier: Option<RegisterIdentifier>,
	
	instruction_flags: u32,
	
	cpu_feature_required: Option<CpuFeature>,
}

impl MnemonicDefinitionSignature
{
	#[inline(always)]
	pub(crate) const fn new(parameters: &'static [MnemonicParameter], opcode_bytes: &'static [u8], register_identifier: Option<RegisterIdentifier>, instruction_flags: u32, cpu_feature_required: Option<CpuFeature>) -> Self
	{
		Self
		{
			parameters,
			opcode_bytes,
			register_identifier,
			instruction_flags,
			cpu_feature_required,
		}
	}
}
