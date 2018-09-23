// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An ordinary instruction stream.
#[derive(Debug)]
pub struct OrdinaryInstructionStream
{
	byte_emitter: ByteEmitter,
}

impl OrdinaryInstructionStream
{
	/// `REX.W` prefix.
	const REX_W: u8 = 0x48;
	
	/// `REX.R` prefix.
	pub(crate) const REX_R: u8 = 0x44;
	
	/// `REX.X` prefix.
	pub(crate) const REX_X: u8 = 0x42;
	
	/// `REX.B` prefix.
	pub(crate) const REX_B: u8 = 0x41;
	
	/// `REX` prefix.
	pub(crate) const REX: u8 = 0x40;
	
	// See Figure 2-9, Intel Manual Volume 2A Section 2-15 (May 2018).
	#[inline(always)]
	fn vex_7(&mut self, mmmmm: u8, L: u8, pp: u8, w: u8, vvvv: impl Register, rm: impl MemoryOrRegister, r: impl Register)
	{
		rm.emit_vex_prefix(&mut self.byte_emitter, mmmmm, L, pp, w, vvvv, r)
	}
	
	// See Figure 2-9, Intel Manual Volume 2A Section 2-15 (May 2018).
	#[inline(always)]
	fn vex_5(&mut self, mmmmm: u8, L: u8, pp: u8, w: u8, vvvv: impl Register)
	{
		if mmmmm == 0x01 && w == 0
		{
			self.byte_emitter.emit_2_byte_vex_prefix(0x80, vvvv, L, pp)
		}
		else
		{
			self.byte_emitter.emit_3_byte_vex_prefix(0x80, 0x40, 0x20, mmmmm, w, vvvv, L, pp)
		}
	}
	
	#[inline(always)]
	fn prefix_fwait(&mut self, byte: u8)
	{
		self.byte_emitter.emit_u8(byte)
	}
	
	#[inline(always)]
	fn prefix_group1(&mut self, byte: u8)
	{
		self.byte_emitter.emit_u8(byte)
	}
	
	#[inline(always)]
	fn prefix_group2(&mut self, memory_operand_or_branch_hint: impl MemoryOrBranchHint)
	{
		memory_operand_or_branch_hint.emit_prefix_group2(&mut self.byte_emitter)
	}
	
	#[inline(always)]
	fn prefix_group3(&mut self)
	{
		self.byte_emitter.emit_u8(0x66)
	}
	
	#[inline(always)]
	fn prefix_group4(&mut self, memory: impl Memory)
	{
		memory.memory_operand().emit_prefix_group4(&mut self.byte_emitter)
	}
	
	#[inline(always)]
	fn rex_3(&mut self, rm: impl MemoryOrRegister, r: impl Register, byte: u8)
	{
		rm.emit_rex_3(&mut self.byte_emitter, r, byte)
	}
	
	#[inline(always)]
	fn rex_2(&mut self, rm: impl MemoryOrRegister, byte: u8)
	{
		rm.emit_rex_2(&mut self.byte_emitter, byte)
	}
	
	#[inline(always)]
	fn rex_1(&mut self, byte: u8)
	{
		self.byte_emitter.emit_u8_if_not_zero(byte)
	}
	
	#[inline(always)]
	fn opcode_1(&mut self, opcode: u8)
	{
		self.byte_emitter.emit_u8(opcode)
	}
	
	#[inline(always)]
	fn opcode_2(&mut self, opcode1: u8, opcode2: impl ToOpcode)
	{
		self.opcode_1(opcode1);
		self.opcode_1(opcode2.to_opcode());
	}
	
	#[inline(always)]
	fn opcode_3(&mut self, opcode1: u8, opcode2: u8, opcode3: impl ToOpcode)
	{
		self.opcode_1(opcode1);
		self.opcode_1(opcode2);
		self.opcode_1(opcode3.to_opcode());
	}
	
	#[inline(always)]
	fn mod_rm_sib(&mut self, rm: impl MemoryOrRegister, reg: impl Register)
	{
		rm.emit_mod_rm_sib(&mut self.byte_emitter, reg)
	}
	
	#[inline(always)]
	fn displacement_immediate_1<D: Displacement>(&mut self, displacement: impl AsDisplacement<D=D>)
	{
		displacement.displacement().emit(&mut self.byte_emitter)
	}
	
	#[inline(always)]
	fn displacement_immediate_2(&mut self, displacement1: Immediate8Bit, displacement2: Immediate16Bit)
	{
		// NOTE: This order is correct, with the second displacement emitted before the first.
		self.displacement_immediate_1(displacement2);
		self.displacement_immediate_1(displacement1);
	}
	
	/// Records internal state for a label reference.
	/// Saves the current code position and reserves space for the resolved address by emitting zero bytes.
	#[inline(always)]
	fn displacement_label_8bit(&mut self, _l: Label)
	{
		// TODO: fxn_->label8_rels_.push_back(std::make_pair(fxn_->size(), l.val_));
		unimplemented!();
		self.byte_emitter.emit_u8(0);
	}
	
	/// Records internal state for a label reference.
	/// Saves the current code position and reserves space for the resolved address by emitting zero bytes.
	#[inline(always)]
	fn displacement_label_32bit(&mut self, _l: Label)
	{
		// TODO: fxn_->label32_rels_.push_back(std::make_pair(fxn_->size(), l.val_));
		unimplemented!();
		self.byte_emitter.emit_u32(0);
	}
}
