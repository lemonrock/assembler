// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`XMM0`) is defined on this enumeration but optimal encodings of it in combination with mnemonics exist and should be preferred as they increase code density.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum XMMRegister
{
	/// Register 0.
	///
	/// Contains the first floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// Contains the floating-point return value from a function call when using the System V Application Binary Interface (ABI) for AMD64.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM0 = 0,
	
	/// Register 1.
	///
	/// Contains the second floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// When returning 256-bit floating point values, contains the high bits of such as value (and `XMM0` / `SystemVApplicationBinaryInterface64FloatingPointFunctionReturn` contains the low bits) when using the System V Application Binary Interface (ABI) for AMD64.
	///
	/// Useful for returning tuples of 128-bit floating point values.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM1 = 1,
	
	/// Register 2.
	///
	/// Contains the third floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM2 = 2,
	
	/// Register 3.
	///
	/// Contains the fourth floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM3 = 3,
	
	/// Register 4.
	///
	/// Contains the fifth floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM4 = 4,
	
	/// Register 5.
	///
	/// Contains the sixth floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM5 = 5,
	
	/// Register 6.
	///
	/// Contains the seventh floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM6 = 6,
	
	/// Register 7.
	///
	/// Contains the eigth floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM7 = 7,
	
	/// Register 8.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM8 = 8,
	
	/// Register 9.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM9 = 9,
	
	/// Register 10.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM10 = 10,
	
	/// Register 11.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM11 = 11,
	
	/// Register 12.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM12 = 12,
	
	/// Register 13.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM13 = 13,
	
	/// Register 14.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM14 = 14,
	
	/// Register 15.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for AMD64.
	XMM15 = 15,
}

impl Default for XMMRegister
{
	#[inline(always)]
	fn default() -> Self
	{
		XMMRegister::XMM0
	}
}

impl Into<u8> for XMMRegister
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl Register for XMMRegister
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self.into()
	}
}

impl AsDisplacement for XMMRegister
{
	type D = u8;
	
	#[inline(always)]
	fn displacement(self) -> Self::D
	{
		(self as u8) << 4
	}
}

impl From<YMMRegister> for XMMRegister
{
	#[inline(always)]
	fn from(value: YMMRegister) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl XMMRegister
{
	/// Contains the floating-point return value from a function call when using the System V Application Binary Interface (ABI) for AMD64.
	pub const SystemVApplicationBinaryInterface64FloatingPointFunctionReturn: Self = XMMRegister::XMM0;
	
	/// When returning 256-bit floating point values, contains the high bits of such as value (and `XMM0` / `SystemVApplicationBinaryInterface64FloatingPointFunctionReturn` contains the low bits) when using the System V Application Binary Interface (ABI) for AMD64.
	///
	/// Useful for returning tuples of 128-bit floating point values.
	pub const SystemVApplicationBinaryInterface64HighFloatingPointFunctionReturn: Self = XMMRegister::XMM1;
	
	/// Contains the first floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	pub const SystemVApplicationBinaryInterface64FloatingPointFunctionArgument0: Self = XMMRegister::XMM0;
	
	/// Contains the second floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	pub const SystemVApplicationBinaryInterface64FloatingPointFunctionArgument1: Self = XMMRegister::XMM1;
	
	/// Contains the third floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	pub const SystemVApplicationBinaryInterface64FloatingPointFunctionArgument2: Self = XMMRegister::XMM2;
	
	/// Contains the fourth floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	pub const SystemVApplicationBinaryInterface64FloatingPointFunctionArgument3: Self = XMMRegister::XMM3;
	
	/// Contains the fifth floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	pub const SystemVApplicationBinaryInterface64FloatingPointFunctionArgument4: Self = XMMRegister::XMM4;
	
	/// Contains the sixth floating-point function argument to a function call when using the System V Application Binary Interface for x86-64.
	pub const SystemVApplicationBinaryInterface64FloatingPointFunctionArgument5: Self = XMMRegister::XMM5;
	
	/// Contains the floating-point return value from a function call when using the Microsoft x64 Calling Convention.
	pub const MicrosoftX64CallingConventionFloatingPointFunctionArgumentReturn: Self = XMMRegister::XMM0;
	
	/// Contains the first floating-point function argument to a function call when using the Microsoft x64 Calling Convention.
	pub const MicrosoftX64CallingConventionFloatingPointFunctionArgument0: Self = XMMRegister::XMM0;
	
	/// Contains the second floating-point function argument to a function call when using the Microsoft x64 Calling Convention.
	pub const MicrosoftX64CallingConventionFloatingPointFunctionArgument1: Self = XMMRegister::XMM1;
	
	/// Contains the third floating-point function argument to a function call when using the Microsoft x64 Calling Convention.
	pub const MicrosoftX64CallingConventionFloatingPointFunctionArgument2: Self = XMMRegister::XMM2;
	
	/// Contains the fourth floating-point function argument to a function call when using the Microsoft x64 Calling Convention.
	pub const MicrosoftX64CallingConventionFloatingPointFunctionArgument3: Self = XMMRegister::XMM3;
}
