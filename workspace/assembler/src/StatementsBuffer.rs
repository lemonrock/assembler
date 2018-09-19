// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A list of 'statements' - encoded bytes and the like.
#[derive(Debug)]
pub struct StatementsBuffer<W: Write>
{
	instruction_stream: InstructionStream<W>,
}

impl<W: Write> StatementsBuffer<W>
{
	/// Used when encoding the Mod.R/M byte for ordinary addressing.
	const MOD_NO_DISPLACEMENT: u8 = 0b00;
	
	/// Used when encoding the Mod.R/M byte when the displacement is 8.
	const MOD_DISPLACEMENT_8:  u8 = 0b01;
	
	/// Used when encoding the Mod.R/M byte when the displacement is 32.
	const MOD_DISPLACEMENT_32: u8 = 0b10;
	
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(instruction_stream: InstructionStream<W>) -> Self
	{
		Self
		{
			instruction_stream
		}
	}
	
	/// VEX and XOP prefixes embed the operand size prefix or modification prefixes in them.
	#[inline(always)]
	pub(crate) fn push_vex_and_xop_prefixes_or_operand_size_modification_and_rex_prefixes<'a>(&mut self, assembling_for_architecture_variant: &AssemblingForArchitectureVariant, signature: &MnemonicDefinitionSignature, remaining_signature_opcode_bytes: &'a [u8], size_prefix_is_needed: bool, legacy_prefix_modification: Option<u8>, rex_prefix_is_needed: bool, rex_w_prefix_is_needed: bool, vex_l_prefix_is_needed: bool, reg: &Option<SizedMnemonicArgument>, rm: &Option<SizedMnemonicArgument>, vvvv: &Option<SizedMnemonicArgument>) -> Result<&'a [u8], InstructionEncodingError>
	{
		if signature.intersects_flags(InstructionFlags::VexOperationOrXopOperation)
		{
			let prefix = if size_prefix_is_needed
			{
				0b01
			}
			else
			{
				match legacy_prefix_modification
				{
					Some(0xF3) => 0b01,
					Some(0xF2) => 0b11,
					_ => 0b00,
				}
			};
			
			let (map_sel_opcode_base_byte, tail) = remaining_signature_opcode_bytes.split_first().expect("invalid mnemonic signature parameters; expected at least a `map_sel` in the first byte of the remaining signature opcodes because the mnemonic signature has the instruction flags `VEX_OP` or `XOP_OP`");
			
			self.push_vex_xop(assembling_for_architecture_variant.mode, signature, reg, rm, *map_sel_opcode_base_byte, rex_w_prefix_is_needed, vvvv, vex_l_prefix_is_needed, prefix);
			
			Ok(tail)
		}
		// Otherwise, the operand size and modification prefixes have to be pushed if required and REX prefix is created and pushed if needed.
		else
		{
			self.instruction_stream.push_u8_if_some(legacy_prefix_modification);
			
			const OperandSizeOverridePrefix: u8 = 0x66;
			self.instruction_stream.push_u8_if(size_prefix_is_needed, OperandSizeOverridePrefix);
			
			if rex_prefix_is_needed
			{
				if assembling_for_architecture_variant.is_for_protected_mode()
				{
					return Err(InstructionEncodingError("Some SSE and AVX legacy encoded operations are not possible in 32-bit mode as they require a REX.W prefix to be encoded, which is impossible"))
				}
				
				self.push_rex_prefix_opcode_byte(rex_w_prefix_is_needed, reg, rm);
			}
			
			Ok(remaining_signature_opcode_bytes)
		}
	}
	
	/// If `r/m` is embedded in the last opcode byte, push it.
	#[inline(always)]
	pub(crate) fn push_r_m_last_opcode_byte(&mut self, signature: &MnemonicDefinitionSignature, remaining_signature_opcode_bytes: &[u8], rm: &mut Option<SizedMnemonicArgument>)
	{
		if signature.contains_flags(InstructionFlags::SHORT_ARG)
		{
			let (last_opcode_byte, head) = remaining_signature_opcode_bytes.split_last().expect("invalid mnemonic signature parameters");
			
			self.instruction_stream.push_u8s(head);
			
			match rm.take()
			{
				None => panic!("invalid mnemonic signature parameters"),
				
				Some(SizedMnemonicArgument::DirectRegisterReference { register: rm_k, .. }) =>
				{
					let rescaled_for_r8_to_r15 = rm_k.identifier().code_and_7();
					self.instruction_stream.push_u8(last_opcode_byte + (rescaled_for_r8_to_r15));
				}
				
				Some(_) => (),
			}
		}
		else
		{
			self.instruction_stream.push_u8s(remaining_signature_opcode_bytes);
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_addressing(&mut self, mode: SupportedOperationalMode, signature: &MnemonicDefinitionSignature, rm: Option<SizedMnemonicArgument>, reg: Option<SizedMnemonicArgument>, address_size: AddressSize) -> Result<Relocations, InstructionEncodingError>
	{
		use self::SizedMnemonicArgument::*;
		
		let relocations = match rm
		{
			Some(DirectRegisterReference { register, .. }) => self.direct_mod_rm_addressing(mode, signature, reg, register),
			
			Some(IndirectJumpTarget { jump_target_reference, .. }) => self.jump_target_reference_relative_addressing(mode, signature, reg, jump_target_reference),
			
			Some(IndirectMemoryReference { displacement_size, base, index, displacement, .. }) => self.indirect_mod_rm_and_scaled_index_byte_addressing(mode, signature, reg, displacement_size, base, index, displacement, address_size).map_err(InstructionEncodingError::error_when_writing)?,
			
			_ => mode.new_relocations(),
		};
		
		Ok(relocations)
	}
	
	#[inline(always)]
	pub(crate) fn push_immediate_opcode_byte_after_addressing_displacement(&mut self, immediate_opcode_byte: Option<u8>, relocations: &mut Relocations)
	{
		if let Some(immediate_opcode_byte) = immediate_opcode_byte
		{
			self.instruction_stream.push_u8(immediate_opcode_byte);
			relocations.bump(Size::BYTE)
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_register_in_immediate(&mut self, ireg: Option<SizedMnemonicArgument>, remaining_arguments: &mut ArrayVec<[SizedMnemonicArgument; 8]>, relocations: &mut Relocations) -> Result<(), InstructionEncodingError>
	{
		use self::SizedMnemonicArgument::*;
		
		if let Some(DirectRegisterReference { register: ireg, .. }) = ireg
		{
			use self::Size::*;
			
			let byte_literal = RustExpression::literal_byte(ireg.identifier().code() << 4);
			
			// If immediates are present, the register argument will be merged into the first immediate byte.
			let byte_expression = if remaining_arguments.len() > 0
			{
				match remaining_arguments.remove(0)
				{
					Immediate { value, size: BYTE } => byte_literal.or_with_masked_value(value, 0xF),
					_ => panic!("Invalid mnemonic argument definition"),
				}
			}
			else
			{
				byte_literal
			};
			
			self.instruction_stream.push_unsigned_expression(byte_expression, BYTE).map_err(InstructionEncodingError::error_when_writing)?;
			relocations.bump(BYTE);
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn push_immediates(&mut self, remaining_arguments: ArrayVec<[SizedMnemonicArgument; 8]>, relocations: &mut Relocations) -> Result<(), InstructionEncodingError>
	{
		for immedate_like_argument in remaining_arguments
		{
			use self::SizedMnemonicArgument::*;
			
			match immedate_like_argument
			{
				Immediate { value, size } =>
				{
					self.instruction_stream.push_signed_expression(value, size).map_err(InstructionEncodingError::error_when_writing)?;
					relocations.bump(size);
				},
				
				JumpTarget { jump_target_reference, size } =>
				{
					self.instruction_stream.push_unsigned_constant(0, size);
					relocations.bump(size);
					
					if let JumpTarget::Bare(_) = jump_target_reference
					{
						relocations.push_extern(jump_target_reference, size)?
					}
					else
					{
						relocations.push_relative(jump_target_reference, size)
					}
				},
				
				_ => panic!("Invalid argument '{:?}' for immedate_like_argument", immedate_like_argument)
			};
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn push_relocation(&mut self, target: JumpTargetReference, relocation_offset: u8, size: Size, protected_mode_relocation_kind: RelocationKind, mode: SupportedOperationalMode) -> io::Result<()>
	{
		use self::JumpTargetReference::*;
		
		let result = match target
		{
			Labelled { prefix, number_u32 } => self.instruction_stream.push_dynamic_jump_target_reference(relocation_offset, size, protected_mode_relocation_kind, mode, prefix, number_32),
			
			Bare { target_address_usize } => self.instruction_stream.push_bare_jump_target_reference(relocation_offset, size, protected_mode_relocation_kind, mode, target_address_usize),
		};
	}
	
	#[inline(always)]
	fn push_vex_xop(&mut self, mode: SupportedOperationalMode, signature: &MnemonicDefinitionSignature, reg: &Option<SizedMnemonicArgument>, rm: &Option<SizedMnemonicArgument>, map_sel_opcode_base_byte: u8, rex_w_prefix_is_needed: bool, vvvv: &Option<SizedMnemonicArgument>, vex_l_prefix_is_needed: bool, prefix: u8)
	{
		// Used as a placeholder for zero.
		use self::RegisterIdentifier::RAX;
		use self::SizedMnemonicArgument::*;
		use self::SupportedOperationalMode::*;
		
		let byte1 = match mode
		{
			Long =>
			{
				let reg_k = match *reg
				{
					Some(DirectRegisterReference { register, .. }) => register.identifier(),
					
					_ => RAX,
				};
				
				let (base_k, index_k) = match *rm
				{
					Some(DirectRegisterReference { register, .. }) => (register.identifier(), RAX),
					
					Some(IndirectMemoryReference { base, ref index, .. }) =>
					(
						match base
						{
							Some(base) => base.identifier(),
							_ => RAX,
						},
						match *index
						{
							Some(ParsedIndirectMemoryReferenceIndex { ref register, .. }) => register.identifier(),
							_ => RAX,
						}
					),
					
					_ => (RAX, RAX),
				};
				
				(map_sel_opcode_base_byte & 0x1F) | reg_k.code_and_8_then_invert() << 4 | index_k.code_and_8_then_invert() << 3 | base_k.code_and_8_then_invert() << 2
			},
			
			Protected => (map_sel_opcode_base_byte & 0x1f) | 0xE0,
		};
		
		let vvvv_k = match *vvvv
		{
			Some(DirectRegisterReference { register, .. }) => register.identifier(),
			_ => RAX,
		};
		
		let byte2 = (prefix & 0x3) | (rex_w_prefix_is_needed as u8) << 7 | (!vvvv_k.code() & 0xF) << 3 | (vex_l_prefix_is_needed as u8) << 2;
		
		if signature.contains_flags(InstructionFlags::VEX_OP) && (byte1 & 0x7F) == 0x61 && (byte2 & 0x80) == 0
		{
			// 2-byte VEX.L prefix.
			self.instruction_stream.push_u8(0xC5);
			self.instruction_stream.push_u8((byte1 & 0x80) | (byte2 & 0x7F));
		}
		
		{
			let vex_opcode_byte = if signature.contains_flags(InstructionFlags::VEX_OP)
			{
				0xC4
			}
			else
			{
				0x8F
			};
			self.instruction_stream.push_u8(vex_opcode_byte);
		}
		
		self.instruction_stream.push_u8(byte1);
		
		self.instruction_stream.push_u8(byte2);
	}
	
	#[inline(always)]
	fn push_rex_prefix_opcode_byte(&mut self, rex_w_prefix_is_needed: bool, reg: &Option<SizedMnemonicArgument>, rm: &Option<SizedMnemonicArgument>)
	{
		// Used as a placeholder for zero.
		use self::RegisterIdentifier::RAX;
		use self::SizedMnemonicArgument::*;
		
		let reg_k = match *reg
		{
			Some(DirectRegisterReference { register, .. }) => register.identifier(),
			_ => RAX,
		};
		
		let (base_k, index_k) = match *rm
		{
			Some(DirectRegisterReference { register, .. }) => (register.identifier(), RAX),
			
			Some(IndirectMemoryReference { base, ref index, .. }) =>
			(
				match base
				{
					Some(base_register) => base_register.identifier(),
					None => RAX,
				},
				match *index
				{
					Some(ParsedIndirectMemoryReferenceIndex { ref register, .. } ) => register.identifier(),
					_ => RAX,
				}
			),
			
			_ => (RAX, RAX),
		};
		
		const BaseRexPrefix: u8 = 0x40;
		let w_bit = (rex_w_prefix_is_needed as u8) << 3;
		let r_bit = reg_k.code_and_8() >> 1;
		let x_bit = index_k.code_and_8() >> 2;
		let b_bit = base_k.code_and_8() >> 3;
		
		let rex_prefix_opcode_byte = BaseRexPrefix | w_bit | r_bit | x_bit | b_bit;
		self.instruction_stream.push_u8(rex_prefix_opcode_byte)
	}
	
	#[inline(always)]
	fn direct_mod_rm_addressing(&mut self, mode: SupportedOperationalMode, signature: &MnemonicDefinitionSignature, reg: Option<SizedMnemonicArgument>, rm: Register) -> Relocations
	{
		let reg_k = signature.reg_k(reg);
		
		const MOD_DIRECT: u8 = 0b11;
		self.push_mod_rm_byte_or_scaled_index_byte(MOD_DIRECT, reg_k, rm.identifier());
		
		mode.new_relocations()
	}
	
	#[inline(always)]
	fn jump_target_reference_relative_addressing(&mut self, mode: SupportedOperationalMode, signature: &MnemonicDefinitionSignature, reg: Option<SizedMnemonicArgument>, target: JumpTarget) -> Relocations
	{
		use self::Size::*;
		
		let reg_k = signature.reg_k(reg);
		
		self.push_mod_rm_byte_or_scaled_index_byte(Self::MOD_NO_DISPLACEMENT, reg_k, RegisterIdentifier::RBP);
		
		// Displacement.
		self.instruction_stream.push_u32(0);
		
		// Relocations.
		{
			let mut relocations = mode.new_relocations();
			relocations.push_jump_target_reference_addressing(target, DWORD);
			relocations
		}
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing(&mut self,  mode: SupportedOperationalMode, signature: &MnemonicDefinitionSignature, reg: Option<SizedMnemonicArgument>, displacement_size: Option<Size>, base: Option<Register>, index: Option<ParsedIndirectMemoryReferenceIndex>, displacement: Option<RustExpression>, address_size: AddressSize) -> io::Result<Relocations>
	{
		let reg_k = signature.reg_k(reg);
		
		let addressing_mode_is_vector_scaled_index_byte = ParsedIndirectMemoryReferenceIndex::addressing_mode_is_vector_scaled_index_byte(&index);
		let addressing_mode_is_16bit = address_size.is_16_bit_addressing();
		let addressing_mode_is_rip_relative = Register::addressing_mode_is_rip_relative(base);
		
		if addressing_mode_is_vector_scaled_index_byte
		{
			self.indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_vector_scaled_index_byte(mode, reg_k, displacement_size, base, index, displacement)
		}
		else if addressing_mode_is_16bit
		{
			self.indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_16_bit(mode, reg_k, displacement_size, base, displacement)
		}
		else if addressing_mode_is_rip_relative
		{
			self.indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_rip_relative(reg_k, displacement, mode)
		}
		else
		{
			self.indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_ordinary(reg_k, displacement_size, base, index, displacement, mode)
		}
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_vector_scaled_index_byte(&mut self, mode: SupportedOperationalMode, reg_k: RegisterIdentifier, displacement_size: Option<Size>, base: Option<Register>, index: Option<ParsedIndirectMemoryReferenceIndex>, displacement: Option<RustExpression>) -> io::Result<Relocations>
	{
		use self::RegisterIdentifier::*;
		use self::Size::*;
		
		// Vector Scaled Index Byte addressing has simplified rules.
		const MOD_NO_BASE_REGISTER: u8 = 0b00;
		let (base, mod_) = match base
		{
			None =>
			(
				RBP,
				MOD_NO_BASE_REGISTER
			),
			
			Some(base) =>
			(
				base.identifier(),
				match (&displacement, displacement_size)
				{
					(&Some(_), Some(BYTE)) => Self::MOD_DISPLACEMENT_8,
					(&Some(_), Some(DWORD)) => Self::MOD_DISPLACEMENT_32,
					(&Some(_), _) => Self::MOD_DISPLACEMENT_32,
					(_, _) => Self::MOD_DISPLACEMENT_8,
				}
			),
		};
		
		// A SIB byte is always needed for Vector Scaled Index Byte addressing.
		self.push_mod_rm_byte_or_scaled_index_byte(mod_, reg_k, RSP);
		
		let ParsedIndirectMemoryReferenceIndex { register, scale, expression } = index.unwrap();
		let index_register_identifier = register.identifier();
		match expression
		{
			Some(expression) => self.push_scaled_index_byte_with_scale_calculated_by_expression(scale, expression, index_register_identifier, base)?,
			None => self.push_mod_rm_byte_or_scaled_index_byte(ParsedIndirectMemoryReferenceIndex::encode_scale(scale), index_register_identifier, base),
		}
		
		match displacement
		{
			Some(displacement) => self.push_signed_expression(displacement, if mod_ == Self::MOD_DISPLACEMENT_8 {BYTE} else {DWORD})?,
			None => if mod_ == Self::MOD_DISPLACEMENT_8
			{
				self.instruction_stream.push_u8(0);
			}
			else
			{
				self.instruction_stream.push_u32(0);
			},
		}
		
		Ok(mode.new_relocations())
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_16_bit(&mut self, mode: SupportedOperationalMode, reg_k: RegisterIdentifier, displacement_size: Option<Size>, base: Option<Register>, displacement: Option<RustExpression>) -> io::Result<Relocations>
	{
		use self::Size::*;
		
		// The index and base combination has been encoded in the base register.
		// This register is guaranteed to be present.
		let base_k = base.unwrap().identifier();
		
		let mod_ = match (&displacement, displacement_size)
		{
			(&Some(_), Some(BYTE)) => Self::MOD_DISPLACEMENT_8,
			(&Some(_), _) => Self::MOD_DISPLACEMENT_32,
			(&None, _) => if Register::addressing_uses_rbp_base(base)
			{
				Self::MOD_DISPLACEMENT_8
			}
			else
			{
				Self::MOD_NO_DISPLACEMENT
			}
		};
		
		// only need a mod.r/m byte for 16-bit addressing.
		self.push_mod_rm_byte_or_scaled_index_byte(mod_, reg_k, base_k);
		
		match displacement
		{
			Some(displacement) => self.instruction_stream.push_signed_expression(displacement, if mod_ == Self::MOD_DISPLACEMENT_8 {BYTE} else {WORD})?,
			
			None => if mod_ == Self::MOD_DISPLACEMENT_8
			{
				self.instruction_stream.push_u8(0);
			},
		}
		
		Ok(mode.new_relocations())
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_rip_relative(&mut self, reg_k: RegisterIdentifier, displacement: Option<RustExpression>, mode: SupportedOperationalMode) -> io::Result<Relocations>
	{
		use self::SupportedOperationalMode::*;
		use self::Size::*;
		
		self.push_mod_rm_byte_or_scaled_index_byte(Self::MOD_NO_DISPLACEMENT, reg_k, RegisterIdentifier::RBP);
		
		let mut relocations = mode.new_relocations();
		match mode
		{
			Long => if let Some(displacement) = displacement
			{
				self.instruction_stream.push_signed_expression(displacement, DWORD)?
			}
			else
			{
				self.instruction_stream.push_u32(0)
			},
			
			Protected =>
			{
				// x86 doesn't actually allow RIP-relative addressing.
				// Hack: worked around using with relocations and re-using the JumpTarget::Bare.
				self.instruction_stream.push_u32(0);
				let displacement = displacement.unwrap_or_else(|| RustExpression::zero());
				relocations.push_jump_target_reference_addressing(JumpTarget::Bare(displacement), DWORD)
			},
		}
		Ok(relocations)
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_ordinary(&mut self, reg_k: RegisterIdentifier, displacement_size: Option<Size>, base: Option<Register>, index: Option<ParsedIndirectMemoryReferenceIndex>, displacement: Option<RustExpression>, mode: SupportedOperationalMode) -> io::Result<Relocations>
	{
		use self::RegisterIdentifier::*;
		use self::Size::*;
		
		let no_base = base.is_none();
		let no_displacement = displacement.is_none();
		
		let mod_ = if Register::addressing_uses_rbp_base(base) && no_displacement
		{
			Self::MOD_DISPLACEMENT_8
		}
		else if no_displacement || no_base
		{
			Self::MOD_NO_DISPLACEMENT
		}
		else
		{
			match displacement_size
			{
				Some(BYTE) => Self::MOD_DISPLACEMENT_8,
				Some(DWORD) => Self::MOD_DISPLACEMENT_32,
				
				_ => Self::MOD_DISPLACEMENT_32,
			}
		};
		
		// Encode the Scaled Index Byte that follows the Mod-R/M byte.
		match index
		{
			// There is an index, thus we need to escape the Scaled Index Byte.
			Some(ParsedIndirectMemoryReferenceIndex { register, scale, expression }) =>
			{
				// To encode the lack of a base we encode RBP.
				let base = match base
				{
					Some(base) => base.identifier(),
					None => RBP,
				};
				
				self.push_mod_rm_byte_or_scaled_index_byte(mod_, reg_k, RSP);
				
				let index_register_identifier = register.identifier();
				match expression
				{
					Some(expression) => self.push_scaled_index_byte_with_scale_calculated_by_expression(scale, expression, index_register_identifier, base)?,
					None => self.push_mod_rm_byte_or_scaled_index_byte(ParsedIndirectMemoryReferenceIndex::encode_scale(scale), index_register_identifier, base),
				}
			}
			
			None => match base
			{
				// Base but no index.
				// `RBP` at `MOD_NODISP` is used to encode RIP, but this is already handled.
				Some(base) => self.push_mod_rm_byte_or_scaled_index_byte(mod_, reg_k, base.identifier()),
				
				// No base, no index and only displacement.
				None =>
				{
					use self::SupportedOperationalMode::*;
					
					match mode
					{
						Protected => self.push_mod_rm_byte_or_scaled_index_byte(mod_, reg_k, RBP),
						
						// AMD64 uses `RBP` as a base and `RSP` as an index.
						Long =>
						{
							self.push_mod_rm_byte_or_scaled_index_byte(mod_, reg_k, RSP);
							self.push_mod_rm_byte_or_scaled_index_byte(0, RSP, RBP);
						}
					}
				}
			},
		}
		
		// Encode displacement.
		if let Some(displacement) = displacement
		{
			self.instruction_stream.push_signed_expression(displacement, if mod_ == Self::MOD_DISPLACEMENT_8 {BYTE} else {DWORD})?;
		}
		else if no_base
		{
			self.instruction_stream.push_u32(0);
		}
		else if mod_ == Self::MOD_DISPLACEMENT_8
		{
			self.instruction_stream.push_u8(0);
		}
		
		Ok(mode.new_relocations())
	}
	
	#[inline(always)]
	fn push_mod_rm_byte_or_scaled_index_byte(&mut self, mod_or_scale: u8, reg1: RegisterIdentifier, reg2: RegisterIdentifier)
	{
		let scaled_index_byte = mod_or_scale << 6 | reg1.code_and_7() << 3 | reg2.code_and_7();
		self.instruction_stream.push_u8(scaled_index_byte)
	}
}
