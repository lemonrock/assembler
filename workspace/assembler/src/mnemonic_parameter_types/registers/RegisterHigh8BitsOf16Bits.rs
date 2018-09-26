// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// High 8-bits of the first four general purpose registers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RegisterHigh8BitsOf16Bits
{
	/// Register 0.
	///
	/// Contains the integer return value from a function call when using the System V Application Binary Interface (ABI) for AMD64.
	///
	/// Contains the integer return value from a function call when using the Microsoft x64 Calling Convention.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	AH = 4,
	
	/// Register 1.
	///
	/// Contains the fourth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the first integer function argument to a function call when using the Microsoft x64 Calling Convention.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	CH = 5,
	
	/// Register 2.
	///
	/// Contains the third integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the second integer function argument to a function call when using the Microsoft x64 Calling Convention.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	DH = 6,
	
	/// Register 3.
	///
	/// A callee-saved register: a called function must preserve the value in this register when using the System V Application Binary Interface (ABI) for AMD64.
	/// In other words, upon returning, it should make sure the value originally in this register when the call was made is the current value of this register.
	BH = 7,
}

impl Default for RegisterHigh8BitsOf16Bits
{
	#[inline(always)]
	fn default() -> Self
	{
		RegisterHigh8BitsOf16Bits::AH
	}
}

impl Into<u8> for RegisterHigh8BitsOf16Bits
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl Register for RegisterHigh8BitsOf16Bits
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self.into()
	}
}

impl GeneralPurposeRegister for RegisterHigh8BitsOf16Bits
{
	const SystemVApplicationBinaryInterface64IntegerFunctionReturn: Self = RegisterHigh8BitsOf16Bits::AH;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument2: Self = RegisterHigh8BitsOf16Bits::DH;
	
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument3: Self = RegisterHigh8BitsOf16Bits::CH;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgumentReturn: Self = RegisterHigh8BitsOf16Bits::AH;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument0: Self = RegisterHigh8BitsOf16Bits::CH;
	
	const MicrosoftX64CallingConventionIntegerFunctionArgument1: Self = RegisterHigh8BitsOf16Bits::DH;
}
