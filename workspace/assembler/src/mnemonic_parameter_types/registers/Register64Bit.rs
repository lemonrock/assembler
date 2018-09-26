// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`RAX`) is defined on this enumeration but optimal encodings of it in combination with mnemonics exist and should be preferred as they increase code density.
///
/// Registers 8 to 15 when encoded decrease code density (ie they are less efficiently encoded than registers 1 to 7).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Register64Bit
{
	/// Register 0.
	///
	/// Contains the integer return value from a function call when using the System V Application Binary Interface (ABI) for x86-64.
	///
	/// Contains the integer return value from a function call when using the Microsoft x64 Calling Convention.
	RAX = 0,

	/// Register 1.
	///
	/// Contains the fourth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the first integer function argument to a function call when using the Microsoft x64 Calling Convention.
	RCX = 1,

	/// Register 2.
	///
	/// Contains the third integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the second integer function argument to a function call when using the Microsoft x64 Calling Convention.
	RDX = 2,

	/// Register 3.
	RBX = 3,

	/// Register 4.
	RSP = 4,

	/// Register 5.
	RBP = 5,

	/// Register 6.
	///
	/// The 'source' operand in string instructions.
	///
	/// Contains the second integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	RSI = 6,

	/// Register 7.
	///
	/// The 'destination' operand in string instructions.
	///
	/// Contains the first integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	RDI = 7,

	/// Register 8.
	///
	/// Contains the fifth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the third integer function argument to a function call when using the Microsoft x64 Calling Convention.
	R8 = 8,

	/// Register 9.
	///
	/// Contains the sixth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the fourth integer function argument to a function call when using the Microsoft x64 Calling Convention.
	R9 = 9,

	/// Register 10.
	R10 = 10,

	/// Register 11.
	R11 = 11,

	/// Register 12.
	R12 = 12,

	/// Register 13.
	R13 = 13,

	/// Register 14.
	R14 = 14,

	/// Register 15.
	R15 = 15,
}

impl Default for Register64Bit
{
	#[inline(always)]
	fn default() -> Self
	{
		Register64Bit::RAX
	}
}

impl Into<u8> for Register64Bit
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl Register for Register64Bit
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self.into()
	}
}

impl GeneralPurposeRegister for Register64Bit
{
	const SystemVApplicationBinaryInterface64IntegerFunctionReturn: Self = Register64Bit::RAX;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument2: Self = Register64Bit::RDX;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument3: Self = Register64Bit::RCX;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgumentReturn: Self = Register64Bit::RAX;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument0: Self = Register64Bit::RCX;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument1: Self = Register64Bit::RDX;
}

impl LowGeneralPurposeRegister for Register64Bit
{
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument0: Self = Register64Bit::RDI;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument1: Self = Register64Bit::RSI;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument4: Self = Register64Bit::R8;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument5: Self = Register64Bit::R9;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument2: Self = Register64Bit::R8;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument3: Self = Register64Bit::R9;
}
