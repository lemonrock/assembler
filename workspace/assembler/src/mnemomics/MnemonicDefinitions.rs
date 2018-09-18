// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Definitions of all known mnemonics.
#[derive(Default, Debug, Clone)]
pub struct MnemonicDefinitions
{
	/// An ASCII name, one byte or more long.
	///
	/// Mnemonic names should be compared ASCII case insensitively, although it is probably easier to normalize them.
	pub mnemonic_name_to_definition: HashMap<String, MnemonicDefinition>,
}

impl MnemonicDefinitions
{
	/// Finds a matching definition for the `mnemonic_name`.
	#[inline(always)]
	pub fn find_definition_ascii_case_insensitively<'a>(&'a self, assembling_for_architecture_variant: &AssemblingForArchitectureVariant, mnemonic_name: &String, parsed_mnemonic_arguments: &[ParsedMnemonicArgument]) -> Option<&'a MnemonicDefinitionSignature>
	{
		match self.find_ascii_case_insensitively(mnemonic_name)
		{
			None => None,
			Some(mnemonic_definition) => mnemonic_definition.find_definition(assembling_for_architecture_variant, parsed_mnemonic_arguments),
		}
	}
	
	#[inline(always)]
	pub(crate) fn find_ascii_case_insensitively(&self, mnemonic_name: &String) -> Option<&MnemonicDefinition>
	{
		self.find_case_sensitively(&mnemonic_name.to_ascii_lowercase())
	}
	
	#[inline(always)]
	fn find_case_sensitively(&self, mnemonic_name: &str) -> Option<&MnemonicDefinition>
	{
		self.mnemonic_name_to_definition.get(mnemonic_name)
	}
}
