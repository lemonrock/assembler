// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Represents the result of attempting a mnemonic with a near jump.
///
/// Will be an error if a label could be resolved and the jump exceeded the (inclusive range) i32::MIN to i32::MAX bytes.
///
/// In this case, the instruction stream will be reset to where it was before the prefixes, opcodes and displacements where emitted for the `Jcc` instruction.
type NearJmpResult = Result<(), ()>;
