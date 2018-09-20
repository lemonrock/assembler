// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


pub struct Parser
{

}

impl Parser
{
	#[inline(always)]
	pub fn parse_prefixes(&mut self)
	{
		let mut prefixes: ArrayVec<[String; 16]> = ArrayVec::new();
		let mut span = parser.span;
		let mut token = parse_ident_or_rust_keyword(parser)?;
		
		while Self::is_prefix(token)
		{
			prefixes.push(token.to_string());
			token = parse_ident_or_rust_keyword(parser)?
		}
	}
	
	#[inline(always)]
	fn is_prefix(token: &str) -> bool
	{
		phf_set!("lock", "rep", "repe", "repz", "repne", "repnz", "ss", "cs", "ds", "es", "fs", "gs").contains(token)
	}
}
