// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A parsed mnemonic argument which has had correct sizes calculated.
#[derive(Debug)]
pub(crate) enum SizedMnemonicArgument
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
		/// Jump target reference.
		jump_target_reference: JumpTargetReference,
		
		/// Size.
		size: Size
	},
	
	/// A memory reference to a label, eg `[->foo]`.
	IndirectJumpTarget
	{
		/// Jump target reference.
		jump_target_reference: JumpTargetReference,
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
			
			JumpTarget { ref jump_target_reference, .. } => SizedMnemonicArgument::JumpTarget { jump_target_reference: jump_target_reference.clone(), size },
			
			IndirectJumpTarget { ref jump_target_reference, .. } => SizedMnemonicArgument::IndirectJumpTarget { jump_target_reference: jump_target_reference.clone() },
			
			Immediate { ref value, .. } => SizedMnemonicArgument::Immediate { value: value.clone(), size },
			
			IndirectMemoryReference { displacement_size, base, ref index, ref displacement, .. } => SizedMnemonicArgument::IndirectMemoryReference { displacement_size, base, index: index.clone(), displacement: displacement.clone() },
		}
	}
}
