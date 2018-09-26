// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Register 0 (`ST(0)`) is defined on this enumeration but optimal encodings of it in combination with mnemonics exist and should be preferred as they increase code density.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum X87Register
{
	/// Register `ST(0)`.
	///
	/// Contains the first `long double` floating-point return value from a function call when using the System V Application Binary Interface (ABI) for x86-64.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for x86-64.
	ST0 = 0,
	
	/// Register `ST(1)`.
	///
	/// Contains the second `long double` floating-point return value from a function call when using the System V Application Binary Interface (ABI) for x86-64; typically used for returning values of type `complex long double`, where register `ST(0)` contains the 'real' part as a `long double` and `ST(1)` contains the 'imaginary' part as a `long double`.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for x86-64.
	ST1 = 1,
	
	/// Register `ST(2)`.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for x86-64.
	ST2 = 2,
	
	/// Register `ST(3)`.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for x86-64.
	ST3 = 3,
	
	/// Register `ST(4)`.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for x86-64.
	ST4 = 4,
	
	/// Register `ST(5)`.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for x86-64.
	ST5 = 5,
	
	/// Register `ST(6)`.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for x86-64.
	ST6 = 6,
	
	/// Register `ST(7)`.
	///
	/// A scratch register (also known as a temporary or (function) caller-saved register) when using the System V Application Binary Interface (ABI) for x86-64.
	ST7 = 7,
}

impl Default for X87Register
{
	#[inline(always)]
	fn default() -> Self
	{
		X87Register::ST0
	}
}

impl Into<u8> for X87Register
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl Register for X87Register
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self.into()
	}
}

impl X87Register
{
	/// Contains the first `long double` floating-point return value from a function call when using the System V Application Binary Interface (ABI) for x86-64.
	pub const SystemVApplicationBinaryInterface64LongDoubleFloatingPointFunctionReturn0: Self = X87Register::ST0;
	
	/// Contains the second `long double` floating-point return value from a function call when using the System V Application Binary Interface (ABI) for x86-64.
	pub const SystemVApplicationBinaryInterface64LongDoubleFloatingPointFunctionReturn1: Self = X87Register::ST1;
}
