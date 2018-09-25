// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Hints to use when creating an instruction stream.
///
/// Also can be retrieved using `InstructionStream.hints_for_next_instance()`, to fine-tune future instances to avoid reallocations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstructionStreamHints
{
	/// Number of labels.
	pub number_of_labels: usize,
	
	/// Number of 8-bit jumps.
	pub number_of_8_bit_jumps: usize,
	
	/// Number of 32-bit jumps.
	pub number_of_32_bit_jumps: usize,
}

impl Default for InstructionStreamHints
{
	#[inline(always)]
	fn default() -> Self
	{
		InstructionStreamHints
		{
			number_of_labels: Self::MinimumValue,
			number_of_8_bit_jumps: Self::MinimumValue,
			number_of_32_bit_jumps: Self::MinimumValue,
		}
	}
}

impl InstructionStreamHints
{
	const MinimumValue: usize = 4096;
	
	/// Ensures that sensible minimals are present.
	#[inline(always)]
	pub fn adjust(&mut self)
	{
		self.number_of_labels = Self::adjust_value(self.number_of_labels);
		self.number_of_8_bit_jumps = Self::adjust_value(self.number_of_8_bit_jumps);
		self.number_of_32_bit_jumps = Self::adjust_value(self.number_of_32_bit_jumps);
	}
	
	fn adjust_value(value: usize) -> usize
	{
		if value < Self::MinimumValue
		{
			Self::MinimumValue
		}
		else
		{
			value.next_power_of_two()
		}
	}
}
