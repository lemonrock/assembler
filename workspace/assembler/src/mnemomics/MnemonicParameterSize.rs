// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Size of the mnemonic parameter (operand).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub(crate) enum MnemonicParameterSize
{
	/// A fixed, known size.
	Fixed(Size),
	
	/// All possible sizes for this parameter's MnemonicParameterKind; a wildcard.
	///
	/// ie:-
	///
	/// * `WORD` or `DWORD` for immediate (`i`).
	/// * `WORD` `DWORD` or `QWORD` for `legacy_register` (`r`) or `legacy_register_and_memory` (`v`).
	/// * `OWORD` or `HWORD` for `legacy_register_and_memory` (`y`) or `xmm_or_ymm_register_and_memory` (`w`).
	/// * Any size for `memory` (`m`).
	///
	/// Coded as `*`.
	AllPossible,
	
	/// Matches a lack of size; only useful in combination with a MnemonicParameterKind of `memory`.
	///
	/// Coded as `!`.
	NoSize,
	
	/// Matches any size and doesn't participate in the operand size calculation.
	///
	/// Coded as `?`.
	Any,
}
