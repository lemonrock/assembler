// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ParsingInstruction
{
	repeat_prefix: Option<RepeatPrefix>,
	segment_prefix: Option<SegmentPrefix>,
}

impl ParsingInstruction
{
	pub(crate) fn parse<'a>(line: &mut Line<'a>, assembling_for_architecture_variant: &AssemblingForArchitectureVariant) -> Result<(), InstructionParsingError>
	{
		line.eat_whitespace();
		
		let (_repeat_prefix, _segment_prefix, _signatures) = Self::parse_prefixes_and_mnemonic(line)?;
		
		while
		{
			line.eat_whitespace();
			let mut has_mnemonic_arguments = line.has_more();
			has_mnemonic_arguments
		}
		{
			// TODO: Nasm only allows BYTE, WORD and DWORD.
			let value_size_hint = Self::parse_size_hint(line, assembling_for_architecture_variant);
			
			// A reference, eg [.mylabel], etc
			let is_opening_square_bracket = line.eat_value("[");
			if is_opening_square_bracket
			{
				match what_comes_next
				{
					// Start of a bare label.
					"extern" => (),
					
					// Start of a label.
					"." => (),
					
					// Regular memory location.
					_ =>
					{
						let no_split = Self::parse_nosplit_hint(line);
						let displacement_size_hint = Self::parse_size_hint(line, assembling_for_architecture_variant);
						
						// The NASM syntax is ambiguous and tedious and error prone to parse.
						
						// Syntax:-
						// base[, index[, scale[, displacement]]]
						
						let mut base = None;
						let mut index = None;
						let mut scale = None;
						let mut displacement = None;
						
						let (tag, ) line.read_mandatory_tag_discarding_trailing_whitespace_or(&phf_set!(',', ']'))?
						{
							(tag @ _, ')
						}
						
						// A missing base defaults to 'RAX' or 'RBP' in some circumstances.
						let base_register = Self::parse_register(line.read_mandatory_tag_discarding_trailing_whitespace_or(&phf_set!(',', ']'))?);
						
						
						
						
						
						
						
						// There are several recognisable variants of the basic form of encoding of the 'offset':-
						//
						// * `[index_register * scale + base_register + displacement]`
						//
						// * At least one of `index_register * scale`, `base_register` or `displacement` must be present.
						// * All of these items may be present in any order.
						// * `scale` may or may not be present; if present, it can have the values `1`, `2`, `4`, or `8`.
						// * The first item may be preceeded by a `-`.
						// * The `+` before `base_register` may also be a `-`.
						// * The `+` before `displacement` may also be a `-`.
						// * `displacement` is a signed 8, 16 or 32 bit value (i8, i16, i64).
						// * `displacement` may be represented by a Rust expression `%some_expression()`.
						// * `scale` may be represented by a Rust expression but frankly that makes life too hard (dynasm supports this).
						//
						// Any of these components may be absent; at least one component should be present.
						//
						// * Simple Examples:-
						// 	* `[0xFFFF]` => `displacement = 0xFFFF`
						// 	* `[RAX]` => `index_register = RAX`
						// 	* [RAX + 1] => `index_register = RAX, displacement = 1`
						// 	* [SomeRegister + 1]  where SomeRegister is an alias to a register
						//
						// More complex examples:-
						// 	* [EBX * 2 + ECX + %offset%] => `index_register = EBX, scale = 2, base_register = ECX, displacement = %offset%`
						// 	* [BP + DI + 8] => `index_register = BP, scale = 1 (implicit), base_register = 9, displacement = 8`
						//
						// Calculations:-
						// 	* [EBX * 5] => [EBX * 4 + EBX]
						// 	* [SomeLabel1 * 2 - SomeLabel2] => [SomeLabel1 + (SomeLabel1 - SomeLabel2)]
						//
						// Transformations for encoding efficiency:-
						// 	* [EAX * 2 + 0] => [EAX + EAX] as it is a more efficient encoding
						// 	* [EAX * 2] => [EAX, EAX]
						// 	* [EAX * 2 + Offset] => [EAX, EAX, OFFSET]
						//
						// Very useful: https://stackoverflow.com/questions/34058101/referencing-the-contents-of-a-memory-location-x86-addressing-modes
						
						enum PreceedingBinaryOperator
						{
							Add,
							Multiply,
						}
						
						let mut preceeding_binary_operator = PreceedingBinaryOperator::Add;
						let mut sign_implied_by_preceeding_binary_operator: u64 = 1;
						
						let mut parsed_registers = HashSet::with_capacity(3);
						loop
						{
							let our_sign = match line.optional_tag_and_eat_whitespace_after_it("-", |_| ())
							{
								None => 1,
								Some(_) => -1,
							};
							let sign = sign_implied_by_preceeding_binary_operator * our_sign;
							
							static Operators: Set<char> = phf_set![']', '+', '-', '*'];
							let (operator, tag) = line.read_mandatory_tag_discarding_trailing_whitespace_or(&Operators)?;
							
							enum MemoryReference
							{
								RegisterReference
								{
									sign: u64,
									register: Register,
								},
								
								IntegerLiteral(u64),
							
								RustExpression
								{
									sign: u64,
									expression: String,
								},
							}
							
							let memory_reference = if let Some(register) = Self::parse_register(tag)
							{
								if parsed_registers.insert(register)
								{
									if parsed_registers.len() > 2
									{
										return Err(InstructionParseError("A maximum of two registers may be used in a memory reference"))
									}
									MemoryReference::RegisterReference { sign, register }
								}
								else
								{
									return Err(InstructionParseError("The same register may not be used for both base and index"))
								}
							}
							else if let Some(integer_literal) = Self::parse_integer_literal(tag)
							{
								// TODO: Validate not bigger than size hint
								MemoryReference::IntegerLiteral(integer_literal * sign_implied_by_preceeding_binary_operator)
							}
							else
							{
								MemoryReference::RustExpression { sign, expression: tag }
							};
							
							match operator
							{
								']' => break,
								
								'+' =>
								{
									// TODO: Do something with memory reference
									sign_implied_by_preceeding_binary_operator = 1;
									preceeding_binary_operator = PreceedingBinaryOperator::Add;
								}
								
								'-' =>
								{
									// TODO: Do something with memory reference
									sign_implied_by_preceeding_binary_operator = -1;
									preceeding_binary_operator = PreceedingBinaryOperator::Add;
								}
								
								'*' =>
								{
									sign_implied_by_preceeding_binary_operator = 1;
									preceeding_binary_operator = PreceedingBinaryOperator::Multiply;
								},
								
								_ => unreachable!(),
							}
						}
						
						
						match (operator, tag)
						{
							(']', "") => break,
						}
					}
					
					// TODO will need to eat whitespace before next tag is parsed.
				}
			}
			
			// Commas separate mnemonic arguments.
			line.eat_whitespace();
			if line.eat_value_or_end_of_line(",")?
			{
				break
			}
		}
		
		Ok(result)
	}
	
	#[inline(always)]
	fn parse_integer_literal(tag: &str) -> Option<u64>
	{
		// Rust and nasm both support numeric literals with embedded underscores.
		let tag_with_underscores_removed = tag.replace("_", "");
		
		if tag_with_underscores_removed.len() == 0
		{
			return None
		}
		
		let could_have_a_radix_prefix = tag_with_underscores_removed.len() > 2;
		if could_have_a_radix_prefix
		{
			static RadixPrefixes: Map<&'static str, u32> = phf_map!
			{
				// Rust and nasm.
				"0x" => 16,
				
				// nasm.
				"0X" => 16,
				"0H" => 16,
				"0h" => 16,
				
				// nasm.
				"0D" => 10,
				"0d" => 10,
				"0T" => 10,
				"0t" => 10,
				
				// Rust and nasm.
				"0o" => 8,
				
				// nasm.
				"0O" => 8,
				
				// nasm.
				"0q" => 8,
				
				// nasm.
				"0Q" => 8,
				
				// Rust and nasm.
				"0b" => 2,
				
				// nasm.
				"0B" => 2,
				"0Y" => 2,
				"0y" => 2,
			};
			
			if let Some(radix) = RadixPrefixes.get(&tag_with_underscores_removed[0 .. 2])
			{
				return u64::from_str_radix(&tag_with_underscores_removed[2 ..]  *radix).ok()
			}
		}
		
		let could_have_a_radix_suffix = tag_with_underscores_removed.len() > 1;
		if could_have_a_radix_suffix
		{
			static RadixSuffixes: Map<&'static str, u32> = phf_map!
			{
				"H" => 16,
				"h" => 16,
				"x" => 16,
				"X" => 16,
				"D" => 10,
				"d" => 10,
				"T" => 10,
				"t" => 10,
				"Q" => 8,
				"q" => 8,
				"B" => 2,
				"b" => 2,
				"Y" => 2,
				"y" => 2,
			};
			
			let penultimate = tag_with_underscores_removed.len() - 1;
			
			if let Some(radix) = RadixPrefixes.get(&tag_with_underscores_removed[penultimate ..])
			{
				return u64::from_str_radix(&tag_with_underscores_removed[.. penultimate], *radix).ok()
			}
		}
		
		return u64::from_str_radix(&tag_with_underscores_removed[..], 10).ok()
	}
	
	#[inline(always)]
	fn parse_size_hint<'a>(line: &mut Line<'a>, assembling_for_architecture_variant: &AssemblingForArchitectureVariant) -> Option<Size>
	{
		use self::Size::*;
		
		static Known32BitSizes: Map<&'static str, Size> = phf_map!
		{
			"byte" => BYTE,
			"word" => WORD,
			"dword" => DWORD,
			"aword" => DWORD,
			"fword" => FWORD,
			"qword" => QWORD,
			"tword" => PWORD,
			"oword" => OWORD,
			"yword" => HWORD,
		};
		
		static Known64BitSizes: Map<&'static str, Size> = phf_map!
		{
			"byte" => BYTE,
			"word" => WORD,
			"dword" => DWORD,
			"fword" => FWORD,
			"aword" => QWORD,
			"qword" => QWORD,
			"tword" => PWORD,
			"oword" => OWORD,
			"yword" => HWORD,
		};
		
		line.optional_tag_and_eat_whitespace_after_it(|might_be_size_hint|
		{
			use self::SupportedOperationalMode::*;
			
			let map = match assembling_for_architecture_variant.mode
			{
				Protected => &Known32BitSizes,
				Long => &Known64BitSizes,
			};
			match map.get(&might_be_size_hint.to_ascii_lowercase())
			{
				None => None,
				Some(size_hint) => Some(*size_hint)
			}
		})
	}
	
	#[inline(always)]
	fn parse_nosplit_hint(line: &mut Line) -> bool
	{
		line.optional_tag_and_eat_whitespace_after_it(|might_be_no_split_hint|
		{
			might_be_nosplit_hint.eq_ignore_ascii_case("nosplit")
		})
	}
	
	fn parse_prefixes_and_mnemonic<'a>(line: &mut Line<'a>) -> Result<(Option<RepeatPrefix>, Option<SegmentPrefix>, &'static MnemonicSignatureDefinition), InstructionParsingError>
	{
		let repeat_prefix_or_segment_prefix_or_mnemonic = line.mandatory_tag_ascii_lower_case()?;
		
		let result = match Self::parse_prefix(repeat_prefix_or_segment_prefix_or_mnemonic)
		{
			Some(Left(repeat_prefix)) =>
			{
				let segment_prefix_or_mnemonic = line.mandatory_tag_ascii_lower_case()?;
				
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
				let repeat_prefix_or_mnemonic = line.mandatory_tag_ascii_lower_case()?;
				
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
		
		match Mnemonic::mnemonic_tag_to_signature(potential_mnemonic_tag)
		{
			None => Err(InstructionParsingError("Invalid mnemonic")),
			Some(&signatures) => Ok((repeat_prefix, segment_prefix, signature)),
		}
	}
	
	#[inline(always)]
	fn parse_prefix(potential_prefix_tag_already_in_ascii_lower_case: &str) -> Option<Either<RepeatPrefix, SegmentPrefix>>
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
		
		KnownPrefixes.get(potential_prefix_tag_already_in_ascii_lower_case).map(|reference| *reference)
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
