// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Definition of a mnemonic.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MnemonicDefinition
{
	name: &'static str,
	
	// One or more overloaded signatures (forms).
	overloaded_signatures: ArrayVec<[MnemonicDefinitionSignature; MnemonicDefinition::MaximumMnemonicDefinitionSignatures]>,
}

impl MnemonicDefinition
{
	const MaximumMnemonicDefinitionSignatures: usize = 16;
	
	#[inline(always)]
	pub(crate) fn find_definition(&self, assembling_for_architecture_variant: &AssemblingForArchitectureVariant, parsed_mnemonic_arguments: &[ParsedMnemonicArgument]) -> Option<&MnemonicDefinitionSignature>
	{
		for overloaded_signature in self.overloaded_signatures.iter()
		{
			if overloaded_signature.matches(assembling_for_architecture_variant, parsed_mnemonic_arguments)
			{
				return Some(overloaded_signature)
			}
		}
		
		None
	}
}
