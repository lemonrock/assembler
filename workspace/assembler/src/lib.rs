// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of assembler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]


//! #assembler
//!
//! This is a partial fork of dynasm, for compiling x86 assembler templates ar build or run time.


extern crate arrayvec;
#[macro_use] extern crate bitflags;


use ::arrayvec::ArrayVec;
use ::std::cmp::min;
use ::std::collections::HashMap;
use ::std::mem::swap;
use ::std::mem::transmute;


/// An error when encoding an instruction.
pub struct InstructionEncodingError(&'static str);


/// Kind of relocation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum RelocationKind
{
	/// A RIP-relative relocation.
	/// No need to keep track of.
	Relative = 0,
	
	/// An absolute offset to a RIP-relative location.
	Absolute = 1,
	
	/// A relative offset to an absolute location.
	///
	/// Not supported in x64 Long mode.
	Extern = 2,
}

impl RelocationKind
{
	#[inline(always)]
	pub(crate) fn to_id(self) -> u8
	{
		self as u8
	}
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Relocation
{
	target: JumpVariant,
	offset: u8,
	size: Size,
	protected_mode_relocation_kind: RelocationKind,
}

impl Relocation
{
	#[inline(always)]
	pub(crate) fn new(target: JumpVariant, size: Size, protected_mode_relocation_kind: RelocationKind) -> Self
	{
		Self
		{
			target,
			offset: 0,
			size,
			protected_mode_relocation_kind,
		}
	}
	
	#[inline(always)]
	pub(crate) fn bump(&mut self, increment: u8)
	{
		self.offset += increment
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Relocations(ArrayVec<[Relocation; 16]>);

impl Default for Relocations
{
	#[inline(always)]
	fn default() -> Self
	{
		Relocations(Vec::default())
	}
}

impl Relocations
{
	#[inline(always)]
	pub(crate) fn push_long_mode(&mut self, target: JumpVariant, size: Size)
	{
		self.push_relative(target, size)
	}
	
	#[inline(always)]
	pub(crate) fn push_protected_mode(&mut self, target: JumpVariant, size: Size, protected_mode_relocation_kind: RelocationKind)
	{
		self.push(target, size, protected_mode_relocation_kind)
	}
	
	#[inline(always)]
	pub(crate) fn push_relative(&mut self, target: JumpVariant, size: Size)
	{
		self.push(target, size, RelocationKind::Relative)
	}
	
	#[inline(always)]
	fn push(&mut self, target: JumpVariant, size: Size, protected_mode_relocation_kind: RelocationKind)
	{
		self.0.push(Relocation::new(target, size, protected_mode_relocation_kind))
	}
	
	#[inline(always)]
	pub(crate) fn bump(&mut self, increment: u8)
	{
		for relocation in self.0.iter_mut()
		{
			relocation.bump(increment)
		}
	}
}

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
					let rescaled_for_r8_to_r15 = rm_k.register_identifier.code_and_7();
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
					Some(DirectRegisterReference { register, .. }) => register.register_identifier,
					
					_ => RAX,
				};
				
				let (base_k, index_k) = match *rm
				{
					Some(DirectRegisterReference { register, .. }) => (register.register_identifier, RAX),
					
					Some(IndirectMemoryReference { base, ref index, .. }) =>
					(
						match base
						{
							Some(base) => base.register_identifier,
							_ => RAX,
						},
						match *index
						{
							Some(ParsedIndirectMemoryReferenceIndex { ref register, .. }) => register.register_identifier,
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
			Some(DirectRegisterReference { register, .. }) => register.register_identifier,
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
			Some(DirectRegisterReference { register, .. }) => register.register_identifier,
			_ => RAX,
		};
		
		let (base_k, index_k) = match *rm
		{
			Some(DirectRegisterReference { register, .. }) => (register.register_identifier, RAX),
			
			Some(IndirectMemoryReference { base, ref index, .. }) =>
			(
				match base
				{
					Some(base_register) => base_register.register_identifier,
					None => RAX,
				},
				match *index
				{
					Some(ParsedIndirectMemoryReferenceIndex { ref register, .. } ) => register.register_identifier,
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
			Some(DirectRegisterReference { register, .. }) => self.direct_mod_rm_addressing(signature, reg, register),
			
			Some(IndirectJumpTarget { jump_variant, .. }) => self.jump_target_relative_addressing(mode, signature, reg, jump_variant),
			
			Some(IndirectMemoryReference { displacement_size, base, index, displacement, .. }) => self.indirect_mod_rm_and_scaled_index_byte_addressing(mode, signature, reg, displacement_size, base, index, displacement, address_size),
			
			_ => Relocations::default(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_immediate_opcode_byte_after_addressing_displacement(&mut self, immediate_opcode_byte: Option<u8>, relocations: &mut Relocations)
	{
		if let Some(immediate_opcode_byte) = immediate_opcode_byte
		{
			statements_buffer.push_byte(immediate_opcode_byte);
			relocations.bump(1)
		}
	}
	
	#[inline(always)]
	fn direct_mod_rm_addressing(&mut self, signature: &MnemonicDefinitionSignature, reg: Option<SizedMnemonicArgument>, rm: Register) -> Relocations
	{
		let reg_k = signature.reg_k(reg);
		
		const MOD_DIRECT: u8 = 0b11;
		self.push_mod_rm_byte_or_scaled_index_byte(MOD_DIRECT, reg_k, rm.register_identifier);
		
		Relocations::default()
	}
	
	#[inline(always)]
	fn jump_target_relative_addressing(&mut self, mode: SupportedOperationalMode, signature: &MnemonicDefinitionSignature, reg: Option<SizedMnemonicArgument>, target: JumpVariant) -> Relocations
	{
		use self::RelocationKind::*;
		use self::Size::*;
		use self::SupportedOperationalMode::*;
		
		let reg_k = signature.reg_k(reg);
		
		self.push_mod_rm_byte_or_scaled_index_byte(Self::MOD_NO_DISPLACEMENT, reg_k, RegisterIdentifier::RBP);
		
		// Displacement.
		self.push_u32(0);
		
		// Relocations.
		{
			let mut relocations = Relocations::default();
			match mode
			{
				Long => relocations.push_long_mode(target, DWORD),
				Protected => relocations.push_protected_mode(target, DWORD, Absolute)
			}
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
			self.indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_vector_scaled_index_byte(reg_k, displacement_size, base, index, displacement)
		}
		else if addressing_mode_is_16bit
		{
			self.indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_16_bit(reg_k, displacement_size, base, displacement)
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
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_vector_scaled_index_byte(&mut self, reg_k: RegisterIdentifier, displacement_size: Option<Size>, base: Option<Register>, index: Option<ParsedIndirectMemoryReferenceIndex>, displacement: Option<RustExpression>) -> Relocations
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
				base.register_identifier,
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
		let index_register_identifier = register.register_identifier;
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
		
		Relocations::default()
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_16_bit(&mut self, reg_k: RegisterIdentifier, displacement_size: Option<Size>, base: Option<Register>, displacement: Option<RustExpression>) -> Relocations
	{
		use self::Size::*;
		
		// The index and base combination has been encoded in the base register.
		// This register is guaranteed to be present.
		let base_k = base.unwrap().register_identifier;
		
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
		
		Relocations::default()
	}
	
	#[inline(always)]
	fn indirect_mod_rm_and_scaled_index_byte_addressing_mode_is_rip_relative(&mut self, reg_k: RegisterIdentifier, displacement: Option<RustExpression>, mode: SupportedOperationalMode) -> Relocations
	{
		use self::SupportedOperationalMode::*;
		use self::Size::*;
		
		self.push_mod_rm_byte_or_scaled_index_byte(Self::MOD_NO_DISPLACEMENT, reg_k, RegisterIdentifier::RBP);
		
		let mut relocations = Relocations::default();
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
				relocations.push_protected_mode(JumpVariant::Bare(displacement), DWORD, RelocationKind::Absolute);
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
					Some(base) => base.register_identifier,
					None => RBP,
				};
				
				self.push_mod_rm_byte_or_scaled_index_byte(mod_, reg_k, RSP);
				
				let index_register_identifier = register.register_identifier;
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
				Some(base) => self.push_mod_rm_byte_or_scaled_index_byte(mod_, reg_k, base.register_identifier),
				
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
		
		Relocations::default()
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
	fn push_signed_expression(&mut self, _value: RustExpression, _size: Size)
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
		
		let (mut rm, reg, vvvv, ireg, mut args) = signature.extract_instruction_arguments(sized_mnemonic_arguments);
		
		let (immediate_opcode_byte, remaining_signature_opcode_bytes) = signature.immediate_opcode_and_remaining_opcodes();
		
		let remaining_signature_opcode_bytes = statements_buffer.push_vex_and_xop_prefixes_or_operand_size_modification_and_rex_prefixes(assembling_for_architecture_variant, signature, remaining_signature_opcode_bytes, size_prefix_is_needed, legacy_prefix_modification, rex_prefix_is_needed, rex_w_prefix_is_needed, vex_l_prefix_is_needed, &reg, &rm, &vvvv)?;
		
		statements_buffer.push_r_m_last_opcode_byte(signature, remaining_signature_opcode_bytes, &mut rm);
		
		let mut relocations = statements_buffer.push_addressing(assembling_for_architecture_variant.mode, signature, rm, reg, address_size);
		
		statements_buffer.push_immediate_opcode_byte_after_addressing_displacement(immediate_opcode_byte, &mut relocations);
		
		
		
		// register in immediate argument
		if let Some(SizedMnemonicArgument::DirectRegisterReference { register: ireg, .. }) = ireg
		{
			use self::Size::*;
			
			let byte = ireg.kind.encode() << 4;
			
			let mut byte = ecx.expr_lit(ecx.call_site(), ast::LitKind::Byte(byte));
//			if let RegKind::Dynamic(_, expr) = ireg
//			{
//				byte = serialize::expr_mask_shift_or(ecx, byte, expr, 0xF, 4);
//			}
			
			// if immediates are present, the register argument will be merged into the first immediate byte.
			if !args.is_empty()
			{
				if let SizedMnemonicArgument::Immediate { value, size: BYTE } = args.remove(0)
				{
					byte = serialize::expr_mask_shift_or(ecx, byte, value, 0xF, 0);
				}
				else
				{
					panic!("Invalid mnemonic argument definition")
				}
			}
			// TODO: Messed up: signed and unsigned expressions! Stmt::ExprSigned vs Stmt::ExprUnsigned
			xxxx;
			statements_buffer.push_unsigned_expression(byte, BYTE);
			relocations.bump(1);
		}
		
		// immediates
		for immedate_like_argument in args
		{
			match immedate_like_argument
			{
				SizedMnemonicArgument::Immediate { value, size } =>
				{
					statements_buffer.push_signed_expression(value, size);
					relocations.bump(size.to_bytes());
				},
				
				SizedMnemonicArgument::JumpTarget { jump_target, size } =>
				{
					use self::RelocationKind::*;
					
					// TODO
					statements_buffer.push(Stmt::Const(0, size));
					relocations.bump(size.to_bytes());
					
					if let JumpVariant::Bare(_) = jump_target
					{
						use self::SupportedOperationalMode::*;
						
						match assembling_for_architecture_variant.mode
						{
							Protected => relocations.push_protected_mode(jump_target, size, Extern),
							Long => return Err(InstructionEncodingError("x64 Long mode does not support RelocationKind::Extern; caused by SizedMnemonicArgument::JumpTarget with JumpVariant::Bare"))
						}
					}
					else
					{
						relocations.push_relative(jump_target, size);
					}
				},
				
				_ => panic!("Invalid argument for immedate_like_argument")
			};
		}
		
		// Push relocations.
		for Relocation { target, offset, size, protected_mode_relocation_kind } in relocations
		{
			use self::SupportedOperationalMode::*;
			let data = match assembling_for_architecture_variant.mode
			{
				Protected => &[offset, size.to_bytes(), protected_mode_relocation_kind.to_id()],
				Long => &[offset, size.to_bytes()],
			};
			
			use self::JumpVariant::*;
			statements_buffer.push
			(
				match target
				{
					Global(ident) => Stmt::GlobalJumpTarget(ident, serialize::expr_tuple_of_u8s(ecx, ident.span, data)),
					
					Forward(ident) => Stmt::ForwardJumpTarget(ident, serialize::expr_tuple_of_u8s(ecx, ident.span, data)),
					
					Backward(ident) => Stmt::BackwardJumpTarget(ident, serialize::expr_tuple_of_u8s(ecx, ident.span, data)),
					
					Dynamic(expr) =>
					{
						let span = expr.span;
						Stmt::DynamicJumpTarget(expr, serialize::expr_tuple_of_u8s(ecx, span, data))
					}
					
					Bare(expr) =>
					{
						let span = expr.span;
						Stmt::BareJumpTarget(expr, serialize::expr_tuple_of_u8s(ecx, span, data))
					}
				}
			)
		}
		
		// TODO: raw to clean arguments.
		
		Ok(())
	}
}

/// Supported Operational Mode.
///
/// These are affected by ring level, which on the x86 architecture, can vary between 0 and 3.
///
/// There is also Real mode (first defined for the 8086 Intel CPUs), Unreal mode (first defined for the Intel 80286 CPUs), Virtual 8086 mode (first defined for the
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum SupportedOperationalMode
{
	/// Long.
	Long,

	/// Protected.
	Protected,
}

impl Default for SupportedOperationalMode
{
	#[inline(always)]
	fn default() -> Self
	{
		SupportedOperationalMode::Long
	}
}

impl SupportedOperationalMode
{
	#[inline(always)]
	pub(crate) fn default_address_size(self) -> AddressSize
	{
		use self::AddressSize::*;
		use self::SupportedOperationalMode::*;
		
		match self
		{
			Long => DWORD,
			Protected => QWORD,
		}
	}
	
	#[inline(always)]
	pub(crate) fn address_size_override_prefix_required(self, address_size: AddressSize) -> bool
	{
		use self::AddressSize::*;
		use self::SupportedOperationalMode::*;
		
		match (self, address_size)
		{
			(Long, QWORD) => false,
			(Long, DWORD) => true,
			(Long, WORD) => panic!("Long should never be used with WORD"),
			(Protected, QWORD) => panic!("Protected should never be used with QWORD"),
			(Protected, DWORD) => false,
			(Protected, WORD) => true,
		}
	}
}

/// An address size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AddressSize
{
	/// Sixteen-Bit bit (`WORD`) address sizes.
	WORD = 2,
	
	/// Thirty-Two bit (`DWORD`) address sizes.
	DWORD = 4,
	
	/// Sixty-four bit (`QWORD`) address sizes.
	QWORD = 8,
}

impl Default for AddressSize
{
	#[inline(always)]
	fn default() -> Self
	{
		AddressSize::QWORD
	}
}

impl AddressSize
{
	#[inline(always)]
	pub(crate) fn is_16_bit_addressing(self) -> bool
	{
		self == AddressSize::WORD
	}
}

/// Any potential size.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Size
{
	/// A byte.
	///
	/// Coded as `b`.
	BYTE = 1,
	
	/// A word, 2 bytes.
	///
	/// Coded as `w`.
	WORD = 2,
	
	/// A doubleword, 4 bytes.
	///
	/// Coded as `d`.
	DWORD = 4,
	
	/// 6 bytes.
	///
	/// Coded as `f`.
	FWORD = 6,
	
	/// A quadword, 8 bytes.
	///
	/// Coded as `q`.
	QWORD = 8,
	
	/// 10 bytes.
	///
	/// Coded as `p`.
	PWORD = 10,
	
	/// An octword, 16 bytes.
	///
	/// Coded as `o`.
	OWORD = 16,
	
	/// A hexadecword, 32 bytes.
	///
	/// Coded as `h`.
	HWORD = 32,
}

impl Size
{
	#[inline(always)]
	pub(crate) fn assign_if_size_valid_as_address_size(this: Option<Self>, assign_to: &mut AddressSize)
	{
		match this
		{
			Some(size) => match size.to_address_size()
			{
				Some(address_size) =>
				{
					*assign_to = address_size
				}
				
				None => (),
			},
			
			None => (),
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_16_bit_addressing_option(this: Option<Self>) -> bool
	{
		this == Some(Size::WORD)
	}
	
	#[inline(always)]
	pub(crate) fn is_16_bit_addressing(self) -> bool
	{
		self == Size::WORD
	}
	
	#[inline(always)]
	pub(crate) fn is_64_bit(self) -> bool
	{
		self == Size::QWORD
	}
	
	#[inline(always)]
	pub(crate) fn to_address_size(self) -> Option<AddressSize>
	{
		use self::Size::*;
		
		match self
		{
			DWORD => Some(AddressSize::DWORD),
			QWORD => Some(AddressSize::QWORD),
			_ => None,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_none_or(this: Option<Self>, is: Self) -> bool
	{
		match this
		{
			None => true,
			Some(value) => value == is,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_byte_or_word(self) -> bool
	{
		use self::Size::*;
		
		match self
		{
			BYTE => true,
			WORD => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_byte_or_dword(self) -> bool
	{
		use self::Size::*;
		
		match self
		{
			BYTE => true,
			DWORD => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_word_or_dword_or_qword(self) -> bool
	{
		use self::Size::*;
		
		match self
		{
			WORD => true,
			DWORD => true,
			QWORD => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_oword_or_hword(self) -> bool
	{
		use self::Size::*;
		
		match self
		{
			OWORD => true,
			HWORD => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn validate_and_cap_sizes(operand_size: Option<Self>, immediate_size: Option<Self>, parsed_at_least_one_argument_with_a_potential_operand_size: bool) -> Result<(Option<Self>, Option<Self>), InstructionEncodingError>
	{
		match operand_size
		{
			None => if parsed_at_least_one_argument_with_a_potential_operand_size
			{
				Err(InstructionEncodingError("Unknown operand size (all parsed arguments lack knowledge of size)"))
			}
			else
			{
				Ok((operand_size, immediate_size))
			},
			
			Some(operand_size) =>
			{
				let maximum_immediate_size = operand_size.maximum_immediate_size();
				
				if let Some(immediate_size) = immediate_size
				{
					if immediate_size > maximum_immediate_size
					{
						return Err(InstructionEncodingError("Immediate size exceeds maximum for operand"))
					}
				}
				
				Ok((Some(operand_size), Some(maximum_immediate_size)))
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn to_bytes(self) -> u8
	{
		self as u8
	}
	
	#[inline(always)]
	fn maximum_immediate_size(self) -> Self
	{
		min(self, Size::DWORD)
	}
}

bitflags!
{
	/// CPU 'features', effectively sets of supported additional instructions and registers.
	pub struct CpuFeatures: u32
	{
		/// CpuFeatures implicitly present in x86-64; effectively a default.
		const X64_IMPLICIT = 0x0000_0000;
		
		/// Floating Point Unit (FPU).
		const FPU = 0x0000_0001;
		
		/// Original Multimedia extensions (MMX).
		const MMX = 0x0000_0002;
		
		/// AMD 3DNow! extensions (similar to MMX).
		const TDNOW = 0x0000_0004;
		
		/// Streaming SIMD Extensions (SSE).
		const SSE = 0x0000_0008;
		
		/// Streaming SIMD Extensions 2 (SSE2).
		const SSE2 = 0x0000_0010;
		
		/// Streaming SIMD Extensions 3 (SSE3).
		const SSE3 = 0x0000_0020;
		
		/// Virtualization (eg Intel VT-x) instructions (VMX).
		const VMX = 0x0000_0040;
		
		/// Supplemental Streaming SIMD Extensions 3 (SSSE3).
		const SSSE3 = 0x0000_0080;
		
		/// Streaming SIMD Extensions 4a (SSE 4a).
		const SSE4A = 0x0000_0100;
		
		/// Streaming SIMD Extensions 4.1 (SSE4.1).
		const SSE41 = 0x0000_0200;
		
		/// Streaming SIMD Extensions 4.2 (SSE4.2).
		const SSE42 = 0x0000_0400;
		
		/// Streaming SIMD Extensions 5 (SSE5).
		const SSE5 = 0x0000_0800;
		
		/// Advanced Vector Extensions (AVX).
		const AVX = 0x0000_1000;
		
		/// Advanced Vector Extensions 2 (AVX2).
		const AVX2 = 0x0000_2000;
		
		/// Fused Multiply Add (FMA).
		const FMA = 0x0000_4000;
		
		/// Bit Manipulation Instructions 1 (BMI1).
		const BMI1 = 0x0000_8000;
		
		/// Bit Manipulation Instructions 2 (BMI2).
		const BMI2 = 0x0001_0000;
		
		/// Trailing Bit Manipulation, introduced by AMD Piledriver as an instruction to BM1 but dropped from AMD Zen (TBM).
		const TBM = 0x0002_0000;
		
		/// Restricted Transaction Memory (RTM).
		const RTM = 0x0004_0000;
		
		/// Invalidate Process Context Identifier (INVPCID).
		///
		/// Used only by the OS Kernel.
		const INVPCID = 0x0008_0000;
		
		/// Memory Protection Extensions (MPX).
		const MPX = 0x0010_0000;
		
		/// SHA-1 and SHA-2 acceleration instructions (SHA).
		const SHA = 0x0020_0000;
		
		/// Some cache pre-fetching instructions.
		const PREFETCHWT1 = 0x0040_0000;
		
		/// Cyrix only instructions.
		const CYRIX = 0x0080_0000;
		
		/// AMD only instructions.
		const AMD = 0x0100_0000;
		
		/// All.
		const All = Self::X64_IMPLICIT.bits | Self::FPU.bits | Self::MMX.bits | Self::TDNOW.bits | Self::SSE.bits | Self::SSE2.bits | Self::SSE3.bits | Self::VMX.bits | Self::SSSE3.bits | Self::SSE4A.bits | Self::SSE41.bits | Self::SSE42.bits | Self::SSE5.bits | Self::AVX.bits | Self::AVX2.bits | Self::FMA.bits | Self::BMI1.bits | Self::BMI2.bits | Self::TBM.bits | Self::RTM.bits | Self::INVPCID.bits | Self::MPX.bits | Self::SHA.bits | Self::PREFETCHWT1.bits | Self::CYRIX.bits | Self::AMD.bits;
	}
}

impl Default for CpuFeatures
{
	#[inline(always)]
	fn default() -> Self
	{
		CpuFeatures::X64_IMPLICIT
	}
}

bitflags!
{
	/// Instruction flags.
	pub struct InstructionFlags: u32
	{
		/// Instruction has default encoding.
		const DEFAULT = 0x0000_0000;

		/// Instruction requires a `VEX` prefix to be encoded.
		const VEX_OP = 0x0000_0001;

		/// Instruction requires a `XOP` prefix to be encoded.
		const XOP_OP = 0x0000_0002;

		/// Instruction encodes the final opcode byte in the immediate position, eg 3DNow! extensions.
		const IMM_OP = 0x0000_0004;
		
		/// `AUTO_SIZE`, `AUTO_NO32`, `AUTO_REXW` and `AUTO_VEXL` are mutually exclusive; see `OneOfTheAutomaticFlagsIsSet` below.
		///
		/// For 16-bit, implies `OPSIZE` prefix.
		/// For 32-bit, implies no prefix.
		/// For 64-bit, implies `REX.W`, `VEX.W` or `XOP.W` prefix.
		const AUTO_SIZE = 0x0000_0008;

		/// For 16-bit, implies `OPSIZE` prefix.
		/// For 32-bit, implies no prefix.
		/// For 64-bit, implies no prefix.
		const AUTO_NO32 = 0x0000_0010;
		
		/// Illegal for 16-bit.
		/// For 64-bit implies no prefix.
		/// For 64-bit, implies `REX.W`, `VEX.W` or `XOP.W` prefix.
		const AUTO_REXW = 0x0000_0020;

		/// For 128-bit, implies no prefix.
		/// For 256-bit, implies `VEX.L` prefix.
		const AUTO_VEXL = 0x0000_0040;

		/// Implies `OPSIZE` prefix.
		const WORD_SIZE = 0x0000_0080;

		/// Implies `REX.W`, `VEX.W` or `XOP.W` prefix.
		const WITH_REXW = 0x0000_0100;

		/// Implies `VEX.L` or `XOP.L` prefixes.
		const WITH_VEXL = 0x0000_0200;

		/// Operands with unknown sizes cannot be assumed to match.
		const EXACT_SIZE = 0x0000_0400;

		/// Mandatory prefix (same as `WORD_SIZE`).
		const PREF_66 = Self::WORD_SIZE.bits;

		/// Mandatory prefix (same as `SMALL_ADDRESS`).
		const PREF_67 = 0x0000_0800;

		/// Mandatory prefix (same as `LOCK`).
		const PREF_F0 = 0x0000_1000;

		/// Mandatory prefix `REPNE`.
		const PREF_F2 = 0x0000_2000;

		/// Mandatory prefix `REP`.
		const PREF_F3 = 0x0000_4000;
		
		/// User `LOCK` prefix is valid with instruction.
		const LOCK = 0x0000_8000;

		/// User `REP` prefix is valid with instruction.
		const REP = 0x0001_0000;
		
		/// User `REP` prefix is valid with instruction.
		const REPE = 0x0002_0000;
		
		/// A register argument is encoded in the last byte of the opcode.
		const SHORT_ARG = 0x0004_0000;

		/// Select alternate argument encoding `MR`.
		const ENC_MR = 0x0008_0000;

		/// Select alternate argument encoding `VM`.
		const ENC_VM = 0x0010_0000;

		/// A special encoding using the SIB to specify an immediate and two registers.
		const ENC_MIB = 0x0020_0000;

		/// Instruction *not* available in Long mode.
		const X86_ONLY = 0x0040_0000;
		
		/// `AUTO_SIZE`, `AUTO_NO32`, `AUTO_REXW` or `AUTO_VEXL`.
		const OneOfTheAutomaticFlagsIsSet = Self::AUTO_SIZE.bits | Self::AUTO_NO32.bits | Self::AUTO_REXW.bits | Self::AUTO_VEXL.bits;
		
		/// `VEX_OP` or `XOP_OP`.
		const VexOperationOrXopOperation = Self::VEX_OP.bits | Self::XOP_OP.bits;
	}
}

impl Default for InstructionFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		InstructionFlags::DEFAULT
	}
}

/// Represents the architecture variant we are assembling for.
///
/// Default to Long mode with all x86-64 features enabled.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssemblingForArchitectureVariant
{
	/// Are we using Long mode or Protected mode?
	pub mode: SupportedOperationalMode,
	
	/// Which combination of features are being used?
	pub features: CpuFeatures,
}

impl Default for AssemblingForArchitectureVariant
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			mode: Default::default(),
			features: CpuFeatures::All,
		}
	}
}

impl AssemblingForArchitectureVariant
{
	#[inline(always)]
	pub(crate) fn default_address_size(&self) -> AddressSize
	{
		self.mode.default_address_size()
	}
	
	#[inline(always)]
	pub(crate) fn address_size_override_prefix_required(self, address_size: AddressSize) -> bool
	{
		self.mode.address_size_override_prefix_required(address_size)
	}
	
	#[inline(always)]
	pub(crate) fn is_for_protected_mode(&self) -> bool
	{
		self.mode == SupportedOperationalMode::Protected
	}
	
	#[inline(always)]
	pub(crate) fn not_supported_in_operational_mode(&self, not_supported_in_long_mode: bool) -> bool
	{
		not_supported_in_long_mode && self.mode == SupportedOperationalMode::Long
	}
	
	#[inline(always)]
	pub(crate) fn does_not_support_one_or_more_features(&self, features_required: CpuFeatures) -> bool
	{
		!self.features.contains(features_required)
	}
}

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

/// Based on `syntax::ast::LitKind`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustLiteral
{
	/// Maps to `syntax::ast::LitKind::Byte`.
	Byte(u8),
	
	/// Maps to `syntax::ast::LitKind::Int`.
	Integer(u128, ()),
	
	#[doc(hidden)]
	Other,
}

/// Based on `syntax::ast::UnOp`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustUnaryOpt
{
	/// Maps to `syntax::ast::UnOp::Neg`.
	Negate,
	
	#[doc(hidden)]
	Other,
}

/// Based on `syntax::ast::Lit`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RustLiteralNode
{
	node: RustLiteral,
}

/// Based on `syntax::ast::ExprKind`.
///
/// See <https://docs.rs/rustc-ap-syntax/224.0.0/syntax/ast/enum.ExprKind.html>.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustExpressionKind
{
	/// Based on `syntax::ast::ExprKind::Lit`.
	Literal(RustLiteral),
	
	/// Based on `syntax::ast::ExprKind::Unary`.
	Unary(RustUnaryOpt, Box<RustExpression>),
	
	#[doc(hidden)]
	Other,
}

/// Based on `syntax::ptr::P<syntax::ast::Expr>`.
///
/// See <https://docs.rs/rustc-ap-syntax/224.0.0/syntax/ast/struct.Expr.html>.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RustExpression
{
	node: RustExpressionKind,
}

impl RustExpression
{
	#[inline(always)]
	pub(crate) fn zero() -> Self
	{
		Self
		{
			node: RustExpressionKind::Literal(RustLiteral::Integer(0, ()))
		}
	}
	
	#[inline(always)]
	pub(crate) fn derive_size(&self) -> Option<Size>
	{
		use self::Size::*;
		use self::RustExpressionKind::*;
		use self::RustLiteral::*;
		use self::RustUnaryOpt::*;
		
		match self.node
		{
			Literal(ref literal) => match *literal
			{
				Byte(_) => Some(BYTE),
				
				Integer(value, _) => if value < 0x80
				{
					Some(BYTE)
				}
				else if value < 0x8000
				{
					Some(WORD)
				}
				else if value < 0x8000_0000
				{
					Some(DWORD)
				}
				else
				{
					Some(QWORD)
				},
				
				_ => None,
			},
			
			Unary(Negate, ref rust_expression) => match rust_expression.node
			{
				Literal(ref literal) => match *literal
				{
					Byte(_) => Some(BYTE),
					
					Integer(value, _) => if value >= 0x80
					{
						Some(BYTE)
					}
					else if value >= 0x8000
					{
						Some(WORD)
					}
					else if value >= 0x8000_0000
					{
						Some(DWORD)
					}
					else
					{
						Some(QWORD)
					},
					
					_ => None
				},
				
				_ => None,
			}
			
			_ => None,
		}
	}
}

// TODO: Revise.
/// `pub type Ident = Spanned<ast::Ident>`.
pub type RustIdent = String;

/// A parsed register.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register
{
	/// Size of the register.
	size: Size,
	
	/// Kind of the register.
	register_identifier: RegisterIdentifier,
}

impl Register
{
	#[inline(always)]
	pub(crate) fn base_and_index_to_family_size_and_vector_scaled_index_byte_mode(base: Option<Self>, index: Option<Self>) -> Result<Option<(RegisterFamily, Size, bool)>, InstructionEncodingError>
	{
		let base_register_family_and_size = base.map(|base| base.to_family_and_size());
		let index_register_family_and_size = index.map(|index| index.to_family_and_size());
		
		const DisableVectorScaledIndexByteAddressingMode: bool = false;
		const EnableVectorScaledIndexByteAddressingMode: bool = true;
		
		let ok = match (base_register_family_and_size, index_register_family_and_size)
		{
			(None, None) => return Ok(None),
		
			(Some((register_family, size)), None) | (None, Some((register_family, size))) => (register_family, size, DisableVectorScaledIndexByteAddressingMode),
			
			(Some((base_register_family, base_size)), Some((index_register_family, index_size))) => if base_register_family == index_register_family
			{
				if base_size != index_size
				{
					return Err(InstructionEncodingError("Base and index registers are of differing sizes"))
				}
				(base_register_family, base_size, DisableVectorScaledIndexByteAddressingMode)
			}
			else
			{
				use self::RegisterFamily::*;
				
				// Vector Scaled Index Byte addressing.
				match (base_register_family, index_register_family)
				{
					(XMM, _) => (index_register_family, index_size, EnableVectorScaledIndexByteAddressingMode),
					(_, XMM) => (base_register_family, base_size, EnableVectorScaledIndexByteAddressingMode),
					_ => return Err(InstructionEncodingError("Register type combination not supported")),
				}
			}
		};
		Ok(Some(ok))
	}
	
	#[inline(always)]
	pub(crate) fn to_family_and_size(&self) -> (RegisterFamily, Size)
	{
		(self.family(), self.size())
	}
	
	#[inline(always)]
	pub(crate) fn addressing_mode_is_rip_relative(base: Option<Self>) -> bool
	{
		base.map_or(false, |register| register.is_rip())
	}
	
	#[inline(always)]
	pub(crate) fn addressing_uses_rbp_base(base: Option<Self>) -> bool
	{
		base.map_or(false, |register| register.is_rbp_or_r13())
	}
	
	#[inline(always)]
	pub(crate) fn is_rbp_or_r13(&self) -> bool
	{
		use self::RegisterIdentifier::*;
		
		self.is(RBP) || self.is(R13)
	}
	
	#[inline(always)]
	pub(crate) fn is(&self, register_identifier: RegisterIdentifier) -> bool
	{
		self.register_identifier == register_identifier
	}
	
	#[inline(always)]
	pub(crate) fn is_not(&self, register_identifier: RegisterIdentifier) -> bool
	{
		self.register_identifier != register_identifier
	}
	
	#[inline(always)]
	fn has_family(&self, register_family: RegisterFamily) -> bool
	{
		self.family() == register_family
	}
	
	#[inline(always)]
	pub(crate) fn is_legacy(&self) -> bool
	{
		self.has_family(RegisterFamily::LEGACY)
	}
	
	#[inline(always)]
	pub(crate) fn is_rip(&self) -> bool
	{
		self.has_family(RegisterFamily::RIP)
	}
	
	#[inline(always)]
	pub(crate) fn is_xmm(&self) -> bool
	{
		self.has_family(RegisterFamily::XMM)
	}
	
	#[inline(always)]
	pub(crate) fn is_high_byte(&self) -> bool
	{
		self.has_family(RegisterFamily::HIGH_BYTE)
	}
	
	#[inline(always)]
	pub(crate) fn is_extended(&self) -> bool
	{
		use self::RegisterFamily::*;
		
		match self.family()
		{
			LEGACY | XMM | CONTROL | DEBUG => self.code() > 7,
			
			_ => false
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_byte_sized_and_rsp_rbp_rsi_or_rdi(&self) -> bool
	{
		if self.size == Size::BYTE
		{
			use self::RegisterIdentifier::*;
			
			match self.register_identifier
			{
				RSP | RBP | RSI | RDI => true,
				_ => false,
			}
		}
		else
		{
			false
		}
	}
	
	#[inline(always)]
	pub(crate) fn some_size(&self) -> Option<Size>
	{
		Some(self.size)
	}
	
	#[inline(always)]
	fn size(&self) -> Size
	{
		self.size
	}
	
	#[inline(always)]
	fn code(&self) -> u8
	{
		self.register_identifier.code()
	}
	
	#[inline(always)]
	fn family(&self) -> RegisterFamily
	{
		self.register_identifier.family()
	}
}

/// A register identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RegisterIdentifier
{
	/// General purpose register 0, although certain legacy instructions prefer it.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for:-
	///
	/// * Returning an integer value from a function call;
	/// * Passing the number of final variable arguments ('varargs', the `...` parameter) in a function call;
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rax`: 8 bytes wide (`QWORD`);
	/// * `eax`: 4 bytes wide (`DWORD`);
	/// * `ax`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `al`: 1 byte wide (`WORD`);
	///
	/// Bits 8 - 15 can be accessed using the pseudo-register `AH` (`A` 'high').
	///
	/// Was often used as an `accumulator` on the 8086, hence the `A` in the name.
	RAX = 0x00,
	
	/// General purpose register 1, although certain legacy instructions prefer it.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the fourth (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rcx`: 8 bytes wide (`QWORD`);
	/// * `ecx`: 4 bytes wide (`DWORD`);
	/// * `cx`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `cl`: 1 byte wide (`WORD`);
	///
	/// Bits 8 - 15 can be accessed using the pseudo-register `CH` (`C` 'high').
	///
	/// Was often used as a `counter` for loops on the 8086, hence the `C` in the name.
	RCX = 0x01,
	
	/// General purpose register 2, although certain legacy instructions prefer it.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the third (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rdx`: 8 bytes wide (`QWORD`);
	/// * `edx`: 4 bytes wide (`DWORD`);
	/// * `dx`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `dl`: 1 byte wide (`WORD`);
	///
	/// Bits 8 - 15 can be accessed using the pseudo-register `DH` (`D` 'high').
	///
	/// Introduced with the 8086.
	RDX = 0x02,
	
	/// General purpose register 3, although certain legacy instructions prefer it.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rbx`: 8 bytes wide (`QWORD`);
	/// * `ebx`: 4 bytes wide (`DWORD`);
	/// * `bx`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `bl`: 1 byte wide (`WORD`);
	///
	/// Bits 8 - 15 can be accessed using the pseudo-register `BH` (`B` 'high').
	///
	/// Introduced with the 8086.
	RBX = 0x03,
	
	/// General purpose register 4.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rsp`: 8 bytes wide (`QWORD`);
	/// * `esp`: 4 bytes wide (`DWORD`);
	/// * `sp`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `spl`: 1 byte wide (`WORD`) (final `l` is for 'lower');
	///
	/// Usually used as the Stack Pointer, hence the `SP` in the name.
	RSP = 0x04,
	
	/// General purpose register 5.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rbp`: 8 bytes wide (`QWORD`);
	/// * `ebp`: 4 bytes wide (`DWORD`);
	/// * `sb`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `bpl`: 1 byte wide (`WORD`) (final `l` is for 'lower');
	///
	/// Usually used as the Base Pointer, hence the `BP` in the name.
	RBP = 0x05,
	
	/// General purpose register 6.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the second (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rsi`: 8 bytes wide (`QWORD`);
	/// * `esi`: 4 bytes wide (`DWORD`);
	/// * `si`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `sil`: 1 byte wide (`WORD`) (final `l` is for 'lower');
	RSI = 0x06,
	
	/// General purpose register 7.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the first (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rdi`: 8 bytes wide (`QWORD`);
	/// * `edi`: 4 bytes wide (`DWORD`);
	/// * `di`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `dil`: 1 byte wide (`WORD`) (final `l` is for 'lower');
	RDI = 0x07,
	
	/// General purpose register 8.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the fifth (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r8`: 8 bytes wide (`QWORD`);
	/// * `r8d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r8w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r8b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R8 = 0x08,
	
	/// General purpose register 9.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the sixth (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r9`: 8 bytes wide (`QWORD`);
	/// * `r9d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r9w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r9b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R9 = 0x09,
	
	/// General purpose register 10.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r10`: 8 bytes wide (`QWORD`);
	/// * `r10d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r10w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r10b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R10 = 0x0A,
	
	/// General purpose register 11.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r11`: 8 bytes wide (`QWORD`);
	/// * `r11d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r11w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r11b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R11 = 0x0B,
	
	/// General purpose register 12.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r12`: 8 bytes wide (`QWORD`);
	/// * `r12d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r12w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r12b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R12 = 0x0C,
	
	/// General purpose register 13.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r13`: 8 bytes wide (`QWORD`);
	/// * `r13d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r13w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r13b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R13 = 0x0D,
	
	/// General purpose register 14.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r14`: 8 bytes wide (`QWORD`);
	/// * `r14d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r14w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r14b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R14 = 0x0E,
	
	/// General purpose register 15.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r15`: 8 bytes wide (`QWORD`);
	/// * `r15d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r15w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r15b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R15 = 0x0F,
	
	/// Instruction Pointer register.
	///
	/// 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rip`: 8 bytes wide (`QWORD`);
	/// * `eip`: 4 bytes wide (`DWORD`);
	RIP = 0x15,
	
	/// Pseudo-register for bits 8-15 of general purpose register 0 (RAX).
	///
	/// One byte wide.
	///
	/// Encoded in assembler as `ah`.
	AH = 0x24,
	
	/// Pseudo-register for bits 8-15 of general purpose register 1 (RCX).
	///
	/// One byte wide.
	///
	/// Encoded in assembler as `ch`.
	CH = 0x25,
	
	/// Pseudo-register for bits 8-15 of general purpose register 2 (RDX).
	///
	/// One byte wide.
	///
	/// Encoded in assembler as `dh`.
	DH = 0x26,
	
	/// Pseudo-register for bits 8-15 of general purpose register 3 (RBX).
	///
	/// One byte wide.
	///
	/// Encoded in assembler as `bh`.
	BH = 0x27,
	
	/// x87 floating point register 0.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st0`.
	ST0 = 0x30,
	
	/// x87 floating point register 1.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st1`.
	ST1 = 0x31,
	
	/// x87 floating point register 2.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st2`.
	ST2 = 0x32,
	
	/// x87 floating point register 3.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st3`.
	ST3 = 0x33,
	
	/// x87 floating point register 4.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st4`.
	ST4 = 0x34,
	
	/// x87 floating point register 5.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st5`.
	ST5 = 0x35,
	
	/// x87 floating point register 6.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st6`.
	ST6 = 0x36,
	
	/// x87 floating point register 7.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st7`.
	ST7 = 0x37,
	
	/// Multimedia Extension Register 0; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx0`.
	MMX0 = 0x40,
	
	/// Multimedia Extension Register 1; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx1`.
	MMX1 = 0x41,
	
	/// Multimedia Extension Register 2; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx2`.
	MMX2 = 0x42,
	
	/// Multimedia Extension Register 3; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx3`.
	MMX3 = 0x43,
	
	/// Multimedia Extension Register 4; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx4`.
	MMX4 = 0x44,
	
	/// Multimedia Extension Register 5; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx5`.
	MMX5 = 0x45,
	
	/// Multimedia Extension Register 6; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx6`.
	MMX6 = 0x46,
	
	/// Multimedia Extension Register 7; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx7`.
	MMX7 = 0x47,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 0; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM0  = 0x50,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 1; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM1  = 0x51,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 2; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM2  = 0x52,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 3; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM3  = 0x53,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 4; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM4  = 0x54,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 5; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM5  = 0x55,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 6; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM6  = 0x56,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 7; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM7  = 0x57,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 8; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM8  = 0x58,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 9; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM9  = 0x59,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 10; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM10 = 0x5A,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 11; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM11 = 0x5B,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 12; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM12 = 0x5C,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 13; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM13 = 0x5D,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 14; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM14 = 0x5E,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 15; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM15 = 0x5F,
	
	/// Segment register 0; alternative encoding exits.
	///
	/// 2 bytes wide.
	ES = 0x60,
	
	/// Segment register 1; alternative encoding exits.
	///
	/// 2 bytes wide.
	CS = 0x61,
	
	/// Segment register 2; alternative encoding exits.
	///
	/// 2 bytes wide.
	SS = 0x62,
	
	/// Segment register 3; alternative encoding exits.
	///
	/// 2 bytes wide.
	DS = 0x63,
	
	/// Segment register 4; alternative encoding exits.
	///
	/// 2 bytes wide.
	FS = 0x64,
	
	/// Segment register 5; alternative encoding exits.
	///
	/// 2 bytes wide.
	GS = 0x65,
	
	/// Control register 0.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Has various control flags (bits) that modify the basic operation of the processor.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/Control_register).
	CR0  = 0x70,
	
	/// Control register 1.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	CR1  = 0x71,
	
	/// Control register 2.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Contains the Page Fault Linear Address (PFLA); when a page fault occurs, this is the memory address that a program attempted to access.
	CR2  = 0x72,
	
	/// Control register 3.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Interpretation of value depends on setting in control register 0 and control register 4.
	CR3  = 0x73,
	
	
	/// Control register 4.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Has various control flags (bits) that modify the operation of the processor.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/Control_register).
	CR4  = 0x74,
	
	/// Control register 5.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	CR5  = 0x75,
	
	/// Control register 6.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	CR6  = 0x76,
	
	/// Control register 7.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	CR7  = 0x77,
	
	/// Control register 8.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used to prioritize external interrupts and is referred to as the task-priority register (TPR).
	/// See [Wikipedia](https://en.wikipedia.org/wiki/Control_register).
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR8  = 0x78,
	
	/// Control register 9.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR9  = 0x79,
	
	/// Control register 10.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR10 = 0x7A,
	
	/// Control register 11.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR11 = 0x7B,
	
	/// Control register 12.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR12 = 0x7C,
	
	/// Control register 13.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR13 = 0x7D,
	
	/// Control register 14.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR14 = 0x7E,
	
	/// Control register 15.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR15 = 0x7F,
	
	/// Debug register 0.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for a debug breakpoint address in conjunction with debug register 7.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR0  = 0x80,
	
	/// Debug register 1.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for a debug breakpoint address in conjunction with debug register 7.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR1  = 0x81,
	
	/// Debug register 2.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for a debug breakpoint address in conjunction with debug register 7.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR2  = 0x82,
	
	/// Debug register 3.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for a debug breakpoint address in conjunction with debug register 7.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR3  = 0x83,
	
	/// Debug register 4.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	DR4  = 0x84,
	
	/// Debug register 5.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	DR5  = 0x85,
	
	/// Debug register 6.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for debug status.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR6  = 0x86,
	
	/// Debug register 7.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for debug control.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR7  = 0x87,
	
	/// Debug register 8.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR8  = 0x88,
	
	/// Debug register 9.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR9  = 0x89,
	
	/// Debug register 10.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR10 = 0x8A,
	
	/// Debug register 11.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR11 = 0x8B,
	
	/// Debug register 12.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR12 = 0x8C,
	
	/// Debug register 13.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR13 = 0x8D,
	
	/// Debug register 14.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR14 = 0x8E,
	
	/// Debug register 15.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR15 = 0x8F,
	
	/// Bound register 0.
	///
	/// 16 bytes wide.
	///
	/// Encoded in assembler as `bnd0`.
	BND0 = 0x90,
	
	/// Bound register 1.
	///
	/// 16 bytes wide.
	///
	/// Encoded in assembler as `bnd1`.
	BND1 = 0x91,
	
	/// Bound register 2.
	///
	/// 16 bytes wide.
	///
	/// Encoded in assembler as `bnd2`.
	BND2 = 0x92,
	
	/// Bound register 3.
	///
	/// 16 bytes wide.
	///
	/// Encoded in assembler as `bnd3`.
	BND3 = 0x93
}

impl RegisterIdentifier
{
	#[inline(always)]
	pub(crate) fn code_and_8_then_invert(&self) -> u8
	{
		!self.code_and_8()
	}
	
	#[inline(always)]
	pub(crate) fn code_and_8(&self) -> u8
	{
		self.code() & 8
	}
	
	#[inline(always)]
	pub(crate) fn code_and_7(&self) -> u8
	{
		self.code() & 7
	}
	
	#[inline(always)]
	pub(crate) fn code(self) -> u8
	{
		(self as u8) & 0x0F
	}
	
	#[inline(always)]
	pub(crate) fn family(self) -> RegisterFamily
	{
		unsafe { transmute((self as u8) >> 4) }
	}
}

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

/// Jump variant.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub enum JumpVariant
{
	/// Coded as `->label`.
	Global(RustIdent),

	/// Coded as `>label`.
	Backward(RustIdent),

	/// Coded as `<label`.
	Forward(RustIdent),

	/// Coded as `=>expr`.
	Dynamic(RustExpression),
	
	/// An address.
	Bare(RustExpression),
}

/// An index.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct ParsedIndirectMemoryReferenceIndex
{
	register: Register,
	scale: isize,
	expression: Option<RustExpression>,
}

impl ParsedIndirectMemoryReferenceIndex
{
	#[inline(always)]
	pub(crate) fn new_for_base_register(register: Register) -> Self
	{
		Self
		{
			register,
			scale: 1,
			expression: None,
		}
	}
	
	#[inline(always)]
	pub(crate) fn addressing_mode_is_vector_scaled_index_byte(index: &Option<Self>) -> bool
	{
		match index.as_ref()
		{
			None => false,
			Some(&ParsedIndirectMemoryReferenceIndex { register, .. }) => register.is_xmm(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_suitable_for_base_register(&self) -> bool
	{
		self.scale == 1 && self.expression.is_none()
	}
	
	#[inline(always)]
	pub(crate) fn does_not_use_xmm_register(index: &Option<Self>) -> bool
	{
		match *index
		{
			None => true,
			Some(ref this) => !this.register.is_xmm()
		}
	}
	
	#[inline(always)]
	pub(crate) fn scale_is_invalid(scale: isize) -> bool
	{
		Self::encode_scale_internal(scale).is_none()
	}
	
	#[inline(always)]
	pub(crate) fn encode_scale(scale: isize) -> u8
	{
		Self::encode_scale_internal(scale).expect("should already have been validated")
	}
	
	#[inline(always)]
	fn encode_scale_internal(scale: isize) -> Option<u8>
	{
		match scale
		{
			1 => Some(0),
			2 => Some(1),
			4 => Some(2),
			8 => Some(3),
			_ => None
		}
	}
}

/// A parsed mnemonic argument.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
		jump_variant: JumpVariant,
		
		/// A size hint.
		size: Option<Size>,
	},
	
	/// A memory reference to a label, eg `[->foo]`.
	IndirectJumpTarget
	{
		/// Jump variant.
		jump_variant: JumpVariant,
		
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

/// A parsed mnemonic argument which has had correct sizes calculated.
#[derive(Debug)]
pub enum SizedMnemonicArgument
{
	/// An arbitrary immediate.
	Immediate
	{
		/// Value.
		value: RustExpression,
		
		/// Size.
		size: Size,
	},
	
	/// A jump offset, eg `->foo`.
	JumpTarget
	{
		/// Jump variant.
		jump_variant: JumpVariant,
		
		/// Size.
		size: Size
	},
	
	/// A memory reference to a label, eg `[->foo]`.
	IndirectJumpTarget
	{
		/// Jump variant.
		jump_variant: JumpVariant,
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
		/// Displacement size, if displacement used.
		displacement_size: Option<Size>,
		
		/// Base register for adding to address (offset), if displacement used.
		base: Option<Register>,
		
		/// Index register for adding to address (offset), if displacement used.
		index: Option<ParsedIndirectMemoryReferenceIndex>,
		
		/// Displacement, if any.
		displacement: Option<RustExpression>,
	},
}

impl SizedMnemonicArgument
{
	#[inline(always)]
	pub(crate) fn from_parsed(parsed_mnemonic_argument: &ParsedMnemonicArgument, size: Size) -> SizedMnemonicArgument
	{
		use self::ParsedMnemonicArgument::*;
		
		match *parsed_mnemonic_argument
		{
			DirectRegisterReference { register } => SizedMnemonicArgument::DirectRegisterReference { register: register.clone() },
			
			JumpTarget { ref jump_variant, .. } => SizedMnemonicArgument::JumpTarget { jump_variant: jump_variant.clone(), size },
			
			IndirectJumpTarget { ref jump_variant, .. } => SizedMnemonicArgument::IndirectJumpTarget { jump_variant: jump_variant.clone() },
			
			Immediate { ref value, .. } => SizedMnemonicArgument::Immediate { value: value.clone(), size },
			
			IndirectMemoryReference { displacement_size, base, ref index, ref displacement, .. } => SizedMnemonicArgument::IndirectMemoryReference { displacement_size, base, index: index.clone(), displacement: displacement.clone() },
		}
	}
}

/// Officially known as an `operand` rather than a parameter, but official terminology is very confusing.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MnemonicParameter
{
	kind: MnemonicParameterKind,
	size: MnemonicParameterSize,
}

impl MnemonicParameter
{
	#[inline(always)]
	pub(crate) fn to_size(&self, operand_size: Option<Size>, immediate_size: Option<Size>) -> Size
	{
		use self::MnemonicParameterKind::*;
		use self::MnemonicParameterSize::*;
		use self::Size::*;
		
		match (self.size, self.kind)
		{
			(Fixed(BYTE), _) => BYTE,
			(Fixed(WORD), _) => WORD,
			(_, vector_scaled_index_byte_32) | (Fixed(DWORD), _) => DWORD,
			(_, vector_scaled_index_byte_64) | (Fixed(QWORD), _) => QWORD,
			(Fixed(FWORD), _) => FWORD,
			(Fixed(PWORD), _) => PWORD,
			(Fixed(OWORD), _) => OWORD,
			(Fixed(HWORD), _) => HWORD,
			
			// Computed by first pass.
			(AllPossible, immediate) => immediate_size.unwrap(),
			(AllPossible, _) => operand_size.unwrap(),
			
			// Should never be used; a placeholder.
			(NoSize, _) => BYTE,
			
			_ => unreachable!()
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_all_possible(&self) -> bool
	{
		self.size == MnemonicParameterSize::AllPossible
	}
}

/// Kind of the mnemonic parameter (operand).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MnemonicParameterKind
{
	/// `i`.
	immediate,
	
	/// `o`.
	instruction_offset,
	
	/// `A`
	rax,
	
	/// `B`
	rcx,
	
	/// `C`
	rdx,
	
	/// `D`
	rbx,
	
	/// `E`
	rsp,
	
	/// `F`
	rbp,
	
	/// `G`
	rsi,
	
	/// `H`
	rdi,
	
	/// `I`
	r8,
	
	/// `J`
	r9,
	
	/// `K`
	r10,
	
	/// `L`
	r11,
	
	/// `M`
	r12,
	
	/// `N`
	r13,
	
	/// `O`
	r14,
	
	/// `P`
	r15,
	
	/// `Q`
	///
	/// Matches segment register ES.
	es,
	
	/// `R`
	///
	/// Matches segment register CS.
	cs,
	
	/// `S`
	///
	/// Matches segment register SS.
	ss,
	
	/// `T`
	///
	/// Matches segment register DS.
	ds,
	
	/// `U`
	///
	/// Matches segment register FS.
	fs,
	
	/// `V`
	///
	/// Matches segment register GS.
	gs,
	
	/// `W`
	///
	/// Matches register CR8.
	cr8,
	
	/// `X`
	///
	/// Matches register ST0.
	st0,
	
	/// `r`.
	legacy_register,
	
	/// `f`.
	floating_pointer_register,
	
	/// `x`.
	mmx_register,
	
	/// `y`.
	xmm_or_ymm_register,
	
	/// `s`.
	segment_register,
	
	/// `c`.
	control_register,
	
	/// `d`.
	debug_register,
	
	/// `b`.
	bound_register,
	
	/// `m`.
	memory,
	
	/// `v` (`r` and `m`).
	legacy_register_and_memory,
	
	/// `u` (`x` and `m`).
	mmx_register_and_memory,
	
	/// `w` (`y` and `m`).
	xmm_or_ymm_register_and_memory,
	
	/// `k`.
	///
	/// Vector Scaled Index Byte addressing, 32 bit result, size determines xmm or ymm.
	vector_scaled_index_byte_32,
	
	/// `l`.
	///
	/// Vector Scaled Index Byte addressing, 64 bit result, size determines xmm or ymm.
	vector_scaled_index_byte_64,
}

impl MnemonicParameterKind
{
	#[inline(always)]
	pub(crate) fn is_encoded(self) -> bool
	{
		use self::MnemonicParameterKind::*;
		
		match self
		{
			immediate | instruction_offset | legacy_register | floating_pointer_register | mmx_register | xmm_or_ymm_register | segment_register | control_register | debug_register | bound_register | memory | legacy_register_and_memory | mmx_register_and_memory | xmm_or_ymm_register_and_memory | vector_scaled_index_byte_32 | vector_scaled_index_byte_64 => true,
			
			_ => false,
		}
	}
}

/// Size of the mnemonic parameter (operand).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MnemonicParameterSize
{
	/// A fixed, known size.
	Fixed(Size),
	
	/// All possible sizes for this parameter's MnemonicParameterKind; a wildcard.
	///
	/// ie:-
	///
	/// * `WORD` or `DWORD` for immediate (`i`).
	/// * `WORD` `DWORD` or `QWORD` for `legacy_register` (`r`) or `legacy_register_and_memory` (`v`).
	/// * `OWORD` or `HWORD` for `legacy_register_and_memory` (`y`) or `xmm_or_ymm_register_and_memory` (`w`).
	/// * Any size for `memory` (`m`).
	///
	/// Coded as `*`.
	AllPossible,
	
	/// Matches a lack of size; only useful in combination with a MnemonicParameterKind of `memory`.
	///
	/// Coded as `!`.
	NoSize,
	
	/// Matches any size and doesn't participate in the operand size calculation.
	///
	/// Coded as `?`.
	Any,
}

/// Officially known as `operands` rather than parameters, but official terminology is very confusing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MnemonicDefinitionSignature
{
	parameters: Vec<MnemonicParameter>,
	
	// One or more opcode bytes.
	opcode_bytes: &'static [u8],
	
	register_identifier: Option<RegisterIdentifier>,
	
	// Defaults to InstructionFlags::DEFAULT.
	instruction_flags: InstructionFlags,
	
	// Defaults to CpuFeatures::X64_IMPLICIT.
	features_required: CpuFeatures,
}

impl MnemonicDefinitionSignature
{
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
			Some(SizedMnemonicArgument::DirectRegisterReference { register, .. }) => register.register_identifier,
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
