// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


impl<'a> InstructionStream<'a>
{
	/// Add with carry `imm8` to `AL`.
	#[inline(always)]
	pub fn adc_AL_Immediate8Bit(&mut self, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x14);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with carry `imm16` to `AX`.
	#[inline(always)]
	pub fn adc_AX_Immediate16Bit(&mut self, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x15);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with carry `imm32` to `EAX`.
	#[inline(always)]
	pub fn adc_EAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x15);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with carry `imm16` to `r/m16`.
	#[inline(always)]
	pub fn adc_Any16BitMemory_Immediate16Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) sign-extended `imm8` to `r/m16`.
	#[inline(always)]
	pub fn adc_Any16BitMemory_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with carry `r16` to `r/m16`.
	#[inline(always)]
	pub fn adc_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `imm32` to `r/m32`.
	#[inline(always)]
	pub fn adc_Any32BitMemory_Immediate32Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) sign-extended `imm8` into `r/m32`.
	#[inline(always)]
	pub fn adc_Any32BitMemory_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `r32` to `r/m32`.
	#[inline(always)]
	pub fn adc_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `imm32` sign extended to 64-bits to `r/m64`.
	#[inline(always)]
	pub fn adc_Any64BitMemory_Immediate32Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) sign-extended `imm8` into `r/m64`.
	#[inline(always)]
	pub fn adc_Any64BitMemory_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `r64` to `r/m64`.
	#[inline(always)]
	pub fn adc_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `imm8` to `r/m8`.
	#[inline(always)]
	pub fn adc_Any8BitMemory_Immediate8Bit(&mut self, arg0: Any8BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with carry byte register to `r/m8`.
	#[inline(always)]
	pub fn adc_Any8BitMemory_Register8Bit(&mut self, arg0: Any8BitMemory, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x10);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry byte register to `r/m8`.
	#[inline(always)]
	pub fn adc_Any8BitMemory_RegisterHigh8BitsOf16Bits(&mut self, arg0: Any8BitMemory, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x10);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `imm16` to `r/m16`.
	#[inline(always)]
	pub fn adc_Register16Bit_Immediate16Bit(&mut self, arg0: Register16Bit, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) sign-extended `imm8` to `r/m16`.
	#[inline(always)]
	pub fn adc_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with carry `r/m16` to `r16`.
	#[inline(always)]
	pub fn adc_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x13);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `r16` to `r/m16`.
	#[inline(always)]
	pub fn adc_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `r/m16` to `r16`.
	#[inline(always)]
	pub fn adc_Register16Bit_Register16Bit_REX_W(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x13);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `imm32` to `r/m32`.
	#[inline(always)]
	pub fn adc_Register32Bit_Immediate32Bit(&mut self, arg0: Register32Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) sign-extended `imm8` into `r/m32`.
	#[inline(always)]
	pub fn adc_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `r/m32` to `r32`.
	#[inline(always)]
	pub fn adc_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x13);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `r32` to `r/m32`.
	#[inline(always)]
	pub fn adc_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `r/m32` to `r32`.
	#[inline(always)]
	pub fn adc_Register32Bit_Register32Bit_REX_W(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x13);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `imm32` sign extended to 64-bits to `r/m64`.
	#[inline(always)]
	pub fn adc_Register64Bit_Immediate32Bit(&mut self, arg0: Register64Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) sign-extended `imm8` into `r/m64`.
	#[inline(always)]
	pub fn adc_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `r/m64` to `r64`.
	#[inline(always)]
	pub fn adc_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x13);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `r64` to `r/m64`.
	#[inline(always)]
	pub fn adc_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with Carry Flag (CF) `r/m64` to `r64`.
	#[inline(always)]
	pub fn adc_Register64Bit_Register64Bit_REX_W(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x13);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `imm8` to `r/m8`.
	#[inline(always)]
	pub fn adc_Register8Bit_Immediate8Bit(&mut self, arg0: Register8Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with carry `r/m8` to byte register.
	#[inline(always)]
	pub fn adc_Register8Bit_Any8BitMemory(&mut self, arg0: Register8Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry byte register to `r/m8`.
	#[inline(always)]
	pub fn adc_Register8Bit_Register8Bit(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x10);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `r/m8` to byte register.
	#[inline(always)]
	pub fn adc_Register8Bit_Register8Bit_REX_W(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry byte register to `r/m8`.
	#[inline(always)]
	pub fn adc_Register8Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x10);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `r/m8` to byte register.
	#[inline(always)]
	pub fn adc_Register8Bit_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `imm32` sign extended to 64-bits to `RAX`.
	#[inline(always)]
	pub fn adc_RAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0x15);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with carry `imm8` to `r/m8`.
	#[inline(always)]
	pub fn adc_RegisterHigh8BitsOf16Bits_Immediate8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add with carry `r/m8` to byte register.
	#[inline(always)]
	pub fn adc_RegisterHigh8BitsOf16Bits_Any8BitMemory(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry byte register to `r/m8`.
	#[inline(always)]
	pub fn adc_RegisterHigh8BitsOf16Bits_Register8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x10);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `r/m8` to byte register.
	#[inline(always)]
	pub fn adc_RegisterHigh8BitsOf16Bits_Register8Bit_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry byte register to `r/m8`.
	#[inline(always)]
	pub fn adc_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x10);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add with carry `r/m8` to byte register.
	#[inline(always)]
	pub fn adc_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `imm8` to `AL`.
	#[inline(always)]
	pub fn add_AL_Immediate8Bit(&mut self, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x04);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `imm16` to `AX`.
	#[inline(always)]
	pub fn add_AX_Immediate16Bit(&mut self, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x05);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `imm32` to `EAX`.
	#[inline(always)]
	pub fn add_EAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x05);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `imm16` to `r/m16`.
	#[inline(always)]
	pub fn add_Any16BitMemory_Immediate16Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add sign-extended `imm8` to `r/m16`.
	#[inline(always)]
	pub fn add_Any16BitMemory_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `r16` to `r/m16`.
	#[inline(always)]
	pub fn add_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x01);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `imm32` to `r/m32`.
	#[inline(always)]
	pub fn add_Any32BitMemory_Immediate32Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add sign-extended `imm8` to `r/m32`.
	#[inline(always)]
	pub fn add_Any32BitMemory_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `r32` to `r/m32`.
	#[inline(always)]
	pub fn add_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x01);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `imm32` sign-extended to 64-bits to `r/m64`.
	#[inline(always)]
	pub fn add_Any64BitMemory_Immediate32Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add sign-extended `imm8` to `r/m64`.
	#[inline(always)]
	pub fn add_Any64BitMemory_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `r64` to `r/m64`.
	#[inline(always)]
	pub fn add_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x01);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `imm8` to `r/m8`.
	#[inline(always)]
	pub fn add_Any8BitMemory_Immediate8Bit(&mut self, arg0: Any8BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `r8` to `r/m8`.
	#[inline(always)]
	pub fn add_Any8BitMemory_Register8Bit(&mut self, arg0: Any8BitMemory, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x00);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r8` to `r/m8`.
	#[inline(always)]
	pub fn add_Any8BitMemory_RegisterHigh8BitsOf16Bits(&mut self, arg0: Any8BitMemory, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x00);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `imm16` to `r/m16`.
	#[inline(always)]
	pub fn add_Register16Bit_Immediate16Bit(&mut self, arg0: Register16Bit, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add sign-extended `imm8` to `r/m16`.
	#[inline(always)]
	pub fn add_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `r/m16` to `r16`.
	#[inline(always)]
	pub fn add_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r16` to `r/m16`.
	#[inline(always)]
	pub fn add_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x01);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r/m16` to `r16`.
	#[inline(always)]
	pub fn add_Register16Bit_Register16Bit_REX_W(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `imm32` to `r/m32`.
	#[inline(always)]
	pub fn add_Register32Bit_Immediate32Bit(&mut self, arg0: Register32Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add sign-extended `imm8` to `r/m32`.
	#[inline(always)]
	pub fn add_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `r/m32` to `r32`.
	#[inline(always)]
	pub fn add_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r32` to `r/m32`.
	#[inline(always)]
	pub fn add_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x01);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r/m32` to `r32`.
	#[inline(always)]
	pub fn add_Register32Bit_Register32Bit_REX_W(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `imm32` sign-extended to 64-bits to `r/m64`.
	#[inline(always)]
	pub fn add_Register64Bit_Immediate32Bit(&mut self, arg0: Register64Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add sign-extended `imm8` to `r/m64`.
	#[inline(always)]
	pub fn add_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `r/m64` to `r64`.
	#[inline(always)]
	pub fn add_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r64` to `r/m64`.
	#[inline(always)]
	pub fn add_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x01);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r/m64` to `r64`.
	#[inline(always)]
	pub fn add_Register64Bit_Register64Bit_REX_W(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `imm8` to `r/m8`.
	#[inline(always)]
	pub fn add_Register8Bit_Immediate8Bit(&mut self, arg0: Register8Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `r/m8` to `r8`.
	#[inline(always)]
	pub fn add_Register8Bit_Any8BitMemory(&mut self, arg0: Register8Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r8` to `r/m8`.
	#[inline(always)]
	pub fn add_Register8Bit_Register8Bit(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x00);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r/m8` to `r8`.
	#[inline(always)]
	pub fn add_Register8Bit_Register8Bit_REX_W(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r8` to `r/m8`.
	#[inline(always)]
	pub fn add_Register8Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x00);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r/m8` to `r8`.
	#[inline(always)]
	pub fn add_Register8Bit_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `imm32` sign-extended to 64-bits to `RAX`.
	#[inline(always)]
	pub fn add_RAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0x05);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `imm8` to `r/m8`.
	#[inline(always)]
	pub fn add_RegisterHigh8BitsOf16Bits_Immediate8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Add `r/m8` to `r8`.
	#[inline(always)]
	pub fn add_RegisterHigh8BitsOf16Bits_Any8BitMemory(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r8` to `r/m8`.
	#[inline(always)]
	pub fn add_RegisterHigh8BitsOf16Bits_Register8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x00);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r/m8` to `r8`.
	#[inline(always)]
	pub fn add_RegisterHigh8BitsOf16Bits_Register8Bit_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r8` to `r/m8`.
	#[inline(always)]
	pub fn add_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x00);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `r/m8` to `r8`.
	#[inline(always)]
	pub fn add_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn addpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x58);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn addpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x58);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed single-precision floating-point values from `xmm2/m128` to `xmm1` and stores result in `xmm1`.
	#[inline(always)]
	pub fn addps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x58);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed single-precision floating-point values from `xmm2/m128` to `xmm1` and stores result in `xmm1`.
	#[inline(always)]
	pub fn addps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x58);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add the low double-precision floating-point value from `xmm2/m64` to `xmm1`.
	#[inline(always)]
	pub fn addsd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x58);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add the low double-precision floating-point value from `xmm2/m64` to `xmm1`.
	#[inline(always)]
	pub fn addsd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x58);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add the low single-precision floating-point value from `xmm2/m32` to `xmm1`.
	#[inline(always)]
	pub fn addss_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x58);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add the low single-precision floating-point value from `xmm2/m32` to `xmm1`.
	#[inline(always)]
	pub fn addss_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x58);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add/subtract double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn addsubpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add/subtract double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn addsubpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add/subtract single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn addsubps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add/subtract single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn addsubps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform one round of an AES decryption flow, using the Equivalent Inverse Cipher, operating on a 128-bit data (state) from `xmm1` with a 128-bit round key from `xmm2/m128`.
	#[inline(always)]
	pub fn aesdec_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform one round of an AES decryption flow, using the Equivalent Inverse Cipher, operating on a 128-bit data (state) from `xmm1` with a 128-bit round key from `xmm2/m128`.
	#[inline(always)]
	pub fn aesdec_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform the last round of an AES decryption flow, using the Equivalent Inverse Cipher, operating on a 128-bit data (state) from `xmm1` with a 128-bit round key from `xmm2/m128`.
	#[inline(always)]
	pub fn aesdeclast_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform the last round of an AES decryption flow, using the Equivalent Inverse Cipher, operating on a 128-bit data (state) from `xmm1` with a 128-bit round key from `xmm2/m128`.
	#[inline(always)]
	pub fn aesdeclast_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform one round of an AES encryption flow, operating on a 128-bit data (state) from `xmm1` with a 128-bit round key from `xmm2/m128`.
	#[inline(always)]
	pub fn aesenc_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform one round of an AES encryption flow, operating on a 128-bit data (state) from `xmm1` with a 128-bit round key from `xmm2/m128`.
	#[inline(always)]
	pub fn aesenc_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform the last round of an AES encryption flow, operating on a 128-bit data (state) from `xmm1` with a 128-bit round key from `xmm2/m128`.
	#[inline(always)]
	pub fn aesenclast_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform the last round of an AES encryption flow, operating on a 128-bit data (state) from `xmm1` with a 128-bit round key from `xmm2/m128`.
	#[inline(always)]
	pub fn aesenclast_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform the InvMixColumn transformation on a 128-bit round key from `xmm2/m128` and store the result in `xmm1`.
	#[inline(always)]
	pub fn aesimc_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform the InvMixColumn transformation on a 128-bit round key from `xmm2/m128` and store the result in `xmm1`.
	#[inline(always)]
	pub fn aesimc_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xDB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Assist in AES round key generation using an 8 bits Round Constant (RCON) specified in the `imm8`, operating on 128 bits of data specified in `xmm2/m128` and stores the result in `xmm1`.
	#[inline(always)]
	pub fn aeskeygenassist_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0xDF);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Assist in AES round key generation using an 8 bits Round Constant (RCON) specified in the `imm8`, operating on 128 bits of data specified in `xmm2/m128` and stores the result in `xmm1`.
	#[inline(always)]
	pub fn aeskeygenassist_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0xDF);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// `AL && imm8`.
	#[inline(always)]
	pub fn and_AL_Immediate8Bit(&mut self, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x24);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `AX && imm16`.
	#[inline(always)]
	pub fn and_AX_Immediate16Bit(&mut self, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x25);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `EAX && imm32`.
	#[inline(always)]
	pub fn and_EAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x25);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m16 && imm16`.
	#[inline(always)]
	pub fn and_Any16BitMemory_Immediate16Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m16 && imm8` (sign-extended).
	#[inline(always)]
	pub fn and_Any16BitMemory_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m16 && r16`.
	#[inline(always)]
	pub fn and_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x21);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m32 && imm32`.
	#[inline(always)]
	pub fn and_Any32BitMemory_Immediate32Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m32 && imm8` (sign-extended).
	#[inline(always)]
	pub fn and_Any32BitMemory_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m32 && r32`.
	#[inline(always)]
	pub fn and_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x21);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m64 && imm32` sign extended to 64-bits.
	#[inline(always)]
	pub fn and_Any64BitMemory_Immediate32Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m64 && imm8` (sign-extended).
	#[inline(always)]
	pub fn and_Any64BitMemory_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m64 && r32`.
	#[inline(always)]
	pub fn and_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x21);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8 && imm8`.
	#[inline(always)]
	pub fn and_Any8BitMemory_Immediate8Bit(&mut self, arg0: Any8BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m8 && r8`.
	#[inline(always)]
	pub fn and_Any8BitMemory_Register8Bit(&mut self, arg0: Any8BitMemory, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x20);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8 && r8`.
	#[inline(always)]
	pub fn and_Any8BitMemory_RegisterHigh8BitsOf16Bits(&mut self, arg0: Any8BitMemory, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x20);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m16 && imm16`.
	#[inline(always)]
	pub fn and_Register16Bit_Immediate16Bit(&mut self, arg0: Register16Bit, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m16 && imm8` (sign-extended).
	#[inline(always)]
	pub fn and_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r16 && r/m16`.
	#[inline(always)]
	pub fn and_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x23);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m16 && r16`.
	#[inline(always)]
	pub fn and_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x21);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r16 && r/m16`.
	#[inline(always)]
	pub fn and_Register16Bit_Register16Bit_REX_W(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x23);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m32 && imm32`.
	#[inline(always)]
	pub fn and_Register32Bit_Immediate32Bit(&mut self, arg0: Register32Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m32 && imm8` (sign-extended).
	#[inline(always)]
	pub fn and_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r32 && r/m32`.
	#[inline(always)]
	pub fn and_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x23);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m32 && r32`.
	#[inline(always)]
	pub fn and_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x21);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r32 && r/m32`.
	#[inline(always)]
	pub fn and_Register32Bit_Register32Bit_REX_W(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x23);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m64 && imm32` sign extended to 64-bits.
	#[inline(always)]
	pub fn and_Register64Bit_Immediate32Bit(&mut self, arg0: Register64Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m64 && imm8` (sign-extended).
	#[inline(always)]
	pub fn and_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r64 && r/m64`.
	#[inline(always)]
	pub fn and_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x23);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m64 && r32`.
	#[inline(always)]
	pub fn and_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x21);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r64 && r/m64`.
	#[inline(always)]
	pub fn and_Register64Bit_Register64Bit_REX_W(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x23);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8 && imm8`.
	#[inline(always)]
	pub fn and_Register8Bit_Immediate8Bit(&mut self, arg0: Register8Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r8 && r/m8`.
	#[inline(always)]
	pub fn and_Register8Bit_Any8BitMemory(&mut self, arg0: Register8Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x22);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8 && r8`.
	#[inline(always)]
	pub fn and_Register8Bit_Register8Bit(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x20);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r8 && r/m8`.
	#[inline(always)]
	pub fn and_Register8Bit_Register8Bit_REX_W(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x22);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8 && r8`.
	#[inline(always)]
	pub fn and_Register8Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x20);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r8 && r/m8`.
	#[inline(always)]
	pub fn and_Register8Bit_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x22);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `RAX AND imm32` sign-extended to 64-bits.
	#[inline(always)]
	pub fn and_RAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0x25);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m8 && imm8`.
	#[inline(always)]
	pub fn and_RegisterHigh8BitsOf16Bits_Immediate8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r8 && r/m8`.
	#[inline(always)]
	pub fn and_RegisterHigh8BitsOf16Bits_Any8BitMemory(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x22);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8 && r8`.
	#[inline(always)]
	pub fn and_RegisterHigh8BitsOf16Bits_Register8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x20);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r8 && r/m8`.
	#[inline(always)]
	pub fn and_RegisterHigh8BitsOf16Bits_Register8Bit_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x22);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8 && r8`.
	#[inline(always)]
	pub fn and_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x20);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r8 && r/m8`.
	#[inline(always)]
	pub fn and_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x22);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise AND of inverted `r32b` with `r/m32` with result stored in in `r32a`.
	#[inline(always)]
	pub fn andn_Register32Bit_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Any32BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg2);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg2);

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg1, arg2, arg0);

		self.opcode_1(0xF2);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Bitwise AND of inverted `r32b` with `r/m32` with result stored in in `r32a`.
	#[inline(always)]
	pub fn andn_Register32Bit_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg1, arg2, arg0);

		self.opcode_1(0xF2);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Bitwise AND of inverted `r64b` with `r/m64` with result stored in in `r64a`.
	#[inline(always)]
	pub fn andn_Register64Bit_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Any64BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg2);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg2);

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg1, arg2, arg0);

		self.opcode_1(0xF2);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Bitwise AND of inverted `r64b` with `r/m64` with result stored in in `r64a`.
	#[inline(always)]
	pub fn andn_Register64Bit_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg1, arg2, arg0);

		self.opcode_1(0xF2);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Bitwise logical AND NOT of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn andnpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x55);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise logical AND NOT of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn andnpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x55);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise logical AND NOT of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn andnps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x55);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise logical AND NOT of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn andnps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x55);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the bitwise logical AND of packed double-precision floating-point values in `xmm1` and `xmm2/m128`.
	#[inline(always)]
	pub fn andpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x54);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the bitwise logical AND of packed double-precision floating-point values in `xmm1` and `xmm2/m128`.
	#[inline(always)]
	pub fn andpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x54);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise logical AND of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn andps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x54);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise logical AND of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn andps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x54);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Contiguous bitwise extract from `r/m32` using `r32b` as control.
	///
	/// Stores result in `r32a`.
	#[inline(always)]
	pub fn bextr_Register32Bit_Any32BitMemory_Register32Bit(&mut self, arg0: Register32Bit, arg1: Any32BitMemory, arg2: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg2, arg1, arg0);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Contiguous bitwise extract from `r/m32` using `r32b` as control.
	///
	/// Stores result in `r32a`.
	#[inline(always)]
	pub fn bextr_Register32Bit_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg2, arg1, arg0);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Contiguous bitwise extract from `r/m64` using `r64b` as control.
	///
	/// Stores result in `r64a`.
	#[inline(always)]
	pub fn bextr_Register64Bit_Any64BitMemory_Register64Bit(&mut self, arg0: Register64Bit, arg1: Any64BitMemory, arg2: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg2, arg1, arg0);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Contiguous bitwise extract from `r/m64` using `r64b` as control.
	///
	/// Stores result in `r64a`.
	#[inline(always)]
	pub fn bextr_Register64Bit_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg2, arg1, arg0);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Select packed DP-FP values from `xmm1` and `xmm2/m128` from mask specified in `imm8` and store the values into `xmm1`.
	#[inline(always)]
	pub fn blendpd_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0D);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Select packed DP-FP values from `xmm1` and `xmm2/m128` from mask specified in `imm8` and store the values into `xmm1`.
	#[inline(always)]
	pub fn blendpd_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0D);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Select packed single-precision floating-point values from `xmm1` and `xmm2/m128` from mask specified in `imm8` and store the values into `xmm1`.
	#[inline(always)]
	pub fn blendps_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0C);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Select packed single-precision floating-point values from `xmm1` and `xmm2/m128` from mask specified in `imm8` and store the values into `xmm1`.
	#[inline(always)]
	pub fn blendps_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0C);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Select packed DP FP values from `xmm1` and `xmm2` from mask specified in `XMM0` and store the values in `xmm1`.
	#[inline(always)]
	pub fn blendvpd_XMMRegister_Any128BitMemory_XMMRegister0(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x15);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Select packed DP FP values from `xmm1` and `xmm2` from mask specified in `XMM0` and store the values in `xmm1`.
	#[inline(always)]
	pub fn blendvpd_XMMRegister_XMMRegister_XMMRegister0(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x15);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Select packed single-precision floating-point values from `xmm1` and `xmm2/m128` from mask specified in `XMM0` and store the values into `xmm1`.
	#[inline(always)]
	pub fn blendvps_XMMRegister_Any128BitMemory_XMMRegister0(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x14);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Select packed single-precision floating-point values from `xmm1` and `xmm2/m128` from mask specified in `XMM0` and store the values into `xmm1`.
	#[inline(always)]
	pub fn blendvps_XMMRegister_XMMRegister_XMMRegister0(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x14);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Extract lowest set bit from `r/m32` and set that bit in `r32`.
	#[inline(always)]
	pub fn blsi_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg0, arg1, Register64Bit::RBX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Extract lowest set bit from `r/m32` and set that bit in `r32`.
	#[inline(always)]
	pub fn blsi_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg0, arg1, Register64Bit::RBX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Extract lowest set bit from `r/m64`, and set that bit in `r64`.
	#[inline(always)]
	pub fn blsi_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg0, arg1, Register64Bit::RBX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Extract lowest set bit from `r/m64`, and set that bit in `r64`.
	#[inline(always)]
	pub fn blsi_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg0, arg1, Register64Bit::RBX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Set all lower bits in `r32` to `1` starting from bit 0 to lowest set bit in `r/m32`.
	#[inline(always)]
	pub fn blsmsk_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg0, arg1, Register64Bit::RDX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Set all lower bits in `r32` to `1` starting from bit 0 to lowest set bit in `r/m32`.
	#[inline(always)]
	pub fn blsmsk_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg0, arg1, Register64Bit::RDX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Set all lower bits in `r64` to `1` starting from bit 0 to lowest set bit in `r/m64`.
	#[inline(always)]
	pub fn blsmsk_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg0, arg1, Register64Bit::RDX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Set all lower bits in `r64` to `1` starting from bit 0 to lowest set bit in `r/m64`.
	#[inline(always)]
	pub fn blsmsk_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg0, arg1, Register64Bit::RDX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Reset lowest set bit of `r/m32`, keep all other bits of `r/m32` and write result to `r32`.
	#[inline(always)]
	pub fn blsr_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg0, arg1, Register64Bit::RCX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Reset lowest set bit of `r/m32`, keep all other bits of `r/m32` and write result to `r32`.
	#[inline(always)]
	pub fn blsr_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg0, arg1, Register64Bit::RCX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Reset lowest set bit of `r/m64`, keep all other bits of `r/m64` and write result to `r64`.
	#[inline(always)]
	pub fn blsr_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg0, arg1, Register64Bit::RCX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Reset lowest set bit of `r/m64`, keep all other bits of `r/m64` and write result to `r64`.
	#[inline(always)]
	pub fn blsr_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg0, arg1, Register64Bit::RCX);

		self.opcode_1(0xF3);

		self.mod_rm_sib(arg1, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Bit scan forward on `r/m16`.
	#[inline(always)]
	pub fn bsf_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan forward on `r/m16`.
	#[inline(always)]
	pub fn bsf_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan forward on `r/m32`.
	#[inline(always)]
	pub fn bsf_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan forward on `r/m32`.
	#[inline(always)]
	pub fn bsf_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan forward on `r/m64`.
	#[inline(always)]
	pub fn bsf_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan forward on `r/m64`.
	#[inline(always)]
	pub fn bsf_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan reverse on `r/m16`.
	#[inline(always)]
	pub fn bsr_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan reverse on `r/m16`.
	#[inline(always)]
	pub fn bsr_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan reverse on `r/m32`.
	#[inline(always)]
	pub fn bsr_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan reverse on `r/m32`.
	#[inline(always)]
	pub fn bsr_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan reverse on `r/m64`.
	#[inline(always)]
	pub fn bsr_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bit scan reverse on `r/m64`.
	#[inline(always)]
	pub fn bsr_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverses the byte order of a 32-bit register.
	#[inline(always)]
	pub fn bswap_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0x0F, 0xC8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverses the byte order of a 64-bit register.
	#[inline(always)]
	pub fn bswap_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_3(0x0F, 0xC8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Any16BitMemory_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xA3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Any32BitMemory_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xA3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Any64BitMemory_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xA3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xA3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xA3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF).
	#[inline(always)]
	pub fn bt_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xA3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Any16BitMemory_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xBB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Any32BitMemory_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xBB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Any64BitMemory_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xBB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xBB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xBB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and complement.
	#[inline(always)]
	pub fn btc_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xBB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Any16BitMemory_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Any32BitMemory_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Any64BitMemory_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xB3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and clear.
	#[inline(always)]
	pub fn btr_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xB3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Any16BitMemory_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xAB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Any32BitMemory_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xAB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Any64BitMemory_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xAB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xAB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xAB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBA);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Store selected bit in Carry Flag (CF) and set.
	#[inline(always)]
	pub fn bts_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xAB);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero bits in `r/m32` starting with the position in `r32b`, write result to `r32a`.
	#[inline(always)]
	pub fn bzhi_Register32Bit_Any32BitMemory_Register32Bit(&mut self, arg0: Register32Bit, arg1: Any32BitMemory, arg2: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg2, arg1, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Zero bits in `r/m32` starting with the position in `r32b`, write result to `r32a`.
	#[inline(always)]
	pub fn bzhi_Register32Bit_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x0, arg2, arg1, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Zero bits in `r/m64` starting with the position in `r64b`, write result to `r64a`.
	#[inline(always)]
	pub fn bzhi_Register64Bit_Any64BitMemory_Register64Bit(&mut self, arg0: Register64Bit, arg1: Any64BitMemory, arg2: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg1);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg1);

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg2, arg1, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Zero bits in `r/m64` starting with the position in `r64b`, write result to `r64a`.
	#[inline(always)]
	pub fn bzhi_Register64Bit_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x0, 0x1, arg2, arg1, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Call far, absolute indirect address given in `m16:16`.
	///
	/// In 32-bit mode: if selector points to a gate, then `RIP` = 32-bit zero extended displacement taken from gate else `RIP` = zero extended 16-bit offset from far pointer referenced in the instruction.
	#[inline(always)]
	pub fn call_FarPointer16BitTo16Bit(&mut self, arg0: FarPointer16BitTo16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Call far, absolute indirect address given in `m16:32`.
	///
	/// In 64-bit mode: If selector points to a gate, then `RIP` = 64-bit displacement taken from gate else `RIP` = zero extended 32-bit offset from far pointer referenced in the instruction.
	#[inline(always)]
	pub fn call_FarPointer16BitTo32Bit(&mut self, arg0: FarPointer16BitTo32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Call far, absolute indirect address given in `m16:64`.
	///
	/// In 64-bit mode: If selector points to a gate, then `RIP` = 64-bit displacement taken from gate else `RIP` = 64-bit offset from far pointer referenced in the instruction.
	#[inline(always)]
	pub fn call_FarPointer16BitTo64Bit(&mut self, arg0: FarPointer16BitTo64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Call near, relative, displacement relative to next instruction.
	///
	/// 32-bit displacement sign extended to 64-bits in 64-bit mode.
	#[inline(always)]
	pub fn call_Label(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Call near, absolute indirect, address given in `r/m64`.
	#[inline(always)]
	pub fn call_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Call near, absolute indirect, address given in `r/m64`.
	#[inline(always)]
	pub fn call_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Call near, relative, displacement relative to next instruction.
	///
	/// 32-bit displacement sign extended to 64-bits in 64-bit mode.
	#[inline(always)]
	pub fn call_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// `AX` = sign-extend of `AL`.
	#[inline(always)]
	pub fn cbw(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x98);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// `EDX:EAX` = sign-extend of `EAX`.
	#[inline(always)]
	pub fn cdq(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x99);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// `RAX` = sign-extend of `EAX`.
	#[inline(always)]
	pub fn cdqe(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0x98);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Clear Carry Flag (CF).
	#[inline(always)]
	pub fn clc(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xF8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Clear Direction Flag (DF).
	#[inline(always)]
	pub fn cld(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xFC);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Flushes cache line containing `m8`.
	#[inline(always)]
	pub fn clflush_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xAE);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Clear interrupt flag; interrupts disabled when interrupt flag cleared.
	#[inline(always)]
	pub fn cli(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xFA);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Complement Carry Flag (CF).
	#[inline(always)]
	pub fn cmc(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xF5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmova_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmova_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmova_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmova_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmova_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmova_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovae_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovae_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovae_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovae_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovae_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovae_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovb_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovb_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovb_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovb_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovb_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovb_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovbe_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovbe_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovbe_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovbe_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovbe_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovbe_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovc_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovc_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovc_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovc_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovc_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovc_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmove_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmove_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmove_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmove_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmove_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmove_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovg_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovg_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovg_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovg_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovg_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovg_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovge_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovge_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovge_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovge_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovge_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovge_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovl_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovl_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovl_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovl_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovl_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovl_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovle_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovle_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovle_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovle_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovle_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovle_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovna_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovna_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovna_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovna_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovna_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovna_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x46);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovnae_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovnae_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovnae_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovnae_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovnae_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn cmovnae_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x42);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnb_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnb_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnb_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnb_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnb_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnb_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnbe_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnbe_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnbe_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnbe_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnbe_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnbe_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x47);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnc_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnc_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnc_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnc_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnc_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn cmovnc_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x43);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovne_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovne_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovne_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovne_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovne_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovne_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovng_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovng_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovng_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovng_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovng_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovng_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnge_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnge_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnge_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnge_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnge_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnge_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnl_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnl_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnl_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnl_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnl_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnl_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnle_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnle_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnle_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnle_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnle_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn cmovnle_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn cmovno_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x41);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn cmovno_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x41);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn cmovno_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x41);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn cmovno_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x41);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn cmovno_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x41);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn cmovno_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x41);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovnp_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovnp_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovnp_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovnp_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovnp_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovnp_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn cmovns_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x49);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn cmovns_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x49);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn cmovns_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x49);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn cmovns_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x49);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn cmovns_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x49);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn cmovns_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x49);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnz_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnz_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnz_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnz_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnz_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn cmovnz_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x45);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn cmovo_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x40);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn cmovo_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x40);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn cmovo_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x40);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn cmovo_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x40);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn cmovo_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x40);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn cmovo_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x40);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovp_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovp_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovp_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovp_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovp_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovp_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovpe_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovpe_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovpe_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovpe_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovpe_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn cmovpe_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovpo_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovpo_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovpo_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovpo_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovpo_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn cmovpo_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x4B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn cmovs_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x48);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn cmovs_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x48);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn cmovs_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x48);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn cmovs_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x48);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn cmovs_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x48);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn cmovs_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x48);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovz_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovz_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovz_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovz_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovz_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn cmovz_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x44);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `imm8` with `AL`.
	#[inline(always)]
	pub fn cmp_AL_Immediate8Bit(&mut self, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x3C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm16` with `AX`.
	#[inline(always)]
	pub fn cmp_AX_Immediate16Bit(&mut self, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x3D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm32` with `EAX`.
	#[inline(always)]
	pub fn cmp_EAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x3D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm16` with `r/m16`.
	#[inline(always)]
	pub fn cmp_Any16BitMemory_Immediate16Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm8` with `r/m16`.
	#[inline(always)]
	pub fn cmp_Any16BitMemory_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `r16` with `r/m16`.
	#[inline(always)]
	pub fn cmp_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x39);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `imm32` with `r/m32`.
	#[inline(always)]
	pub fn cmp_Any32BitMemory_Immediate32Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm8` with `r/m32`.
	#[inline(always)]
	pub fn cmp_Any32BitMemory_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `r32` with `r/m32`.
	#[inline(always)]
	pub fn cmp_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x39);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `imm32` sign-extended to 64-bits with `r/m64`.
	#[inline(always)]
	pub fn cmp_Any64BitMemory_Immediate32Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm8` with `r/m64`.
	#[inline(always)]
	pub fn cmp_Any64BitMemory_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `r64` with `r/m64`.
	#[inline(always)]
	pub fn cmp_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x39);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `imm8` with `r/m8`.
	#[inline(always)]
	pub fn cmp_Any8BitMemory_Immediate8Bit(&mut self, arg0: Any8BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `r8` with `r/m8`.
	#[inline(always)]
	pub fn cmp_Any8BitMemory_Register8Bit(&mut self, arg0: Any8BitMemory, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x38);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r8` with `r/m8`.
	#[inline(always)]
	pub fn cmp_Any8BitMemory_RegisterHigh8BitsOf16Bits(&mut self, arg0: Any8BitMemory, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x38);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `imm16` with `r/m16`.
	#[inline(always)]
	pub fn cmp_Register16Bit_Immediate16Bit(&mut self, arg0: Register16Bit, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm8` with `r/m16`.
	#[inline(always)]
	pub fn cmp_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `r/m16` with `r16`.
	#[inline(always)]
	pub fn cmp_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r16` with `r/m16`.
	#[inline(always)]
	pub fn cmp_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x39);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r/m16` with `r16`.
	#[inline(always)]
	pub fn cmp_Register16Bit_Register16Bit_REX_W(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `imm32` with `r/m32`.
	#[inline(always)]
	pub fn cmp_Register32Bit_Immediate32Bit(&mut self, arg0: Register32Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm8` with `r/m32`.
	#[inline(always)]
	pub fn cmp_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `r/m32` with `r32`.
	#[inline(always)]
	pub fn cmp_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r32` with `r/m32`.
	#[inline(always)]
	pub fn cmp_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x39);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r/m32` with `r32`.
	#[inline(always)]
	pub fn cmp_Register32Bit_Register32Bit_REX_W(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `imm32` sign-extended to 64-bits with `r/m64`.
	#[inline(always)]
	pub fn cmp_Register64Bit_Immediate32Bit(&mut self, arg0: Register64Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm8` with `r/m64`.
	#[inline(always)]
	pub fn cmp_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `r/m64` with `r64`.
	#[inline(always)]
	pub fn cmp_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x3B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r64` with `r/m64`.
	#[inline(always)]
	pub fn cmp_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x39);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r/m64` with `r64`.
	#[inline(always)]
	pub fn cmp_Register64Bit_Register64Bit_REX_W(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x3B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `imm8` with `r/m8`.
	#[inline(always)]
	pub fn cmp_Register8Bit_Immediate8Bit(&mut self, arg0: Register8Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `r/m8` with `r8`.
	#[inline(always)]
	pub fn cmp_Register8Bit_Any8BitMemory(&mut self, arg0: Register8Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r8` with `r/m8`.
	#[inline(always)]
	pub fn cmp_Register8Bit_Register8Bit(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x38);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r/m8` with `r8`.
	#[inline(always)]
	pub fn cmp_Register8Bit_Register8Bit_REX_W(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r8` with `r/m8`.
	#[inline(always)]
	pub fn cmp_Register8Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x38);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r/m8` with `r8`.
	#[inline(always)]
	pub fn cmp_Register8Bit_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `imm32` sign-extended to 64-bits with `RAX`.
	#[inline(always)]
	pub fn cmp_RAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0x3D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `imm8` with `r/m8`.
	#[inline(always)]
	pub fn cmp_RegisterHigh8BitsOf16Bits_Immediate8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Compare `r/m8` with `r8`.
	#[inline(always)]
	pub fn cmp_RegisterHigh8BitsOf16Bits_Any8BitMemory(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r8` with `r/m8`.
	#[inline(always)]
	pub fn cmp_RegisterHigh8BitsOf16Bits_Register8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x38);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r/m8` with `r8`.
	#[inline(always)]
	pub fn cmp_RegisterHigh8BitsOf16Bits_Register8Bit_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r8` with `r/m8`.
	#[inline(always)]
	pub fn cmp_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x38);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `r/m8` with `r8`.
	#[inline(always)]
	pub fn cmp_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x3A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed double-precision floating-point values in `xmm2/m128` and `xmm1` using `imm8` as comparison predicate.
	#[inline(always)]
	pub fn cmppd_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC2);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Compare packed double-precision floating-point values in `xmm2/m128` and `xmm1` using `imm8` as comparison predicate.
	#[inline(always)]
	pub fn cmppd_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC2);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Compare packed single-precision floating-point values in `xmm2/mem` and `xmm1` using `imm8` as comparison predicate.
	#[inline(always)]
	pub fn cmpps_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC2);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Compare packed single-precision floating-point values in `xmm2/mem` and `xmm1` using `imm8` as comparison predicate.
	#[inline(always)]
	pub fn cmpps_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC2);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// For legacy mode, compare word at address `DS:(E)SI` with word at address `ES:(E)DI`.
	///
	/// For 64-bit mode compare word at address `(R
	///E)SI` with word at address `(R
	///E)DI`.
	///
	/// The status flags are set accordingly.
	#[inline(always)]
	pub fn cmps_Any16BitMemory_Any16BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA7);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, compare dword at address `DS:(E)SI` at dword at address `ES:(E)DI`.
	///
	/// For 64-bit mode compare dword at address ``(R
	///E)SI`` at dword at address `(R
	///E)DI`.
	///
	/// The status flags are set accordingly.
	#[inline(always)]
	pub fn cmps_Any32BitMemory_Any32BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA7);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compares quadword at address `(R
	///E)SI` with quadword at address `(R
	///E)DI` and sets the status flags accordingly.
	#[inline(always)]
	pub fn cmps_Any64BitMemory_Any64BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xA7);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, compare byte at address `DS:(E)SI` with byte at address `ES:(E)DI`.
	///
	/// For 64-bit mode compare byte at address `(R
	///E)SI` to byte at address `(R
	///E)DI`.
	///
	/// The status flags are set accordingly.
	#[inline(always)]
	pub fn cmps_Any8BitMemory_Any8BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA6);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, compare byte at address `DS:(E)SI` with byte at address `ES:(E)DI`.
	///
	/// For 64-bit mode compare byte at address `(R
	///E)SI` with byte at address `(R
	///E)DI`.
	///
	/// The status flags are set accordingly.
	#[inline(always)]
	pub fn cmpsb(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA6);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, compare dword at address `DS:(E)SI` with dword at address `ES:(E)DI`.
	///
	/// For 64-bit mode compare dword at address `(R
	///E)SI` with dword at address `(R
	///E)DI`.
	///
	/// The status flags are set accordingly.
	#[inline(always)]
	pub fn cmpsd(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA7);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare low double-precision floating-point value in `xmm2/m64` and `xmm1` using `imm8` as comparison predicate.
	#[inline(always)]
	pub fn cmpsd_XMMRegister_Any64BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any64BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC2);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Compare low double-precision floating-point value in `xmm2/m64` and `xmm1` using `imm8` as comparison predicate.
	#[inline(always)]
	pub fn cmpsd_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC2);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Compares quadword at address `(R
	///E)SI` with quadword at address `(R
	///E)DI` and sets the status flags accordingly.
	#[inline(always)]
	pub fn cmpsq(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xA7);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare low single-precision floating-point value in `xmm2/m32` and `xmm1` using `imm8` as comparison predicate.
	#[inline(always)]
	pub fn cmpss_XMMRegister_Any32BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any32BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC2);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Compare low single-precision floating-point value in `xmm2/m32` and `xmm1` using `imm8` as comparison predicate.
	#[inline(always)]
	pub fn cmpss_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC2);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// For legacy mode, compare word at address `DS:(E)SI` with word at address `ES:(E)DI`.
	///
	/// For 64-bit mode compare word at address `(R
	///E)SI` with word at address `(R
	///E)DI`.
	///
	/// The status flags are set accordingly.
	#[inline(always)]
	pub fn cmpsw(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA7);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `AX` with `r/m16`.
	///
	/// If equal the Zero Flag (ZF) is set and `r16` is loaded into `r/m16`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m16` into `AX`.
	#[inline(always)]
	pub fn cmpxchg_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB1);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `EAX` with `r/m32`.
	///
	/// If equal the Zero Flag (ZF) is set and `r32` is loaded into `r/m32`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m32` into `EAX`.
	#[inline(always)]
	pub fn cmpxchg_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB1);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `RAX` with `r/m64`.
	///
	/// If equal the Zero Flag (ZF) is set and `r64` is loaded into `r/m64`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m64` into `RAX`.
	#[inline(always)]
	pub fn cmpxchg_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xB1);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `AL` with `r/m8`.
	///
	/// If equal the Zero Flag (ZF) is set and `r8` is loaded into `r/m8`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m8` into `AL`.
	#[inline(always)]
	pub fn cmpxchg_Any8BitMemory_Register8Bit(&mut self, arg0: Any8BitMemory, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB0);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `AL` with `r/m8`.
	///
	/// If equal the Zero Flag (ZF) is set and `r8` is loaded into `r/m8`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m8` into `AL`.
	#[inline(always)]
	pub fn cmpxchg_Any8BitMemory_RegisterHigh8BitsOf16Bits(&mut self, arg0: Any8BitMemory, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB0);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `AX` with `r/m16`.
	///
	/// If equal the Zero Flag (ZF) is set and `r16` is loaded into `r/m16`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m16` into `AX`.
	#[inline(always)]
	pub fn cmpxchg_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB1);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `EAX` with `r/m32`.
	///
	/// If equal the Zero Flag (ZF) is set and `r32` is loaded into `r/m32`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m32` into `EAX`.
	#[inline(always)]
	pub fn cmpxchg_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB1);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `RAX` with `r/m64`.
	///
	/// If equal the Zero Flag (ZF) is set and `r64` is loaded into `r/m64`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m64` into `RAX`.
	#[inline(always)]
	pub fn cmpxchg_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xB1);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `AL` with `r/m8`.
	///
	/// If equal the Zero Flag (ZF) is set and `r8` is loaded into `r/m8`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m8` into `AL`.
	#[inline(always)]
	pub fn cmpxchg_Register8Bit_Register8Bit(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB0);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `AL` with `r/m8`.
	///
	/// If equal the Zero Flag (ZF) is set and `r8` is loaded into `r/m8`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m8` into `AL`.
	#[inline(always)]
	pub fn cmpxchg_Register8Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB0);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `AL` with `r/m8`.
	///
	/// If equal the Zero Flag (ZF) is set and `r8` is loaded into `r/m8`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m8` into `AL`.
	#[inline(always)]
	pub fn cmpxchg_RegisterHigh8BitsOf16Bits_Register8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB0);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `AL` with `r/m8`.
	///
	/// If equal the Zero Flag (ZF) is set and `r8` is loaded into `r/m8`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `r/m8` into `AL`.
	#[inline(always)]
	pub fn cmpxchg_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xB0);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `RDX:RAX` with `m128`.
	///
	/// If equal, set ZF and load `RCX:RBX` into `m128`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `m128` into `RDX:RAX`.
	#[inline(always)]
	pub fn cmpxchg16b_Any128BitMemory(&mut self, arg0: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xC7);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `EDX:EAX` with `m64`.
	///
	/// If equal, set ZF and load `ECX:EBX` into `m64`.
	///
	/// Otherwise clears the Zero Flag (ZF) and loads `m64` into `EDX:EAX`.
	#[inline(always)]
	pub fn cmpxchg8b_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xC7);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare low double-precision floating-point values in `xmm1` and `xmm2/mem64` and sets the appropriate flags in `EFLAGS` accordingly.
	#[inline(always)]
	pub fn comisd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare low double-precision floating-point values in `xmm1` and `xmm2/mem64` and sets the appropriate flags in `EFLAGS` accordingly.
	#[inline(always)]
	pub fn comisd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare low single-precision floating-point values in `xmm1` and `xmm2/mem32` and sets the appropriate flags in `EFLAGS` accordingly.
	#[inline(always)]
	pub fn comiss_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare low single-precision floating-point values in `xmm1` and `xmm2/mem32` and sets the appropriate flags in `EFLAGS` accordingly.
	#[inline(always)]
	pub fn comiss_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Returns processor identification and feature information into the `EAX`, `EBX`, `ECX`, and `EDX` registers as determined by the value in `EAX`, and, in some cases, `ECX` as well.
	#[inline(always)]
	pub fn cpuid(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0xA2);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// `RDX:RAX` = sign-extend of `RAX`.
	#[inline(always)]
	pub fn cqo(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0x99);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m16`.
	#[inline(always)]
	pub fn crc32_Register32Bit_Any16BitMemory(&mut self, arg0: Register32Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m32`.
	#[inline(always)]
	pub fn crc32_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m8`.
	#[inline(always)]
	pub fn crc32_Register32Bit_Any8BitMemory(&mut self, arg0: Register32Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m16`.
	#[inline(always)]
	pub fn crc32_Register32Bit_Register16Bit(&mut self, arg0: Register32Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m32`.
	#[inline(always)]
	pub fn crc32_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m8`.
	#[inline(always)]
	pub fn crc32_Register32Bit_Register8Bit(&mut self, arg0: Register32Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m8`.
	#[inline(always)]
	pub fn crc32_Register32Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register32Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m64`.
	#[inline(always)]
	pub fn crc32_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_3(0x0F, 0x38, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m8`.
	#[inline(always)]
	pub fn crc32_Register64Bit_Any8BitMemory(&mut self, arg0: Register64Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_3(0x0F, 0x38, 0xF0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m64`.
	#[inline(always)]
	pub fn crc32_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_3(0x0F, 0x38, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Accumulate Cyclic Redundancy Check 32 (CRC32) on `r/m8`.
	#[inline(always)]
	pub fn crc32_Register64Bit_Register8Bit(&mut self, arg0: Register64Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_3(0x0F, 0x38, 0xF0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed signed doubleword integers from `xmm2/m128` to two packed double-precision floating-point values in `xmm1`.
	#[inline(always)]
	pub fn cvtdq2pd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed signed doubleword integers from `xmm2/m128` to two packed double-precision floating-point values in `xmm1`.
	#[inline(always)]
	pub fn cvtdq2pd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert four packed signed doubleword integers from `xmm2/m128` to four packed single-precision floating-point values in `xmm1`.
	#[inline(always)]
	pub fn cvtdq2ps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert four packed signed doubleword integers from `xmm2/m128` to four packed single-precision floating-point values in `xmm1`.
	#[inline(always)]
	pub fn cvtdq2ps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed double-precision floating-point values from `xmm2/m128` to two packed signed doubleword integers in `xmm1`.
	#[inline(always)]
	pub fn cvtpd2dq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed double-precision floating-point values from `xmm2/m128` to two packed signed doubleword integers in `xmm1`.
	#[inline(always)]
	pub fn cvtpd2dq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed double-precision floating-point values from `xmm/m32` to two packed signed doubleword integers in `mm`.
	#[inline(always)]
	pub fn cvtpd2pi_MMRegister_Any128BitMemory(&mut self, arg0: MMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed double-precision floating-point values from `xmm/m32` to two packed signed doubleword integers in `mm`.
	#[inline(always)]
	pub fn cvtpd2pi_MMRegister_XMMRegister(&mut self, arg0: MMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed double-precision floating-point values in `xmm2/m128` to two packed single-precision floating-point values in `xmm1`.
	#[inline(always)]
	pub fn cvtpd2ps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed double-precision floating-point values in `xmm2/m128` to two packed single-precision floating-point values in `xmm1`.
	#[inline(always)]
	pub fn cvtpd2ps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed signed doubleword integers from `mm/mem64` to two packed double-precision floating-point values in `xmm`.
	#[inline(always)]
	pub fn cvtpi2pd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed signed doubleword integers from `mm/mem64` to two packed double-precision floating-point values in `xmm`.
	#[inline(always)]
	pub fn cvtpi2pd_XMMRegister_MMRegister(&mut self, arg0: XMMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two signed doubleword integers from `mm/m64` to two single-precision floating-point values in `xmm`.
	#[inline(always)]
	pub fn cvtpi2ps_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two signed doubleword integers from `mm/m64` to two single-precision floating-point values in `xmm`.
	#[inline(always)]
	pub fn cvtpi2ps_XMMRegister_MMRegister(&mut self, arg0: XMMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert four packed single-precision floating-point values from `xmm2/m128` to four packed signed doubleword integers in `xmm1`.
	#[inline(always)]
	pub fn cvtps2dq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert four packed single-precision floating-point values from `xmm2/m128` to four packed signed doubleword integers in `xmm1`.
	#[inline(always)]
	pub fn cvtps2dq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed single-precision floating-point values in `xmm2/m64` to two packed double-precision floating-point values in `xmm1`.
	#[inline(always)]
	pub fn cvtps2pd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed single-precision floating-point values in `xmm2/m64` to two packed double-precision floating-point values in `xmm1`.
	#[inline(always)]
	pub fn cvtps2pd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed single-precision floating-point values from `xmm/m64` to two packed signed doubleword integers in `mm`.
	#[inline(always)]
	pub fn cvtps2pi_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed single-precision floating-point values from `xmm/m64` to two packed signed doubleword integers in `mm`.
	#[inline(always)]
	pub fn cvtps2pi_MMRegister_XMMRegister(&mut self, arg0: MMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value from `xmm/m64` to one signed doubleword integer `r32`.
	#[inline(always)]
	pub fn cvtsd2si_Register32Bit_Any64BitMemory(&mut self, arg0: Register32Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value from `xmm/m64` to one signed doubleword integer `r32`.
	#[inline(always)]
	pub fn cvtsd2si_Register32Bit_XMMRegister(&mut self, arg0: Register32Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value from `xmm/m64` to one signed quadword integer sign-extended into `r64`.
	#[inline(always)]
	pub fn cvtsd2si_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value from `xmm/m64` to one signed quadword integer sign-extended into `r64`.
	#[inline(always)]
	pub fn cvtsd2si_Register64Bit_XMMRegister(&mut self, arg0: Register64Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value in `xmm2/m64` to one single-precision floating-point value in `xmm1`.
	#[inline(always)]
	pub fn cvtsd2ss_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value in `xmm2/m64` to one single-precision floating-point value in `xmm1`.
	#[inline(always)]
	pub fn cvtsd2ss_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one signed doubleword integer from `r/m32` to one double-precision floating-point value in `xmm`.
	#[inline(always)]
	pub fn cvtsi2sd_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one signed quadword integer from `r/m64` to one double-precision floating-point value in `xmm`.
	#[inline(always)]
	pub fn cvtsi2sd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one signed doubleword integer from `r/m32` to one double-precision floating-point value in `xmm`.
	#[inline(always)]
	pub fn cvtsi2sd_XMMRegister_Register32Bit(&mut self, arg0: XMMRegister, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one signed quadword integer from `r/m64` to one double-precision floating-point value in `xmm`.
	#[inline(always)]
	pub fn cvtsi2sd_XMMRegister_Register64Bit(&mut self, arg0: XMMRegister, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one signed doubleword integer from `r/m32` to one single-precision floating-point value in `xmm`.
	#[inline(always)]
	pub fn cvtsi2ss_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one signed quadword integer from `r/m64` to one single-precision floating-point value in `xmm`.
	#[inline(always)]
	pub fn cvtsi2ss_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one signed doubleword integer from `r/m32` to one single-precision floating-point value in `xmm`.
	#[inline(always)]
	pub fn cvtsi2ss_XMMRegister_Register32Bit(&mut self, arg0: XMMRegister, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one signed quadword integer from `r/m64` to one single-precision floating-point value in `xmm`.
	#[inline(always)]
	pub fn cvtsi2ss_XMMRegister_Register64Bit(&mut self, arg0: XMMRegister, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value in `xmm2/m32` to one double-precision floating-point value in `xmm1`.
	#[inline(always)]
	pub fn cvtss2sd_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value in `xmm2/m32` to one double-precision floating-point value in `xmm1`.
	#[inline(always)]
	pub fn cvtss2sd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value from `xmm/m32` to one signed doubleword integer in `r32`.
	#[inline(always)]
	pub fn cvtss2si_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value from `xmm/m32` to one signed doubleword integer in `r32`.
	#[inline(always)]
	pub fn cvtss2si_Register32Bit_XMMRegister(&mut self, arg0: Register32Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value from `xmm/m32` to one signed quadword integer in `r64`.
	#[inline(always)]
	pub fn cvtss2si_Register64Bit_Any32BitMemory(&mut self, arg0: Register64Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value from `xmm/m32` to one signed quadword integer in `r64`.
	#[inline(always)]
	pub fn cvtss2si_Register64Bit_XMMRegister(&mut self, arg0: Register64Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed double-precision floating-point values from `xmm2/m128` to two packed signed doubleword integers in `xmm1` using truncation.
	#[inline(always)]
	pub fn cvttpd2dq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packed double-precision floating-point values from `xmm2/m128` to two packed signed doubleword integers in `xmm1` using truncation.
	#[inline(always)]
	pub fn cvttpd2dq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packer double-precision floating-point values from `xmm/m32` to two packed signed doubleword integers in `mm` using truncation.
	#[inline(always)]
	pub fn cvttpd2pi_MMRegister_Any128BitMemory(&mut self, arg0: MMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two packer double-precision floating-point values from `xmm/m32` to two packed signed doubleword integers in `mm` using truncation.
	#[inline(always)]
	pub fn cvttpd2pi_MMRegister_XMMRegister(&mut self, arg0: MMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert four single-precision floating-point values from `xmm2/m128` to four signed doubleword integers in `xmm1` using truncation.
	#[inline(always)]
	pub fn cvttps2dq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert four single-precision floating-point values from `xmm2/m128` to four signed doubleword integers in `xmm1` using truncation.
	#[inline(always)]
	pub fn cvttps2dq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two single-precision floating-point values from `xmm/m64` to two signed doubleword signed integers in `mm` using truncation.
	#[inline(always)]
	pub fn cvttps2pi_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert two single-precision floating-point values from `xmm/m64` to two signed doubleword signed integers in `mm` using truncation.
	#[inline(always)]
	pub fn cvttps2pi_MMRegister_XMMRegister(&mut self, arg0: MMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value from `xmm/m64` to one signed doubleword integer in `r32` using truncation.
	#[inline(always)]
	pub fn cvttsd2si_Register32Bit_Any64BitMemory(&mut self, arg0: Register32Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value from `xmm/m64` to one signed doubleword integer in `r32` using truncation.
	#[inline(always)]
	pub fn cvttsd2si_Register32Bit_XMMRegister(&mut self, arg0: Register32Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value from `xmm/m64` to one signedquadword integer in `r64` using truncation.
	#[inline(always)]
	pub fn cvttsd2si_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one double-precision floating-point value from `xmm/m64` to one signedquadword integer in `r64` using truncation.
	#[inline(always)]
	pub fn cvttsd2si_Register64Bit_XMMRegister(&mut self, arg0: Register64Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value from `xmm/m32` to one signed doubleword integer in `r32` using truncation.
	#[inline(always)]
	pub fn cvttss2si_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value from `xmm/m32` to one signed doubleword integer in `r32` using truncation.
	#[inline(always)]
	pub fn cvttss2si_Register32Bit_XMMRegister(&mut self, arg0: Register32Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value from `xmm/m32` to one signed quadword integer in `r64` using truncation.
	#[inline(always)]
	pub fn cvttss2si_Register64Bit_Any32BitMemory(&mut self, arg0: Register64Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert one single-precision floating-point value from `xmm/m32` to one signed quadword integer in `r64` using truncation.
	#[inline(always)]
	pub fn cvttss2si_Register64Bit_XMMRegister(&mut self, arg0: Register64Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x2C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `DX:AX` = sign-extend of `AX`.
	#[inline(always)]
	pub fn cwd(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x99);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// `EAX` = sign-extend of `AX`.
	#[inline(always)]
	pub fn cwde(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x98);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `r/m16` by 1.
	#[inline(always)]
	pub fn dec_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `r/m32` by 1.
	#[inline(always)]
	pub fn dec_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `r/m64` by 1.
	#[inline(always)]
	pub fn dec_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `r/m8` by 1.
	#[inline(always)]
	pub fn dec_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFE);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `r/m16` by 1.
	#[inline(always)]
	pub fn dec_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `r/m32` by 1.
	#[inline(always)]
	pub fn dec_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `r/m64` by 1.
	#[inline(always)]
	pub fn dec_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `r/m8` by 1.
	#[inline(always)]
	pub fn dec_Register8Bit(&mut self, arg0: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFE);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `r/m8` by 1.
	#[inline(always)]
	pub fn dec_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFE);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned divide `DX:AX` by `r/m16`, with result stored in `AX` (Quotient) and `DX` (Remainder).
	#[inline(always)]
	pub fn div_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned divide `EDX:EAX` by `r/m32`, with result stored in `EAX` (Quotient) and `EDX` (Remainder).
	#[inline(always)]
	pub fn div_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned divide `RDX:RAX` by `r/m64`, with result stored in `RAX` (Quotient) and `RDX` (Remainder).
	#[inline(always)]
	pub fn div_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned divide `AX` by `r/m8`, with result stored in `AL` (Quotient) and `AH` (Remainder).
	#[inline(always)]
	pub fn div_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned divide `DX:AX` by `r/m16`, with result stored in `AX` (Quotient) and `DX` (Remainder).
	#[inline(always)]
	pub fn div_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned divide `EDX:EAX` by `r/m32`, with result stored in `EAX` (Quotient) and `EDX` (Remainder).
	#[inline(always)]
	pub fn div_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned divide `RDX:RAX` by `r/m64`, with result stored in `RAX` (Quotient) and `RDX` (Remainder).
	#[inline(always)]
	pub fn div_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned divide `AX` by `r/m8`, with result stored in `AL` (Quotient) and `AH` (Remainder).
	#[inline(always)]
	pub fn div_Register8Bit(&mut self, arg0: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned divide `AX` by `r/m8`, with result stored in `AL` (Quotient) and `AH` (Remainder).
	#[inline(always)]
	pub fn div_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide packed double-precision floating-point values in `xmm1` by packed double-precision floating-point values `xmm2/m128`.
	#[inline(always)]
	pub fn divpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide packed double-precision floating-point values in `xmm1` by packed double-precision floating-point values `xmm2/m128`.
	#[inline(always)]
	pub fn divpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide packed single-precision floating-point values in `xmm1` by packed single-precision floating-point values `xmm2/m128`.
	#[inline(always)]
	pub fn divps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide packed single-precision floating-point values in `xmm1` by packed single-precision floating-point values `xmm2/m128`.
	#[inline(always)]
	pub fn divps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide low double-precision floating-point value in `xmm1` by low double-precision floating-point value in `xmm2/mem64`.
	#[inline(always)]
	pub fn divsd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide low double-precision floating-point value in `xmm1` by low double-precision floating-point value in `xmm2/mem64`.
	#[inline(always)]
	pub fn divsd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide low single-precision floating-point value in `xmm1` by low single-precision floating-point value in `xmm2/m32`.
	#[inline(always)]
	pub fn divss_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide low single-precision floating-point value in `xmm1` by low single-precision floating-point value in `xmm2/m32`.
	#[inline(always)]
	pub fn divss_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Selectively multiply packed double-precision floating-point values from `xmm1` with packed double-precision floating-point values from `xmm2`. add and selectively store the packed double-precision floating-point values to `xmm1`.
	#[inline(always)]
	pub fn dppd_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x41);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Selectively multiply packed double-precision floating-point values from `xmm1` with packed double-precision floating-point values from `xmm2`. add and selectively store the packed double-precision floating-point values to `xmm1`.
	#[inline(always)]
	pub fn dppd_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x41);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Selectively multiply packed single-precision floating-point values from `xmm1` with packed single-precision floating-point values from `xmm2`. add and selectively store the packed single-precision floating-point values or zero values to `xmm1`.
	#[inline(always)]
	pub fn dpps_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x40);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Selectively multiply packed single-precision floating-point values from `xmm1` with packed single-precision floating-point values from `xmm2`. add and selectively store the packed single-precision floating-point values or zero values to `xmm1`.
	#[inline(always)]
	pub fn dpps_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x40);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Set the x87 Floating Point Unit (FPU) tag word to empty.
	#[inline(always)]
	pub fn emms(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x77);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Create a nested stack frame for a procedure.
	#[inline(always)]
	pub fn enter_Immediate8Bit_Immediate16Bit(&mut self, arg0: Immediate8Bit, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xC8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_2(arg0, arg1);

		// No label displacement.
	}

	/// Create a nested stack frame for a procedure.
	#[inline(always)]
	pub fn enter_One_Immediate16Bit(&mut self, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xC8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_2(Immediate8Bit(1), arg1);

		// No label displacement.
	}

	/// Create a stack frame for a procedure.
	#[inline(always)]
	pub fn enter_Zero_Immediate16Bit(&mut self, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xC8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_2(Immediate8Bit(0), arg1);

		// No label displacement.
	}

	/// Extract a single-precision floating-point value from `xmm2` at the source offset specified by `imm8` and store the result to `reg` or `m32`.
	///
	/// The upper 32 bits of `r64` is zeroed if `reg` is `r64`.
	#[inline(always)]
	pub fn extractps_Any32BitMemory_XMMRegister_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x17);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract a single-precision floating-point value from `xmm2` at the source offset specified by `imm8` and store the result to `reg` or `m32`.
	///
	/// The upper 32 bits of `r64` is zeroed if `reg` is `r64`.
	#[inline(always)]
	pub fn extractps_Register32Bit_XMMRegister_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x17);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract a single-precision floating-point value from `xmm2` at the source offset specified by `imm8` and store the result to `reg` or `m32`.
	///
	/// The upper 32 bits of `r64` is zeroed if `reg` is `r64`.
	#[inline(always)]
	pub fn extractps_Register64Bit_XMMRegister_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x17);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Replace `ST(0)` with `(2^(ST(0)) - 1)`.
	#[inline(always)]
	pub fn f2xm1(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Replace `ST` with its absolute value.
	#[inline(always)]
	pub fn fabs(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xE1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `m32fp` to `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fadd_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD8);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `m64fp` to `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fadd_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDC);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `ST(i)` to `ST(0)` and store result in `ST(i)`.
	#[inline(always)]
	pub fn fadd_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDC, 0xC0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `ST(0)` to `ST(i)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fadd_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xD8, 0xC0, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `ST(0)` to `ST(1)` with result stored in in `ST(1)`, and pop the register stack.
	#[inline(always)]
	pub fn faddp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDE, 0xC1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `ST(0)` to `ST(i)` with result stored in in `ST(i)`, and pop the register stack.
	#[inline(always)]
	pub fn faddp_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDE, 0xC0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert binary coded decimal value to floating-point and push onto the Floating Point Unit (FPU) stack.
	#[inline(always)]
	pub fn fbld_X87BinaryCodedDecimal80BitMemory(&mut self, arg0: X87BinaryCodedDecimal80BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDF);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store `ST(0)` in `m80bcd` and pop `ST(0)`.
	#[inline(always)]
	pub fn fbstp_X87BinaryCodedDecimal80BitMemory(&mut self, arg0: X87BinaryCodedDecimal80BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDF);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Complements sign of `ST(0)`.
	#[inline(always)]
	pub fn fchs(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xE0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Clear floating-point exception flags after checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fclex(&mut self)
	{
		// This is not a VEX encoded instruction.

		self.prefix_fwait(0x9B);

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDB, 0xE2);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn fcmovb_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDA, 0xC0, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn fcmovbe_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDA, 0xD0, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn fcmove_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDA, 0xC8, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn fcmovnb_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDB, 0xC0, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn fcmovnbe_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDB, 0xD0, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn fcmovne_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDB, 0xC8, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if not unordered (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn fcmovnu_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDB, 0xD8, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move if unordered (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn fcmovu_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDA, 0xD8, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(1)`.
	#[inline(always)]
	pub fn fcom(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD8, 0xD1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `m32fp`.
	#[inline(always)]
	pub fn fcom_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD8);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `m64fp`.
	#[inline(always)]
	pub fn fcom_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDC);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(i)`.
	#[inline(always)]
	pub fn fcom_X87Register(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0xD8, 0xD0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(i)` and set status flags accordingly.
	#[inline(always)]
	pub fn fcomi_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDB, 0xF0, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(i)`, set status flags accordingly, and pop register stack.
	#[inline(always)]
	pub fn fcomip_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDF, 0xF0, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(1)` and pop register stack.
	#[inline(always)]
	pub fn fcomp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD8, 0xD9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `m32fp` and pop register stack.
	#[inline(always)]
	pub fn fcomp_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD8);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `m64fp` and pop register stack.
	#[inline(always)]
	pub fn fcomp_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDC);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(i)` and pop register stack.
	#[inline(always)]
	pub fn fcomp_X87Register(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0xD8, 0xD8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(1)` and pop register stack twice.
	#[inline(always)]
	pub fn fcompp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDE, 0xD9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Replace `ST(0)` with its cosine.
	#[inline(always)]
	pub fn fcos(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xFF);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement `TOP` field in Floating Point Unit (FPU) status word.
	#[inline(always)]
	pub fn fdecstp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF6);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(0)` by `m32fp` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fdiv_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD8);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(i)`, set status flags accordingly, and pop register stack.
	#[inline(always)]
	pub fn fdiv_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDC);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(i)` by `ST(0)` and store result in `ST(i)`.
	#[inline(always)]
	pub fn fdiv_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDC, 0xF0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(0)` by `ST(i)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fdiv_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xD8, 0xF0, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(1)` by `ST(0)` with result stored in in `ST(1)`, and pop the register stack.
	#[inline(always)]
	pub fn fdivp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDE, 0xF1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(i)` by `ST(0)` with result stored in in `ST(i)`, and pop the register stack.
	#[inline(always)]
	pub fn fdivp_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDE, 0xF0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `m32fp` by `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fdivr_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD8);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `m64fp` by `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fdivr_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDC);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(0)` by `ST(i)` and store result in `ST(i)`.
	#[inline(always)]
	pub fn fdivr_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDC, 0xF8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(i)` by `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fdivr_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xD8, 0xF8, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(0)` by `ST(1)` with result stored in in `ST(1)`, and pop the register stack.
	#[inline(always)]
	pub fn fdivrp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDE, 0xF9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(0)` by `ST(i)` with result stored in in `ST(i)`, and pop the register stack.
	#[inline(always)]
	pub fn fdivrp_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDE, 0xF8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sets tag for `ST(i)` to empty.
	#[inline(always)]
	pub fn ffree_X87Register(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0xDD, 0xC0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `m16int` to `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fiadd_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDE);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add `m32int` to `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fiadd_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDA);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `m16int`.
	#[inline(always)]
	pub fn ficom_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDE);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `m32int`.
	#[inline(always)]
	pub fn ficom_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDA);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `m16int` and pop stack register.
	#[inline(always)]
	pub fn ficomp_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDE);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `m32int` and pop stack register.
	#[inline(always)]
	pub fn ficomp_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDA);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(0)` by `m64int` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fidiv_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDE);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `ST(0)` by `m32int` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fidiv_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDA);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `m16int` by `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fidivr_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDE);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Divide `m32int` by `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fidivr_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDA);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `m16int` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fild_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDF);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `m32int` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fild_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDB);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `m64int` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fild_X87Integer64BitMemory(&mut self, arg0: X87Integer64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDF);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply `ST(0)` by `m16int` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fimul_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDE);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply `ST(0)` by `m32int` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fimul_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDA);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Increment the TOP field in the Floating Point Unit (FPU) status register.
	#[inline(always)]
	pub fn fincstp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF7);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Initialize Floating Point Unit (FPU) after checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn finit(&mut self)
	{
		// This is not a VEX encoded instruction.

		self.prefix_fwait(0x9B);

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDB, 0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store `ST(0)` in `m16int`.
	#[inline(always)]
	pub fn fist_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDF);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store `ST(0)` in `m32int`.
	#[inline(always)]
	pub fn fist_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDB);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store `ST(0)` in `m16int` and pop register stack.
	#[inline(always)]
	pub fn fistp_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDF);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store `ST(0)` in `m32int` and pop register stack.
	#[inline(always)]
	pub fn fistp_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDB);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store `ST(0)` in `m64int` and pop register stack.
	#[inline(always)]
	pub fn fistp_X87Integer64BitMemory(&mut self, arg0: X87Integer64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDF);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store `ST(0)` in `m16int` with truncation.
	#[inline(always)]
	pub fn fisttp_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDF);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store `ST(0)` in `m32int` with truncation.
	#[inline(always)]
	pub fn fisttp_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDB);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store `ST(0)` in `m64int` with truncation.
	#[inline(always)]
	pub fn fisttp_X87Integer64BitMemory(&mut self, arg0: X87Integer64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDD);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `m16int` from `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fisub_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDE);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `m32int` from `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fisub_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDA);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(0)` from `m16int` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fisubr_X87Integer16BitMemory(&mut self, arg0: X87Integer16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDE);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(0)` from `m32int` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fisubr_X87Integer32BitMemory(&mut self, arg0: X87Integer32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDA);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `m32fp` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fld_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD9);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `m64fp` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fld_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDD);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `m80fp` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fld_X87Float80BitMemory(&mut self, arg0: X87Float80BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDB);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `ST(i)` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fld_X87Register(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0xD9, 0xC0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `+1.0` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fld1(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xE8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load Floating Point Unit (FPU) control word from `m2byte`.
	#[inline(always)]
	pub fn fldcw_Contiguous2ByteMemory(&mut self, arg0: Contiguous2ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD9);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load Floating Point Unit (FPU) environment from `m14byte` or `m28byte`.
	#[inline(always)]
	pub fn fldenv_Contiguous28ByteMemory(&mut self, arg0: Contiguous28ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD9);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `log2e` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fldl2e(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xEA);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `log210` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fldl2t(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xE9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `log102` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fldlg2(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xEC);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `loge2` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fldln2(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xED);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `pi` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fldpi(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xEB);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Push `+0.0` onto the Floating Point Unit (FPU) register stack.
	#[inline(always)]
	pub fn fldz(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xEE);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply `ST(0)` by `m32fp` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fmul_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD8);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply `ST(0)` by `m64fp` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fmul_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDC);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply `ST(i)` by `ST(0)` and store result in `ST(i)`.
	#[inline(always)]
	pub fn fmul_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDC, 0xC8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply `ST(0)` by `ST(i)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fmul_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xD8, 0xC8, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply `ST(1)` by `ST(0)` with result stored in in `ST(1)`, and pop the register stack.
	#[inline(always)]
	pub fn fmulp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDE, 0xC9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply `ST(i)` by `ST(0)` with result stored in in `ST(i)`, and pop the register stack.
	#[inline(always)]
	pub fn fmulp_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDE, 0xC8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Clear floating-point exception flags without checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fnclex(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDB, 0xE2);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Initialize Floating Point Unit (FPU) without checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fninit(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDB, 0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// No operation is performed.
	#[inline(always)]
	pub fn fnop(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xD0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) environment to `m94byte` or `m108byte` without checking for pending unmasked floating-point exceptions.
	///
	/// Then re-initialize the Floating Point Unit (FPU).
	#[inline(always)]
	pub fn fnsave_Contiguous108ByteMemory(&mut self, arg0: Contiguous108ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDD);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) control word to `m2byte` without checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fnstcw_Contiguous2ByteMemory(&mut self, arg0: Contiguous2ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD9);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) environment to `m14byte` or `m28byte` without checking for pending unmasked floating-point exceptions.
	///
	/// Then mask all floating-point exceptions.
	#[inline(always)]
	pub fn fnstenv_Contiguous28ByteMemory(&mut self, arg0: Contiguous28ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD9);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) status word in `AX` register without checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fnstsw_AX(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDF, 0xE0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) status word at `m2byte` without checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fnstsw_Contiguous2ByteMemory(&mut self, arg0: Contiguous2ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDD);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Replace `ST(1)` with `arctan(ST(1)/ST(0))` and pop the register stack.
	#[inline(always)]
	pub fn fpatan(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Replace `ST(0)` with the remainder obtained from dividing `ST(0)` by `ST(1)`.
	#[inline(always)]
	pub fn fprem(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Replace `ST(0)` with the IEEE remainder obtained from dividing `ST(0)` by `ST(1)`.
	#[inline(always)]
	pub fn fprem1(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Replace `ST(0)` with its tangent and push 1 onto the Floating Point Unit (FPU) stack.
	#[inline(always)]
	pub fn fptan(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF2);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Round `ST(0)` to an integer.
	#[inline(always)]
	pub fn frndint(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xFC);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load Floating Point Unit (FPU) state from `m94byte` or `m108byte`.
	#[inline(always)]
	pub fn frstor_Contiguous108ByteMemory(&mut self, arg0: Contiguous108ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDD);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) state to `m94byte` or `m108byte` after checking for pending unmasked floating-point exceptions.
	///
	/// Then re-initialize the Floating Point Unit (FPU).
	#[inline(always)]
	pub fn fsave_Contiguous108ByteMemory(&mut self, arg0: Contiguous108ByteMemory)
	{
		// This is not a VEX encoded instruction.

		self.prefix_fwait(0x9B);

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDD);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Scale `ST(0)` by `ST(1)`.
	#[inline(always)]
	pub fn fscale(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xFD);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Replace `ST(0)` with its sine.
	#[inline(always)]
	pub fn fsin(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xFE);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the sine and cosine of `ST(0)`; replace `ST(0)` with the sine, and push the cosine onto the register stack.
	#[inline(always)]
	pub fn fsincos(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xFB);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Computes square root of `ST(0)` and stores the result in `ST(0)`.
	#[inline(always)]
	pub fn fsqrt(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xFA);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Copy `ST(0)` to `m32fp`.
	#[inline(always)]
	pub fn fst_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD9);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Copy `ST(0)` to `m64fp`.
	#[inline(always)]
	pub fn fst_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDD);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Copy `ST(0)` to `ST(i)`.
	#[inline(always)]
	pub fn fst_X87Register(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0xDD, 0xD0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) control word to `m2byte` after checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fstcw_Contiguous2ByteMemory(&mut self, arg0: Contiguous2ByteMemory)
	{
		// This is not a VEX encoded instruction.

		self.prefix_fwait(0x9B);

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD9);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) environment to `m14byte` or `m28byte` after checking for pending unmasked floating-point exceptions.
	///
	/// Then mask all floating-point exceptions.
	#[inline(always)]
	pub fn fstenv_Contiguous28ByteMemory(&mut self, arg0: Contiguous28ByteMemory)
	{
		// This is not a VEX encoded instruction.

		self.prefix_fwait(0x9B);

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD9);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Copy `ST(0)` to `m32fp` and pop register stack.
	#[inline(always)]
	pub fn fstp_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD9);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Copy `ST(0)` to `m64fp` and pop register stack.
	#[inline(always)]
	pub fn fstp_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDD);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Copy `ST(0)` to `m80fp` and pop register stack.
	#[inline(always)]
	pub fn fstp_X87Float80BitMemory(&mut self, arg0: X87Float80BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDB);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Copy `ST(0)` to `ST(i)` and pop register stack.
	#[inline(always)]
	pub fn fstp_X87Register(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0xDD, 0xD8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) status word in `AX` register after checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fstsw_AX(&mut self)
	{
		// This is not a VEX encoded instruction.

		self.prefix_fwait(0x9B);

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDF, 0xE0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store Floating Point Unit (FPU) status word at `m2byte` after checking for pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fstsw_Contiguous2ByteMemory(&mut self, arg0: Contiguous2ByteMemory)
	{
		// This is not a VEX encoded instruction.

		self.prefix_fwait(0x9B);

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDD);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `m32fp` from `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fsub_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD8);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `m64fp` from `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fsub_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDC);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(0)` from `ST(i)` and store result in `ST(i)`.
	#[inline(always)]
	pub fn fsub_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDC, 0xE0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(i)` from `ST(0)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fsub_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xD8, 0xE0, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(0)` from `ST(1)` with result stored in in `ST(1)`, and pop register stack.
	#[inline(always)]
	pub fn fsubp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDE, 0xE1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(0)` from `ST(i)` with result stored in in `ST(i)`, and pop register stack.
	#[inline(always)]
	pub fn fsubp_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDE, 0xE0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(0)` from `m32fp` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fsubr_X87Float32BitMemory(&mut self, arg0: X87Float32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xD8);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(0)` from `m64fp` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fsubr_X87Float64BitMemory(&mut self, arg0: X87Float64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xDC);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(i)` from `ST(0)` and store result in `ST(i)`.
	#[inline(always)]
	pub fn fsubr_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDC, 0xE8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(0)` from `ST(i)` and store result in `ST(0)`.
	#[inline(always)]
	pub fn fsubr_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xD8, 0xE8, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(1)` from `ST(0)` with result stored in in `ST(1)`, and pop register stack.
	#[inline(always)]
	pub fn fsubrp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDE, 0xE9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract `ST(i)` from `ST(0)` with result stored in in `ST(i)`, and pop register stack.
	#[inline(always)]
	pub fn fsubrp_X87Register_ST0(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDE, 0xE8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `0.0`.
	#[inline(always)]
	pub fn ftst(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xE4);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(1)`.
	#[inline(always)]
	pub fn fucom(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDD, 0xE1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(i)`.
	#[inline(always)]
	pub fn fucom_X87Register(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0xDD, 0xE0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(i)`, check for ordered values, and set status flags accordingly.
	#[inline(always)]
	pub fn fucomi_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDB, 0xE8, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(i)`, check for ordered values, set status flags accordingly, and pop register stack.
	#[inline(always)]
	pub fn fucomip_ST0_X87Register(&mut self, arg1: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0xDF, 0xE8, arg1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(1)` and pop register stack.
	#[inline(always)]
	pub fn fucomp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDD, 0xE9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(i)` and pop register stack.
	#[inline(always)]
	pub fn fucomp_X87Register(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0xDD, 0xE8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare `ST(0)` with `ST(1)` and pop register stack twice.
	#[inline(always)]
	pub fn fucompp(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xDA, 0xE9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Check pending unmasked floating-point exceptions.
	#[inline(always)]
	pub fn fwait(&mut self)
	{
		// This is not a VEX encoded instruction.

		self.prefix_fwait(0x9B);

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		// No Opcode Bytes	// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Classify value or number in `ST(0)`.
	#[inline(always)]
	pub fn fxam(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xE5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Exchange the contents of `ST(0)` and `ST(1)`.
	#[inline(always)]
	pub fn fxch(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xC9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Exchange the contents of `ST(0)` and `ST(i)`.
	#[inline(always)]
	pub fn fxch_X87Register(&mut self, arg0: X87Register)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_3(0xD9, 0xC8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Restore the x87 Floating Point Unit (FPU), `MMX`, `XMM`, and `MXCSR` register state from `m512byte`.
	#[inline(always)]
	pub fn fxrstor_Contiguous512ByteMemory(&mut self, arg0: Contiguous512ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xAE);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Restore the x87 Floating Point Unit (FPU), `MMX`, `XMM`, and `MXCSR` register state from `m512byte`.
	#[inline(always)]
	pub fn fxrstor64_Contiguous512ByteMemory(&mut self, arg0: Contiguous512ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xAE);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Save the x87 Floating Point Unit (FPU), `MMX`, `XMM`, and `MXCSR` register state to `m512byte`.
	#[inline(always)]
	pub fn fxsave_Contiguous512ByteMemory(&mut self, arg0: Contiguous512ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xAE);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Save the x87 Floating Point Unit (FPU), `MMX`, `XMM`, and `MXCSR` register state to `m512byte`.
	#[inline(always)]
	pub fn fxsave64_Contiguous512ByteMemory(&mut self, arg0: Contiguous512ByteMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xAE);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Separate value in `ST(0)` into exponent and significand, store exponent in `ST(0)`, and push the significand onto the register stack.
	#[inline(always)]
	pub fn fxtract(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF4);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Replace `ST(1)` with (`ST(1) * log2ST(0))` and pop the register stack.
	#[inline(always)]
	pub fn fyl2x(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Replace `ST(1)` with `ST(1) * log2(ST(0) + 1.0)` and pop the register stack.
	#[inline(always)]
	pub fn fyl2xp1(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0xD9, 0xF9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Horizontal add packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn haddpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Horizontal add packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn haddpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Horizontal add packed single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn haddps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Horizontal add packed single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn haddps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Horizontal subtract packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn hsubpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Horizontal subtract packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn hsubpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Horizontal subtract packed single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn hsubps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Horizontal subtract packed single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn hsubps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Signed divide `DX:AX` by `r/m16`, with result stored in `AX` (Quotient) and `DX` (Remainder).
	#[inline(always)]
	pub fn idiv_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Signed divide `EDX:EAX` by `r/m32`, with result stored in `EAX` (Quotient) and `EDX` (Remainder).
	#[inline(always)]
	pub fn idiv_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Signed divide `RDX:RAX` by `r/m64`, with result stored in `RAX` (Quotient) and `RDX` (Remainder).
	#[inline(always)]
	pub fn idiv_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Signed divide `AX` by `r/m8`, with result stored in `AL` (Quotient) and `AH` (Remainder).
	#[inline(always)]
	pub fn idiv_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Signed divide `DX:AX` by `r/m16`, with result stored in `AX` (Quotient) and `DX` (Remainder).
	#[inline(always)]
	pub fn idiv_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Signed divide `EDX:EAX` by `r/m32`, with result stored in `EAX` (Quotient) and `EDX` (Remainder).
	#[inline(always)]
	pub fn idiv_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Signed divide `RDX:RAX` by `r/m64`, with result stored in `RAX` (Quotient) and `RDX` (Remainder).
	#[inline(always)]
	pub fn idiv_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Signed divide `AX` by `r/m8`, with result stored in `AL` (Quotient) and `AH` (Remainder).
	#[inline(always)]
	pub fn idiv_Register8Bit(&mut self, arg0: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Signed divide `AX` by `r/m8`, with result stored in `AL` (Quotient) and `AH` (Remainder).
	#[inline(always)]
	pub fn idiv_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `DX:AX` = `AX` * `r/m16`.
	#[inline(always)]
	pub fn imul_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `EDX:EAX` = `EAX` * `r/m32`.
	#[inline(always)]
	pub fn imul_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `RDX:RAX` = `RAX` * `r/m64`.
	#[inline(always)]
	pub fn imul_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `AX` = `AL` * `r/m8`.
	#[inline(always)]
	pub fn imul_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `DX:AX` = `AX` * `r/m16`.
	#[inline(always)]
	pub fn imul_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// word register = word register * `r/m16`.
	#[inline(always)]
	pub fn imul_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xAF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// word register = `r/m16` * immediate word.
	#[inline(always)]
	pub fn imul_Register16Bit_Any16BitMemory_Immediate16Bit(&mut self, arg0: Register16Bit, arg1: Any16BitMemory, arg2: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x69);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// word register = `r/m16` * sign-extended `imm8`.
	#[inline(always)]
	pub fn imul_Register16Bit_Any16BitMemory_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Any16BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x6B);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// word register = word register * `r/m16`.
	#[inline(always)]
	pub fn imul_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xAF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// word register = `r/m16` * immediate word.
	#[inline(always)]
	pub fn imul_Register16Bit_Register16Bit_Immediate16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit, arg2: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x69);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// word register = `r/m16` * sign-extended `imm8`.
	#[inline(always)]
	pub fn imul_Register16Bit_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x6B);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// `EDX:EAX` = `EAX` * `r/m32`.
	#[inline(always)]
	pub fn imul_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// doubleword register = doubleword register * `r/m32`.
	#[inline(always)]
	pub fn imul_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xAF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// doubleword register = `r/m32` * `imm32`.
	#[inline(always)]
	pub fn imul_Register32Bit_Any32BitMemory_Immediate32Bit(&mut self, arg0: Register32Bit, arg1: Any32BitMemory, arg2: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x69);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// doubleword register = `r/m32` * sign-extended `imm8`.
	#[inline(always)]
	pub fn imul_Register32Bit_Any32BitMemory_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Any32BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x6B);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// doubleword register = doubleword register * `r/m32`.
	#[inline(always)]
	pub fn imul_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xAF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// doubleword register = `r/m32` * `imm32`.
	#[inline(always)]
	pub fn imul_Register32Bit_Register32Bit_Immediate32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x69);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// doubleword register = `r/m32` * sign-extended `imm8`.
	#[inline(always)]
	pub fn imul_Register32Bit_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x6B);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// `RDX:RAX` = `RAX` * `r/m64`.
	#[inline(always)]
	pub fn imul_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Quadword register = Quadword register * `r/m64`.
	#[inline(always)]
	pub fn imul_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xAF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Quadword register = `r/m64` * `imm32`.
	#[inline(always)]
	pub fn imul_Register64Bit_Any64BitMemory_Immediate32Bit(&mut self, arg0: Register64Bit, arg1: Any64BitMemory, arg2: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x69);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Quadword register = `r/m64` * sign-extended `imm8`.
	#[inline(always)]
	pub fn imul_Register64Bit_Any64BitMemory_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Any64BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x6B);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Quadword register = Quadword register * `r/m64`.
	#[inline(always)]
	pub fn imul_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xAF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Quadword register = `r/m64` * `imm32`.
	#[inline(always)]
	pub fn imul_Register64Bit_Register64Bit_Immediate32Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x69);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Quadword register = `r/m64` * sign-extended `imm8`.
	#[inline(always)]
	pub fn imul_Register64Bit_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x6B);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// `AX` = `AL` * `r/m8`.
	#[inline(always)]
	pub fn imul_Register8Bit(&mut self, arg0: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `AX` = `AL` * `r/m8`.
	#[inline(always)]
	pub fn imul_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RBP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Input byte from I/O port in `DX` into `AL`.
	#[inline(always)]
	pub fn in_AL_DX(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xEC);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Input byte from `imm8` I/O port address into `AL`.
	#[inline(always)]
	pub fn in_AL_Immediate8Bit(&mut self, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE4);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Input word from I/O port in `DX` into `AX`.
	#[inline(always)]
	pub fn in_AX_DX(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xED);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Input word from `imm8` I/O port address into `AX`.
	#[inline(always)]
	pub fn in_AX_Immediate8Bit(&mut self, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Input doubleword from I/O port in `DX` into `EAX`.
	#[inline(always)]
	pub fn in_EAX_DX(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xED);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Input dword from `imm8` I/O port address into `EAX`.
	#[inline(always)]
	pub fn in_EAX_Immediate8Bit(&mut self, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Increment `r/m16` by 1.
	#[inline(always)]
	pub fn inc_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Increment `r/m32` by 1.
	#[inline(always)]
	pub fn inc_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Increment `r/m64` by 1.
	#[inline(always)]
	pub fn inc_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Increment `r/m8` by 1.
	#[inline(always)]
	pub fn inc_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFE);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Increment `r/m16` by 1.
	#[inline(always)]
	pub fn inc_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Increment `r/m32` by 1.
	#[inline(always)]
	pub fn inc_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Increment `r/m64` by 1.
	#[inline(always)]
	pub fn inc_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Increment `r/m8` by 1.
	#[inline(always)]
	pub fn inc_Register8Bit(&mut self, arg0: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFE);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Increment `r/m8` by 1.
	#[inline(always)]
	pub fn inc_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFE);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Input word from I/O port specified in `DX` into memory location specified in `ES:(E)DI` or `RDI`.
	#[inline(always)]
	pub fn ins_Any16BitMemory_DX(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Input doubleword from I/O port specified in `DX` into memory location specified in `ES:(E)DI` or `RDI`.
	#[inline(always)]
	pub fn ins_Any32BitMemory_DX(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Input byte from I/O port specified in `DX` into memory location specified in `ES:(E)DI` or `RDI`.
	#[inline(always)]
	pub fn ins_Any8BitMemory_DX(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Input byte from I/O port specified in `DX` into memory location specified with `ES:(E)DI` or `RDI`.
	#[inline(always)]
	pub fn insb(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Input doubleword from I/O port specified in `DX` into memory location specified in `ES:(E)DI` or `RDI`.
	#[inline(always)]
	pub fn insd(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Insert a single-precision floating-point value selected by `imm8` from `xmm2/m32` into `xmm1` at the specified destination element specified by `imm8` and zero out destination elements in `xmm1` as indicated in `imm8`.
	#[inline(always)]
	pub fn insertps_XMMRegister_Any32BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any32BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x21);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Insert a single-precision floating-point value selected by `imm8` from `xmm2/m32` into `xmm1` at the specified destination element specified by `imm8` and zero out destination elements in `xmm1` as indicated in `imm8`.
	#[inline(always)]
	pub fn insertps_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x21);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Input word from I/O port specified in `DX` into memory location specified in `ES:(E)DI` or `RDI`.
	#[inline(always)]
	pub fn insw(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Interrupt vector number specified by `imm8`.
	#[inline(always)]
	pub fn int_Immediate8Bit(&mut self, arg0: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xCD);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Interrupt 3-trap to debugger.
	#[inline(always)]
	pub fn int_Three(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xCC);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Invalidates entries in the TLBs and paging-structure caches based on invalidation type in `r64` and descriptor in `m128`.
	#[inline(always)]
	pub fn invpcid_Register64Bit_Any128BitMemory(&mut self, arg0: Register64Bit, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x82);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Interrupt return (16-bit operand size).
	#[inline(always)]
	pub fn iret(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xCF);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Interrupt return (32-bit operand size).
	#[inline(always)]
	pub fn iretd(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xCF);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Interrupt return (64-bit operand size).
	#[inline(always)]
	pub fn iretq(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xCF);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Jump short if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn ja_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x77);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn ja_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x87);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn ja_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x77);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn ja_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x87);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn ja_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x87);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn ja_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x87);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn ja_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x77);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if above (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn ja_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x77);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jae_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jae_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jae_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jae_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jae_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jae_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jae_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if above or equal (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jae_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jb_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jb_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jb_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jb_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jb_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jb_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jb_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if below (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jb_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jbe_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x76);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jbe_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x86);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jbe_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x76);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jbe_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x86);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jbe_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x86);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jbe_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x86);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jbe_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x76);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if below or equal (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jbe_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x76);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jc_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jc_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jc_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jc_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jc_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jc_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jc_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if carry (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jc_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn je_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x74);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if 0 (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn je_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x84);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn je_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x74);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if 0 (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn je_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x84);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if 0 (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn je_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x84);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if 0 (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn je_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x84);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn je_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x74);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if equal (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn je_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x74);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if `ECX` register is 0.
	#[inline(always)]
	pub fn jecxz_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump short if `ECX` register is 0.
	#[inline(always)]
	pub fn jecxz_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump short if `ECX` register is 0.
	#[inline(always)]
	pub fn jecxz_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if `ECX` register is 0.
	#[inline(always)]
	pub fn jecxz_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jg_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jg_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jg_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jg_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jg_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jg_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jg_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if greater (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jg_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jge_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jge_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jge_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jge_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jge_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jge_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jge_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if greater or equal (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jge_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jl_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jl_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jl_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jl_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jl_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jl_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jl_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if less (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jl_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jle_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jle_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jle_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jle_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jle_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jle_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jle_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if less or equal (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jle_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump far, absolute indirect, address given in `m16:16`.
	#[inline(always)]
	pub fn jmp_FarPointer16BitTo16Bit(&mut self, arg0: FarPointer16BitTo16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xFF);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Jump far, absolute indirect, address given in m16:32.
	#[inline(always)]
	pub fn jmp_FarPointer16BitTo32Bit(&mut self, arg0: FarPointer16BitTo32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xFF);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Jump far, absolute indirect, address given in m16:64.
	#[inline(always)]
	pub fn jmp_FarPointer16BitTo64Bit(&mut self, arg0: FarPointer16BitTo64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xFF);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Jump short, `RIP` = `RIP` + 8-bit displacement sign extended to 64-bits.
	#[inline(always)]
	pub fn jmp_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xEB);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near, relative, `RIP` = `RIP` + 32-bit displacement sign extended to 64-bits.
	#[inline(always)]
	pub fn jmp_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near, absolute indirect, `RIP` = 64-Bit offset from register or memory.
	#[inline(always)]
	pub fn jmp_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Jump near, absolute indirect, `RIP` = 64-Bit offset from register or memory.
	#[inline(always)]
	pub fn jmp_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xFF);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Jump near, relative, `RIP` = `RIP` + 32-bit displacement sign extended to 64-bits.
	#[inline(always)]
	pub fn jmp_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short, `RIP` = `RIP` + 8-bit displacement sign extended to 64-bits.
	#[inline(always)]
	pub fn jmp_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xEB);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jna_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x76);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jna_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x86);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jna_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x76);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jna_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x86);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jna_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x86);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jna_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x86);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jna_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x76);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not above (Carry Flag (CF) is 1 or Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jna_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x76);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jnae_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jnae_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jnae_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jnae_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jnae_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jnae_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x82);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jnae_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not above or equal (Carry Flag (CF) is 1).
	#[inline(always)]
	pub fn jnae_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x72);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnb_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnb_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnb_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnb_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnb_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnb_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnb_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not below (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnb_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnbe_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x77);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnbe_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x87);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnbe_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x77);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnbe_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x87);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnbe_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x87);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnbe_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x87);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnbe_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x77);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not below or equal (Carry Flag (CF) is 0 and Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnbe_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x77);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnc_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnc_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnc_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnc_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnc_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnc_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x83);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnc_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not carry (Carry Flag (CF) is 0).
	#[inline(always)]
	pub fn jnc_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x73);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jne_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x75);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jne_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x85);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jne_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x75);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jne_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x85);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jne_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x85);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jne_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x85);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jne_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x75);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not equal (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jne_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x75);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jng_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jng_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jng_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jng_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jng_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jng_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jng_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not greater (Zero Flag (ZF) is 1 or Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jng_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnge_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnge_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnge_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnge_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnge_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnge_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnge_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not greater or equal (Sign Flag (SF) != Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnge_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnl_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnl_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnl_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnl_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnl_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnl_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnl_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not less (Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnl_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnle_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnle_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnle_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnle_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnle_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnle_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnle_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not less or equal (Zero Flag (ZF) is 0 and Sign Flag (SF) == Overflow Flag (OF)).
	#[inline(always)]
	pub fn jnle_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn jno_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x71);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn jno_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x81);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn jno_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x71);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn jno_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x81);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn jno_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x81);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn jno_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x81);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn jno_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x71);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not overflow (Overflow Flag (OF) is 0).
	#[inline(always)]
	pub fn jno_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x71);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jnp_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jnp_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jnp_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jnp_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jnp_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jnp_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jnp_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not parity (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jnp_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn jns_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x79);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn jns_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x89);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn jns_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x79);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn jns_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x89);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn jns_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x89);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn jns_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x89);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn jns_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x79);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not sign (Sign Flag (SF) is 0).
	#[inline(always)]
	pub fn jns_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x79);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnz_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x75);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnz_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x85);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnz_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x75);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnz_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x85);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnz_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x85);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnz_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x85);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnz_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x75);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if not zero (Zero Flag (ZF) is 0).
	#[inline(always)]
	pub fn jnz_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x75);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn jo_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x70);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn jo_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x80);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn jo_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x70);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn jo_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x80);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn jo_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x80);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn jo_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x80);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn jo_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x70);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if overflow (Overflow Flag (OF) is 1).
	#[inline(always)]
	pub fn jo_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x70);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jp_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jp_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jp_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jp_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jp_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jp_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jp_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if parity (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jp_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jpe_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jpe_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jpe_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jpe_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jpe_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jpe_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jpe_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if parity even (Parity Flag (PF) is 1).
	#[inline(always)]
	pub fn jpe_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7A);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jpo_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jpo_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jpo_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jpo_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jpo_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jpo_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x8B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jpo_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if parity odd (Parity Flag (PF) is 0).
	#[inline(always)]
	pub fn jpo_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x7B);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if `RCX` register is 0.
	#[inline(always)]
	pub fn jrcxz_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump short if `RCX` register is 0.
	#[inline(always)]
	pub fn jrcxz_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump short if `RCX` register is 0.
	#[inline(always)]
	pub fn jrcxz_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if `RCX` register is 0.
	#[inline(always)]
	pub fn jrcxz_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn js_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x78);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn js_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x88);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn js_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x78);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn js_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x88);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn js_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x88);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn js_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x88);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn js_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x78);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if sign (Sign Flag (SF) is 1).
	#[inline(always)]
	pub fn js_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x78);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jz_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x74);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if 0 (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jz_Label_REX_W(&mut self, arg0: Label)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x84);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump short if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jz_Label_BranchHint(&mut self, arg0: Label, arg1: BranchHint) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x74);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Jump near if 0 (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jz_Label_BranchHint_REX_W(&mut self, arg0: Label, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x84);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_32bit(arg0);
	}

	/// Jump near if 0 (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jz_RelativeAddress32Bit(&mut self, arg0: RelativeAddress32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x84);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump near if 0 (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jz_RelativeAddress32Bit_BranchHint(&mut self, arg0: RelativeAddress32Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0x84);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jz_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x74);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Jump short if zero (Zero Flag (ZF) is 1).
	#[inline(always)]
	pub fn jz_RelativeAddress8Bit_BranchHint(&mut self, arg0: RelativeAddress8Bit, arg1: BranchHint)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x74);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Load: `AH = EFLAGS(SF:ZF:0:AF:0:PF:1:CF)`.
	#[inline(always)]
	pub fn lahf(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x9F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r16` = access rights referenced by `r16/m16`.
	#[inline(always)]
	pub fn lar_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r16` = access rights referenced by `r16/m16`.
	#[inline(always)]
	pub fn lar_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `reg` = access rights referenced by `r32/m16`.
	#[inline(always)]
	pub fn lar_Register32Bit_Any16BitMemory(&mut self, arg0: Register32Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `reg` = access rights referenced by `r32/m16`.
	#[inline(always)]
	pub fn lar_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `reg` = access rights referenced by `r32/m16`.
	#[inline(always)]
	pub fn lar_Register64Bit_Any16BitMemory(&mut self, arg0: Register64Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `reg` = access rights referenced by `r32/m16`.
	#[inline(always)]
	pub fn lar_Register64Bit_Register32Bit(&mut self, arg0: Register64Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load unaligned data from mem and return double quadword in `xmm1`.
	#[inline(always)]
	pub fn lddqu_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `MXCSR` register from `m32`.
	#[inline(always)]
	pub fn ldmxcsr_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0xAE);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store effective address for `m16` in register `r16`.
	#[inline(always)]
	pub fn lea_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store effective address for `m32` in register `r16`.
	#[inline(always)]
	pub fn lea_Register16Bit_Any32BitMemory(&mut self, arg0: Register16Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store effective address for `m64` in register `r16`.
	#[inline(always)]
	pub fn lea_Register16Bit_Any64BitMemory(&mut self, arg0: Register16Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store effective address for `m16` in register `r32`.
	#[inline(always)]
	pub fn lea_Register32Bit_Any16BitMemory(&mut self, arg0: Register32Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store effective address for `m32` in register `r32`.
	#[inline(always)]
	pub fn lea_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store effective address for `m64` in register `r32`.
	#[inline(always)]
	pub fn lea_Register32Bit_Any64BitMemory(&mut self, arg0: Register32Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store effective address for `m16` in register `r64`.
	#[inline(always)]
	pub fn lea_Register64Bit_Any16BitMemory(&mut self, arg0: Register64Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x8D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store effective address for `m32` in register `r64`.
	#[inline(always)]
	pub fn lea_Register64Bit_Any32BitMemory(&mut self, arg0: Register64Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x8D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Store effective address for `m64` in register `r64`.
	#[inline(always)]
	pub fn lea_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x8D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Set `RSP` to `RBP`, then pop `RBP`.
	#[inline(always)]
	pub fn leave(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xC9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Set `SP` to `BP`, then pop `BP`.
	#[inline(always)]
	pub fn leave_Prefix66(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xC9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Serializes load operations.
	#[inline(always)]
	pub fn lfence(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0x0F, 0xAE, 0xE8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `FS:r16` with far pointer from memory.
	#[inline(always)]
	pub fn lfs_Register16Bit_FarPointer16BitTo16Bit(&mut self, arg0: Register16Bit, arg1: FarPointer16BitTo16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `FS:r32` with far pointer from memory.
	#[inline(always)]
	pub fn lfs_Register32Bit_FarPointer16BitTo32Bit(&mut self, arg0: Register32Bit, arg1: FarPointer16BitTo32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `FS:r64` with far pointer from memory.
	#[inline(always)]
	pub fn lfs_Register64Bit_FarPointer16BitTo64Bit(&mut self, arg0: Register64Bit, arg1: FarPointer16BitTo64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xB4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `GS:r16` with far pointer from memory.
	#[inline(always)]
	pub fn lgs_Register16Bit_FarPointer16BitTo16Bit(&mut self, arg0: Register16Bit, arg1: FarPointer16BitTo16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `GS:r32` with far pointer from memory.
	#[inline(always)]
	pub fn lgs_Register32Bit_FarPointer16BitTo32Bit(&mut self, arg0: Register32Bit, arg1: FarPointer16BitTo32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `GS:r64` with far pointer from memory.
	#[inline(always)]
	pub fn lgs_Register64Bit_FarPointer16BitTo64Bit(&mut self, arg0: Register64Bit, arg1: FarPointer16BitTo64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xB5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Asserts `LOCK#` signal for duration of the accompanying instruction.
	#[inline(always)]
	pub fn lock(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xF0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode load word at address `DS:(E)SI` into `AX`.
	///
	/// For 64-bit mode load word at address `(R)SI` into `AX`.
	#[inline(always)]
	pub fn lods_Any16BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xAD);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode load dword at address `DS:(E)SI` into `EAX`.
	///
	/// For 64-bit mode load dword at address `(R)SI` into `EAX`.
	#[inline(always)]
	pub fn lods_Any32BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xAD);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load qword at address `(R)SI` into `RAX`.
	#[inline(always)]
	pub fn lods_Any64BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xAD);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode load byte at address `DS:(E)SI` into `AL`.
	///
	/// For 64-bit mode load byte at address `(R)SI` into `AL`.
	#[inline(always)]
	pub fn lods_Any8BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xAC);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode load byte at address `DS:(E)SI` into `AL`.
	///
	/// For 64-bit mode load byte at address `(R)SI` into `AL`.
	#[inline(always)]
	pub fn lodsb(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xAC);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode load dword at address `DS:(E)SI` into `EAX`.
	///
	/// For 64-bit mode load dword at address `(R)SI` into `EAX`.
	#[inline(always)]
	pub fn lodsd(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xAD);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load qword at address `(R)SI` into `RAX`.
	#[inline(always)]
	pub fn lodsq(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xAD);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode load word at address `DS:(E)SI` into `AX`.
	///
	/// For 64-bit mode load word at address `(R)SI` into `AX`.
	#[inline(always)]
	pub fn lodsw(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xAD);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Decrement count; jump short if count != 0.
	#[inline(always)]
	pub fn loop_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE2);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Decrement count; jump short if count != 0.
	#[inline(always)]
	pub fn loop_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE2);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Decrement count; jump short if count != 0 and Zero Flag (ZF) is 1.
	#[inline(always)]
	pub fn loope_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Decrement count; jump short if count != 0 and Zero Flag (ZF) is 1.
	#[inline(always)]
	pub fn loope_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Decrement count; jump short if count != 0 and Zero Flag (ZF) is 0.
	#[inline(always)]
	pub fn loopne_Label(&mut self, arg0: Label) -> ShortJmpResult
	{
		self.bookmark();
		
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		self.displacement_label_8bit(arg0)
	}

	/// Decrement count; jump short if count != 0 and Zero Flag (ZF) is 0.
	#[inline(always)]
	pub fn loopne_RelativeAddress8Bit(&mut self, arg0: RelativeAddress8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Load: `r16` = segment limit, selector `r16/m16`.
	#[inline(always)]
	pub fn lsl_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load: `r16` = segment limit, selector `r16/m16`.
	#[inline(always)]
	pub fn lsl_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load: `r32` = segment limit, selector `r32/m16`.
	#[inline(always)]
	pub fn lsl_Register32Bit_Any16BitMemory(&mut self, arg0: Register32Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load: `r32` = segment limit, selector `r32/m16`.
	#[inline(always)]
	pub fn lsl_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load: `r64` = segment limit, selector `r32/m16`.
	#[inline(always)]
	pub fn lsl_Register64Bit_Any16BitMemory(&mut self, arg0: Register64Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load: `r64` = segment limit, selector `r32/m16`.
	#[inline(always)]
	pub fn lsl_Register64Bit_Register32Bit(&mut self, arg0: Register64Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `SS:r16` with far pointer from memory.
	#[inline(always)]
	pub fn lss_Register16Bit_FarPointer16BitTo16Bit(&mut self, arg0: Register16Bit, arg1: FarPointer16BitTo16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `SS:r32` with far pointer from memory.
	#[inline(always)]
	pub fn lss_Register32Bit_FarPointer16BitTo32Bit(&mut self, arg0: Register32Bit, arg1: FarPointer16BitTo32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Load `SS:r64` with far pointer from memory.
	#[inline(always)]
	pub fn lss_Register64Bit_FarPointer16BitTo64Bit(&mut self, arg0: Register64Bit, arg1: FarPointer16BitTo64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xB2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Count the number of leading zero bits in `r/m16` and return result in `r16`.
	#[inline(always)]
	pub fn lzcnt_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Count the number of leading zero bits in `r/m16` and return result in `r16`.
	#[inline(always)]
	pub fn lzcnt_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Count the number of leading zero bits in `r/m32` and return result in `r32`.
	#[inline(always)]
	pub fn lzcnt_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Count the number of leading zero bits in `r/m32` and return result in `r32`.
	#[inline(always)]
	pub fn lzcnt_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Count the number of leading zero bits in `r/m64` and return result in `r64`.
	#[inline(always)]
	pub fn lzcnt_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Count the number of leading zero bits in `r/m64` and return result in `r64`.
	#[inline(always)]
	pub fn lzcnt_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Selectively write bytes from `xmm1` to memory location using the byte mask in `xmm2`.
	///
	/// The default memory location is specified by `DS:DI`, `EDI` or `RDI`.
	#[inline(always)]
	pub fn maskmovdqu_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Selectively write bytes from `mm1` to memory location using the byte mask in mm2.
	///
	/// The default memory location is specified by `DS:DI`, `EDI` or `RDI`.
	#[inline(always)]
	pub fn maskmovq_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the maximum double-precision floating-point values between `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn maxpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the maximum double-precision floating-point values between `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn maxpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the maximum single-precision floating-point values between `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn maxps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the maximum single-precision floating-point values between `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn maxps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the maximum scalar double-precision floating-point value between `xmm2/mem64` and `xmm1`.
	#[inline(always)]
	pub fn maxsd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the maximum scalar double-precision floating-point value between `xmm2/mem64` and `xmm1`.
	#[inline(always)]
	pub fn maxsd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the maximum scalar single-precision floating-point value between `xmm2/mem32` and `xmm1`.
	#[inline(always)]
	pub fn maxss_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the maximum scalar single-precision floating-point value between `xmm2/mem32` and `xmm1`.
	#[inline(always)]
	pub fn maxss_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Serializes load and store operations.
	#[inline(always)]
	pub fn mfence(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0x0F, 0xAE, 0xF0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the minimum double-precision floating-point values between `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn minpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the minimum double-precision floating-point values between `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn minpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the minimum single-precision floating-point values between `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn minps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the minimum single-precision floating-point values between `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn minps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the minimum scalar double-precision floating-point value between `xmm2/mem64` and `xmm1`.
	#[inline(always)]
	pub fn minsd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the minimum scalar double-precision floating-point value between `xmm2/mem64` and `xmm1`.
	#[inline(always)]
	pub fn minsd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the minimum scalar single-precision floating-point value between `xmm2/mem32` and `xmm1`.
	#[inline(always)]
	pub fn minss_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Return the minimum scalar single-precision floating-point value between `xmm2/mem32` and `xmm1`.
	#[inline(always)]
	pub fn minss_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x5D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sets up a linear address range to be monitored by hardware and activates the monitor.
	///
	/// The address range should be a write-back memory caching type.
	///
	/// The address is `DS:EAX` (`DS:RAX` in 64-bit mode).
	#[inline(always)]
	pub fn monitor(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0x0F, 0x01, 0xC8);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move byte at `seg:offset` to `AL`.
	#[inline(always)]
	pub fn mov_AL_MemoryOffset8Bit(&mut self, arg1: MemoryOffset8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move byte at `offset` to `AL`.
	#[inline(always)]
	pub fn mov_AL_MemoryOffset8Bit_PrefixRexW(&mut self, arg1: MemoryOffset8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xA0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move word at `seg:offset` to `AX`.
	#[inline(always)]
	pub fn mov_AX_MemoryOffset16Bit(&mut self, arg1: MemoryOffset16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move doubleword at `seg:offset` to `EAX`.
	#[inline(always)]
	pub fn mov_EAX_MemoryOffset32Bit(&mut self, arg1: MemoryOffset32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `imm16` to `r/m16`.
	#[inline(always)]
	pub fn mov_Any16BitMemory_Immediate16Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xC7);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `r16` to `r/m16`.
	#[inline(always)]
	pub fn mov_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x89);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move segment register to `r/m16`.
	#[inline(always)]
	pub fn mov_Any16BitMemory_SegmentRegister(&mut self, arg0: Any16BitMemory, arg1: SegmentRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x8C);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `imm32` to `r/m32`.
	#[inline(always)]
	pub fn mov_Any32BitMemory_Immediate32Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xC7);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `r32` to `r/m32`.
	#[inline(always)]
	pub fn mov_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x89);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `imm32` sign extended to 64-bits to `r/m64`.
	#[inline(always)]
	pub fn mov_Any64BitMemory_Immediate32Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xC7);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `r64` to `r/m64`.
	#[inline(always)]
	pub fn mov_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x89);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move zero extended 16-bit segment register to `r/m64`.
	#[inline(always)]
	pub fn mov_Any64BitMemory_SegmentRegister(&mut self, arg0: Any64BitMemory, arg1: SegmentRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x8C);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `imm8` to `r/m8`.
	#[inline(always)]
	pub fn mov_Any8BitMemory_Immediate8Bit(&mut self, arg0: Any8BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xC6);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `r8` to `r/m8`.
	#[inline(always)]
	pub fn mov_Any8BitMemory_Register8Bit(&mut self, arg0: Any8BitMemory, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x88);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r8` to `r/m8`.
	#[inline(always)]
	pub fn mov_Any8BitMemory_RegisterHigh8BitsOf16Bits(&mut self, arg0: Any8BitMemory, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x88);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `AX` to `seg:offset`.
	#[inline(always)]
	pub fn mov_MemoryOffset16Bit_AX(&mut self, arg0: MemoryOffset16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Move `EAX` to `seg:offset`.
	#[inline(always)]
	pub fn mov_MemoryOffset32Bit_EAX(&mut self, arg0: MemoryOffset32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Move `RAX` to `offset`.
	#[inline(always)]
	pub fn mov_MemoryOffset64Bit_RAX(&mut self, arg0: MemoryOffset64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xA3);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Move `AL` to `seg:offset`.
	#[inline(always)]
	pub fn mov_MemoryOffset8Bit_AL(&mut self, arg0: MemoryOffset8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA2);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Move `AL` to `offset`.
	#[inline(always)]
	pub fn mov_MemoryOffset8Bit_AL_PrefixRexW(&mut self, arg0: MemoryOffset8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xA2);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Move `imm16` to `r16`.
	#[inline(always)]
	pub fn mov_Register16Bit_Immediate16Bit(&mut self, arg0: Register16Bit, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0xB8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `imm16` to `r/m16`.
	#[inline(always)]
	pub fn mov_Register16Bit_Immediate16Bit_REX_W(&mut self, arg0: Register16Bit, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xC7);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `r/m16` to `r16`.
	#[inline(always)]
	pub fn mov_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r16` to `r/m16`.
	#[inline(always)]
	pub fn mov_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x89);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m16` to `r16`.
	#[inline(always)]
	pub fn mov_Register16Bit_Register16Bit_REX_W(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move segment register to `r/m16`.
	#[inline(always)]
	pub fn mov_Register16Bit_SegmentRegister(&mut self, arg0: Register16Bit, arg1: SegmentRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x8C);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `imm32` to `r32`.
	#[inline(always)]
	pub fn mov_Register32Bit_Immediate32Bit(&mut self, arg0: Register32Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0xB8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `imm32` to `r/m32`.
	#[inline(always)]
	pub fn mov_Register32Bit_Immediate32Bit_REX_W(&mut self, arg0: Register32Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xC7);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `r/m32` to `r32`.
	#[inline(always)]
	pub fn mov_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r32` to `r/m32`.
	#[inline(always)]
	pub fn mov_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x89);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m32` to `r32`.
	#[inline(always)]
	pub fn mov_Register32Bit_Register32Bit_REX_W(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `imm32` sign extended to 64-bits to `r/m64`.
	#[inline(always)]
	pub fn mov_Register64Bit_Immediate32Bit(&mut self, arg0: Register64Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xC7);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `imm64` to `r64`.
	#[inline(always)]
	pub fn mov_Register64Bit_Immediate64Bit(&mut self, arg0: Register64Bit, arg1: Immediate64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_2(0xB8, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `r/m64` to `r64`.
	#[inline(always)]
	pub fn mov_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x8B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r64` to `r/m64`.
	#[inline(always)]
	pub fn mov_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x89);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m64` to `r64`.
	#[inline(always)]
	pub fn mov_Register64Bit_Register64Bit_REX_W(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x8B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move zero extended 16-bit segment register to `r/m64`.
	#[inline(always)]
	pub fn mov_Register64Bit_SegmentRegister(&mut self, arg0: Register64Bit, arg1: SegmentRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x8C);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `imm8` to `r8`.
	#[inline(always)]
	pub fn mov_Register8Bit_Immediate8Bit(&mut self, arg0: Register8Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0xB0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `imm8` to `r/m8`.
	#[inline(always)]
	pub fn mov_Register8Bit_Immediate8Bit_REX_W(&mut self, arg0: Register8Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xC6);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `r/m8` to `r8`.
	#[inline(always)]
	pub fn mov_Register8Bit_Any8BitMemory(&mut self, arg0: Register8Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r8` to `r/m8`.
	#[inline(always)]
	pub fn mov_Register8Bit_Register8Bit(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x88);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r8`.
	#[inline(always)]
	pub fn mov_Register8Bit_Register8Bit_REX_W(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r8` to `r/m8`.
	#[inline(always)]
	pub fn mov_Register8Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x88);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r8`.
	#[inline(always)]
	pub fn mov_Register8Bit_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword at `offset` to `RAX`.
	#[inline(always)]
	pub fn mov_RAX_MemoryOffset64Bit(&mut self, arg1: MemoryOffset64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xA1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `imm8` to `r8`.
	#[inline(always)]
	pub fn mov_RegisterHigh8BitsOf16Bits_Immediate8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0xB0, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `imm8` to `r/m8`.
	#[inline(always)]
	pub fn mov_RegisterHigh8BitsOf16Bits_Immediate8Bit_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xC6);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Move `r/m8` to `r8`.
	#[inline(always)]
	pub fn mov_RegisterHigh8BitsOf16Bits_Any8BitMemory(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r8` to `r/m8`.
	#[inline(always)]
	pub fn mov_RegisterHigh8BitsOf16Bits_Register8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x88);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r8`.
	#[inline(always)]
	pub fn mov_RegisterHigh8BitsOf16Bits_Register8Bit_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r8` to `r/m8`.
	#[inline(always)]
	pub fn mov_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x88);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r8`.
	#[inline(always)]
	pub fn mov_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m16` to segment register.
	#[inline(always)]
	pub fn mov_SegmentRegister_Any16BitMemory(&mut self, arg0: SegmentRegister, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move lower 16 bits of `r/m64` to segment register.
	#[inline(always)]
	pub fn mov_SegmentRegister_Any64BitMemory(&mut self, arg0: SegmentRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x8E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m16` to segment register.
	#[inline(always)]
	pub fn mov_SegmentRegister_Register16Bit(&mut self, arg0: SegmentRegister, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x8E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move lower 16 bits of `r/m64` to segment register.
	#[inline(always)]
	pub fn mov_SegmentRegister_Register64Bit(&mut self, arg0: SegmentRegister, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x8E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed double-precision floating-point values from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movapd_Any128BitMemory_XMMRegister(&mut self, arg0: Any128BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x29);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movapd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x28);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movapd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x28);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed double-precision floating-point values from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movapd_XMMRegister_XMMRegister_REX_W(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x29);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed single-precision floating-point values from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movaps_Any128BitMemory_XMMRegister(&mut self, arg0: Any128BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x29);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movaps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x28);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movaps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x28);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed single-precision floating-point values from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movaps_XMMRegister_XMMRegister_REX_W(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x29);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse byte order in `r16` and move to `m16`.
	#[inline(always)]
	pub fn movbe_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF1);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse byte order in `r32` and move to `m32`.
	#[inline(always)]
	pub fn movbe_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF1);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse byte order in `r64` and move to `m64`.
	#[inline(always)]
	pub fn movbe_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_3(0x0F, 0x38, 0xF1);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse byte order in `m16` and move to `r16`.
	#[inline(always)]
	pub fn movbe_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse byte order in `m32` and move to `r32`.
	#[inline(always)]
	pub fn movbe_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0xF0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse byte order in `m64` and move to `r64`.
	#[inline(always)]
	pub fn movbe_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_3(0x0F, 0x38, 0xF0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move doubleword from `mm` to `r/m32`.
	#[inline(always)]
	pub fn movd_Any32BitMemory_MMRegister(&mut self, arg0: Any32BitMemory, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move doubleword from `xmm` register to `r/m32`.
	#[inline(always)]
	pub fn movd_Any32BitMemory_XMMRegister(&mut self, arg0: Any32BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move doubleword from `r/m32` to `mm`.
	#[inline(always)]
	pub fn movd_MMRegister_Any32BitMemory(&mut self, arg0: MMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move doubleword from `r/m32` to `mm`.
	#[inline(always)]
	pub fn movd_MMRegister_Register32Bit(&mut self, arg0: MMRegister, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move doubleword from `mm` to `r/m32`.
	#[inline(always)]
	pub fn movd_Register32Bit_MMRegister(&mut self, arg0: Register32Bit, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move doubleword from `xmm` register to `r/m32`.
	#[inline(always)]
	pub fn movd_Register32Bit_XMMRegister(&mut self, arg0: Register32Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move doubleword from `r/m32` to `xmm`.
	#[inline(always)]
	pub fn movd_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move doubleword from `r/m32` to `xmm`.
	#[inline(always)]
	pub fn movd_XMMRegister_Register32Bit(&mut self, arg0: XMMRegister, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move one double-precision floating-point value from the lower 64-bit operand in `xmm2/m64` to `xmm1` and duplicate.
	#[inline(always)]
	pub fn movddup_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move one double-precision floating-point value from the lower 64-bit operand in `xmm2/m64` to `xmm1` and duplicate.
	#[inline(always)]
	pub fn movddup_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move low quadword from `xmm` to `mm`.
	#[inline(always)]
	pub fn movdq2q_MMRegister_XMMRegister(&mut self, arg0: MMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move aligned double quadword from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movdqa_Any128BitMemory_XMMRegister(&mut self, arg0: Any128BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7F);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move aligned double quadword from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movdqa_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move aligned double quadword from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movdqa_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move aligned double quadword from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movdqa_XMMRegister_XMMRegister_REX_W(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7F);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move unaligned double quadword from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movdqu_Any128BitMemory_XMMRegister(&mut self, arg0: Any128BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7F);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move unaligned double quadword from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movdqu_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move unaligned double quadword from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movdqu_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move unaligned double quadword from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movdqu_XMMRegister_XMMRegister_REX_W(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7F);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two packed single-precision floating-point values from high quadword of `xmm2` to low quadword of `xmm1`.
	#[inline(always)]
	pub fn movhlps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move double-precision floating-point value from high quadword of `xmm` to `m64`.
	#[inline(always)]
	pub fn movhpd_Any64BitMemory_XMMRegister(&mut self, arg0: Any64BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x17);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move double-precision floating-point value from `m64` to high quadword of `xmm`.
	#[inline(always)]
	pub fn movhpd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x16);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two packed single-precision floating-point values from high quadword of `xmm` to `m64`.
	#[inline(always)]
	pub fn movhps_Any64BitMemory_XMMRegister(&mut self, arg0: Any64BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x17);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two packed single-precision floating-point values from `m64` to high quadword of `xmm`.
	#[inline(always)]
	pub fn movhps_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x16);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two packed single-precision floating-point values from low quadword of `xmm2` to high quadword of `xmm1`.
	#[inline(always)]
	pub fn movlhps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x16);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move double-precision floating-point nvalue from low quadword of `xmm` register to `m64`.
	#[inline(always)]
	pub fn movlpd_Any64BitMemory_XMMRegister(&mut self, arg0: Any64BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x13);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move double-precision floating-point value from `m64` to low quadword of `xmm` register.
	#[inline(always)]
	pub fn movlpd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two packed single-precision floating-point values from low quadword of `xmm` to `m64`.
	#[inline(always)]
	pub fn movlps_Any64BitMemory_XMMRegister(&mut self, arg0: Any64BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x13);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two packed single-precision floating-point values from `m64` to low quadword of `xmm`.
	#[inline(always)]
	pub fn movlps_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Extract 2-bit sign mask from `xmm` and store in `r32`.
	///
	/// The upper bits of `r32` or `r64` are filled with zeros.
	#[inline(always)]
	pub fn movmskpd_Register32Bit_XMMRegister(&mut self, arg0: Register32Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x50);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Extract 2-bit sign mask from `xmm` and store in `r64`.
	///
	/// The upper bits of `r32` or `r64` are filled with zeros.
	#[inline(always)]
	pub fn movmskpd_Register64Bit_XMMRegister(&mut self, arg0: Register64Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x50);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Extract 4-bit sign mask from `xmm` and store in `r32`.
	///
	/// The upper bits of `r32` or `r64` are filled with zeros.
	#[inline(always)]
	pub fn movmskps_Register32Bit_XMMRegister(&mut self, arg0: Register32Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x50);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Extract 4-bit sign mask from `xmm` and store in `r64`.
	///
	/// The upper bits of `r32` or `r64` are filled with zeros.
	#[inline(always)]
	pub fn movmskps_Register64Bit_XMMRegister(&mut self, arg0: Register64Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x50);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move double quadword from `xmm` to `m128` using non-temporal hint.
	#[inline(always)]
	pub fn movntdq_Any128BitMemory_XMMRegister(&mut self, arg0: Any128BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xE7);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move double quadword from `m128` to `xmm` using non-temporal hint if Write Commit (WC) memory type.
	#[inline(always)]
	pub fn movntdqa_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x2A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move doubleword from `r32` to `m32` using non-temporal hint.
	#[inline(always)]
	pub fn movnti_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xC3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `r64` to `m64` using non-temporal hint.
	#[inline(always)]
	pub fn movnti_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0xC3);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed double-precision floating-point values from `xmm` to `m128` using non-temporal hint.
	#[inline(always)]
	pub fn movntpd_Any128BitMemory_XMMRegister(&mut self, arg0: Any128BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x2B);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed single-precision floating-point values from `xmm` to `m128` using non-temporal hint.
	#[inline(always)]
	pub fn movntps_Any128BitMemory_XMMRegister(&mut self, arg0: Any128BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x2B);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `mm` to `m64` using non-temporal hint.
	#[inline(always)]
	pub fn movntq_Any64BitMemory_MMRegister(&mut self, arg0: Any64BitMemory, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xE7);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `mm` to `r/m64`.
	#[inline(always)]
	pub fn movq_Any64BitMemory_MMRegister(&mut self, arg0: Any64BitMemory, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `mm` to `mm/m64`.
	#[inline(always)]
	pub fn movq_Any64BitMemory_MMRegister_REX_W(&mut self, arg0: Any64BitMemory, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7F);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `xmm` register to `r/m64`.
	#[inline(always)]
	pub fn movq_Any64BitMemory_XMMRegister(&mut self, arg0: Any64BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `xmm1` to `xmm2/mem64`.
	#[inline(always)]
	pub fn movq_Any64BitMemory_XMMRegister_REX_W(&mut self, arg0: Any64BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xD6);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `r/m64` to `mm`.
	#[inline(always)]
	pub fn movq_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x6E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `mm/m64` to `mm`.
	#[inline(always)]
	pub fn movq_MMRegister_Any64BitMemory_REX_W(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `mm/m64` to `mm`.
	#[inline(always)]
	pub fn movq_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `mm` to `mm/m64`.
	#[inline(always)]
	pub fn movq_MMRegister_MMRegister_REX_W(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x7F);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `r/m64` to `mm`.
	#[inline(always)]
	pub fn movq_MMRegister_Register64Bit(&mut self, arg0: MMRegister, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x6E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `mm` to `r/m64`.
	#[inline(always)]
	pub fn movq_Register64Bit_MMRegister(&mut self, arg0: Register64Bit, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `xmm` register to `r/m64`.
	#[inline(always)]
	pub fn movq_Register64Bit_XMMRegister(&mut self, arg0: Register64Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `r/m64` to `xmm`.
	#[inline(always)]
	pub fn movq_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x6E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `xmm2/mem64` to `xmm1`.
	#[inline(always)]
	pub fn movq_XMMRegister_Any64BitMemory_REX_W(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `r/m64` to `xmm`.
	#[inline(always)]
	pub fn movq_XMMRegister_Register64Bit(&mut self, arg0: XMMRegister, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0x6E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `xmm2/mem64` to `xmm1`.
	#[inline(always)]
	pub fn movq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x7E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from `xmm1` to `xmm2/mem64`.
	#[inline(always)]
	pub fn movq_XMMRegister_XMMRegister_REX_W(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0xD6);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move quadword from mmx to low quadword of `xmm`.
	#[inline(always)]
	pub fn movq2dq_XMMRegister_MMRegister(&mut self, arg0: XMMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, move word from address `DS:(E)SI` to `ES:(E)DI`.
	///
	/// For 64-bit mode move word at address (R
	///E)SI to `(R
	///E)DI`.
	#[inline(always)]
	pub fn movs_Any16BitMemory_Any16BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, move dword from address `DS:(E)SI` to `ES:(E)DI`.
	///
	/// For 64-bit mode move dword from address (R
	///E)SI to `(R
	///E)DI`.
	#[inline(always)]
	pub fn movs_Any32BitMemory_Any32BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move qword from address (R
	///E)SI to `(R
	///E)DI`.
	#[inline(always)]
	pub fn movs_Any64BitMemory_Any64BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xA5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, Move byte from address `DS:(E)SI` to `ES:(E)DI`.
	///
	/// For 64-bit mode move byte from address `(R
	///E)SI` to `(R
	///E)DI`.
	#[inline(always)]
	pub fn movs_Any8BitMemory_Any8BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA4);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, Move byte from address `DS:(E)SI` to `ES:(E)DI`.
	///
	/// For 64-bit mode move byte from address `(R
	///E)SI` to `(R
	///E)DI`.
	#[inline(always)]
	pub fn movsb(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA4);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, move dword from address `DS:(E)SI` to `ES:(E)DI`.
	///
	/// For 64-bit mode move dword from address `(R
	///E)SI` to `(R
	///E)DI`.
	#[inline(always)]
	pub fn movsd(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move scalar double-precision floating-point value from `xmm1` register to `xmm2/m64`.
	#[inline(always)]
	pub fn movsd_Any64BitMemory_XMMRegister(&mut self, arg0: Any64BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move scalar double-precision floating-point value from `xmm2/m64` to `xmm1`.
	#[inline(always)]
	pub fn movsd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move scalar double-precision floating-point value from `xmm2/m64` to `xmm1`.
	#[inline(always)]
	pub fn movsd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move scalar double-precision floating-point value from `xmm1` register to `xmm2/m64`.
	#[inline(always)]
	pub fn movsd_XMMRegister_XMMRegister_REX_W(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two single-precision floating-point values from the higher 32-bit operand of each qword in `xmm2/m128` to `xmm1` and duplicate each 32-bit operand to the lower 32-bits of each qword.
	#[inline(always)]
	pub fn movshdup_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x16);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two single-precision floating-point values from the higher 32-bit operand of each qword in `xmm2/m128` to `xmm1` and duplicate each 32-bit operand to the lower 32-bits of each qword.
	#[inline(always)]
	pub fn movshdup_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x16);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two single-precision floating-point values from the lower 32-bit operand of each qword in `xmm2/m128` to `xmm1` and duplicate each 32-bit operand to the higher 32-bits of each qword.
	#[inline(always)]
	pub fn movsldup_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move two single-precision floating-point values from the lower 32-bit operand of each qword in `xmm2/m128` to `xmm1` and duplicate each 32-bit operand to the higher 32-bits of each qword.
	#[inline(always)]
	pub fn movsldup_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x12);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move qword from address `(R
	///E)SI` to `(R
	///E)DI`.
	#[inline(always)]
	pub fn movsq(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0xA5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move scalar single-precision floating-point value from `xmm1` register to `xmm2/m32`.
	#[inline(always)]
	pub fn movss_Any32BitMemory_XMMRegister(&mut self, arg0: Any32BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move scalar single-precision floating-point value from `xmm2/m32` to `xmm1`.
	#[inline(always)]
	pub fn movss_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move scalar single-precision floating-point value from `xmm2/m32` to `xmm1`.
	#[inline(always)]
	pub fn movss_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move scalar single-precision floating-point value from `xmm1` register to `xmm2/m32`.
	#[inline(always)]
	pub fn movss_XMMRegister_XMMRegister_REX_W(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// For legacy mode, move word from address `DS:(E)SI` to `ES:(E)DI`.
	///
	/// For 64-bit mode move word at address `(R
	///E)SI` to `(R
	///E)DI`.
	#[inline(always)]
	pub fn movsw(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xA5);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r16` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register16Bit_Any8BitMemory(&mut self, arg0: Register16Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r8` to `r16` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register16Bit_Register8Bit(&mut self, arg0: Register16Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r16` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register16Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register16Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m16` to `r32`, with sign-extension.
	#[inline(always)]
	pub fn movsx_Register32Bit_Any16BitMemory(&mut self, arg0: Register32Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r32` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register32Bit_Any8BitMemory(&mut self, arg0: Register32Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r16` to `r32`, with sign-extension.
	#[inline(always)]
	pub fn movsx_Register32Bit_Register16Bit(&mut self, arg0: Register32Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r8` to `r32` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register32Bit_Register8Bit(&mut self, arg0: Register32Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r32` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register32Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register32Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xBE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m16` to `r64` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register64Bit_Any16BitMemory(&mut self, arg0: Register64Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r64` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register64Bit_Any8BitMemory(&mut self, arg0: Register64Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r16` to `r64` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register64Bit_Register16Bit(&mut self, arg0: Register64Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r8` to `r64` with sign-extension.
	#[inline(always)]
	pub fn movsx_Register64Bit_Register8Bit(&mut self, arg0: Register64Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xBE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m32` to `r64` with sign-extension.
	#[inline(always)]
	pub fn movsxd_Register64Bit_Any32BitMemory(&mut self, arg0: Register64Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x63);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r32` to `r64` with sign-extension.
	#[inline(always)]
	pub fn movsxd_Register64Bit_Register32Bit(&mut self, arg0: Register64Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x63);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed double-precision floating-point values from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movupd_Any128BitMemory_XMMRegister(&mut self, arg0: Any128BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movupd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed double-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movupd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed double-precision floating-point values from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movupd_XMMRegister_XMMRegister_REX_W(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed single-precision floating-point values from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movups_Any128BitMemory_XMMRegister(&mut self, arg0: Any128BitMemory, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movups_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed single-precision floating-point values from `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn movups_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move packed single-precision floating-point values from `xmm1` to `xmm2/m128`.
	#[inline(always)]
	pub fn movups_XMMRegister_XMMRegister_REX_W(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_2(0x0F, 0x11);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r16` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register16Bit_Any8BitMemory(&mut self, arg0: Register16Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r16` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register16Bit_Register8Bit(&mut self, arg0: Register16Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r16` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register16Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register16Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m16` to `r32` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register32Bit_Any16BitMemory(&mut self, arg0: Register32Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r32` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register32Bit_Any8BitMemory(&mut self, arg0: Register32Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m16` to `r32` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register32Bit_Register16Bit(&mut self, arg0: Register32Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r32` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register32Bit_Register8Bit(&mut self, arg0: Register32Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r32` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register32Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register32Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m16` to `r64` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register64Bit_Any16BitMemory(&mut self, arg0: Register64Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xB7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r64` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register64Bit_Any8BitMemory(&mut self, arg0: Register64Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xB6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m16` to `r64` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register64Bit_Register16Bit(&mut self, arg0: Register64Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xB7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move `r/m8` to `r64` with zero-extension.
	#[inline(always)]
	pub fn movzx_Register64Bit_Register8Bit(&mut self, arg0: Register64Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xB6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sums absolute 8-bit integer difference of adjacent groups of 4 byte integers in `xmm1` and `xmm2/m128` and writes the results in `xmm1`.
	///
	/// Starting offsets within `xmm1` and `xmm2/m128` are determined by `imm8`.
	#[inline(always)]
	pub fn mpsadbw_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x42);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Sums absolute 8-bit integer difference of adjacent groups of 4 byte integers in `xmm1` and `xmm2/m128` and writes the results in `xmm1`.
	///
	/// Starting offsets within `xmm1` and `xmm2/m128` are determined by `imm8`.
	#[inline(always)]
	pub fn mpsadbw_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x42);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Unsigned multiply (`DX:AX` = `AX` * `r/m16`).
	#[inline(always)]
	pub fn mul_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned multiply (`EDX:EAX` = `EAX` * `r/m32`).
	#[inline(always)]
	pub fn mul_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned multiply (`RDX:RAX` = `RAX` * `r/m64`.
	#[inline(always)]
	pub fn mul_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned multiply (`AX` = `AL` * `r/m8`).
	#[inline(always)]
	pub fn mul_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned multiply (`DX:AX` = `AX` * `r/m16`).
	#[inline(always)]
	pub fn mul_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned multiply (`EDX:EAX` = `EAX` * `r/m32`).
	#[inline(always)]
	pub fn mul_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned multiply (`RDX:RAX` = `RAX` * `r/m64`.
	#[inline(always)]
	pub fn mul_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned multiply (`AX` = `AL` * `r/m8`).
	#[inline(always)]
	pub fn mul_Register8Bit(&mut self, arg0: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned multiply (`AX` = `AL` * `r/m8`).
	#[inline(always)]
	pub fn mul_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply packed double-precision floating-point values in `xmm2/m128` by `xmm1`.
	#[inline(always)]
	pub fn mulpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x59);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply packed double-precision floating-point values in `xmm2/m128` by `xmm1`.
	#[inline(always)]
	pub fn mulpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x59);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply packed single-precision floating-point values in `xmm2/mem` by `xmm1`.
	#[inline(always)]
	pub fn mulps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x59);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply packed single-precision floating-point values in `xmm2/mem` by `xmm1`.
	#[inline(always)]
	pub fn mulps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x59);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the low double-precision floating-point value in `xmm2/mem64` by low double-precision floating-point value in `xmm1`.
	#[inline(always)]
	pub fn mulsd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x59);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the low double-precision floating-point value in `xmm2/mem64` by low double-precision floating-point value in `xmm1`.
	#[inline(always)]
	pub fn mulsd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x59);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the low single-precision floating-point value in `xmm2/mem` by the low single-precision floating-point value in `xmm1`.
	#[inline(always)]
	pub fn mulss_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x59);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the low single-precision floating-point value in `xmm2/mem` by the low single-precision floating-point value in `xmm1`.
	#[inline(always)]
	pub fn mulss_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x59);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unsigned multiply of `r/m32` with `EDX` without affecting arithmetic flags.
	#[inline(always)]
	pub fn mulx_Register32Bit_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Any32BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg2);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg2);

		self.vex_7(0x02, 0x0, 0x3, 0x0, arg1, arg2, arg0);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Unsigned multiply of `r/m32` with `EDX` without affecting arithmetic flags.
	#[inline(always)]
	pub fn mulx_Register32Bit_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x3, 0x0, arg1, arg2, arg0);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Unsigned multiply of `r/m64` with `RDX` without affecting arithmetic flags.
	#[inline(always)]
	pub fn mulx_Register64Bit_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Any64BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg2);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg2);

		self.vex_7(0x02, 0x0, 0x3, 0x1, arg1, arg2, arg0);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Unsigned multiply of `r/m64` with `RDX` without affecting arithmetic flags.
	#[inline(always)]
	pub fn mulx_Register64Bit_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x3, 0x1, arg1, arg2, arg0);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// A hint that allow the processor to stop instruction execution and enter an implementation-dependent optimized state until occurrence of a class of events.
	#[inline(always)]
	pub fn mwait(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_3(0x0F, 0x01, 0xC9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Two's complement negate `r/m16`.
	#[inline(always)]
	pub fn neg_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Two's complement negate `r/m32`.
	#[inline(always)]
	pub fn neg_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Two's complement negate `r/m64`.
	#[inline(always)]
	pub fn neg_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Two's complement negate `r/m8`.
	#[inline(always)]
	pub fn neg_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Two's complement negate `r/m16`.
	#[inline(always)]
	pub fn neg_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Two's complement negate `r/m32`.
	#[inline(always)]
	pub fn neg_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Two's complement negate `r/m64`.
	#[inline(always)]
	pub fn neg_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Two's complement negate `r/m8`.
	#[inline(always)]
	pub fn neg_Register8Bit(&mut self, arg0: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Two's complement negate `r/m8`.
	#[inline(always)]
	pub fn neg_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// One byte no-operation instruction.
	#[inline(always)]
	pub fn nop(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x90);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multi-byte no-operation instruction.
	#[inline(always)]
	pub fn nop_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x1F);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multi-byte no-operation instruction.
	#[inline(always)]
	pub fn nop_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x1F);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multi-byte no-operation instruction.
	#[inline(always)]
	pub fn nop_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x1F);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multi-byte no-operation instruction.
	#[inline(always)]
	pub fn nop_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x1F);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse each bit of `r/m16`.
	#[inline(always)]
	pub fn not_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse each bit of `r/m32`.
	#[inline(always)]
	pub fn not_Any32BitMemory(&mut self, arg0: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse each bit of `r/m64`.
	#[inline(always)]
	pub fn not_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse each bit of `r/m8`.
	#[inline(always)]
	pub fn not_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse each bit of `r/m16`.
	#[inline(always)]
	pub fn not_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse each bit of `r/m32`.
	#[inline(always)]
	pub fn not_Register32Bit(&mut self, arg0: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse each bit of `r/m64`.
	#[inline(always)]
	pub fn not_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0xF7);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse each bit of `r/m8`.
	#[inline(always)]
	pub fn not_Register8Bit(&mut self, arg0: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Reverse each bit of `r/m8`.
	#[inline(always)]
	pub fn not_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0xF6);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `AL` OR `imm8`.
	#[inline(always)]
	pub fn or_AL_Immediate8Bit(&mut self, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x0C);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `AX` OR `imm16`.
	#[inline(always)]
	pub fn or_AX_Immediate16Bit(&mut self, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x0D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `EAX` OR `imm32`.
	#[inline(always)]
	pub fn or_EAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x0D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m16` OR `imm16`.
	#[inline(always)]
	pub fn or_Any16BitMemory_Immediate16Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m16` OR `imm8` (sign-extended).
	#[inline(always)]
	pub fn or_Any16BitMemory_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m16` OR `r16`.
	#[inline(always)]
	pub fn or_Any16BitMemory_Register16Bit(&mut self, arg0: Any16BitMemory, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x09);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m32` OR `imm32`.
	#[inline(always)]
	pub fn or_Any32BitMemory_Immediate32Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m32` OR `imm8` (sign-extended).
	#[inline(always)]
	pub fn or_Any32BitMemory_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m32` OR `r32`.
	#[inline(always)]
	pub fn or_Any32BitMemory_Register32Bit(&mut self, arg0: Any32BitMemory, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x09);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m64` OR `imm32` (sign-extended).
	#[inline(always)]
	pub fn or_Any64BitMemory_Immediate32Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m64` OR `imm8` (sign-extended).
	#[inline(always)]
	pub fn or_Any64BitMemory_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m64` OR `r64`.
	#[inline(always)]
	pub fn or_Any64BitMemory_Register64Bit(&mut self, arg0: Any64BitMemory, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x09);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8` OR `imm8`.
	#[inline(always)]
	pub fn or_Any8BitMemory_Immediate8Bit(&mut self, arg0: Any8BitMemory, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m8` OR `r8`.
	#[inline(always)]
	pub fn or_Any8BitMemory_Register8Bit(&mut self, arg0: Any8BitMemory, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x08);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8` OR `r8`.
	#[inline(always)]
	pub fn or_Any8BitMemory_RegisterHigh8BitsOf16Bits(&mut self, arg0: Any8BitMemory, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x08);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m16` OR `imm16`.
	#[inline(always)]
	pub fn or_Register16Bit_Immediate16Bit(&mut self, arg0: Register16Bit, arg1: Immediate16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m16` OR `imm8` (sign-extended).
	#[inline(always)]
	pub fn or_Register16Bit_Immediate8Bit(&mut self, arg0: Register16Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r16` OR `r/m16`.
	#[inline(always)]
	pub fn or_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m16` OR `r16`.
	#[inline(always)]
	pub fn or_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x09);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r16` OR `r/m16`.
	#[inline(always)]
	pub fn or_Register16Bit_Register16Bit_REX_W(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m32` OR `imm32`.
	#[inline(always)]
	pub fn or_Register32Bit_Immediate32Bit(&mut self, arg0: Register32Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m32` OR `imm8` (sign-extended).
	#[inline(always)]
	pub fn or_Register32Bit_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r32` OR `r/m32`.
	#[inline(always)]
	pub fn or_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m32` OR `r32`.
	#[inline(always)]
	pub fn or_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x09);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r32` OR `r/m32`.
	#[inline(always)]
	pub fn or_Register32Bit_Register32Bit_REX_W(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m64` OR `imm32` (sign-extended).
	#[inline(always)]
	pub fn or_Register64Bit_Immediate32Bit(&mut self, arg0: Register64Bit, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x81);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m64` OR `imm8` (sign-extended).
	#[inline(always)]
	pub fn or_Register64Bit_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, Self::REX_W);

		self.opcode_1(0x83);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r64` OR `r/m64`.
	#[inline(always)]
	pub fn or_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m64` OR `r64`.
	#[inline(always)]
	pub fn or_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_1(0x09);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r64` OR `r/m64`.
	#[inline(always)]
	pub fn or_Register64Bit_Register64Bit_REX_W(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_1(0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8` OR `imm8`.
	#[inline(always)]
	pub fn or_Register8Bit_Immediate8Bit(&mut self, arg0: Register8Bit, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r8` OR `r/m8`.
	#[inline(always)]
	pub fn or_Register8Bit_Any8BitMemory(&mut self, arg0: Register8Bit, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8` OR `r8`.
	#[inline(always)]
	pub fn or_Register8Bit_Register8Bit(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x08);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r8` OR `r/m8`.
	#[inline(always)]
	pub fn or_Register8Bit_Register8Bit_REX_W(&mut self, arg0: Register8Bit, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8` OR `r8`.
	#[inline(always)]
	pub fn or_Register8Bit_RegisterHigh8BitsOf16Bits(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x08);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r8` OR `r/m8`.
	#[inline(always)]
	pub fn or_Register8Bit_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: Register8Bit, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `RAX` OR `imm32` (sign-extended).
	#[inline(always)]
	pub fn or_RAX_Immediate32Bit(&mut self, arg1: Immediate32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_1(Self::REX_W);

		self.opcode_1(0x0D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r/m8` OR `imm8`.
	#[inline(always)]
	pub fn or_RegisterHigh8BitsOf16Bits_Immediate8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x80);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// `r8` OR `r/m8`.
	#[inline(always)]
	pub fn or_RegisterHigh8BitsOf16Bits_Any8BitMemory(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8` OR `r8`.
	#[inline(always)]
	pub fn or_RegisterHigh8BitsOf16Bits_Register8Bit(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x08);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r8` OR `r/m8`.
	#[inline(always)]
	pub fn or_RegisterHigh8BitsOf16Bits_Register8Bit_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: Register8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r/m8` OR `r8`.
	#[inline(always)]
	pub fn or_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_1(0x08);

		self.mod_rm_sib(arg0, arg1);

		// No displacement or immediate.

		// No label displacement.
	}

	/// `r8` OR `r/m8`.
	#[inline(always)]
	pub fn or_RegisterHigh8BitsOf16Bits_RegisterHigh8BitsOf16Bits_REX_W(&mut self, arg0: RegisterHigh8BitsOf16Bits, arg1: RegisterHigh8BitsOf16Bits)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_1(0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise OR of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn orpd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x56);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise OR of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn orpd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x56);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise OR of `xmm1` and `xmm2/m128`.
	#[inline(always)]
	pub fn orps_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x56);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise OR of `xmm1` and `xmm2/m128`.
	#[inline(always)]
	pub fn orps_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x56);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Output byte in `AL` to I/O port address in `DX`.
	#[inline(always)]
	pub fn out_DX_AL(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xEE);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Output word in `AX` to I/O port address in `DX`.
	#[inline(always)]
	pub fn out_DX_AX(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xEF);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Output doubleword in `EAX` to I/O port address in `DX`.
	#[inline(always)]
	pub fn out_DX_EAX(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xEF);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Output byte in `AL` to I/O port address `imm8`.
	#[inline(always)]
	pub fn out_Immediate8Bit_AL(&mut self, arg0: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE6);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Output word in `AX` to I/O port address `imm8`.
	#[inline(always)]
	pub fn out_Immediate8Bit_AX(&mut self, arg0: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE7);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Output doubleword in `EAX` to I/O port address `imm8`.
	#[inline(always)]
	pub fn out_Immediate8Bit_EAX(&mut self, arg0: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0xE7);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		self.displacement_immediate_1(arg0);

		// No label displacement.
	}

	/// Output word from memory location specified in `DS:(E)SI` or `RSI` to I/O port specified in `DX`.
	#[inline(always)]
	pub fn outs_DX_Any16BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Output doubleword from memory location specified in `DS:(E)SI` or `RSI` to I/O port specified in `DX`.
	#[inline(always)]
	pub fn outs_DX_Any32BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Output byte from memory location specified in `DS:(E)SI` or `RSI` to I/O port specified in `DX`.
	#[inline(always)]
	pub fn outs_DX_Any8BitMemory(&mut self, arg0: Option<SegmentRegister>, address_override_for_32_bit: bool)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4_if_address_override(address_override_for_32_bit);

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Output byte from memory location specified in `DS:(E)SI` or `RSI` to I/O port specified in `DX`.
	#[inline(always)]
	pub fn outsb(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6E);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Output doubleword from memory location specified in `DS:(E)SI` or `RSI` to I/O port specified in `DX`.
	#[inline(always)]
	pub fn outsd(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Output word from memory location specified in `DS:(E)SI` or `RSI` to I/O port specified in `DX`.
	#[inline(always)]
	pub fn outsw(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x6F);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of bytes in `mm2/m64` and store *unsigned* result in `mm1`.
	#[inline(always)]
	pub fn pabsb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of bytes in `mm2/m64` and store *unsigned* result in `mm1`.
	#[inline(always)]
	pub fn pabsb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of bytes in `xmm2/m128` and store *unsigned* result in `xmm1`.
	#[inline(always)]
	pub fn pabsb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of bytes in `xmm2/m128` and store *unsigned* result in `xmm1`.
	#[inline(always)]
	pub fn pabsb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of 32-bit integers in `mm2/m64` and store *unsigned* result in `mm1`.
	#[inline(always)]
	pub fn pabsd_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of 32-bit integers in `mm2/m64` and store *unsigned* result in `mm1`.
	#[inline(always)]
	pub fn pabsd_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of 32-bit integers in `xmm2/m128` and store *unsigned* result in `xmm1`.
	#[inline(always)]
	pub fn pabsd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of 32-bit integers in `xmm2/m128` and store *unsigned* result in `xmm1`.
	#[inline(always)]
	pub fn pabsd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of 16-bit integers in `mm2/m64` and store *unsigned* result in `mm1`.
	#[inline(always)]
	pub fn pabsw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of 16-bit integers in `mm2/m64` and store *unsigned* result in `mm1`.
	#[inline(always)]
	pub fn pabsw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of 16-bit integers in `xmm2/m128` and store *unsigned* result in `xmm1`.
	#[inline(always)]
	pub fn pabsw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compute the absolute value of 16-bit integers in `xmm2/m128` and store *unsigned* result in `xmm1`.
	#[inline(always)]
	pub fn pabsw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x1D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 2 packed signed doubleword integers from `mm1` and from `mm2/m64` into 4 packed signed word integers in `mm1` using signed saturation.
	#[inline(always)]
	pub fn packssdw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 2 packed signed doubleword integers from `mm1` and from `mm2/m64` into 4 packed signed word integers in `mm1` using signed saturation.
	#[inline(always)]
	pub fn packssdw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 4 packed signed doubleword integers from `xmm1` and from `xmm2/m128` into 8 packed signed word integers in `xmm1` using signed saturation.
	#[inline(always)]
	pub fn packssdw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 4 packed signed doubleword integers from `xmm1` and from `xmm2/m128` into 8 packed signed word integers in `xmm1` using signed saturation.
	#[inline(always)]
	pub fn packssdw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x6B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 4 packed signed word integers from `mm1` and from `mm2/m64` into 8 packed signed byte integers in `mm1` using signed saturation.
	#[inline(always)]
	pub fn packsswb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x63);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 4 packed signed word integers from `mm1` and from `mm2/m64` into 8 packed signed byte integers in `mm1` using signed saturation.
	#[inline(always)]
	pub fn packsswb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x63);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 8 packed signed word integers from `xmm1` and from `xmm2/m128` into 16 packed signed byte integers in `xmm1` using signed saturation.
	#[inline(always)]
	pub fn packsswb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x63);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 8 packed signed word integers from `xmm1` and from `xmm2/m128` into 16 packed signed byte integers in `xmm1` using signed saturation.
	#[inline(always)]
	pub fn packsswb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x63);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert 4 packed signed doubleword integers from `xmm1` and 4 packed signed doubleword integers from `xmm2/m128` into 8 packed unsigned word integers in `xmm1` using unsigned saturation.
	#[inline(always)]
	pub fn packusdw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x2B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Convert 4 packed signed doubleword integers from `xmm1` and 4 packed signed doubleword integers from `xmm2/m128` into 8 packed unsigned word integers in `xmm1` using unsigned saturation.
	#[inline(always)]
	pub fn packusdw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x2B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 4 signed word integers from `mm` and 4 signed word integers from `mm/m64` into 8 unsigned byte integers in `mm` using unsigned saturation.
	#[inline(always)]
	pub fn packuswb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x67);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 4 signed word integers from `mm` and 4 signed word integers from `mm/m64` into 8 unsigned byte integers in `mm` using unsigned saturation.
	#[inline(always)]
	pub fn packuswb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x67);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 8 signed word integers from `xmm1` and 8 signed word integers from `xmm2/m128` into 16 unsigned byte integers in `xmm1` using unsigned saturation.
	#[inline(always)]
	pub fn packuswb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x67);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Converts 8 signed word integers from `xmm1` and 8 signed word integers from `xmm2/m128` into 16 unsigned byte integers in `xmm1` using unsigned saturation.
	#[inline(always)]
	pub fn packuswb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x67);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed byte integers from `mm/m64` and `mm`.
	#[inline(always)]
	pub fn paddb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed byte integers from `mm/m64` and `mm`.
	#[inline(always)]
	pub fn paddb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed byte integers from `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn paddb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed byte integers from `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn paddb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed doubleword integers from `mm/m64` and `mm`.
	#[inline(always)]
	pub fn paddd_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed doubleword integers from `mm/m64` and `mm`.
	#[inline(always)]
	pub fn paddd_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed doubleword integers from `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn paddd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed doubleword integers from `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn paddd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add quadword integer `mm2/m64` to `mm1`.
	#[inline(always)]
	pub fn paddq_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add quadword integer `mm2/m64` to `mm1`.
	#[inline(always)]
	pub fn paddq_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed quadword integers `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn paddq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed quadword integers `xmm2/m128` to `xmm1`.
	#[inline(always)]
	pub fn paddq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed signed byte integers from `mm/m64` and `mm` and saturate the results.
	#[inline(always)]
	pub fn paddsb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed signed byte integers from `mm/m64` and `mm` and saturate the results.
	#[inline(always)]
	pub fn paddsb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed signed byte integers from `xmm2/m128` and `xmm1` saturate the results.
	#[inline(always)]
	pub fn paddsb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed signed byte integers from `xmm2/m128` and `xmm1` saturate the results.
	#[inline(always)]
	pub fn paddsb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed signed word integers from `mm/m64` and `mm` and saturate the results.
	#[inline(always)]
	pub fn paddsw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xED);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed signed word integers from `mm/m64` and `mm` and saturate the results.
	#[inline(always)]
	pub fn paddsw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xED);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed signed word integers from `xmm2/m128` and `xmm1` and saturate the results.
	#[inline(always)]
	pub fn paddsw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xED);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed signed word integers from `xmm2/m128` and `xmm1` and saturate the results.
	#[inline(always)]
	pub fn paddsw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xED);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed unsigned byte integers from `mm/m64` and `mm` and saturate the results.
	#[inline(always)]
	pub fn paddusb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed unsigned byte integers from `mm/m64` and `mm` and saturate the results.
	#[inline(always)]
	pub fn paddusb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed unsigned byte integers from `xmm2/m128` and `xmm1` saturate the results.
	#[inline(always)]
	pub fn paddusb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed unsigned byte integers from `xmm2/m128` and `xmm1` saturate the results.
	#[inline(always)]
	pub fn paddusb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDC);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed unsigned word integers from `mm/m64` and `mm` and saturate the results.
	#[inline(always)]
	pub fn paddusw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed unsigned word integers from `mm/m64` and `mm` and saturate the results.
	#[inline(always)]
	pub fn paddusw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed unsigned word integers from `xmm2/m128` to `xmm1` and saturate the results.
	#[inline(always)]
	pub fn paddusw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed unsigned word integers from `xmm2/m128` to `xmm1` and saturate the results.
	#[inline(always)]
	pub fn paddusw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed word integers from `mm/m64` and `mm`.
	#[inline(always)]
	pub fn paddw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed word integers from `mm/m64` and `mm`.
	#[inline(always)]
	pub fn paddw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed word integers from `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn paddw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add packed word integers from `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn paddw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFD);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Concatenate destination and source operands, extract byte-aligned result shifted to the right by constant value in `imm8` into `mm1`.
	#[inline(always)]
	pub fn palignr_MMRegister_Any64BitMemory_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Any64BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0F);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Concatenate destination and source operands, extract byte-aligned result shifted to the right by constant value in `imm8` into `mm1`.
	#[inline(always)]
	pub fn palignr_MMRegister_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: MMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0F);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Concatenate destination and source operands, extract byte-aligned result shifted to the right by constant value in `imm8` into `xmm1`.
	#[inline(always)]
	pub fn palignr_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0F);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Concatenate destination and source operands, extract byte-aligned result shifted to the right by constant value in `imm8` into `xmm1`.
	#[inline(always)]
	pub fn palignr_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0F);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Bitwise AND `mm/m64` and `mm`.
	#[inline(always)]
	pub fn pand_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise AND `mm/m64` and `mm`.
	#[inline(always)]
	pub fn pand_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise AND of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn pand_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise AND of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn pand_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise AND NOT of `mm/m64` and `mm`.
	#[inline(always)]
	pub fn pandn_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise AND NOT of `mm/m64` and `mm`.
	#[inline(always)]
	pub fn pandn_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise AND NOT of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn pandn_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise AND NOT of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn pandn_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDF);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Gives hint to processor that improves performance of spin-wait loops.
	#[inline(always)]
	pub fn pause(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		// No `REX` prefix.

		self.opcode_1(0x90);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Average packed unsigned byte integers from `mm2/m64` and `mm1` with rounding.
	#[inline(always)]
	pub fn pavgb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Average packed unsigned byte integers from `mm2/m64` and `mm1` with rounding.
	#[inline(always)]
	pub fn pavgb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Average packed unsigned byte integers from `xmm2/m128` and `xmm1` with rounding.
	#[inline(always)]
	pub fn pavgb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Average packed unsigned byte integers from `xmm2/m128` and `xmm1` with rounding.
	#[inline(always)]
	pub fn pavgb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE0);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Average packed unsigned word integers from `mm2/m64` and `mm1` with rounding.
	#[inline(always)]
	pub fn pavgw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Average packed unsigned word integers from `mm2/m64` and `mm1` with rounding.
	#[inline(always)]
	pub fn pavgw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Average packed unsigned word integers from `xmm2/m128` and `xmm1` with rounding.
	#[inline(always)]
	pub fn pavgw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Average packed unsigned word integers from `xmm2/m128` and `xmm1` with rounding.
	#[inline(always)]
	pub fn pavgw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Select byte values from `xmm1` and `xmm2/m128` from mask specified in the high bit of each byte in `XMM0` and store the values into `xmm1`.
	#[inline(always)]
	pub fn pblendvb_XMMRegister_Any128BitMemory_XMMRegister0(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Select byte values from `xmm1` and `xmm2/m128` from mask specified in the high bit of each byte in `XMM0` and store the values into `xmm1`.
	#[inline(always)]
	pub fn pblendvb_XMMRegister_XMMRegister_XMMRegister0(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x10);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Select words from `xmm1` and `xmm2/m128` from mask specified in `imm8` and store the values into `xmm1`.
	#[inline(always)]
	pub fn pblendw_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0E);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Select words from `xmm1` and `xmm2/m128` from mask specified in `imm8` and store the values into `xmm1`.
	#[inline(always)]
	pub fn pblendw_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x0E);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Carry-less multiplication of one quadword of `xmm1` by one quadword of `xmm2/m128`, stores the 128-bit result in `xmm1`.
	///
	/// The immediate is used to determine which quadwords of `xmm1` and `xmm2/m128` should be used.
	#[inline(always)]
	pub fn pclmulqdq_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x44);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Carry-less multiplication of one quadword of `xmm1` by one quadword of `xmm2/m128`, stores the 128-bit result in `xmm1`.
	///
	/// The immediate is used to determine which quadwords of `xmm1` and `xmm2/m128` should be used.
	#[inline(always)]
	pub fn pclmulqdq_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x44);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Compare packed bytes in `mm/m64` and `mm` for equality.
	#[inline(always)]
	pub fn pcmpeqb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x74);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed bytes in `mm/m64` and `mm` for equality.
	#[inline(always)]
	pub fn pcmpeqb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x74);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed bytes in `xmm2/m128` and `xmm1` for equality.
	#[inline(always)]
	pub fn pcmpeqb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x74);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed bytes in `xmm2/m128` and `xmm1` for equality.
	#[inline(always)]
	pub fn pcmpeqb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x74);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed doublewords in `mm/m64` and `mm` for equality.
	#[inline(always)]
	pub fn pcmpeqd_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x76);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed doublewords in `mm/m64` and `mm` for equality.
	#[inline(always)]
	pub fn pcmpeqd_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x76);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed doublewords in `xmm2/m128` and `xmm1` for equality.
	#[inline(always)]
	pub fn pcmpeqd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x76);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed doublewords in `xmm2/m128` and `xmm1` for equality.
	#[inline(always)]
	pub fn pcmpeqd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x76);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed qwords in `xmm2/m128` and `xmm1` for equality.
	#[inline(always)]
	pub fn pcmpeqq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x29);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed qwords in `xmm2/m128` and `xmm1` for equality.
	#[inline(always)]
	pub fn pcmpeqq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x29);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed words in `mm/m64` and `mm` for equality.
	#[inline(always)]
	pub fn pcmpeqw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x75);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed words in `mm/m64` and `mm` for equality.
	#[inline(always)]
	pub fn pcmpeqw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x75);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed words in `xmm2/m128` and `xmm1` for equality.
	#[inline(always)]
	pub fn pcmpeqw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x75);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed words in `xmm2/m128` and `xmm1` for equality.
	#[inline(always)]
	pub fn pcmpeqw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x75);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform a packed comparison of string data with explicit lengths, generating an index, and storing the result in `ECX`.
	#[inline(always)]
	pub fn pcmpestri_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x61);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Perform a packed comparison of string data with explicit lengths, generating an index, and storing the result in `ECX`.
	#[inline(always)]
	pub fn pcmpestri_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x61);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Perform a packed comparison of string data with explicit lengths, generating a mask, and storing the result in `XMM0`.
	#[inline(always)]
	pub fn pcmpestrm_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x60);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Perform a packed comparison of string data with explicit lengths, generating a mask, and storing the result in `XMM0`.
	#[inline(always)]
	pub fn pcmpestrm_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x60);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Compare packed signed byte integers in `mm` and `mm/m64` for greater than.
	#[inline(always)]
	pub fn pcmpgtb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x64);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed byte integers in `mm` and `mm/m64` for greater than.
	#[inline(always)]
	pub fn pcmpgtb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x64);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed byte integers in `xmm1` and `xmm2/m128` for greater than.
	#[inline(always)]
	pub fn pcmpgtb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x64);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed byte integers in `xmm1` and `xmm2/m128` for greater than.
	#[inline(always)]
	pub fn pcmpgtb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x64);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed doubleword integers in `mm` and `mm/m64` for greater than.
	#[inline(always)]
	pub fn pcmpgtd_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x66);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed doubleword integers in `mm` and `mm/m64` for greater than.
	#[inline(always)]
	pub fn pcmpgtd_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x66);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed doubleword integers in `xmm1` and `xmm2/m128` for greater than.
	#[inline(always)]
	pub fn pcmpgtd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x66);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed doubleword integers in `xmm1` and `xmm2/m128` for greater than.
	#[inline(always)]
	pub fn pcmpgtd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x66);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed qwords in `xmm2/m128` and `xmm1` for greater than.
	#[inline(always)]
	pub fn pcmpgtq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x37);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed qwords in `xmm2/m128` and `xmm1` for greater than.
	#[inline(always)]
	pub fn pcmpgtq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x37);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed word integers in `mm` and `mm/m64` for greater than.
	#[inline(always)]
	pub fn pcmpgtw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x65);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed word integers in `mm` and `mm/m64` for greater than.
	#[inline(always)]
	pub fn pcmpgtw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x65);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed word integers in `xmm1` and `xmm2/m128` for greater than.
	#[inline(always)]
	pub fn pcmpgtw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x65);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed word integers in `xmm1` and `xmm2/m128` for greater than.
	#[inline(always)]
	pub fn pcmpgtw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x65);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Perform a packed comparison of string data with implicit lengths, generating an index, and storing the result in `ECX`.
	#[inline(always)]
	pub fn pcmpistri_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x63);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Perform a packed comparison of string data with implicit lengths, generating an index, and storing the result in `ECX`.
	#[inline(always)]
	pub fn pcmpistri_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x63);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Perform a packed comparison of string data with implicit lengths, generating a mask, and storing the result in `XMM0`.
	#[inline(always)]
	pub fn pcmpistrm_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x62);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Perform a packed comparison of string data with implicit lengths, generating a mask, and storing the result in `XMM0`.
	#[inline(always)]
	pub fn pcmpistrm_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x62);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Parallel deposit of bits from `r32b` using mask in `r/m32`, result is written to `r32a`.
	#[inline(always)]
	pub fn pdep_Register32Bit_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Any32BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg2);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg2);

		self.vex_7(0x02, 0x0, 0x3, 0x0, arg1, arg2, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Parallel deposit of bits from `r32b` using mask in `r/m32`, result is written to `r32a`.
	#[inline(always)]
	pub fn pdep_Register32Bit_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x3, 0x0, arg1, arg2, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Parallel deposit of bits from `r64b` using mask in `r/m64`, result is written to `r64a`.
	#[inline(always)]
	pub fn pdep_Register64Bit_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Any64BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg2);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg2);

		self.vex_7(0x02, 0x0, 0x3, 0x1, arg1, arg2, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Parallel deposit of bits from `r64b` using mask in `r/m64`, result is written to `r64a`.
	#[inline(always)]
	pub fn pdep_Register64Bit_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x3, 0x1, arg1, arg2, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Parallel extract of bits from `r32b` using mask in `r/m32`, result is written to `r32a`.
	#[inline(always)]
	pub fn pext_Register32Bit_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Any32BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg2);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg2);

		self.vex_7(0x02, 0x0, 0x2, 0x0, arg1, arg2, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Parallel extract of bits from `r32b` using mask in `r/m32`, result is written to `r32a`.
	#[inline(always)]
	pub fn pext_Register32Bit_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit, arg2: Register32Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x2, 0x0, arg1, arg2, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Parallel extract of bits from `r64b` using mask in `r/m64`, result is written to `r64a`.
	#[inline(always)]
	pub fn pext_Register64Bit_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Any64BitMemory)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		self.prefix_group2(arg2);

		// Prefix Group 3 is #UD for VEX.

		self.prefix_group4(arg2);

		self.vex_7(0x02, 0x0, 0x2, 0x1, arg1, arg2, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Parallel extract of bits from `r64b` using mask in `r/m64`, result is written to `r64a`.
	#[inline(always)]
	pub fn pext_Register64Bit_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit, arg2: Register64Bit)
	{
		// This is a VEX encoded instruction.

		// Prefix Group 1 is #UD for VEX.

		// No prefix group 2.

		// Prefix Group 3 is #UD for VEX.

		// No prefix group 4.

		self.vex_7(0x02, 0x0, 0x2, 0x1, arg1, arg2, arg0);

		self.opcode_1(0xF5);

		self.mod_rm_sib(arg2, arg0);

		// No displacement or immediate.

		// No label displacement.

		// No VEX immediate.
	}

	/// Extract a byte integer value from `xmm2` at the source byte offset specified by `imm8` into `rreg` or `m8`.
	///
	/// The upper bits of `r32` or `r64` are zeroed.
	#[inline(always)]
	pub fn pextrb_Any8BitMemory_XMMRegister_Immediate8Bit(&mut self, arg0: Any8BitMemory, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x14);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract a byte integer value from `xmm2` at the source byte offset specified by `imm8` into `rreg` or `m8`.
	///
	/// The upper bits of `r32` or `r64` are zeroed.
	#[inline(always)]
	pub fn pextrb_Register32Bit_XMMRegister_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x14);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract a byte integer value from `xmm2` at the source byte offset specified by `imm8` into `rreg` or `m8`.
	///
	/// The upper bits of `r32` or `r64` are zeroed.
	#[inline(always)]
	pub fn pextrb_Register64Bit_XMMRegister_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x14);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract a dword integer value from `xmm2` at the source dword offset specified by `imm8` into `r/m32`.
	#[inline(always)]
	pub fn pextrd_Any32BitMemory_XMMRegister_Immediate8Bit(&mut self, arg0: Any32BitMemory, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x16);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract a dword integer value from `xmm2` at the source dword offset specified by `imm8` into `r/m32`.
	#[inline(always)]
	pub fn pextrd_Register32Bit_XMMRegister_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x16);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract a qword integer value from `xmm2` at the source qword offset specified by `imm8` into `r/m64`.
	#[inline(always)]
	pub fn pextrq_Any64BitMemory_XMMRegister_Immediate8Bit(&mut self, arg0: Any64BitMemory, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_3(0x0F, 0x3A, 0x16);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract a qword integer value from `xmm2` at the source qword offset specified by `imm8` into `r/m64`.
	#[inline(always)]
	pub fn pextrq_Register64Bit_XMMRegister_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, Self::REX_W);

		self.opcode_3(0x0F, 0x3A, 0x16);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract the word specified by `imm8` from `xmm` and copy it to lowest 16 bits of `reg` or `m16`.
	///
	/// Zero-extend the result in the destination, `r32` or `r64`.
	#[inline(always)]
	pub fn pextrw_Any16BitMemory_XMMRegister_Immediate8Bit(&mut self, arg0: Any16BitMemory, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x15);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract the word specified by `imm8` from `mm` and move it to `reg`, bits 15-0.
	///
	/// The upper bits of `r32` or `r64` is zeroed.
	#[inline(always)]
	pub fn pextrw_Register32Bit_MMRegister_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: MMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC5);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract the word specified by `imm8` from `xmm` and move it to `reg`, bits 15-0.
	///
	/// The upper bits of `r32` or `r64` is zeroed.
	#[inline(always)]
	pub fn pextrw_Register32Bit_XMMRegister_Immediate8Bit(&mut self, arg0: Register32Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC5);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}
	
	/// Extract the word specified by `imm8` from `xmm` and move it to `reg`, bits 15-0.
	///
	/// The upper bits of `r32` or `r64` is zeroed.
	/// Extract the word specified by `imm8` from `xmm` and move it to `reg`, bits 15-0.
	///
	/// The upper bits of `r32` or `r64` is zeroed.
	#[inline(always)]
	pub fn pextrw_Register32Bit_XMMRegister_Immediate8Bit_REX_W(&mut self, arg0: Register32Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x15);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract the word specified by `imm8` from `mm` and move it to `reg`, bits 15-0.
	///
	/// The upper bits of `r32` or `r64` is zeroed.
	#[inline(always)]
	pub fn pextrw_Register64Bit_MMRegister_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: MMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC5);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Extract the word specified by `imm8` from `xmm` and move it to `reg`, bits 15-0.
	///
	/// The upper bits of `r32` or `r64` is zeroed.
	#[inline(always)]
	pub fn pextrw_Register64Bit_XMMRegister_Immediate8Bit(&mut self, arg0: Register64Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC5);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}
	
	/// Extract the word specified by `imm8` from `xmm` and move it to `reg`, bits 15-0.
	///
	/// The upper bits of `r32` or `r64` is zeroed.
	#[inline(always)]
	pub fn pextrw_Register64Bit_XMMRegister_Immediate8Bit_REX_W(&mut self, arg0: Register64Bit, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg0, arg1, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x15);

		self.mod_rm_sib(arg0, arg1);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Add 32-bit integers horizontally, pack to `mm1`.
	#[inline(always)]
	pub fn phaddd_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 32-bit integers horizontally, pack to `mm1`.
	#[inline(always)]
	pub fn phaddd_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 32-bit integers horizontally, pack to `xmm1`.
	#[inline(always)]
	pub fn phaddd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 32-bit integers horizontally, pack to `xmm1`.
	#[inline(always)]
	pub fn phaddd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x02);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 16-bit signed integers horizontally, pack saturated integers to `mm1`.
	#[inline(always)]
	pub fn phaddsw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 16-bit signed integers horizontally, pack saturated integers to `mm1`.
	#[inline(always)]
	pub fn phaddsw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 16-bit signed integers horizontally, pack saturated integers to `xmm1`.
	#[inline(always)]
	pub fn phaddsw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 16-bit signed integers horizontally, pack saturated integers to `xmm1`.
	#[inline(always)]
	pub fn phaddsw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x03);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 16-bit integers horizontally, pack to `mm1`.
	#[inline(always)]
	pub fn phaddw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x01);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 16-bit integers horizontally, pack to `mm1`.
	#[inline(always)]
	pub fn phaddw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x01);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 16-bit integers horizontally, pack to `xmm1`.
	#[inline(always)]
	pub fn phaddw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x01);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Add 16-bit integers horizontally, pack to `xmm1`.
	#[inline(always)]
	pub fn phaddw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x01);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Find the minimum unsigned word in `xmm2/m128` and place its value in the low word of `xmm1` and its index in the second-lowest word of `xmm1`.
	#[inline(always)]
	pub fn phminposuw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x41);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Find the minimum unsigned word in `xmm2/m128` and place its value in the low word of `xmm1` and its index in the second-lowest word of `xmm1`.
	#[inline(always)]
	pub fn phminposuw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x41);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 32-bit signed integers horizontally, pack to `mm1`.
	#[inline(always)]
	pub fn phsubd_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x06);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 32-bit signed integers horizontally, pack to `mm1`.
	#[inline(always)]
	pub fn phsubd_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x06);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 32-bit signed integers horizontally, pack to `xmm1`.
	#[inline(always)]
	pub fn phsubd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x06);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 32-bit signed integers horizontally, pack to `xmm1`.
	#[inline(always)]
	pub fn phsubd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x06);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 16-bit signed integer horizontally, pack saturated integers to `mm1`.
	#[inline(always)]
	pub fn phsubsw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x07);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 16-bit signed integer horizontally, pack saturated integers to `mm1`.
	#[inline(always)]
	pub fn phsubsw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x07);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 16-bit signed integer horizontally, pack saturated integers to `xmm1`.
	#[inline(always)]
	pub fn phsubsw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x07);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 16-bit signed integer horizontally, pack saturated integers to `xmm1`.
	#[inline(always)]
	pub fn phsubsw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x07);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 16-bit signed integers horizontally, pack to `mm1`.
	#[inline(always)]
	pub fn phsubw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x05);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 16-bit signed integers horizontally, pack to `mm1`.
	#[inline(always)]
	pub fn phsubw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x05);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 16-bit signed integers horizontally, pack to `XMM1`.
	#[inline(always)]
	pub fn phsubw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x05);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract 16-bit signed integers horizontally, pack to `xmm1`.
	#[inline(always)]
	pub fn phsubw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x05);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Insert a byte integer value from `r32`/m8 into `xmm1` at the destination element in `xmm1` specified by `imm8`.
	#[inline(always)]
	pub fn pinsrb_XMMRegister_Any8BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any8BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x20);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Insert a byte integer value from `r32`/m8 into `xmm1` at the destination element in `xmm1` specified by `imm8`.
	#[inline(always)]
	pub fn pinsrb_XMMRegister_Register32Bit_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Register32Bit, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x20);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Insert a dword integer value from `r/m32` into the `xmm1` at the destination element specified by `imm8`.
	#[inline(always)]
	pub fn pinsrd_XMMRegister_Any32BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any32BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x22);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Insert a dword integer value from `r/m32` into the `xmm1` at the destination element specified by `imm8`.
	#[inline(always)]
	pub fn pinsrd_XMMRegister_Register32Bit_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Register32Bit, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x3A, 0x22);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Insert the low word from `r32` or from `m16` into `mm` at the word position specified by `imm8`.
	#[inline(always)]
	pub fn pinsrw_MMRegister_Any16BitMemory_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Any16BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC4);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Insert the low word from `r32` or from `m16` into `mm` at the word position specified by `imm8`.
	#[inline(always)]
	pub fn pinsrw_MMRegister_Register32Bit_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Register32Bit, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC4);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Move the low word of `r32` or from `m16` into `xmm` at the word position specified by `imm8`.
	#[inline(always)]
	pub fn pinsrw_XMMRegister_Any16BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any16BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC4);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Move the low word of `r32` or from `m16` into `xmm` at the word position specified by `imm8`.
	#[inline(always)]
	pub fn pinsrw_XMMRegister_Register32Bit_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Register32Bit, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xC4);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Multiply signed and unsigned bytes, add horizontal pair of signed words, pack saturated signed-words to `mm1`.
	#[inline(always)]
	pub fn pmaddubsw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x04);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply signed and unsigned bytes, add horizontal pair of signed words, pack saturated signed-words to `mm1`.
	#[inline(always)]
	pub fn pmaddubsw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x04);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply signed and unsigned bytes, add horizontal pair of signed words, pack saturated signed-words to `xmm1`.
	#[inline(always)]
	pub fn pmaddubsw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x04);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply signed and unsigned bytes, add horizontal pair of signed words, pack saturated signed-words to `xmm1`.
	#[inline(always)]
	pub fn pmaddubsw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x04);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed words in `mm` by the packed words in mm/m64, add adjacent doubleword results, and store in `mm`.
	#[inline(always)]
	pub fn pmaddwd_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed words in `mm` by the packed words in mm/m64, add adjacent doubleword results, and store in `mm`.
	#[inline(always)]
	pub fn pmaddwd_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed word integers in `xmm1` by the packed word integers in `xmm2/m128`, add adjacent doubleword results, and store in `xmm1`.
	#[inline(always)]
	pub fn pmaddwd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed word integers in `xmm1` by the packed word integers in `xmm2/m128`, add adjacent doubleword results, and store in `xmm1`.
	#[inline(always)]
	pub fn pmaddwd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed byte integers in `xmm1` and `xmm2/m128` and store packed maximum values in `xmm1`.
	#[inline(always)]
	pub fn pmaxsb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed byte integers in `xmm1` and `xmm2/m128` and store packed maximum values in `xmm1`.
	#[inline(always)]
	pub fn pmaxsb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3C);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed dword integers in `xmm1` and `xmm2/m128` and store packed maximum values in `xmm1`.
	#[inline(always)]
	pub fn pmaxsd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed dword integers in `xmm1` and `xmm2/m128` and store packed maximum values in `xmm1`.
	#[inline(always)]
	pub fn pmaxsd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3D);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare signed word integers in `mm2/m64` and `mm1` and return maximum values.
	#[inline(always)]
	pub fn pmaxsw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare signed word integers in `mm2/m64` and `mm1` and return maximum values.
	#[inline(always)]
	pub fn pmaxsw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare signed word integers in `xmm2/m128` and `xmm1` and return maximum values.
	#[inline(always)]
	pub fn pmaxsw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare signed word integers in `xmm2/m128` and `xmm1` and return maximum values.
	#[inline(always)]
	pub fn pmaxsw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare unsigned byte integers in `mm2/m64` and `mm1` and returns maximum values.
	#[inline(always)]
	pub fn pmaxub_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare unsigned byte integers in `mm2/m64` and `mm1` and returns maximum values.
	#[inline(always)]
	pub fn pmaxub_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare unsigned byte integers in `xmm2/m128` and `xmm1` and returns maximum values.
	#[inline(always)]
	pub fn pmaxub_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare unsigned byte integers in `xmm2/m128` and `xmm1` and returns maximum values.
	#[inline(always)]
	pub fn pmaxub_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDE);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed unsigned dword integers in `xmm1` and `xmm2/m128` and store packed maximum values in `xmm1`.
	#[inline(always)]
	pub fn pmaxud_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed unsigned dword integers in `xmm1` and `xmm2/m128` and store packed maximum values in `xmm1`.
	#[inline(always)]
	pub fn pmaxud_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3F);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed unsigned word integers in `xmm1` and `xmm2/m128` and store packed maximum values in `xmm1`.
	#[inline(always)]
	pub fn pmaxuw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed unsigned word integers in `xmm1` and `xmm2/m128` and store packed maximum values in `xmm1`.
	#[inline(always)]
	pub fn pmaxuw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3E);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed byte integers in `xmm1` and `xmm2/m128` and store packed minimum values in `xmm1`.
	#[inline(always)]
	pub fn pminsb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x38);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed byte integers in `xmm1` and `xmm2/m128` and store packed minimum values in `xmm1`.
	#[inline(always)]
	pub fn pminsb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x38);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed dword integers in `xmm1` and `xmm2/m128` and store packed minimum values in `xmm1`.
	#[inline(always)]
	pub fn pminsd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x39);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed signed dword integers in `xmm1` and `xmm2/m128` and store packed minimum values in `xmm1`.
	#[inline(always)]
	pub fn pminsd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x39);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare signed word integers in `mm2/m64` and `mm1` and return minimum values.
	#[inline(always)]
	pub fn pminsw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare signed word integers in `mm2/m64` and `mm1` and return minimum values.
	#[inline(always)]
	pub fn pminsw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare signed word integers in `xmm2/m128` and `xmm1` and return minimum values.
	#[inline(always)]
	pub fn pminsw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare signed word integers in `xmm2/m128` and `xmm1` and return minimum values.
	#[inline(always)]
	pub fn pminsw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare unsigned byte integers in `mm2/m64` and `mm1` and returns minimum values.
	#[inline(always)]
	pub fn pminub_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare unsigned byte integers in `mm2/m64` and `mm1` and returns minimum values.
	#[inline(always)]
	pub fn pminub_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare unsigned byte integers in `xmm2/m128` and `xmm1` and returns minimum values.
	#[inline(always)]
	pub fn pminub_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare unsigned byte integers in `xmm2/m128` and `xmm1` and returns minimum values.
	#[inline(always)]
	pub fn pminub_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xDA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed unsigned dword integers in `xmm1` and `xmm2/m128` and store packed minimum values in `xmm1`.
	#[inline(always)]
	pub fn pminud_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed unsigned dword integers in `xmm1` and `xmm2/m128` and store packed minimum values in `xmm1`.
	#[inline(always)]
	pub fn pminud_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed unsigned word integers in `xmm1` and `xmm2/m128` and store packed minimum values in `xmm1`.
	#[inline(always)]
	pub fn pminuw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Compare packed unsigned word integers in `xmm1` and `xmm2/m128` and store packed minimum values in `xmm1`.
	#[inline(always)]
	pub fn pminuw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x3A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move a byte mask of `mm` to register.
	///
	/// The upper bits of `r32` or `r64` are zeroed.
	#[inline(always)]
	pub fn pmovmskb_Register32Bit_MMRegister(&mut self, arg0: Register32Bit, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move a byte mask of `xmm` to register.
	///
	/// The upper bits of `r32` or `r64` are zeroed.
	#[inline(always)]
	pub fn pmovmskb_Register32Bit_XMMRegister(&mut self, arg0: Register32Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move a byte mask of `mm` to register.
	///
	/// The upper bits of `r32` or `r64` are zeroed.
	#[inline(always)]
	pub fn pmovmskb_Register64Bit_MMRegister(&mut self, arg0: Register64Bit, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move a byte mask of `xmm` to register.
	///
	/// The upper bits of `r32` or `r64` are zeroed.
	#[inline(always)]
	pub fn pmovmskb_Register64Bit_XMMRegister(&mut self, arg0: Register64Bit, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD7);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 4 packed signed 8-bit integers in the low 4 bytes of `xmm2/m32` to 4 packed signed 32-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxbd_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x21);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 4 packed signed 8-bit integers in the low 4 bytes of `xmm2/m32` to 4 packed signed 32-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxbd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x21);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 2 packed signed 8-bit integers in the low 2 bytes of `xmm2`.m16 to 2 packed signed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxbq_XMMRegister_Any16BitMemory(&mut self, arg0: XMMRegister, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x22);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 2 packed signed 8-bit integers in the low 2 bytes of `xmm2`.m16 to 2 packed signed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxbq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x22);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 8 packed signed 8-bit integers in the low 8 bytes of `xmm2/m64` to 8 packed signed 16-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxbw_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x20);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 8 packed signed 8-bit integers in the low 8 bytes of `xmm2/m64` to 8 packed signed 16-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxbw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x20);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 2 packed signed 32-bit integers in the low 8 bytes of `xmm2/m64` to 2 packed signed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxdq_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x25);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 2 packed signed 32-bit integers in the low 8 bytes of `xmm2/m64` to 2 packed signed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxdq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x25);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 4 packed signed 16-bit integers in the low 8 bytes of `xmm2/m64` to 4 packed signed 32-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxwd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x23);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 4 packed signed 16-bit integers in the low 8 bytes of `xmm2/m64` to 4 packed signed 32-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxwd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x23);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 2 packed signed 16-bit integers in the low 4 bytes of `xmm2/m32` to 2 packed signed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxwq_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x24);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Sign extend 2 packed signed 16-bit integers in the low 4 bytes of `xmm2/m32` to 2 packed signed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovsxwq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x24);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 4 packed 8-bit integers in the low 4 bytes of `xmm2/m32` to 4 packed 32-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxbd_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x31);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 4 packed 8-bit integers in the low 4 bytes of `xmm2/m32` to 4 packed 32-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxbd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x31);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 2 packed 8-bit integers in the low 2 bytes of `xmm2`.m16 to 2 packed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxbq_XMMRegister_Any16BitMemory(&mut self, arg0: XMMRegister, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x32);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 2 packed 8-bit integers in the low 2 bytes of `xmm2`.m16 to 2 packed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxbq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x32);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 8 packed 8-bit integers in the low 8 bytes of `xmm2/m64` to 8 packed 16-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxbw_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x30);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 8 packed 8-bit integers in the low 8 bytes of `xmm2/m64` to 8 packed 16-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxbw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x30);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 2 packed 32-bit integers in the low 8 bytes of `xmm2/m64` to 2 packed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxdq_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x35);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 2 packed 32-bit integers in the low 8 bytes of `xmm2/m64` to 2 packed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxdq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x35);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 4 packed 16-bit integers in the low 8 bytes of `xmm2/m64` to 4 packed 32-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxwd_XMMRegister_Any64BitMemory(&mut self, arg0: XMMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x33);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 4 packed 16-bit integers in the low 8 bytes of `xmm2/m64` to 4 packed 32-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxwd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x33);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 2 packed 16-bit integers in the low 4 bytes of `xmm2/m32` to 2 packed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxwq_XMMRegister_Any32BitMemory(&mut self, arg0: XMMRegister, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x34);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Zero extend 2 packed 16-bit integers in the low 4 bytes of `xmm2/m32` to 2 packed 64-bit integers in `xmm1`.
	#[inline(always)]
	pub fn pmovzxwq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x34);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed dword integers in `xmm1` and `xmm2/m128` and store the quadword product in `xmm1`.
	#[inline(always)]
	pub fn pmuldq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x28);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed dword integers in `xmm1` and `xmm2/m128` and store the quadword product in `xmm1`.
	#[inline(always)]
	pub fn pmuldq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x28);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply 16-bit signed words, scale and round signed doublewords, pack high 16 bits to `mm1`.
	#[inline(always)]
	pub fn pmulhrsw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply 16-bit signed words, scale and round signed doublewords, pack high 16 bits to `mm1`.
	#[inline(always)]
	pub fn pmulhrsw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply 16-bit signed words, scale and round signed doublewords, pack high 16 bits to `xmm1`.
	#[inline(always)]
	pub fn pmulhrsw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply 16-bit signed words, scale and round signed doublewords, pack high 16 bits to `xmm1`.
	#[inline(always)]
	pub fn pmulhrsw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x0B);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed unsigned word integers in `mm1` register and mm2/m64, and store the high 16 bits of the results in `mm1`.
	#[inline(always)]
	pub fn pmulhuw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed unsigned word integers in `mm1` register and mm2/m64, and store the high 16 bits of the results in `mm1`.
	#[inline(always)]
	pub fn pmulhuw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed unsigned word integers in `xmm1` and `xmm2/m128`, and store the high 16 bits of the results in `xmm1`.
	#[inline(always)]
	pub fn pmulhuw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed unsigned word integers in `xmm1` and `xmm2/m128`, and store the high 16 bits of the results in `xmm1`.
	#[inline(always)]
	pub fn pmulhuw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed word integers in `mm1` register and mm2/m64, and store the high 16 bits of the results in `mm1`.
	#[inline(always)]
	pub fn pmulhw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed word integers in `mm1` register and mm2/m64, and store the high 16 bits of the results in `mm1`.
	#[inline(always)]
	pub fn pmulhw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed word integers in `xmm1` and `xmm2/m128`, and store the high 16 bits of the results in `xmm1`.
	#[inline(always)]
	pub fn pmulhw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed word integers in `xmm1` and `xmm2/m128`, and store the high 16 bits of the results in `xmm1`.
	#[inline(always)]
	pub fn pmulhw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed dword signed integers in `xmm1` and `xmm2/m128` and store the low 32 bits of each product in `xmm1`.
	#[inline(always)]
	pub fn pmulld_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x40);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed dword signed integers in `xmm1` and `xmm2/m128` and store the low 32 bits of each product in `xmm1`.
	#[inline(always)]
	pub fn pmulld_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x40);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed word integers in `mm1` register and mm2/m64, and store the low 16 bits of the results in `mm1`.
	#[inline(always)]
	pub fn pmullw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed word integers in `mm1` register and mm2/m64, and store the low 16 bits of the results in `mm1`.
	#[inline(always)]
	pub fn pmullw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed word integers in `xmm1` and `xmm2/m128`, and store the low 16 bits of the results in `xmm1`.
	#[inline(always)]
	pub fn pmullw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply the packed signed word integers in `xmm1` and `xmm2/m128`, and store the low 16 bits of the results in `xmm1`.
	#[inline(always)]
	pub fn pmullw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD5);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply unsigned doubleword integer in `mm1` by unsigned doubleword integer in mm2/m64, and store the quadword result in `mm1`.
	#[inline(always)]
	pub fn pmuludq_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply unsigned doubleword integer in `mm1` by unsigned doubleword integer in mm2/m64, and store the quadword result in `mm1`.
	#[inline(always)]
	pub fn pmuludq_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply packed unsigned doubleword integers in `xmm1` by packed unsigned doubleword integers in `xmm2/m128`, and store the quadword results in `xmm1`.
	#[inline(always)]
	pub fn pmuludq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Multiply packed unsigned doubleword integers in `xmm1` by packed unsigned doubleword integers in `xmm2/m128`, and store the quadword results in `xmm1`.
	#[inline(always)]
	pub fn pmuludq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF4);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `FS` and increment stack pointer by 64 bits.
	#[inline(always)]
	pub fn pop_FS(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0xA1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `FS` and increment stack pointer by 16 bits.
	#[inline(always)]
	pub fn pop_FS_Prefix66(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0xA1);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `GS` and increment stack pointer by 64 bits.
	#[inline(always)]
	pub fn pop_GS(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0xA9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `GS` and increment stack pointer by 16 bits.
	#[inline(always)]
	pub fn pop_GS_Prefix66(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_2(0x0F, 0xA9);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `m16` and increment stack pointer.
	#[inline(always)]
	pub fn pop_Any16BitMemory(&mut self, arg0: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x8F);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `m64` and increment stack pointer.
	///
	/// Cannot encode 32-bit operand size.
	#[inline(always)]
	pub fn pop_Any64BitMemory(&mut self, arg0: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x8F);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `m16` and increment stack pointer.
	#[inline(always)]
	pub fn pop_Register16Bit(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x8F);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `r16` and increment stack pointer.
	#[inline(always)]
	pub fn pop_Register16Bit_REX_W(&mut self, arg0: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x58, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `m64` and increment stack pointer.
	///
	/// Cannot encode 32-bit operand size.
	#[inline(always)]
	pub fn pop_Register64Bit(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_1(0x8F);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into `r64` and increment stack pointer.
	///
	/// Cannot encode 32-bit operand size.
	#[inline(always)]
	pub fn pop_Register64Bit_REX_W(&mut self, arg0: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x58, arg0);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// POPCNT on `r/m16`.
	#[inline(always)]
	pub fn popcnt_Register16Bit_Any16BitMemory(&mut self, arg0: Register16Bit, arg1: Any16BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// POPCNT on `r/m16`.
	#[inline(always)]
	pub fn popcnt_Register16Bit_Register16Bit(&mut self, arg0: Register16Bit, arg1: Register16Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// POPCNT on `r/m32`.
	#[inline(always)]
	pub fn popcnt_Register32Bit_Any32BitMemory(&mut self, arg0: Register32Bit, arg1: Any32BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// POPCNT on `r/m32`.
	#[inline(always)]
	pub fn popcnt_Register32Bit_Register32Bit(&mut self, arg0: Register32Bit, arg1: Register32Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xB8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// POPCNT on `r/m64`.
	#[inline(always)]
	pub fn popcnt_Register64Bit_Any64BitMemory(&mut self, arg0: Register64Bit, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xB8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// POPCNT on `r/m64`.
	#[inline(always)]
	pub fn popcnt_Register64Bit_Register64Bit(&mut self, arg0: Register64Bit, arg1: Register64Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, Self::REX_W);

		self.opcode_2(0x0F, 0xB8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack into lower 16 bits of `EFLAGS`.
	#[inline(always)]
	pub fn popf(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x9D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Pop top of stack and zero-extend into `RFLAGS`.
	#[inline(always)]
	pub fn popfq(&mut self)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		// No `REX` prefix.

		self.opcode_1(0x9D);

		// No 'ModR/M' byte or Scaled Index Byte (SIB).

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise OR of `mm/m64` and `mm`.
	#[inline(always)]
	pub fn por_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise OR of `mm/m64` and `mm`.
	#[inline(always)]
	pub fn por_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise OR of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn por_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Bitwise OR of `xmm2/m128` and `xmm1`.
	#[inline(always)]
	pub fn por_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xEB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move data from `m8` closer to the processor using `NTA` hint.
	#[inline(always)]
	pub fn prefetchnta_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x18);

		self.mod_rm_sib(arg0, Register64Bit::RAX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move data from `m8` closer to the processor using `T0` hint.
	#[inline(always)]
	pub fn prefetcht0_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x18);

		self.mod_rm_sib(arg0, Register64Bit::RCX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move data from `m8` closer to the processor using `T1` hint.
	#[inline(always)]
	pub fn prefetcht1_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x18);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Move data from `m8` closer to the processor using `T2` hint.
	#[inline(always)]
	pub fn prefetcht2_Any8BitMemory(&mut self, arg0: Any8BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg0);

		self.prefix_group4(arg0);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x18);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Computes the absolute differences of the packed unsigned byte integers from `mm2/m64` and `mm1`; differences are then summed to produce an unsigned word integer result.
	#[inline(always)]
	pub fn psadbw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Computes the absolute differences of the packed unsigned byte integers from `mm2/m64` and `mm1`; differences are then summed to produce an unsigned word integer result.
	#[inline(always)]
	pub fn psadbw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Computes the absolute differences of the packed unsigned byte integers from `xmm2/m128` and `xmm1`; the 8 low differences and 8 high differences are then summed separately to produce two unsigned word integer results.
	#[inline(always)]
	pub fn psadbw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Computes the absolute differences of the packed unsigned byte integers from `xmm2/m128` and `xmm1`; the 8 low differences and 8 high differences are then summed separately to produce two unsigned word integer results.
	#[inline(always)]
	pub fn psadbw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF6);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shuffle bytes in `mm1` according to contents of `mm2/m64`.
	#[inline(always)]
	pub fn pshufb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x00);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shuffle bytes in `mm1` according to contents of `mm2/m64`.
	#[inline(always)]
	pub fn pshufb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x00);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shuffle bytes in `xmm1` according to contents of `xmm2/m128`.
	#[inline(always)]
	pub fn pshufb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x00);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shuffle bytes in `xmm1` according to contents of `xmm2/m128`.
	#[inline(always)]
	pub fn pshufb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x00);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shuffle the doublewords in `xmm2/m128` based on the encoding in `imm8` and store the result in `xmm1`.
	#[inline(always)]
	pub fn pshufd_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x70);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Shuffle the doublewords in `xmm2/m128` based on the encoding in `imm8` and store the result in `xmm1`.
	#[inline(always)]
	pub fn pshufd_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x70);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Shuffle the high words in `xmm2/m128` based on the encoding in `imm8` and store the result in `xmm1`.
	#[inline(always)]
	pub fn pshufhw_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x70);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Shuffle the high words in `xmm2/m128` based on the encoding in `imm8` and store the result in `xmm1`.
	#[inline(always)]
	pub fn pshufhw_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF3);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x70);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Shuffle the low words in `xmm2/m128` based on the encoding in `imm8` and store the result in `xmm1`.
	#[inline(always)]
	pub fn pshuflw_XMMRegister_Any128BitMemory_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Any128BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x70);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Shuffle the low words in `xmm2/m128` based on the encoding in `imm8` and store the result in `xmm1`.
	#[inline(always)]
	pub fn pshuflw_XMMRegister_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: XMMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		self.prefix_group1(0xF2);

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x70);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Shuffle the words in `mm2/m64` based on the encoding in `imm8` and store the result in `mm1`.
	#[inline(always)]
	pub fn pshufw_MMRegister_Any64BitMemory_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Any64BitMemory, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x70);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Shuffle the words in `mm2/m64` based on the encoding in `imm8` and store the result in `mm1`.
	#[inline(always)]
	pub fn pshufw_MMRegister_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: MMRegister, arg2: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x70);

		self.mod_rm_sib(arg1, arg0);

		self.displacement_immediate_1(arg2);

		// No label displacement.
	}

	/// Negate/zero/preserve packed byte integers in `mm1` depending on the corresponding sign in `mm2/m64`.
	#[inline(always)]
	pub fn psignb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x08);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed byte integers in `mm1` depending on the corresponding sign in `mm2/m64`.
	#[inline(always)]
	pub fn psignb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x08);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed byte integers in `xmm1` depending on the corresponding sign in `xmm2/m128`.
	#[inline(always)]
	pub fn psignb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x08);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed byte integers in `xmm1` depending on the corresponding sign in `xmm2/m128`.
	#[inline(always)]
	pub fn psignb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x08);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed doubleword integers in `mm1` depending on the corresponding sign in `mm2/m128`.
	#[inline(always)]
	pub fn psignd_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed doubleword integers in `mm1` depending on the corresponding sign in `mm2/m128`.
	#[inline(always)]
	pub fn psignd_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed doubleword integers in `xmm1` depending on the corresponding sign in `xmm2/m128`.
	#[inline(always)]
	pub fn psignd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed doubleword integers in `xmm1` depending on the corresponding sign in `xmm2/m128`.
	#[inline(always)]
	pub fn psignd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x0A);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed word integers in `mm1` depending on the corresponding sign in `mm2/m128`.
	#[inline(always)]
	pub fn psignw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x09);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed word integers in `mm1` depending on the corresponding sign in `mm2/m128`.
	#[inline(always)]
	pub fn psignw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x09);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed word integers in `xmm1` depending on the corresponding sign in `xmm2/m128`.
	#[inline(always)]
	pub fn psignw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x09);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Negate/zero/preserve packed word integers in `xmm1` depending on the corresponding sign in `xmm2/m128`.
	#[inline(always)]
	pub fn psignw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x09);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `mm` left by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn pslld_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x72);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift doublewords in `mm` left by `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn pslld_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `mm` left by `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn pslld_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `xmm1` left by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn pslld_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x72);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift doublewords in `xmm1` left by `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn pslld_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `xmm1` left by `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn pslld_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift `xmm1` left by `imm8` bytes while shifting in zero-signed.
	#[inline(always)]
	pub fn pslldq_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x73);

		self.mod_rm_sib(arg0, Register64Bit::RDI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift quadword in `mm` left by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllq_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x73);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift quadword in `mm` left by `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllq_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift quadword in `mm` left by `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllq_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift quadwords in `xmm1` left by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllq_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x73);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift quadwords in `xmm1` left by `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift quadwords in `xmm1` left by `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `mm` left by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllw_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x71);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift words in `mm` left `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `mm` left `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `xmm1` left by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllw_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x71);

		self.mod_rm_sib(arg0, Register64Bit::RSI);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift words in `xmm1` left by `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `xmm1` left by `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psllw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `mm` right by `imm8` while shifting in sign bits.
	#[inline(always)]
	pub fn psrad_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x72);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift doublewords in `mm` right by `mm/m64` while shifting in sign bits.
	#[inline(always)]
	pub fn psrad_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `mm` right by `mm/m64` while shifting in sign bits.
	#[inline(always)]
	pub fn psrad_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `xmm1` right by `imm8` while shifting in sign bits.
	#[inline(always)]
	pub fn psrad_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x72);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift doubleword in `xmm1` right by `xmm2/m128` while shifting in sign bits.
	#[inline(always)]
	pub fn psrad_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doubleword in `xmm1` right by `xmm2/m128` while shifting in sign bits.
	#[inline(always)]
	pub fn psrad_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `mm` right by `imm8` while shifting in sign bits.
	#[inline(always)]
	pub fn psraw_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x71);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift words in `mm` right by `mm/m64` while shifting in sign bits.
	#[inline(always)]
	pub fn psraw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `mm` right by `mm/m64` while shifting in sign bits.
	#[inline(always)]
	pub fn psraw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `xmm1` right by `imm8` while shifting in sign bits.
	#[inline(always)]
	pub fn psraw_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x71);

		self.mod_rm_sib(arg0, Register64Bit::RSP);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift words in `xmm1` right by `xmm2/m128` while shifting in sign bits.
	#[inline(always)]
	pub fn psraw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `xmm1` right by `xmm2/m128` while shifting in sign bits.
	#[inline(always)]
	pub fn psraw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `mm` right by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrld_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x72);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift doublewords in `mm` right by amount specified in `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrld_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `mm` right by amount specified in `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrld_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `xmm1` right by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrld_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x72);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift doublewords in `xmm1` right by amount specified in `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrld_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift doublewords in `xmm1` right by amount specified in `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrld_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD2);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift `xmm1` right by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrldq_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x73);

		self.mod_rm_sib(arg0, Register64Bit::RBX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift `mm` right by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlq_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x73);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift `mm` right by amount specified in `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlq_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift `mm` right by amount specified in `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlq_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift quadwords in `xmm1` right by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlq_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x73);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift quadwords in `xmm1` right by amount specified in `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift quadwords in `xmm1` right by amount specified in `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD3);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `mm` right by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlw_MMRegister_Immediate8Bit(&mut self, arg0: MMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x71);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift words in `mm` right by amount specified in `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `mm` right by amount specified in `mm/m64` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `xmm1` right by `imm8` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlw_XMMRegister_Immediate8Bit(&mut self, arg0: XMMRegister, arg1: Immediate8Bit)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_2(arg0, 0x00);

		self.opcode_2(0x0F, 0x71);

		self.mod_rm_sib(arg0, Register64Bit::RDX);

		self.displacement_immediate_1(arg1);

		// No label displacement.
	}

	/// Shift words in `xmm1` right by amount specified in `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Shift words in `xmm1` right by amount specified in `xmm2/m128` while shifting in zero-signed.
	#[inline(always)]
	pub fn psrlw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD1);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed byte integers in `mm/m64` from packed byte integers in `mm`.
	#[inline(always)]
	pub fn psubb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed byte integers in `mm/m64` from packed byte integers in `mm`.
	#[inline(always)]
	pub fn psubb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed byte integers in `xmm2/m128` from packed byte integers in `xmm1`.
	#[inline(always)]
	pub fn psubb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed byte integers in `xmm2/m128` from packed byte integers in `xmm1`.
	#[inline(always)]
	pub fn psubb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed doubleword integers in `mm/m64` from packed doubleword integers in `mm`.
	#[inline(always)]
	pub fn psubd_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed doubleword integers in `mm/m64` from packed doubleword integers in `mm`.
	#[inline(always)]
	pub fn psubd_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed doubleword integers in `xmm2/mem`128 from packed doubleword integers in `xmm1`.
	#[inline(always)]
	pub fn psubd_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed doubleword integers in `xmm2/mem`128 from packed doubleword integers in `xmm1`.
	#[inline(always)]
	pub fn psubd_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFA);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract quadword integer in `mm1` from `mm2/m64`.
	#[inline(always)]
	pub fn psubq_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract quadword integer in `mm1` from `mm2/m64`.
	#[inline(always)]
	pub fn psubq_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed quadword integers in `xmm1` from `xmm2/m128`.
	#[inline(always)]
	pub fn psubq_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed quadword integers in `xmm1` from `xmm2/m128`.
	#[inline(always)]
	pub fn psubq_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xFB);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract signed packed bytes in `mm/m64` from signed packed bytes in `mm` and saturate results.
	#[inline(always)]
	pub fn psubsb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract signed packed bytes in `mm/m64` from signed packed bytes in `mm` and saturate results.
	#[inline(always)]
	pub fn psubsb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed signed byte integers in `xmm2/m128` from packed signed byte integers in `xmm1` and saturate results.
	#[inline(always)]
	pub fn psubsb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed signed byte integers in `xmm2/m128` from packed signed byte integers in `xmm1` and saturate results.
	#[inline(always)]
	pub fn psubsb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract signed packed words in `mm/m64` from signed packed words in `mm` and saturate results.
	#[inline(always)]
	pub fn psubsw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract signed packed words in `mm/m64` from signed packed words in `mm` and saturate results.
	#[inline(always)]
	pub fn psubsw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed signed word integers in `xmm2/m128` from packed signed word integers in `xmm1` and saturate results.
	#[inline(always)]
	pub fn psubsw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed signed word integers in `xmm2/m128` from packed signed word integers in `xmm1` and saturate results.
	#[inline(always)]
	pub fn psubsw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xE9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract unsigned packed bytes in `mm/m64` from unsigned packed bytes in `mm` and saturate result.
	#[inline(always)]
	pub fn psubusb_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract unsigned packed bytes in `mm/m64` from unsigned packed bytes in `mm` and saturate result.
	#[inline(always)]
	pub fn psubusb_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed unsigned byte integers in `xmm2/m128` from packed unsigned byte integers in `xmm1` and saturate result.
	#[inline(always)]
	pub fn psubusb_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed unsigned byte integers in `xmm2/m128` from packed unsigned byte integers in `xmm1` and saturate result.
	#[inline(always)]
	pub fn psubusb_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD8);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract unsigned packed words in `mm/m64` from unsigned packed words in `mm` and saturate result.
	#[inline(always)]
	pub fn psubusw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract unsigned packed words in `mm/m64` from unsigned packed words in `mm` and saturate result.
	#[inline(always)]
	pub fn psubusw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed unsigned word integers in `xmm2/m128` from packed unsigned word integers in `xmm1` and saturate result.
	#[inline(always)]
	pub fn psubusw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed unsigned word integers in `xmm2/m128` from packed unsigned word integers in `xmm1` and saturate result.
	#[inline(always)]
	pub fn psubusw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xD9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed word integers in `mm/m64` from packed word integers in `mm`.
	#[inline(always)]
	pub fn psubw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed word integers in `mm/m64` from packed word integers in `mm`.
	#[inline(always)]
	pub fn psubw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed word integers in `xmm2/m128` from packed word integers in `xmm1`.
	#[inline(always)]
	pub fn psubw_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Subtract packed word integers in `xmm2/m128` from packed word integers in `xmm1`.
	#[inline(always)]
	pub fn psubw_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0xF9);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Set Zero Flag (ZF) if `xmm2/m128 && xmm1` result is all zero-signed.
	///
	/// Set Carry Flag (CF) if `xmm2/m128` AND NOT `xmm1` result is all zero-signed.
	#[inline(always)]
	pub fn ptest_XMMRegister_Any128BitMemory(&mut self, arg0: XMMRegister, arg1: Any128BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x17);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Set Zero Flag (ZF) if `xmm2/m128 && xmm1` result is all zero-signed.
	///
	/// Set Carry Flag (CF) if `xmm2/m128` AND NOT `xmm1` result is all zero-signed.
	#[inline(always)]
	pub fn ptest_XMMRegister_XMMRegister(&mut self, arg0: XMMRegister, arg1: XMMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		self.prefix_group3();

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_3(0x0F, 0x38, 0x17);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unpack and interleave high-order bytes from `mm` and `mm/m64` into `mm`.
	#[inline(always)]
	pub fn punpckhbw_MMRegister_Any64BitMemory(&mut self, arg0: MMRegister, arg1: Any64BitMemory)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		self.prefix_group2(arg1);

		self.prefix_group4(arg1);

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x68);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	/// Unpack and interleave high-order bytes from `mm` and `mm/m64` into `mm`.
	#[inline(always)]
	pub fn punpckhbw_MMRegister_MMRegister(&mut self, arg0: MMRegister, arg1: MMRegister)
	{
		// This is not a VEX encoded instruction.

		// No `FWAIT` Prefix.

		// No prefix group 2.

		// No prefix group 4.

		// No prefix group 3.

		// No prefix group 1.

		self.rex_3(arg1, arg0, 0x00);

		self.opcode_2(0x0F, 0x68);

		self.mod_rm_sib(arg1, arg0);

		// No displacement or immediate.

		// No label displacement.
	}

	#[inline(always)]
	{

		// No `FWAIT` Prefix.

