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
	
	
	/*
	
 TODO: Now remove redundant arguments like arg0: XMM0, RAX, etc.!
	 - Will require us to remove things like RAX::O which are present in the code.
 
	TODO: FINISH VEX
	
	*/
	
	// See Figure 2-9, Intel Manual Volume 2A Section 2-15 (May 2018).
	#[inline(always)]
	fn vex(&mut self, mmmmm: u8, L: u8, pp: u8, w: u8, vvvv: impl Register, rm: Memory, r: impl Register)
	{
		let r_bit = (!r.index() << 4) & 0x80;
		let x_bit = if rm.has_index_register()
		{
			(!rm.get_index().index() << 3) & 0x40
		}
		else
		{
			0x40
		};
		
		let b_bit = if rm.has_base_register()
		{
			(!rm.get_base().index() << 2) & 0x20
		}
		else
		{
			0x20
		};
		
		if x_bit == 0x40 && b_bit == 0x20 && mmmmm == 0x01 && w == 0
		{
			self.emit_2_byte_vex_prefix(r_bit, vvvv, L, pp)
		}
		else
		{
			self.emit_3_byte_vex_prefix(r_bit, x_bit, b_bit, mmmmm, w, vvvv, L, pp)
		}
	}
	
	#[inline(always)]
	fn vex(&mut self, mmmmm: u8, L: u8, pp: u8, w: u8, vvvv: impl Register, rm: Operand, r: impl Register)
	{
		let r_bit = (!r.index() << 4) & 0x80;
		let b_bit = (!rm.index() << 2) & 0x20;
		
		if b_bit == 0x20 && mmmmm == 0x01 && w == 0
		{
			self.emit_2_byte_vex_prefix(r_bit, vvvv, L, pp)
		}
		else
		{
			self.emit_3_byte_vex_prefix(r_bit, 0x40, b_bit, mmmmm, w, vvvv, L, pp)
		}
	}
	
// This form seems to be missing...
//	#[inline(always)]
//	fn vex(&mut self, mmmmm: u8, L: u8, pp: u8, w: u8, vvvv: impl Register)
//	{
//		if mmmmm == 0x01 && w == 0
//		{
//			self.emit_2_byte_vex_prefix(0x80, vvvv, L, pp)
//		}
//		else
//		{
//			self.emit_3_byte_vex_prefix(0x80, 0x40, 0x20, mmmmm, w, vvvv, L, pp)
//		}
//	}
	
	#[inline(always)]
	fn emit_2_byte_vex_prefix(&mut self, r_bit: u8, vvvv: impl Register, l: u8, pp: u8)
	{
		self.byte_emitter.emit_u8(0xC5);
		self.byte_emitter.emit_u8((r_bit | ((!vvvv.index()) << 3) & 0x78) | (l << 2) | pp);
	}
	
	#[inline(always)]
	fn emit_3_byte_vex_prefix(&mut self, r_bit: u8, x_bit: u8, b_bit: u8, mmmmm: u8, w: u8, vvvv: impl Register, l: u8, pp: u8)
	{
		self.byte_emitter.emit_u8(0xC5);
		self.byte_emitter.emit_u8(r_bit | x_bit | b_bit | mmmmm);
		self.byte_emitter.emit_u8((w << 7) | ((!vvvv.index() << 3) & 0x78) | (l << 2) | pp);
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
