// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A general purpose register which uses the lower part (ie all general purpose registers apart from `AH`, `DH`, `CH` and `BH`).
pub trait LowGeneralPurposeRegister: GeneralPurposeRegister
{
	/// Contains the first integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument0: Self;
	
	/// Contains the second integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument1: Self;
	
	/// Contains the fifth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument4: Self;
	
	/// Contains the sixth integer function argument to a function call when using the System V Application Binary Interface for x86-64.
	const SystemVApplicationBinaryInterface64IntegerFunctionArgument5: Self;
	
	/// Contains the third integer function argument to a function call when using the Microsoft x64 Calling Convention.
	const MicrosoftX64CallingConventionIntegerFunctionArgument2: Self;
	
	/// Contains the fourth integer function argument to a function call when using the Microsoft x64 Calling Convention.
	const MicrosoftX64CallingConventionIntegerFunctionArgument3: Self;
}
