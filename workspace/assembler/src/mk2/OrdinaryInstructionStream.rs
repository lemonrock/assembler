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
	
	//self.opcode_1(0x14);
	/*
	
	
	

  /** Emits a register scaled one-byte opcode. */
  void opcode(uint8_t o1, const Operand& rcode) {
    const auto delta = rcode.val_ & 0x7;
    opcode(o1 + delta);
  }


  /** Emits a register scaled two-byte opcode. */
  
  // Look for regex   opcode!\(self, .*, .*, arg.*\)
  	Register32Bit Register64Bit X87Register  need to support this
  	
  
  
  void opcode(uint8_t o1, uint8_t o2, const Operand& rcode) {
    const auto delta = rcode.val_ & 0x7;
    opcode(o1, o2 + delta);
  }
	*/
	
	
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
	
	
	
	/*
	// Recall also XMM constant.
	self.mod_rm_sib(&mut self, arg0, Self::R64S_2);
	*/
	/*
	
  /** Emits a mod/rm sib byte pair. */
  void mod_rm_sib(const Operand& rm, const Operand& r) {
    auto mod = 0xc0 | ((r.val_ << 3) & 0x38) | (rm.val_ & 0x7);
    fxn_->emit_byte(mod);
  }
	*/
	}
	
	
	
	/*
	
 
	*/
	
	#[inline(always)]
	fn disp_label8(&mut self, arg0: x)
	{
	
	}
	
	#[inline(always)]
	fn disp_label32(&mut self, arg0: x)
	{
	
	}
}
*/
