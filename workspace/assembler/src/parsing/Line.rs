// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct Line<'a>
{
	iterator: Peekable<CharIndices<'a>>,
	line: &'a str,
}

impl<'a> Line<'a>
{
	#[inline(always)]
	pub(crate) fn new(line: &'a str) -> Self
	{
		Line
		{
			iterator: line.char_indices().peekable(),
			line,
		}
	}
	
	#[inline(always)]
	pub(crate) fn eat_whitespace(&mut self)
	{
		while let Some(&(_, character)) = self.iterator.peek()
		{
			if character.is_whitespace()
			{
				break
			}
			else
			{
				return
			}
		}
		
		self.iterator.next();
	}
	
	#[inline(always)]
	pub(crate) fn mandatory_tag(&mut self) -> Result<&'a str, InstructionParsingError>
	{
		self.tag().ok_or(InstructionParsingError("Could not parse tag"))
	}
	
	#[inline(always)]
	pub(crate) fn tag(&mut self) -> Option<&'a str>
	{
		let starts_at = match self.iterator.peek()
		{
			None => return None,
			
			Some(&(byte_position, character)) =>
			{
				if character.is_whitespace()
				{
					return Some("")
				}
				
				byte_position
			}
		};
		
		self.iterator.next();
		
		while let Some(&(byte_position, character)) = self.iterator.peek()
		{
			if character.is_whitespace()
			{
				return Some(&self.line[starts_at .. byte_position])
			}
			else
			{
				self.iterator.next();
			}
		}
		
		Some(&self.line[starts_at .. self.line.len()])
	}
}
