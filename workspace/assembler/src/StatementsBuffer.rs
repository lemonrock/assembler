// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A list of 'statements' - encoded bytes and the like.
#[derive(Debug,  Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatementsBuffer
{
}

impl StatementsBuffer
{
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
			self.push_byte_if_some(legacy_prefix_modification);
			
			const OperandSizeOverridePrefix: u8 = 0x66;
			self.push_byte_if(size_prefix_is_needed, OperandSizeOverridePrefix);
			
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
			
			self.push_bytes(head);
			
			match rm.take()
			{
				None => panic!("invalid mnemonic signature parameters"),
				
				Some(SizedMnemonicArgument::DirectRegisterReference { register: rm_k, .. }) =>
				{
					let rescaled_for_r8_to_r15 = rm_k.identifier().code_and_7();
					self.push_byte(last_opcode_byte + (rescaled_for_r8_to_r15));
				}
				
				Some(_) => (),
			}
		}
		else
		{
			self.push_bytes(remaining_signature_opcode_bytes);
		}
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
			self.push_byte(0xC5);
			self.push_byte((byte1 & 0x80) | (byte2 & 0x7F));
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
			self.push_byte(vex_opcode_byte);
		}
		
		self.push_byte(byte1);
		
		self.push_byte(byte2);
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
		self.push_byte(rex_prefix_opcode_byte)
	}
	
	// Used when encoding the Mod.R/M byte for ordinary addressing.
	const MOD_NO_DISPLACEMENT: u8 = 0b00;
	
	// Used when encoding the Mod.R/M byte when the displacement is 8.
	const MOD_DISPLACEMENT_8:  u8 = 0b01;
	
	// Used when encoding the Mod.R/M byte when the displacement is 32.
	const MOD_DISPLACEMENT_32: u8 = 0b10;
	
	#[inline(always)]
	pub(crate) fn push_addressing(&mut self, mode: SupportedOperationalMode, signature: &MnemonicDefinitionSignature, rm: Option<SizedMnemonicArgument>, reg: Option<SizedMnemonicArgument>, address_size: AddressSize) -> Relocations
	{
		use self::SizedMnemonicArgument::*;
		
		match rm
		{
			Some(DirectRegisterReference { register, .. }) => self.direct_mod_rm_addressing(mode, signature, reg, register),
			
			Some(IndirectJumpTarget { jump_variant, .. }) => self.jump_target_relative_addressing(mode, signature, reg, jump_variant),
			
			Some(IndirectMemoryReference { displacement_size, base, index, displacement, .. }) => self.indirect_mod_rm_and_scaled_index_byte_addressing(mode, signature, reg, displacement_size, base, index, displacement, address_size),
			
			_ => mode.new_relocations(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_immediate_opcode_byte_after_addressing_displacement(&mut self, immediate_opcode_byte: Option<u8>, relocations: &mut Relocations)
	{
		if let Some(immediate_opcode_byte) = immediate_opcode_byte
		{
			self.push_byte(immediate_opcode_byte);
			relocations.bump(Size::BYTE)
		}
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
	fn jump_target_relative_addressing(&mut self, mode: SupportedOperationalMode, signature: &MnemonicDefinitionSignature, reg: Option<SizedMnemonicArgument>, target: JumpVariant) -> Relocations
	{
		use self::Size::*;
		
		let reg_k = signature.reg_k(reg);
		
		self.push_mod_rm_byte_or_scaled_index_byte(Self::MOD_NO_DISPLACEMENT, reg_k, RegisterIdentifier::RBP);
		
		// Displacement.
		self.push_u32(0);
		
		// Relocations.
		{
			let mut relocations = mode.new_relocations();
			relocations.push_jump_target_addressing(target, DWORD);
			relocations
		}
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing(&mut self,  mode: SupportedOperationalMode, signature: &MnemonicDefinitionSignature, reg: Option<SizedMnemonicArgument>, displacement_size: Option<Size>, base: Option<Register>, index: Option<ParsedIndirectMemoryReferenceIndex>, displacement: Option<RustExpression>, address_size: AddressSize) -> Relocations
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
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_vector_scaled_index_byte(&mut self, mode: SupportedOperationalMode, reg_k: RegisterIdentifier, displacement_size: Option<Size>, base: Option<Register>, index: Option<ParsedIndirectMemoryReferenceIndex>, displacement: Option<RustExpression>) -> Relocations
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
			Some(expression) => self.push_scaled_index_byte_with_scale_calculated_by_expression(scale, expression, index_register_identifier, base),
			None => self.push_mod_rm_byte_or_scaled_index_byte(ParsedIndirectMemoryReferenceIndex::encode_scale(scale), index_register_identifier, base),
		}
		
		match displacement
		{
			Some(displacement) => self.push_signed_expression(displacement, if mod_ == Self::MOD_DISPLACEMENT_8 {BYTE} else {DWORD}),
			None => if mod_ == Self::MOD_DISPLACEMENT_8
			{
				self.push_byte(0);
			}
			else
			{
				self.push_u32(0);
			},
		}
		
		mode.new_relocations()
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_16_bit(&mut self, mode: SupportedOperationalMode, reg_k: RegisterIdentifier, displacement_size: Option<Size>, base: Option<Register>, displacement: Option<RustExpression>) -> Relocations
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
			Some(displacement) => self.push_signed_expression(displacement, if mod_ == Self::MOD_DISPLACEMENT_8 {BYTE} else {WORD}),
			
			None => if mod_ == Self::MOD_DISPLACEMENT_8
			{
				self.push_byte(0);
			},
		}
		
		mode.new_relocations()
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_rip_relative(&mut self, reg_k: RegisterIdentifier, displacement: Option<RustExpression>, mode: SupportedOperationalMode) -> Relocations
	{
		use self::SupportedOperationalMode::*;
		use self::Size::*;
		
		self.push_mod_rm_byte_or_scaled_index_byte(Self::MOD_NO_DISPLACEMENT, reg_k, RegisterIdentifier::RBP);
		
		let mut relocations = mode.new_relocations();
		match mode
		{
			Long => if let Some(displacement) = displacement
			{
				self.push_signed_expression(displacement, DWORD);
			}
			else
			{
				self.push_u32(0)
			},
			
			Protected =>
			{
				// x86 doesn't actually allow RIP-relative addressing.
				// Hack: worked around using with relocations and re-using the JumpVariant::Bare.
				self.push_u32(0);
				let displacement = displacement.unwrap_or_else(|| RustExpression::zero());
				relocations.push_jump_target_addressing(JumpVariant::Bare(displacement), DWORD)
			},
		}
		relocations
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_ordinary(&mut self, reg_k: RegisterIdentifier, displacement_size: Option<Size>, base: Option<Register>, index: Option<ParsedIndirectMemoryReferenceIndex>, displacement: Option<RustExpression>, mode: SupportedOperationalMode) -> Relocations
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
		// See <http://www.c-jump.com/CIS77/CPU/x86/X77_0100_sib_byte_layout.htm>.
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
					Some(expression) => self.push_scaled_index_byte_with_scale_calculated_by_expression(scale, expression, index_register_identifier, base),
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
			self.push_signed_expression(displacement, if mod_ == Self::MOD_DISPLACEMENT_8 {BYTE} else {DWORD});
		}
		else if no_base
		{
			self.push_u32(0);
		}
		else if mod_ == Self::MOD_DISPLACEMENT_8
		{
			self.push_byte(0);
		}
		
		mode.new_relocations()
	}
	
	#[inline(always)]
	fn push_mod_rm_byte_or_scaled_index_byte(&mut self, mod_or_scale: u8, reg1: RegisterIdentifier, reg2: RegisterIdentifier)
	{
		let scaled_index_byte = mod_or_scale << 6 | reg1.code_and_7() << 3 | reg2.code_and_7();
		self.push_byte(scaled_index_byte)
	}
	
	#[inline(always)]
	fn push_scaled_index_byte_with_scale_calculated_by_expression(&mut self, _scale: isize, _expression: RustExpression, _reg1: RegisterIdentifier, _reg2: RegisterIdentifier)
	{
//		let byte = (scale * expression) << 6 | reg1.code_and_7() << 3 | reg2.code_and_7();
//		self.push_mod_rm_byte_or_scaled_index_byte(scale * expression, reg1, reg2);
//		TODO
		unimplemented!("Won't compile; needs to be replaced by a bit of code that just outputs some Rust")
	}
	
	#[inline(always)]
	pub(crate) fn push_byte_if_some(&mut self, byte: Option<u8>)
	{
		if let Some(byte) = byte
		{
			self.push_byte(byte)
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_byte_if(&mut self, condition: bool, byte: u8)
	{
		if condition
		{
			self.push_byte(byte)
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_global_jump_target(&mut self, _ident: RustIdent, data: &[u8])
	{
		unimplemented!();
	}
	
	#[inline(always)]
	pub(crate) fn push_forward_jump_target(&mut self, _ident: RustIdent, data: &[u8])
	{
		unimplemented!();
	}
	
	#[inline(always)]
	pub(crate) fn push_backward_jump_target(&mut self, _ident: RustIdent, data: &[u8])
	{
		unimplemented!();
	}
	
	#[inline(always)]
	pub(crate) fn push_dynamic_jump_target(&mut self, _expression: RustExpression, data: &[u8])
	{
		unimplemented!();
	}
	
	#[inline(always)]
	pub(crate) fn push_bare_jump_target(&mut self, _expression: RustExpression, data: &[u8])
	{
		unimplemented!();
	}
	
	#[inline(always)]
	pub(crate) fn push_unsigned_constant(&mut self, _value: u64, _size: Size)
	{
		unimplemented!();
	}
	
	#[inline(always)]
	pub(crate) fn push_signed_expression(&mut self, _value: RustExpression, _size: Size)
	{
		unimplemented!();
	}
	
	#[inline(always)]
	pub(crate) fn push_unsigned_expression(&mut self, _value: RustExpression, _size: Size)
	{
		unimplemented!();
	}
	
	#[inline(always)]
	fn push_u32(&mut self, _value: u32)
	{
		unimplemented!();
	}
	
	#[inline(always)]
	fn push_byte(&mut self, _byte: u8)
	{
		unimplemented!();
	}
	
	#[inline(always)]
	fn push_bytes(&mut self, _bytes: &[u8])
	{
		unimplemented!();
	}
}
