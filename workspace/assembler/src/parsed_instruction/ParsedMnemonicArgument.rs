// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A parsed mnemonic argument.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum ParsedMnemonicArgument
{
	/// An arbitrary immediate.
	Immediate
	{
		/// Value.
		value: RustExpression,
		
		/// A size hint.
		size: Option<Size>,
	},
	
	/// A jump offset, eg `->foo`.
	JumpTarget
	{
		/// Jump variant.
		jump_target_reference: JumpTarget,
		
		/// A size hint.
		size: Option<Size>,
	},
	
	/// A memory reference to a label, eg `[->foo]`.
	IndirectJumpTarget
	{
		/// Jump variant.
		jump_target_reference: JumpTarget,
		
		/// A size hint.
		size: Option<Size>,
	},
	
	/// A direct register reference.
	DirectRegisterReference
	{
		/// Register; encodes identifier, family and size.
		register: Register,
	},

	/// An indirect memory reference, ie a reference to a memory address by looking at a value in a register.
	IndirectMemoryReference
	{
		/// Do not split (optimize) SIB offset indices.
		no_split: bool,
		
		/// Size hint.
		size: Option<Size>,
		
		/// Displacement size hint.
		displacement_size: Option<Size>,
		
		/// Base register for adding to address (offset).
		base: Option<Register>,
		
		/// Index register to use in displacement array.
		index: Option<ParsedIndirectMemoryReferenceIndex>,
		
		/// Displacement.
		displacement: Option<RustExpression>
	},
}

impl ParsedMnemonicArgument
{
	/// * Validates indirect memory references can be encoded;
	/// * Fills in immediate size information where missing;
	/// * Fills in displacement size information where missing;
	/// * Finds the effective address size and checks it is acceptable;
	pub(crate) fn validate_and_fill_in_memory_information(parsed_mnemonic_arguments: &mut [Self], assembling_for_architecture_variant: &AssemblingForArchitectureVariant) -> Result<AddressSize, InstructionEncodingError>
	{
		let mut already_seen_a_memory_reference = false;
		let mut address_size = assembling_for_architecture_variant.default_address_size();
		
		for parsed_mnemonic_argument in parsed_mnemonic_arguments.iter_mut()
		{
			use self::ParsedMnemonicArgument::*;
			
			match *parsed_mnemonic_argument
			{
				Immediate { ref value, size: ref mut size @ None } => Self::derive_size_for_immediate(size, value),
			
				IndirectMemoryReference { no_split, ref mut displacement_size, ref mut base, ref mut index, ref displacement, .. } =>
				{
					Self::guard_only_one_memory_reference_mnemonic_argument(&mut already_seen_a_memory_reference)?;
					
					let effective_address_size = Self::sanitize_indirect_memory_reference(assembling_for_architecture_variant, no_split, base, index)?;
					Size::assign_if_size_valid_as_address_size(effective_address_size, &mut address_size);
					
					if let Some(ParsedIndirectMemoryReferenceIndex { scale, .. }) = *index
					{
						if ParsedIndirectMemoryReferenceIndex::scale_is_invalid(scale)
						{
							return Err(InstructionEncodingError("Scale is not 1, 2, 4 or 8"))
						}
					}
					
					use self::Size::*;
					
					match *displacement_size
					{
						// Validate `displacement_size`.
						Some(displacement_size) => match displacement
						{
							None => return Err(InstructionEncodingError("Invalid displacement size as there is no corresponding displacement")),
							
							Some(_) => if Size::is_16_bit_addressing_option(effective_address_size)
							{
								if !displacement_size.is_byte_or_word()
								{
									return Err(InstructionEncodingError("Invalid displacement size for 16-bit addressing, only BYTE or WORD is possible"))
								}
							}
							else
							{
								if !displacement_size.is_byte_or_dword()
								{
									return Err(InstructionEncodingError("Invalid displacement size for addressing, only BYTE or DWORD is possible"))
								}
							},
						},
						
						// No `displacement_size`; work it out.
						None => match *displacement
						{
							None => (),
							
							Some(ref displacement) =>
							{
								match displacement.derive_size()
								{
									Some(BYTE) =>
									{
										*displacement_size = Some(BYTE)
									},
									
									Some(_) =>
									{
										*displacement_size = Some
										(
											if Size::is_16_bit_addressing_option(effective_address_size)
											{
												WORD
											}
											else
											{
												DWORD
											}
										)
									}
									
									None => (),
								}
							}
						}
					}
				}
				
				_ => (),
			}
		}
		
		Ok(address_size)
	}
	
	#[inline(always)]
	fn derive_size_for_immediate(size: &mut Option<Size>, immediate: &RustExpression)
	{
		*size = immediate.derive_size()
	}
	
	#[inline(always)]
	fn guard_only_one_memory_reference_mnemonic_argument(already_seen_a_memory_reference: &mut bool) -> Result<(), InstructionEncodingError>
	{
		if *already_seen_a_memory_reference
		{
			return Err(InstructionEncodingError("More than one parsed mnemonic argument references memory in this instruction"))
		}
		*already_seen_a_memory_reference = true;
		Ok(())
	}
	
	/// Addressing mode can be:-
	///
	/// * Legacy and either 16, 32 or 64 bit;
	/// * RIP-relative and either 32 or 64 bit;
	/// * Vector Scaled Index Byte and either 32 or 64 bit;
	#[inline(always)]
	fn sanitize_indirect_memory_reference(assembling_for_architecture_variant: &AssemblingForArchitectureVariant, no_split: bool, base_register: &mut Option<Register>, index: &mut Option<ParsedIndirectMemoryReferenceIndex>) -> Result<Option<Size>, InstructionEncodingError>
	{
		use self::RegisterFamily::*;
		use self::RegisterIdentifier::*;
		use self::Size::*;
		
		let (register_family, register_size, vector_scaled_index_byte_addressing_mode_with_base) = match Register::base_and_index_to_family_size_and_vector_scaled_index_byte_mode(*base_register, index.as_ref().map(|index| index.register))?
		{
			None => return Ok(None),
			Some(values) => values,
		};
		
		// filter out combinations that are impossible to encode
		match register_family
		{
			LEGACY => match register_size
			{
				DWORD | QWORD => (),
				
				WORD => if assembling_for_architecture_variant.is_for_protected_mode()
				{
					return Err(InstructionEncodingError("16-bit addressing is not supported for legacy registers in protected mode"))
				}
				else if vector_scaled_index_byte_addressing_mode_with_base
				{
					return Err(InstructionEncodingError("16-bit addressing is not supported for legacy registers in Vector Scaled Index Byte mode"))
				},
				
				_ => return Err(InstructionEncodingError("Legacy register type not supported for register size")),
			},
			
			// RIP-relative addressing.
			RegisterFamily::RIP =>
			{
				if base_register.is_some() && index.is_some()
				{
					return Err(InstructionEncodingError("RIP-relative addressing does not support a base register and an index register combination"))
				}
				
				match index.take()
				{
					Some(ParsedIndirectMemoryReferenceIndex { register, scale: 1, expression: None }) => *base_register = Some(register),
					
					Some(_) => return Err(InstructionEncodingError("RIP-relative addressing can have a scale")),
					
					None => ()
				}
				
				return Ok(Some(register_size))
			},
			
			// Vector Scaled Index Byte addressing *without* base.
			XMM =>
			{
				if base_register.is_some() && index.is_some()
				{
					return Err(InstructionEncodingError("Vector Scaled Index Byte addressing does not support a base register and an index register combination"))
				}
				
				if let Some(register) = base_register.take()
				{
					*index = Some(ParsedIndirectMemoryReferenceIndex { register, scale: 1, expression: None });
				}
				
				return Ok(None)
			},
			
			_ => return Err(InstructionEncodingError("Register family not supported for addressing")),
		}
		
		// Vector Scaled Index Byte addressing *with* base; one register is XMM, the other will be LEGACY.
		if vector_scaled_index_byte_addressing_mode_with_base
		{
			// we're guaranteed that the other register is a legacy register, either DWORD or QWORD register_size
			// so we just have to check if an index/base swap is necessary
			
			// Unwrap will always succeed because of the logic inside ParsedRegister::base_and_index_to_family_size_and_vector_scaled_index_byte_mode that identifies `vector_scaled_index_byte_addressing_mode_with_base`.
			if base_register.unwrap().is_xmm()
			{
				match index
				{
					Some(ParsedIndirectMemoryReferenceIndex { ref mut register, scale: 1, expression: None }) =>
					{
						swap(register, base_register.as_mut().unwrap())
					}
					
					_ => return Err(InstructionEncodingError("Vector Scaled Index Byte addressing with a base requires a legacy (general purpose) register as a base"))
				}
			}
		}
		// Legacy 16-bit addressing.
		else if register_size.is_16_bit_addressing()
		{
			let mut second_register = match index.take()
			{
				Some(ParsedIndirectMemoryReferenceIndex { register, scale: 1, expression: None }) => Some(register),
				
				None => None,
				
				Some(_) => return Err(InstructionEncodingError("16-bit addressing has no concept of a scaled index")),
			};
			
			let mut first_register = base_register.take();
			
			if first_register.is_none()
			{
				first_register = second_register.take();
			}
			
			let encoded_base = match (first_register.map(|register| register.register_identifier), second_register.map(|register| register.register_identifier))
			{
				(Some(RBX), Some(RSI)) => RAX,
				(Some(RSI), Some(RBX)) => RAX,
				
				(Some(RBX), Some(RDI)) => RCX,
				(Some(RDI), Some(RBX)) => RCX,
				
				(Some(RBP), Some(RSI)) => RDX,
				(Some(RSI), Some(RBP)) => RDX,
				
				(Some(RBP), Some(RDI)) => RBX,
				(Some(RDI), Some(RBP)) => RBX,
				
				(Some(RSI), None) => RSP,
				
				(Some(RDI), None) => RBP,
				
				(Some(RBP), None) => RSI,
				
				(Some(RBX), None) => RDI,
				
				_ => return Err(InstructionEncodingError("Impossible register combination for 16-bit addressing")),
			};
			
			*base_register = Some
			(
				Register
				{
					size: WORD,
					register_identifier: encoded_base,
				}
			)
		}
		// Common 32/64-bit addressing.
		else
		{
			Self::optimize_index(no_split, base_register, index);
			
			let mut borrow_checker_work_around_new_value_for_index = None;
			match index
			{
				// The RSP register can not be used as an index, only as a base field.
				&mut Some(ref mut index_) => if index_.register.is(RSP)
				{
					let base_register_is_not_rsp = if let Some(base_register) = base_register
					{
						base_register.is_not(RSP)
					}
					else
					{
						false
					};
					
					if base_register_is_not_rsp && index_.is_suitable_for_base_register()
					{
						borrow_checker_work_around_new_value_for_index = base_register.take().map(|register| ParsedIndirectMemoryReferenceIndex::new_for_base_register(register));
						*base_register = Some(index_.register)
					}
					else
					{
						return Err(InstructionEncodingError("Register RSP cannot be used as index field"))
					}
				},
				
				&mut None => if let Some(base_register) = base_register
				{
					// The RSP or R12 registers need an index to escape the Scale Index Byte.
					if base_register.is(RSP) || base_register.is(R12)
					{
						*index = Some
						(
							ParsedIndirectMemoryReferenceIndex::new_for_base_register
							(
								Register
								{
									size: register_size,
									register_identifier: RSP,
								}
							)
						)
					}
				},
			}
			if let Some(borrow_checker_work_around_new_value_for_index) = borrow_checker_work_around_new_value_for_index
			{
				*index = Some(borrow_checker_work_around_new_value_for_index);
			}
		}
		Ok(Some(register_size))
	}
	
	#[inline(always)]
	fn optimize_index(no_split: bool, base_register: &mut Option<Register>, index: &mut Option<ParsedIndirectMemoryReferenceIndex>)
	{
		let allowed_to_try_to_optimize_index = !no_split;
		
		if allowed_to_try_to_optimize_index && base_register.is_none()
		{
			if let Some(ParsedIndirectMemoryReferenceIndex { register, ref mut scale, expression: None }) = *index
			{
				#[inline(always)]
				fn optimize_scale_by_moving_something_into_empty_base_register(reduced_scale: isize, base_register: &mut Option<Register>, register: Register, scale: &mut isize)
				{
					*base_register = Some(register);
					*scale = reduced_scale;
				}
				
				// The 'permitted' scales are 1, 2, 4 and 8, but if no base ie present (`base_register.is_none()`), we can shift one offset into the base and so convert scales 2, 3, 5 and 9 to 1, 2, 4, and 8.
				match *scale
				{
					2 => optimize_scale_by_moving_something_into_empty_base_register(1, base_register, register, scale),
					3 => optimize_scale_by_moving_something_into_empty_base_register(2, base_register, register, scale),
					5 => optimize_scale_by_moving_something_into_empty_base_register(4, base_register, register, scale),
					9 => optimize_scale_by_moving_something_into_empty_base_register(8, base_register, register, scale),
					
					_ => ()
				}
			}
		}
	}
}
