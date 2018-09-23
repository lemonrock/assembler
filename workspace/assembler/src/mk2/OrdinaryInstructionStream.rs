// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


pub struct OrdinaryInstructionStream
{
	byte_emitter: ByteEmitter,
}

impl OrdinaryInstructionStream
{
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
	fn prefix_group2(&mut self, memory_operand_or_branch_hint: MemoryOperandOrBranchHint)
	{
		memory_operand_or_branch_hint.emit_prefix_group2(&mut self.byte_emitter)
	}
	
	#[inline(always)]
	fn prefix_group3(&mut self)
	{
		self.byte_emitter.emit_u8(0x66)
	}
	
	#[inline(always)]
	fn prefix_group4(&mut self, memory_operand: MemoryOperand)
	{
		memory_operand.emit_prefix_group4()
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
		rm.emit(&mut self.byte_emitter, reg)
	}
	
	#[inline(always)]
	fn displacement_immediate_1<D: Displacement>(&mut self, displacement: impl AsDisplacement<D=D>)
	{
		displacement.displacement().write(&mut self.byte_emitter)
	}
	
	#[inline(always)]
	fn displacement_immediate_2(&mut self, displacement1: Immediate8Bit, displacement2: Immediate16Bit)
	{
		// NOTE: This order is correct, with the second displacement emitted before the first.
		self.displacement_immediate_1(displacement2);
		self.displacement_immediate_1(displacement1);
	}
	
	/** A symbolic representation of a Rel32. No Rel8 equivalent is provided. */
	//Label
	
	/// Records internal state for a label reference.
	/// Saves the current code position and reserves space for the resolved address by emitting zero bytes.
	#[inline(always)]
	fn displacement_label_8bit(&mut self, l: Label)
	{
		// TODO: fxn_->label8_rels_.push_back(std::make_pair(fxn_->size(), l.val_));
		unimplemented!();
		self.byte_emitter.emit_u8(0);
	}
	
	/// Records internal state for a label reference.
	/// Saves the current code position and reserves space for the resolved address by emitting zero bytes.
	#[inline(always)]
	fn displacement_label_32bit(&mut self, l: Label)
	{
		// TODO: fxn_->label32_rels_.push_back(std::make_pair(fxn_->size(), l.val_));
		unimplemented!();
		self.byte_emitter.emit_u32(0);
	}
}

/*
impl OrdinaryInstructionStream
{
	self.pref_group2(&mut self, arg0);

	self.pref_group4(&mut self, arg0);
	
	self.pref_group3();
	
	rex!(self, arg1, arg0, 0x00);
	/*
	
  /** Emits a rex prefix for rm/r memory/register instructions */
  template <typename T>
  void rex(const M<T>& rm, const Operand& r, uint8_t byte) {
    byte |= requires_rex_byte(r) ? rex() : 0x00;
    byte |= requires_rex_bit(r) ? rex_r() : 0x00;
    rex(rm, byte);
  }

  /** Emits a rex prefix for rm memory instructions */
  template <typename T>
  void rex(const M<T>& rm, uint8_t byte) {
    byte |= (rm.contains_base() && requires_rex_bit(rm.get_base())) ?
      rex_b() : 0x00;
    byte |= (rm.contains_index() && requires_rex_bit(rm.get_index())) ?
      rex_x() : 0x00;
    rex(byte);
  }

  /** Emits a rex prefix for rm/r register/register instructions */
  void rex(const Operand& rm, const Operand& r, uint8_t byte) {
    byte |= requires_rex_byte(r) ? rex() : 0x00;
    byte |= requires_rex_bit(r) ? rex_r() : 0x00;
    rex(rm, byte);
  }

  /** Emits a rex prefix for rm register instructions */
  void rex(const Operand& rm, uint8_t byte) {
    byte |= requires_rex_byte(rm) ? rex() : 0x00;
    byte |= requires_rex_bit(rm) ? rex_b() : 0x00;
    rex(byte);
  }

	*/
}
*/
