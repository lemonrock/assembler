// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Officially known as `operands` rather than parameters, but official terminology is very confusing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct MnemonicDefinitionSignature
{
	parameters: Vec<MnemonicParameter>,
	
	// One or more opcode bytes.
	opcode_bytes: &'static [u8],
	
	register_identifier: Option<RegisterIdentifier>,
	
	instruction_flags: InstructionFlags,
	
	features_required: CpuFeatures,
}

impl MnemonicDefinitionSignature
{
	#[inline(always)]
	pub(crate) const fn new(_parameters: &'static [MnemonicParameter], _opcode_bytes: OpcodeBytes, _register_identifier: Option<RegisterIdentifier>, _instruction_flags: u32, _cpu_feature_required: Option<CpuFeature2>) -> Self
	{
		// TODO
		unimplemented!();
	}
	
	#[inline(always)]
	pub(crate) fn address_size_override_prefix_required(&self, assembling_for_architecture_variant: &AssemblingForArchitectureVariant, address_size: AddressSize) -> bool
	{
		assembling_for_architecture_variant.address_size_override_prefix_required(address_size) || self.contains_flags(InstructionFlags::PREF_67)
	}
	
	#[inline(always)]
	pub(crate) fn contains_flags(&self, instruction_flags: InstructionFlags) -> bool
	{
		self.instruction_flags.contains(instruction_flags)
	}
	
	#[inline(always)]
	pub(crate) fn intersects_flags(&self, instruction_flags: InstructionFlags) -> bool
	{
		self.instruction_flags.intersects(instruction_flags)
	}
	
	#[inline(always)]
	pub(crate) fn matches(&self, assembling_for_architecture_variant: &AssemblingForArchitectureVariant, parsed_mnemonic_arguments: &[ParsedMnemonicArgument]) -> bool
	{
		if assembling_for_architecture_variant.not_supported_in_operational_mode(self.instruction_flags.contains(InstructionFlags::X86_ONLY))
		{
			return false
		}
		
		if assembling_for_architecture_variant.does_not_support_one_or_more_features(self.features_required)
		{
			return false
		}
		
		if self.parameters.len() != parsed_mnemonic_arguments.len()
		{
			return false
		}
		
		for (parameter, argument) in self.parameters.iter().zip(parsed_mnemonic_arguments.iter())
		{
			use self::MnemonicParameterKind::*;
			use self::MnemonicParameterSize::*;
			use self::ParsedMnemonicArgument::*;
			use self::Size::*;
			
			let kind = parameter.kind;
			
			let size = match *argument
			{
				Immediate { size, .. } => match kind
				{
					immediate => size,
					instruction_offset => size,
					_ => return false,
				},
				
				JumpTarget { size, .. } => match kind
				{
					instruction_offset => size,
					_ => return false,
				},
				
				IndirectJumpTarget { size, .. } => match kind
				{
					memory => size,
					legacy_register_and_memory => size,
					mmx_register_and_memory => size,
					xmm_or_ymm_register_and_memory => size,
					_ => return false,
				},
				
				IndirectMemoryReference { size, ref index, .. } => match kind
				{
					memory | legacy_register_and_memory | mmx_register_and_memory | xmm_or_ymm_register_and_memory => if ParsedIndirectMemoryReferenceIndex::does_not_use_xmm_register(index)
					{
						size
					}
					else
					{
						return false
					},
					
					// 32-bit Vectorized SIB (Vector Scaled Index Byte) addressing.
					vector_scaled_index_byte_32 => if Size::is_none_or(size, DWORD)
					{
						match *index
						{
							Some(ParsedIndirectMemoryReferenceIndex { register, .. }) => register.some_size(),
							_ => return false,
						}
					}
					else
					{
						return false
					},
					
					// 64-bit Vectorized SIB (Vector Scaled Index Byte) addressing.
					vector_scaled_index_byte_64 => if Size::is_none_or(size, QWORD)
					{
						match *index
						{
							Some(ParsedIndirectMemoryReferenceIndex { register, .. }) => register.some_size(),
							_ => return false,
						}
					}
					else
					{
						return false
					},
					
					_ => return false,
				}
				
				DirectRegisterReference { register, .. } =>
				{
					macro_rules! validate_specific_register
					{
						($register: ident, $identifier: tt) =>
						{
							if $register.is(RegisterIdentifier::$identifier)
							{
								$register.some_size()
							}
							else
							{
								return false
							}
						}
					}
					
					macro_rules! validate_generic_register
					{
						($register: ident, $family: tt) =>
						{
							if $register.has_family(RegisterFamily::$family)
							{
								$register.some_size()
							}
							else
							{
								return false
							}
						}
					}
					
					match kind
					{
						// Specific references to registers.
						rax => validate_specific_register!(register, RAX),
						rcx => validate_specific_register!(register, RCX),
						rdx => validate_specific_register!(register, RDX),
						rbx => validate_specific_register!(register, RBX),
						rsp => validate_specific_register!(register, RSP),
						rbp => validate_specific_register!(register, RBP),
						rsi => validate_specific_register!(register, RSI),
						rdi => validate_specific_register!(register, RDI),
						r8 => validate_specific_register!(register, R8),
						r9 => validate_specific_register!(register, R9),
						r10 => validate_specific_register!(register, R10),
						r11 => validate_specific_register!(register, R11),
						r12 => validate_specific_register!(register, R12),
						r13 => validate_specific_register!(register, R13),
						r14 => validate_specific_register!(register, R14),
						r15 => validate_specific_register!(register, R15),
						es => validate_specific_register!(register, ES),
						cs => validate_specific_register!(register, CS),
						ss => validate_specific_register!(register, SS),
						ds => validate_specific_register!(register, DS),
						fs => validate_specific_register!(register, FS),
						gs => validate_specific_register!(register, GS),
						cr8 => validate_specific_register!(register, CR8),
						st0 => validate_specific_register!(register, ST0),
						
						// Generic legacy (general purpose) registers.
						legacy_register | legacy_register_and_memory => if register.is_legacy() || register.is_high_byte()
						{
							register.some_size()
						}
						else
						{
							return false
						},
						
						// All other generic registers.
						floating_pointer_register => validate_generic_register!(register, FP),
						mmx_register => validate_generic_register!(register, MMX),
						xmm_or_ymm_register => validate_generic_register!(register, XMM),
						segment_register => validate_generic_register!(register, SEGMENT),
						control_register => validate_generic_register!(register, CONTROL),
						debug_register => validate_generic_register!(register, DEBUG),
						bound_register => validate_generic_register!(register, BOUND),
						
						_ => return false,
					}
				}
			};
			
			match size
			{
				Some(size) => if !match (parameter.size, kind)
				{
					// An immediate can always fit in a larger register.
					(Fixed(WORD), immediate) => size <= WORD,
					(Fixed(DWORD), immediate) => size <= DWORD,
					(Fixed(QWORD), immediate) => size <= QWORD,
					(AllPossible, immediate) => size <= DWORD,
					
					// Fixed size matches.
					(Fixed(BYTE), _) => size == BYTE,
					(Fixed(WORD), _) => size == WORD,
					(Fixed(DWORD), _) => size == DWORD,
					(Fixed(FWORD), _) => size == FWORD,
					(Fixed(QWORD), _) => size == QWORD,
					(Fixed(PWORD), _) => size == PWORD,
					(Fixed(OWORD), _) => size == OWORD,
					(Fixed(HWORD), _) => size == HWORD,
					
					// Wildcard match.
					(AllPossible, vector_scaled_index_byte_32) => size.is_oword_or_hword(),
					(AllPossible, vector_scaled_index_byte_64) => size.is_oword_or_hword(),
					(AllPossible, xmm_or_ymm_register) => size.is_oword_or_hword(),
					(AllPossible, xmm_or_ymm_register_and_memory) => size.is_oword_or_hword(),
					(AllPossible, legacy_register) => size.is_word_or_dword_or_qword(),
					(AllPossible, legacy_register_and_memory) => size.is_word_or_dword_or_qword(),
					(AllPossible, rax) => size.is_word_or_dword_or_qword(),
					(AllPossible, rcx) => size.is_word_or_dword_or_qword(),
					(AllPossible, rdx) => size.is_word_or_dword_or_qword(),
					(AllPossible, rbx) => size.is_word_or_dword_or_qword(),
					(AllPossible, rsp) => size.is_word_or_dword_or_qword(),
					(AllPossible, rbp) => size.is_word_or_dword_or_qword(),
					(AllPossible, rsi) => size.is_word_or_dword_or_qword(),
					(AllPossible, rdi) => size.is_word_or_dword_or_qword(),
					(AllPossible, r8) => size.is_word_or_dword_or_qword(),
					(AllPossible, r9) => size.is_word_or_dword_or_qword(),
					(AllPossible, r10) => size.is_word_or_dword_or_qword(),
					(AllPossible, r11) => size.is_word_or_dword_or_qword(),
					(AllPossible, r12) => size.is_word_or_dword_or_qword(),
					(AllPossible, r13) => size.is_word_or_dword_or_qword(),
					(AllPossible, r14) => size.is_word_or_dword_or_qword(),
					(AllPossible, r15) => size.is_word_or_dword_or_qword(),
					(AllPossible, memory) => true,
					(AllPossible, _) => panic!("Invalid size wildcard"),
					
					(Any, _) => true,
					
					(NoSize, _) => false,
				}
				{
					return false
				},
				
				None => if parameter.size != AllPossible && self.instruction_flags.contains(InstructionFlags::EXACT_SIZE)
				{
					// Basically, this format is a more specific version of an instruction that also has more general versions.
					// This should only be picked if the size constraints are met - and not if the size is unspecified.
					return false
				},
			}
		}
		
		true
	}
	
	#[inline(always)]
	pub(crate) fn repeat_and_segment_prefixes_if_any(&self, prefixes: &[String]) -> Result<(Option<u8>, Option<u8>), InstructionEncodingError>
	{
		macro_rules! assign_repeat_prefix
		{
			($self: ident, $repeat_prefix: ident, $opcode: expr, $flags: tt, $message: expr) =>
			{
				{
					if !$self.contains_flags(InstructionFlags::$flags)
					{
						return Err(InstructionEncodingError($message))
					}
				
					if $repeat_prefix.is_some()
					{
						return Err(InstructionEncodingError("Duplicate repeat prefix; only one of 'REP', 'REPE', 'REPNZ', 'REPNE' or 'LOCK' is allowed"))
					}
					
					$repeat_prefix = Some($opcode)
				}
			}
		}
		
		macro_rules! assign_segment_prefix
		{
			($segment_prefix: ident, $opcode: expr) =>
			{
				{
					if $segment_prefix.is_some()
					{
						return Err(InstructionEncodingError("Duplicate segment prefix; only one of 'SS', 'CS', 'DS', 'ES', 'FS' or 'GS' is allowed"))
					}
					
					$segment_prefix = Some($opcode)
				}
			}
		}
		
		let mut repeat_prefix = None;
		let mut segment_prefix = None;
		
		for prefix in prefixes.iter()
		{
			match prefix.as_str()
			{
				"rep" => assign_repeat_prefix!(self, repeat_prefix, 0xF3, REP, "Can not use prefix 'REP' on this instruction"),
				
				"repe" | "repz" => assign_repeat_prefix!(self, repeat_prefix, 0xF3, REPE, "Can not use prefix 'REPE' or 'REPZ' on this instruction"),
				
				"repnz" | "repne" => assign_repeat_prefix!(self, repeat_prefix, 0xF2, REPE, "Can not use prefix 'REPNZ' or 'REPNE' on this instruction"),
				
				"lock" => assign_repeat_prefix!(self, repeat_prefix, 0xF0, LOCK, "Can not use prefix 'LOCK' on this instruction"),
				
				"ss" => assign_segment_prefix!(segment_prefix, 0x36),
				
				"cs" => assign_segment_prefix!(segment_prefix, 0x2E),
				
				"ds" => assign_segment_prefix!(segment_prefix, 0x3E),
				
				"es" => assign_segment_prefix!(segment_prefix, 0x26),
				
				"fs" => assign_segment_prefix!(segment_prefix, 0x64),
				
				"gs" => assign_segment_prefix!(segment_prefix, 0x65),
				
				_ => return Err(InstructionEncodingError("Unsupported prefix"))
			}
		}
		
		if self.contains_flags(InstructionFlags::PREF_F0)
		{
			repeat_prefix = Some(0xF0);
		}
		else if self.contains_flags(InstructionFlags::PREF_F2)
		{
			repeat_prefix = Some(0xF2);
		}
		else if self.contains_flags(InstructionFlags::PREF_F3)
		{
			repeat_prefix = Some(0xF3);
		}
		
		Ok((repeat_prefix, segment_prefix))
	}
	
	#[inline(always)]
	pub(crate) fn determine_size_prefixes(&self, operand_size: Option<Size>, assembling_for_architecture_variant: &AssemblingForArchitectureVariant) -> Result<(bool, bool, bool), InstructionEncodingError>
	{
		use self::Size::*;
		use self::SupportedOperationalMode::*;
		
		macro_rules! validate_operand_size
		{
			($operand_size: ident, $assembling_for_architecture_variant: ident) =>
			{
				{
					let operand_size = $operand_size.expect("operand size should have been calculated");
					
					if $assembling_for_architecture_variant.is_for_protected_mode() && operand_size.is_64_bit()
					{
						return Err(InstructionEncodingError("mnemonic does not support 64-bit operands in 32-bit mode"))
					}
					
					operand_size
				}
			}
		}
		
		macro_rules! invalid_mnemonic_argument_definition
		{
			() =>
			{
				panic!("Invalid mnemonic argument definition")
			}
		}
		
		let ok = match self.instruction_flags & InstructionFlags::OneOfTheAutomaticFlagsIsSet
		{
			InstructionFlags::AUTO_NO32 =>
			{
				match (validate_operand_size!(operand_size, assembling_for_architecture_variant), assembling_for_architecture_variant.mode)
				{
					(WORD, _) => (true, self.force_rex_w_prefix(), self.force_vex_l_prefix()),
					
					(QWORD, Long) => self.force_all_prefixes(),
					
					(DWORD, Protected) => self.force_all_prefixes(),
					
					(DWORD, Long) => return Err(InstructionEncodingError("mnemonic does not support 64-bit operands in 32-bit mode")),
					
					(_, _) => invalid_mnemonic_argument_definition!(),
				}
			}
			
			InstructionFlags::AUTO_REXW =>
			{
				match validate_operand_size!(operand_size, assembling_for_architecture_variant)
				{
					QWORD => (self.force_size_prefix(), true, self.force_vex_l_prefix()),
					
					DWORD => self.force_all_prefixes(),
					
					_ => return Err(InstructionEncodingError("mnemonic does not support 16-bit operands")),
				}
			}
			
			InstructionFlags::AUTO_VEXL =>
			{
				match validate_operand_size!(operand_size, assembling_for_architecture_variant)
				{
					HWORD => (self.force_size_prefix(), self.force_rex_w_prefix(), true),
					
					OWORD => self.force_all_prefixes(),
					
					_ => invalid_mnemonic_argument_definition!(),
				}
			},
			
			InstructionFlags::AUTO_SIZE =>
			{
				match validate_operand_size!(operand_size, assembling_for_architecture_variant)
				{
					WORD => (true, self.force_rex_w_prefix(), self.force_vex_l_prefix()),
					
					QWORD => (self.force_size_prefix(), true, self.force_vex_l_prefix()),
					
					DWORD => self.force_all_prefixes(),
					
					_ => invalid_mnemonic_argument_definition!(),
				}
			}
			
			_ => self.force_all_prefixes(),
		};
		Ok(ok)
	}
	
	#[inline(always)]
	pub(crate) fn size_operands(&self, parsed_mnemonic_arguments: &[ParsedMnemonicArgument]) -> Result<(Option<Size>, ArrayVec<[SizedMnemonicArgument; ParsedInstruction::MaximumMnemonicArguments]>), InstructionEncodingError>
	{
		use self::ParsedMnemonicArgument::*;
		
		// First pass: Determine the effective operand sizes for AllPossible (wildcard) values.
		// Will re-write the value of `size` in some IndirectMemoryReference instances if they use Vector Scaled Index Byte addressing.
		let (operand_size, immediate_size) =
		{
			let mut operand_size = None;
			let mut immediate_size = None;
			let mut parsed_at_least_one_argument_with_a_potential_operand_size = false;
			
			for (_, parsed_mnemonic_argument) in self.parameters.iter().zip(parsed_mnemonic_arguments.iter()).filter(|(parameter, _)| parameter.is_all_possible())
			{
				#[inline(always)]
				fn validate_operand_size_is_same_for_all_arguments_this_operand_size_may_be_none(operand_size: &mut Option<Size>, this_operand_size: Option<Size>) -> Result<(), InstructionEncodingError>
				{
					if let Some(this_operand_size) = this_operand_size
					{
						validate_operand_size_is_same_for_all_arguments(operand_size, this_operand_size)
					}
					else
					{
						Ok(())
					}
				}
				
				#[inline(always)]
				fn validate_operand_size_is_same_for_all_arguments(operand_size: &mut Option<Size>, this_operand_size: Size) -> Result<(), InstructionEncodingError>
				{
					if let Some(previous_operand_size) = *operand_size
					{
						if previous_operand_size != this_operand_size
						{
							return Err(InstructionEncodingError("Operand sizes vary for each parsed_mnemonic_argument"));
						}
					}
					else
					{
						*operand_size = Some(this_operand_size);
					}
					
					Ok(())
				}
				
				match *parsed_mnemonic_argument
				{
					DirectRegisterReference { register, .. } =>
					{
						parsed_at_least_one_argument_with_a_potential_operand_size = true;
						
						validate_operand_size_is_same_for_all_arguments(&mut operand_size, register.size())?
					}
					
					IndirectJumpTarget { size, .. } =>
					{
						parsed_at_least_one_argument_with_a_potential_operand_size = true;
						
						validate_operand_size_is_same_for_all_arguments_this_operand_size_may_be_none(&mut operand_size, size)?
					}
					
					IndirectMemoryReference { mut size, ref index, ..} =>
					{
						parsed_at_least_one_argument_with_a_potential_operand_size = true;
						
						// Vector Scaled Index Byte addressing.
						if let Some(ParsedIndirectMemoryReferenceIndex { ref register, .. }) = *index
						{
							if register.is_xmm()
							{
								size = Some(register.size());
							}
						}
						
						validate_operand_size_is_same_for_all_arguments_this_operand_size_may_be_none(&mut operand_size, size)?
					},
					
					Immediate { size, .. } | JumpTarget { size, .. } =>
					{
						debug_assert!(immediate_size.is_none(), "Multiple immediate values with wildcard size");
						immediate_size = size;
					}
				}
			}
			
			Size::validate_and_cap_sizes(operand_size, immediate_size, parsed_at_least_one_argument_with_a_potential_operand_size)?
		};
		
		// Second pass: Create sized arguments now that there are not wildcard arguments.
		let mut sized_arguments = ArrayVec::new();
		for (parameter, parsed_mnemonic_argument) in self.parameters.iter().zip(parsed_mnemonic_arguments.iter())
		{
			let size = parameter.to_size(operand_size, immediate_size);
			sized_arguments.push(SizedMnemonicArgument::from_parsed(parsed_mnemonic_argument, size));
		}
		
		Ok((operand_size, sized_arguments))
	}
	
	#[inline(always)]
	pub(crate) fn check_if_combination_of_arguments_can_be_encoded_and_whether_a_rex_prefix_is_needed(&self, assembling_for_architecture_variant: &AssemblingForArchitectureVariant, sized_mnemonic_arguments: &[SizedMnemonicArgument], rex_w_prefix_is_needed: bool) -> Result<bool, InstructionEncodingError>
	{
		if assembling_for_architecture_variant.is_for_protected_mode()
		{
			if rex_w_prefix_is_needed
			{
				return Err(InstructionEncodingError("Does not support 64 bit operand size in 32-bit mode"))
			}
			else
			{
				return Ok(false)
			}
		}
		
		let mut requires_rex_prefix= rex_w_prefix_is_needed;
		let mut does_not_require_rex_prefix = false;
		
		for (sized_mnemonic_argument, _) in sized_mnemonic_arguments.iter().zip(self.parameters.iter()).filter(|(_, parameter)| parameter.kind.is_encoded())
		{
			use self::SizedMnemonicArgument::*;
			
			match *sized_mnemonic_argument
			{
				DirectRegisterReference { register, ..} =>
				{
					if register.is_high_byte()
					{
						does_not_require_rex_prefix = true;
						
					}
					else if register.is_byte_sized_and_rsp_rbp_rsi_or_rdi() || register.is_extended()
					{
						requires_rex_prefix = true;
					}
				},
				
				IndirectMemoryReference { base, ref index, .. } =>
				{
					if let Some(ref register) = base
					{
						requires_rex_prefix = requires_rex_prefix || register.is_extended()
					}
					
					if let Some(ParsedIndirectMemoryReferenceIndex { register, .. }) = *index
					{
						requires_rex_prefix = requires_rex_prefix || register.is_extended()
					}
				},
				
				_ => (),
			}
		}
		
		if requires_rex_prefix && does_not_require_rex_prefix
		{
			Err(InstructionEncodingError("High byte register combined with extended registers or 64-bit operand size"))
		}
		else
		{
			Ok(requires_rex_prefix)
		}
	}
	
	#[inline(always)]
	pub(crate) fn extract_instruction_arguments(&self, sized_mnemonic_arguments: ArrayVec<[SizedMnemonicArgument; ParsedInstruction::MaximumMnemonicArguments]>) -> (Option<SizedMnemonicArgument>, Option<SizedMnemonicArgument>, Option<SizedMnemonicArgument>, Option<SizedMnemonicArgument>, ArrayVec<[SizedMnemonicArgument; 8]>)
	{
		// Operand order:-
		//
		// * if there's a memory/reg operand, this operand goes into ModRM.r/m.
		// * if there's a segment, control or debug register_argument_index, it goes into reg.
		//
		// Default argument encoding order is as follows:-
		//
		// * no encoding flag: `m`, `rm`, `rvm`, `rvim`.
		// * ENC_MR: `mr`, `rmv`, `rvmi`.
		// * ENC_VM: `vm`, `mvr`.
		//
		// These can also be chosen based on the location of a memory argument (except for `vm`).
		
		let mut memory_argument_index = None;
		let mut register_argument_index = None;
		let mut registers = ArrayVec::<[SizedMnemonicArgument; 4]>::new();
		let mut immediates = ArrayVec::<[SizedMnemonicArgument; 8]>::new();
		
		for (sized_mnemonic_argument, parameter) in sized_mnemonic_arguments.into_iter().zip(self.parameters.iter())
		{
			use self::MnemonicParameterKind::*;
			
			match parameter.kind
			{
				immediate | instruction_offset => immediates.push(sized_mnemonic_argument),
				
				floating_pointer_register | mmx_register | legacy_register | xmm_or_ymm_register | bound_register => registers.push(sized_mnemonic_argument),
				
				control_register | debug_register | segment_register =>
				{
					debug_assert!(register_argument_index.is_none(), "more than one control, debug or segment register_argument_index in mnemonic parameters definition");
					register_argument_index = Some(registers.len());
					registers.push(sized_mnemonic_argument)
				}
				
				memory | mmx_register_and_memory | legacy_register_and_memory | xmm_or_ymm_register_and_memory | vector_scaled_index_byte_32 | vector_scaled_index_byte_64 =>
				{
					debug_assert!(memory_argument_index.is_none(), "more than one memory argument in mnemonic parameters definition");
					memory_argument_index = Some(registers.len());
					registers.push(sized_mnemonic_argument)
				}
				
				// Hardcoded registers RAX - R15, ES - GS, CR8 and ST0 are not encoded.
				_ => (),
			}
		}
		
		let number_of_registers = registers.len();
		
		let mut register_or_none = registers.drain(..).fuse();
		
		if let Some(register_argument_index) = register_argument_index
		{
			if register_argument_index == 0
			{
				let r = register_or_none.next();
				let m = register_or_none.next();
				(m, r, None, None, immediates)
			}
			else
			{
				let m = register_or_none.next();
				let r = register_or_none.next();
				(m, r, None, None, immediates)
			}
		}
		else
		{
			match number_of_registers
			{
				0 => (None, None, None, None, immediates),
				
				1 => (register_or_none.next(), None, None, None, immediates),
				
				2 =>
				{
					if self.contains_flags(InstructionFlags::ENC_MR) || memory_argument_index == Some(0)
					{
						(register_or_none.next(), register_or_none.next(), None, None, immediates)
					}
					else if self.contains_flags(InstructionFlags::ENC_VM)
					{
						let v = register_or_none.next();
						let m = register_or_none.next();
						(m, None, v, None, immediates)
					}
					else
					{
						let r = register_or_none.next();
						let m = register_or_none.next();
						(m, r, None, None, immediates)
					}
				}
				
				3 =>
				{
					if self.contains_flags(InstructionFlags::ENC_MR) || memory_argument_index == Some(1)
					{
						let r = register_or_none.next();
						let m = register_or_none.next();
						let v = register_or_none.next();
						(m, r, v, None, immediates)
					}
					else if self.contains_flags(InstructionFlags::ENC_VM) || memory_argument_index == Some(0)
					{
						let m = register_or_none.next();
						let v = register_or_none.next();
						let r = register_or_none.next();
						(m, r, v, None, immediates)
					}
					else
					{
						let r = register_or_none.next();
						let v = register_or_none.next();
						let m = register_or_none.next();
						(m, r, v, None, immediates)
					}
				}
				
				4 =>
				{
					if self.contains_flags(InstructionFlags::ENC_MR) || memory_argument_index == Some(2)
					{
						let r = register_or_none.next();
						let v = register_or_none.next();
						let m = register_or_none.next();
						let i = register_or_none.next();
						(m, r, v, i, immediates)
					}
					else
					{
						let r = register_or_none.next();
						let v = register_or_none.next();
						let i = register_or_none.next();
						let m = register_or_none.next();
						(m, r, v, i, immediates)
					}
				}
				
				_ => panic!("A mnemonic may not have more than 4 register_argument_index arguments"),
			}
		}
	}
	
	/// Does this mnemonic signature have an immediate opcode?
	#[inline(always)]
	pub(crate) fn immediate_opcode_and_remaining_opcodes(&self) -> (Option<u8>, &[u8])
	{
		if self.intersects_flags(InstructionFlags::IMM_OP)
		{
			let (immediate_opcode_byte, head) = self.opcode_bytes.split_last().expect("invalid mnemonic signature parameters; expected at least an `immediate_opcode_byte` at the end of the signature opcodes because the mnemonic signature has the instruction flag `IMM_OP`");
			(Some(*immediate_opcode_byte), head)
		}
		else
		{
			(None, self.opcode_bytes)
		}
	}
	
	#[inline(always)]
	pub(crate) fn reg_k(&self, reg: Option<SizedMnemonicArgument>) -> RegisterIdentifier
	{
		match reg
		{
			Some(SizedMnemonicArgument::DirectRegisterReference { register, .. }) => register.identifier(),
			_ => self.register_identifier(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn register_identifier(&self) -> RegisterIdentifier
	{
		self.register_identifier.expect("invalid mnemonic definition")
	}
	
	#[inline(always)]
	fn force_all_prefixes(&self) -> (bool, bool, bool)
	{
		(self.force_size_prefix(), self.force_rex_w_prefix(), self.force_vex_l_prefix())
	}
	
	#[inline(always)]
	fn force_size_prefix(&self) -> bool
	{
		self.contains_flags(InstructionFlags::WORD_SIZE)
	}
	
	#[inline(always)]
	fn force_rex_w_prefix(&self) -> bool
	{
		self.contains_flags(InstructionFlags::WITH_REXW)
	}
	
	#[inline(always)]
	fn force_vex_l_prefix(&self) -> bool
	{
		self.contains_flags(InstructionFlags::WITH_VEXL)
	}
}
