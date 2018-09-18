// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Represents a parsed instruction.
#[derive(Debug,  Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedInstruction
{
	prefixes: Vec<String>,
	mnemonic_name: String,
	arguments: ArrayVec<[ParsedMnemonicArgument; ParsedInstruction::MaximumMnemonicArguments]>,
}

impl ParsedInstruction
{
	pub(crate) const MaximumMnemonicArguments: usize = 8;
	
	/// Encodes an assembler instruction.
	#[inline(always)]
	pub fn encode_instruction<'a>(&mut self, assembling_for_architecture_variant: &AssemblingForArchitectureVariant, mnemonic_definitions: &'a MnemonicDefinitions, statements_buffer: &mut StatementsBuffer) -> Result<(), InstructionEncodingError>
	{
		//TODO:compile_instruction : Fold RawArgs into CleanArgs
		// 		let mut args = args.into_iter().map(|a| clean_memoryref(ecx, a)).collect::<Result<Vec<CleanArg>, _>>()?;
		
		// TODO: Mutates arguments; find a better way.
		let address_size = ParsedMnemonicArgument::validate_and_fill_in_memory_information(&mut self.arguments[..], assembling_for_architecture_variant)?;
		
		let signature = mnemonic_definitions.find_definition_ascii_case_insensitively(assembling_for_architecture_variant, &self.mnemonic_name, &self.arguments[..]).ok_or(InstructionEncodingError("No matching mnemonic definition for instruction"))?;
		
		// Push legacy segment register prefix.
		let legacy_prefix_modification =
		{
			let (legacy_prefix_modification, legacy_prefix_segment_register) = signature.repeat_and_segment_prefixes_if_any(&self.prefixes[..])?;
			statements_buffer.push_byte_if_some(legacy_prefix_segment_register);
			legacy_prefix_modification
		};
		
		// Push legacy address size prefix.
		{
			let address_size_override_prefix_required = signature.address_size_override_prefix_required(assembling_for_architecture_variant, address_size);
			const AddressSizeOverridePrefix: u8 = 0x67;
			statements_buffer.push_byte_if(address_size_override_prefix_required, AddressSizeOverridePrefix);
		}
		
		let (operand_size, sized_mnemonic_arguments) = signature.size_operands(&self.arguments[..])?;
		let (size_prefix_is_needed, rex_w_prefix_is_needed, vex_l_prefix_is_needed) = signature.determine_size_prefixes(operand_size, assembling_for_architecture_variant)?;
		let rex_prefix_is_needed = signature.check_if_combination_of_arguments_can_be_encoded_and_whether_a_rex_prefix_is_needed(assembling_for_architecture_variant, &sized_mnemonic_arguments[..], rex_w_prefix_is_needed)?;
		
		let (mut rm, reg, vvvv, ireg, mut remaining_arguments) = signature.extract_instruction_arguments(sized_mnemonic_arguments);
		
		let (immediate_opcode_byte, remaining_signature_opcode_bytes) = signature.immediate_opcode_and_remaining_opcodes();
		
		let remaining_signature_opcode_bytes = statements_buffer.push_vex_and_xop_prefixes_or_operand_size_modification_and_rex_prefixes(assembling_for_architecture_variant, signature, remaining_signature_opcode_bytes, size_prefix_is_needed, legacy_prefix_modification, rex_prefix_is_needed, rex_w_prefix_is_needed, vex_l_prefix_is_needed, &reg, &rm, &vvvv)?;
		
		statements_buffer.push_r_m_last_opcode_byte(signature, remaining_signature_opcode_bytes, &mut rm);
		
		let mut relocations = statements_buffer.push_addressing(assembling_for_architecture_variant.mode, signature, rm, reg, address_size);
		
		statements_buffer.push_immediate_opcode_byte_after_addressing_displacement(immediate_opcode_byte, &mut relocations);
		
		statements_buffer.push_register_in_immediate(ireg, &mut remaining_arguments, &mut relocations);
		
		statements_buffer.push_immediates(remaining_arguments, &mut relocations)?;
		
		relocations.push_to_statements_buffer(statements_buffer);
		
		Ok(())
	}
}
