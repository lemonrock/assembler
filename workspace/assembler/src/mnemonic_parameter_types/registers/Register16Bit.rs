// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`AL`) and Register 2 (`DX`) are defined on this enumeration but optimal encodings of them in combination with mnemonics exist and should be preferred as they increase code density.
///
/// Registers 8 to 15 when encoded decrease code density (ie they are less efficiently encoded than registers 1 to 7).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Register16Bit
{
	/// Register 0.
	///
	/// Contains the integer return value from a function call when using the System V Application Binary Interface (ABI) for x86-64.
	///
	/// Contains the integer return value from a function call when using the Microsoft x64 Calling Convention.
	AX = 0,
	
	/// Register 1.
	///
	/// Contains the fourth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the first integer function argument to a function call when using the Microsoft x64 Calling Convention.
	CX = 1,
	
	/// Register 2.
	///
	/// Contains the third integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the second integer function argument to a function call when using the Microsoft x64 Calling Convention.
	DX = 2,

	/// Register 3.
	BX = 3,

	/// Register 4.
	SP = 4,

	/// Register 5.
	BP = 5,

	/// Register 6.
	///
	/// The 'source' operand in string instructions.
	///
	/// Contains the second integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	SI = 6,

	/// Register 7.
	///
	/// The 'destination' operand in string instructions.
	///
	/// Contains the first integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	DI = 7,

	/// Register 8.
	///
	/// Contains the fifth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the third integer function argument to a function call when using the Microsoft x64 Calling Convention.
	R8W = 8,

	/// Register 9.
	///
	/// Contains the sixth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the fourth integer function argument to a function call when using the Microsoft x64 Calling Convention.
	R9W = 9,

	/// Register 10.
	R10W = 10,

	/// Register 11.
	R11W = 11,

	/// Register 12.
	R12W = 12,

	/// Register 13.
	R13W = 13,

	/// Register 14.
	R14W = 14,

	/// Register 15.
	R15W = 15,
}

impl Default for Register16Bit
{
	#[inline(always)]
	fn default() -> Self
	{
		Register16Bit::AX
	}
}

impl Into<u8> for Register16Bit
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl Register for Register16Bit
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self.into()
	}
}

impl GeneralPurposeRegister for Register16Bit
{
	const SystemVApplicationBinaryInterface64IntegerFunctionReturn: Self = Register16Bit::AX;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument2: Self = Register16Bit::DX;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument3: Self = Register16Bit::CX;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgumentReturn: Self = Register16Bit::AX;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument0: Self = Register16Bit::CX;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument1: Self = Register16Bit::DX;
}

impl LowGeneralPurposeRegister for Register16Bit
{
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument0: Self = Register16Bit::DI;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument1: Self = Register16Bit::SI;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument4: Self = Register16Bit::R8W;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument5: Self = Register16Bit::R9W;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument2: Self = Register16Bit::R8W;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument3: Self = Register16Bit::R9W;
}
