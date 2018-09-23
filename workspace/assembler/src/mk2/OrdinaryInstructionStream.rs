// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


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
	
	#[inline(always)]
	fn vex(&mut self)
	{
		/*
		  /** Emits a 2-byte vex prefix. See Figure 2-9: Intel Manual Vol 2A 2-14. */
  void vex2(uint8_t r_bit, const Operand& vvvv, uint8_t l, uint8_t pp) {
    fxn_->emit_byte(0xc5);
    fxn_->emit_byte(r_bit | ((~vvvv.val_ << 3) & 0x78) | (l << 2) | pp);
  }

  /** Emits a 3-byte vex prefix. See Figure 2-9: Intel Manual Vol 2A 2-14. */
  void vex3(uint8_t r_bit, uint8_t x_bit, uint8_t b_bit, uint8_t mmmmm,
            uint8_t w, const Operand& vvvv, uint8_t l, uint8_t pp) {
    fxn_->emit_byte(0xc4);
    fxn_->emit_byte(r_bit | x_bit | b_bit | mmmmm);
    fxn_->emit_byte((w << 7) | ((~vvvv.val_ << 3) & 0x78) | (l << 2) | pp);
  }

  // Emits a vex prefix. See Figure 2-9: Intel Manual Vol 2A 2-14. */
		template <typename T>
	void vex(uint8_t mmmmm, uint8_t l, uint8_t pp, uint8_t w,
		const Operand& vvvv, const M<T>& rm,
		const Operand& r) {
		uint8_t r_bit = (~r.val_ << 4) & 0x80;
		uint8_t x_bit = rm.contains_index() ?
		(~rm.get_index().val_ << 3) & 0x40 : 0x40;
		uint8_t b_bit = rm.contains_base() ?
		(~rm.get_base().val_ << 2) & 0x20 : 0x20;
		
		if (x_bit == 0x40 && b_bit == 0x20 && mmmmm == 0x01 && w == 0) {
			vex2(r_bit, vvvv, l, pp);
		} else {
			vex3(r_bit, x_bit, b_bit, mmmmm, w, vvvv, l, pp);
		}
	}
		
		/** Emits a vex prefix. See Figure 2-9: Intel Manual Vol 2A 2-14. */
		void vex(uint8_t mmmmm, uint8_t l, uint8_t pp, uint8_t w,
		const Operand& vvvv, const Operand& rm,
		const Operand& r) {
		uint8_t r_bit = (~r.val_ << 4) & 0x80;
		uint8_t b_bit = (~rm.val_ << 2) & 0x20;
		
		if (b_bit == 0x20 && mmmmm == 0x01 && w == 0) {
			vex2(r_bit, vvvv, l, pp);
		} else {
			vex3(r_bit, 0x40, b_bit, mmmmm, w, vvvv, l, pp);
		}
	}
		
		/** Emits a vex prefix. See Figure 2-9: Intel Manual Vol 2A 2-14. */
		void vex(uint8_t mmmmm, uint8_t l, uint8_t pp, uint8_t w,
		const Operand& vvvv) {
		if (mmmmm == 0x01 && w == 0) {
			vex2(0x80, vvvv, l, pp);
		} else {
			vex3(0x80, 0x40, 0x20, mmmmm, w, vvvv, l, pp);
		}
	}
		*/
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
	fn prefix_group2(&mut self, memory_operand_or_branch_hint: MemoryOrBranchHint)
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
	fn rex_3(&mut self, rm: MemoryOrRegister, r: impl Register, byte: u8)
	{
		rm.emit_rex_3(&mut self.byte_emitter, r, byte)
	}
	
	#[inline(always)]
	fn rex_2(&mut self, rm: MemoryOrRegister, byte: u8)
	{
		rm.emit_rex_2(&mut self.byte_emitter, byte)
	}
	
    /*
    
    TODO: self.vex(0x02, 0x0, 0x0, 0x0, arg1, arg2, arg0);
    
 TODO: Now remove redundant arguments like arg0: XMM0, RAX, etc.!
 
	*/
	
	
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
