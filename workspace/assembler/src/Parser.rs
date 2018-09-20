// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


//pub struct Parser
//{
//
//}
//
//impl Parser
//{
//	#[inline(always)]
//	pub fn parse_prefixes(&mut self)
//	{
//		let mut prefixes: ArrayVec<[String; 16]> = ArrayVec::new();
//		let mut span = parser.span;
//		let mut token = parse_ident_or_rust_keyword(parser)?;
//
//		while Self::is_prefix(token)
//		{
//			prefixes.push(token.to_string());
//			token = parse_ident_or_rust_keyword(parser)?
//		}
//	}
//
//	#[inline(always)]
//	fn is_prefix(token: &str) -> bool
//	{
//		phf_set!("lock", "rep", "repe", "repz", "repne", "repnz", "ss", "cs", "ds", "es", "fs", "gs").contains(token)
//	}
//}

/*
	There is a repeat prefix and a segment prefix; order is one or the other.

*/

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstructionParsingError(pub &'static str);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ParsingInstruction
{
	repeat_prefix: Option<RepeatPrefix>,
	segment_prefix: Option<SegmentPrefix>,
}

impl ParsingInstruction
{
	pub(crate) fn parse(mut line: Line) -> Result<(), InstructionParsingError>
	{
		line.eat_whitespace();
		
		let (_repeat_prefix, _segment_prefix, _potential_mnemonic) = Self::parse_prefixes_and_potential_mnemonic(line)?;
		
		
		unimplemented!();
	}
	
	fn parse_prefixes_and_potential_mnemonic<'a>(mut line: Line<'a>) -> Result<(Option<RepeatPrefix>, Option<SegmentPrefix>, &'a str), InstructionParsingError>
	{
		let repeat_prefix_or_segment_prefix_or_mnemonic = line.mandatory_tag()?;
		
		let result = match Self::parse_prefix(repeat_prefix_or_segment_prefix_or_mnemonic)
		{
			Some(Left(repeat_prefix)) =>
			{
				let segment_prefix_or_mnemonic = line.mandatory_tag()?;
				
				let repeat_prefix = Some(repeat_prefix);
				let segment_prefix = SegmentPrefix::parse_prefix(segment_prefix_or_mnemonic);
				let potential_mnemonic = if segment_prefix.is_none()
				{
					segment_prefix_or_mnemonic
				}
				else
				{
					line.mandatory_tag()?
				};
				
				(repeat_prefix, segment_prefix, potential_mnemonic)
			},
			
			Some(Right(segment_prefix_value)) =>
			{
				let repeat_prefix_or_mnemonic = line.mandatory_tag()?;
				
				let segment_prefix = Some(segment_prefix_value);
				let repeat_prefix = RepeatPrefix::parse_prefix(repeat_prefix_or_mnemonic);
				let potential_mnemonic = if repeat_prefix.is_none()
				{
					repeat_prefix_or_mnemonic
				}
				else
				{
					line.mandatory_tag()?
				};
				
				(repeat_prefix, segment_prefix, potential_mnemonic)
			},
			
			None => (None, None, line.mandatory_tag()?),
		};
		Ok(result)
	}
	
	#[inline(always)]
	fn parse_prefix(potential_prefix_tag: &str) -> Option<Either<RepeatPrefix, SegmentPrefix>>
	{
		use self::RepeatPrefix::*;
		use self::SegmentPrefix::*;
		
		static KnownPrefixes: Map<&'static str, Either<RepeatPrefix, SegmentPrefix>> = phf_map!
		{
			"rep" => Left(rep),
			"repe" => Left(repe_or_repz),
			"repz" => Left(repe_or_repz),
			"repnz" => Left(repnz_or_repne),
			"repne" => Left(repnz_or_repne),
			"lock" => Left(lock),
			
			"ss" => Right(ss),
			"cs" => Right(cs),
			"ds" => Right(ds),
			"es" => Right(es),
			"fs" => Right(fs),
			"gs" => Right(gs),
		};
		
		KnownPrefixes.get(potential_prefix_tag).map(|reference| *reference)
	}
}

#[cfg(test)]
mod Parser_tests
{
	
	#[test]
	fn exploration()
	{
		panic!("Make this test fail");
	}
}
