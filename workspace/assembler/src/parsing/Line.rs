// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct Line<'a>
{
	byte_position_at_char_boundary: usize,
	line: &'a str,
}

impl<'a> Line<'a>
{
	#[inline(always)]
	pub(crate) fn new(line: &'a str) -> Self
	{
		Line
		{
			byte_position_at_char_boundary: 0,
			line,
		}
	}
	
	#[inline(always)]
	pub(crate) fn eat_whitespace(&mut self)
	{
		while let Some(character) = self.peek()
		{
			if character.is_whitespace()
			{
				break
			}
			self.swallow(character)
		}
	}
	
	/// Eats the value or end of line; stays where it is if neither of these is true.
	///
	/// Returns true if end-of-line encountered, false if value eatern.
	///
	/// The value need not be followed by whitespace.
	#[inline(always)]
	pub(crate) fn eat_value_or_end_of_line<'b>(&mut self, value: &'b str) -> Result<bool, InstructionParsingError>
	{
		if self.has_no_more()
		{
			Ok(true)
		}
		else if self.remainder().starts_with(value)
		{
			self.byte_position_at_char_boundary += value.len();
			Ok(false)
		}
		else
		{
			Err(InstructionParsingError("Neither value nor end of line encountered"))
		}
	}
	
	/// Eats the value or stays where it is.
	///
	/// The value need not be followed by whitespace.
	#[inline(always)]
	pub(crate) fn eat_value<'b>(&mut self, value: &'b str) -> bool
	{
		if self.remainder().starts_with(value)
		{
			self.byte_position_at_char_boundary += value.len();
			true
		}
		else
		{
			false
		}
	}
	
	/// Eats the value or stays where it is and returns an error.
	///
	/// The value need not be followed by whitespace.
	///
	/// If an error occurs, the position does not change.
	#[inline(always)]
	pub(crate) fn eat_expected_value<'b>(&mut self, value: &'b str) -> Result<(), InstructionParsingError>
	{
		if self.eat_value(value)
		{
			Ok(())
		}
		else
		{
			Err(InstructionParsingError("Could not eat expected value"))
		}
	}
	
	/// Returns the tag and the character from `characters` that caused the tag to be read.
	///
	/// Tag can be empty.
	///
	/// Positioned immediately after the character that caused the tag to be read.
	#[inline(always)]
	pub(crate) fn read_mandatory_tag_discarding_trailing_whitespace_or(&mut self, characters: &Set<char>) -> Result<(char, &'a str), InstructionParsingError>
	{
		let starts_at = self.current_position();
		let mut whitespace_ends_at;
		while let Some(character) = self.peek()
		{
			let ends_at = self.byte_position_at_char_boundary;
			self.swallow(character);
			
			if character.is_whitespace()
			{
				self.eat_whitespace();
				return match self.peek()
				{
					None =>
					{
						self.reset_position(starts_at);
						Err(InstructionParsingError("End of line encountered after eating whitespace"))
					}
					
					Some(character) => if characters.contains(&character)
					{
						Ok((character, &self.line[starts_at .. whitespace_ends_at]))
					}
					else
					{
						self.reset_position(starts_at);
						Err(InstructionParsingError("Tag not terminated by expected character after whitespace"))
					},
				}
			}
			
			if characters.contains(&character)
			{
				return Ok((character, &self.line[starts_at .. self.byte_position_at_char_boundary]))
			}
		}
		Err(InstructionParsingError("End of line encountered"))
	}
	
	/// If the optional tag is present and acceptable according to `evaluate`, eats it and then eats any subsequent whitespace.
	///
	/// Otherwise the position does not change.
	#[inline(always)]
	pub(crate) fn optional_tag_and_eat_whitespace_after_it<'b, R>(&mut self, tag: &'b str, evaluate: impl Fn(&'a str) -> Option<R>) -> Option<R>
	{
		let was_at_position = self.current_position();
		
		match self.tag()
		{
			None => None,
			
			Some(tag) =>
			{
				let result = evaluate(tag);
				if result.is_none()
				{
					line.reset_position(was_at_position)
				}
				else
				{
					self.eat_whitespace()
				}
				result
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn mandatory_tag_ascii_lower_case(&mut self) -> Result<&'a str, InstructionParsingError>
	{
		self.tag().map(|tag| tag.to_ascii_lower_case()).ok_or(InstructionParsingError("Could not parse tag"))
	}
	
	#[inline(always)]
	pub(crate) fn mandatory_tag(&mut self) -> Result<&'a str, InstructionParsingError>
	{
		self.tag().ok_or(InstructionParsingError("Could not parse tag"))
	}
	
	#[inline(always)]
	pub(crate) fn tag(&mut self) -> Option<&'a str>
	{
		let character = match self.peek()
		{
			None => return None,
			
			Some(character) => if character.is_whitespace()
			{
				return Some("")
			}
			else
			{
				character
			},
		};
		
		let starts_at = self.byte_position_at_char_boundary;
		self.swallow(character);
		
		while let Some(character) = self.peek()
		{
			if character.is_whitespace()
			{
				return Some(&self.line[starts_at .. self.byte_position_at_char_boundary])
			}
			else
			{
				self.swallow(character);
			}
		}
		
		Some(&self.line[starts_at .. self.length()])
	}
	
	#[inline(always)]
	pub(crate) fn has_more(&self) -> bool
	{
		self.byte_position_at_char_boundary != self.length()
	}
	
	#[inline(always)]
	pub(crate) fn has_no_more(&self) -> bool
	{
		self.byte_position_at_char_boundary == self.length()
	}
	
	#[inline(always)]
	pub(crate) fn current_position(&self) -> usize
	{
		self.byte_position_at_char_boundary
	}
	
	#[inline(always)]
	pub(crate) fn reset_position(&mut self, byte_position_at_char_boundary: usize)
	{
		self.byte_position_at_char_boundary = byte_position_at_char_boundary;
	}
	
	#[inline(always)]
	pub(crate) fn peek_or_error(&self, error: InstructionParsingError) -> Result<char, InstructionParsingError>
	{
		match self.peek()
		{
			None => Err(error),
			Some(character) => Ok(character),
		}
	}
	
	#[inline(always)]
	fn peek(&self) -> Option<char>
	{
		self.remainder().chars().next()
	}
	
	#[inline(always)]
	pub(crate) fn swallow(&mut self, character: char)
	{
		self.byte_position_at_char_boundary += character.len_utf8()
	}
	
	#[inline(always)]
	fn remainder(&self) -> &str
	{
		&self.line[self.byte_position_at_char_boundary .. ]
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.line.len()
	}
}
