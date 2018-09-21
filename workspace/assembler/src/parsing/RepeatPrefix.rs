// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RepeatPrefix
{
	/// Repeat.
	rep,
	
	/// Overlaps with `repz`.
	repe_or_repz,
	
	/// Overlaps with `repne`.
	repnz_or_repne,

	/// Lock.
	lock,
}

impl RepeatPrefix
{
	#[inline(always)]
	pub(crate) fn parse_prefix(potential_prefix_tag_already_in_ascii_lower_case: &str) -> Option<RepeatPrefix>
	{
		use self::RepeatPrefix::*;
		
		static KnownPrefixes: Map<&'static str, RepeatPrefix> = phf_map!
		{
			"rep" => rep,
			"repe" => repe_or_repz,
			"repz" => repe_or_repz,
			"repnz" => repnz_or_repne,
			"repne" => repnz_or_repne,
			"lock" => lock,
		};
		
		KnownPrefixes.get(potential_prefix_tag_already_in_ascii_lower_case).map(|reference| *reference)
	}
	
	#[inline(always)]
	pub(crate) fn assign(this: Option<Self>, signature: MnemonicDefinitionSignature) -> Result<Option<u8>, InstructionEncodingError>
	{
		use self::RepeatPrefix::*;
		
		if signature.contains_flags(InstructionFlag::pref_F0)
		{
			Ok(Some(0xF0))
		}
		else if signature.contains_flags(InstructionFlag::pref_F2)
		{
			Ok(Some(0xF2))
		}
		else if signature.contains_flags(InstructionFlag::pref_F3)
		{
			Ok(Some(0xF3))
		}
		else
		{
			match this
			{
				None => Ok(None),
				Some(repeat_prefix) =>
				{
					let (prefix_byte, mnemonic_definition_signature_must_support) = match repeat_prefix
					{
						rep => (0xF3, InstructionFlag::rep),
						repe_or_repz => (0xF3, InstructionFlag::repe),
						repnz_or_repne => (0xF2, InstructionFlag::repe),
						lock => (0xF0, InstructionFlag::lock),
					};
					
					if !signature.contains_flags(mnemonic_definition_signature_must_support)
					{
						return Err(InstructionEncodingError("This instruction does not support this repeat prefix"))
					}
					
					Ok(Some(prefix_byte))
				}
			}
		}
	}
}
