// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A general purpose register which can be encoded in a Scaled Index Byte (SIB).
pub trait GeneralPurposeRegister: Register
{
	/// Contains the integer return value from a function call when using the System V Application Binary Interface (ABI) for AMD64.
	const SystemVApplicationBinaryInterface64IntegerFunctionReturn: Self;
	
	/// Contains the third integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument2: Self;
	
	/// Contains the fourth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument3: Self;
	
	/// Contains the integer return value from a function call when using the Microsoft x64 Calling Convention.
	const MicrosoftX64CallingConventionIntegerFunctionArgumentReturn: Self;
	
	/// Contains the first integer function argument to a function call when using the Microsoft x64 Calling Convention.
	const MicrosoftX64CallingConventionIntegerFunctionArgument0: Self;
	
	/// Contains the second integer function argument to a function call when using the Microsoft x64 Calling Convention.
	const MicrosoftX64CallingConventionIntegerFunctionArgument1: Self;
}
