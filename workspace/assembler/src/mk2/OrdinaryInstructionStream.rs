// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


pub struct OrdinaryInstructionStream
{
	byte_emitter: ByteEmitter,
}

impl OrdinaryInstructionStream
{
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
	
	
	
	// Variant 1 assembler.h
	
	/*
	
		General Purpose register needs to include XMMRegister and XMM0
	
	
	Any(8->64)BitMemory & Registers		Register 8 -> 64 RegisterHigh8BitsOf16Bit
	
	self.mod_rm_sib(&mut self, arg0, Self::R64S_2);
	*/
	/*
	
  /** Emits a mod/rm sib byte pair. */
  void mod_rm_sib(const Operand& rm, const Operand& r) {
    auto mod = 0xc0 | ((r.val_ << 3) & 0x38) | (rm.val_ & 0x7);
    fxn_->emit_byte(mod);
  }
	*/
	
	// Variant 2 assembler.c
	/*
	
template <typename T>
void Assembler::mod_rm_sib(const M<T>& rm, const Operand& r) {
  // Every path we take needs these bits for the mod/rm byte
  const auto rrr = (r.val_ << 3) & 0x38;

  // First special case check for RIP+disp32
  if (rm.rip_offset()) {
    const auto mod_byte = 0x00 | rrr | 0x5;
    fxn_->emit_byte(mod_byte);
    disp_imm(rm.get_disp());
    return;
  }

  // Second special case check for no base register
  if (!rm.contains_base()) {
    const auto mod_byte = 0x00 | rrr | 0x4;
    const auto sib_byte = rm.contains_index() ?
                          (((int)rm.get_scale() << 6) & 0xc0) | ((rm.get_index().val_ << 3) & 0x38) | 0x5 :
                          0x00 | 0x20 | 0x5;

    fxn_->emit_byte(mod_byte);
    fxn_->emit_byte(sib_byte);
    disp_imm(rm.get_disp());

    return;
  }

  // Every path we take now requires the non-null base value.
  const auto bbb = rm.get_base().val_ & 0x7;

  // This logic determines what the value of the mod bits will be.
  // It also controls how many immediate bytes we emit later.
  const auto disp = (int32_t)rm.get_disp().val_;
  size_t mod = 0x40;
  if (disp < -128 || disp >= 128) {
    mod = 0x80;
  } else if (disp == 0 && bbb != 0x5) {
    mod = 0x00;
  }

  // Is index non-null?
  if (rm.contains_index()) {
    const auto mod_byte = mod | rrr | 0x4;
    const auto sib_byte = (((int)rm.get_scale() << 6) & 0xc0) |
                          ((rm.get_index().val_ << 3) & 0x38) | bbb;

    fxn_->emit_byte(mod_byte);
    fxn_->emit_byte(sib_byte);
  }
  // Is base sitting in the eip/rip+disp32 row?
  else if (bbb == 0x4) {
    const auto mod_byte = mod | rrr | 0x4;
    const auto sib_byte = (((int)rm.get_scale() << 6) & 0xc0) | 0x20 | bbb;

    fxn_->emit_byte(mod_byte);
    fxn_->emit_byte(sib_byte);
  }
  // No sib byte
  else {
    const auto mod_byte = mod | rrr | bbb;
    fxn_->emit_byte(mod_byte);
  }

  // This logic parallels the logic for the mod bit
  if (mod == 0x40) {
    disp_imm(Imm8(disp));
  } else if (mod == 0x80) {
    disp_imm(Imm32(disp));
  }
}

	*/
	
	// X can be a GeneralPurposeRegister, X87Integer*BitMemory, any Any*BitMemoryOperand?
	//
	// reg can be GP, SegmentRegister, FS, GS, XMMRegister, XMM0, YMM and probably MMRegister
	
	/// Emits one byte containing a a combined `MOD.r/m` and Scaled Index Byte (SIB).
	///
	/// See [this](http://www.c-jump.com/CIS77/CPU/x86/X77_0060_mod_reg_r_m_byte.htm) reference to the bits.
	#[inline(always)]
	fn mod_rm_sib(&mut self, rm: impl MemoryOrRegister, reg: impl Register)
	{
		const ModRegisterAddressingMode: u8 = 0b11;
		
		let mod_rm_and_sib = (0b11 << 6) | ((reg.index() << 3) & 0b0011_1000) | (rm.val_() & 0x07);
		self.emit_u8(mod_rm_and_sib);
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
	self.pref_fwait(0x9B);
	
	self.pref_group2(&mut self, arg0);

	self.pref_group4(&mut self, arg0);
	
	self.pref_group3();

	self.pref_group1(0xF2);
	
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
