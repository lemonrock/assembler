// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Jump variant.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub enum JumpVariant
{
	/// Coded as `->label`.
	Global(RustIdent),

	/// Coded as `>label`.
	Backward(RustIdent),

	/// Coded as `<label`.
	Forward(RustIdent),

	/// Coded as `=>expr`.
	Dynamic(RustExpression),
	
	/// An address.
	Bare(RustExpression),
}
