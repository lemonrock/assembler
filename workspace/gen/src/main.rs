// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
//#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(min_const_fn)]
#![feature(plugin)]
#![plugin(phf_macros)]


// TODO
#![feature(const_panic)]
#![feature(const_slice_len)]
#![feature(const_let)]


//! #gen
//!
//! This is a partial fork of dynasm, for compiling x86 assembler templates at build or run time.


extern crate arrayvec;
#[macro_use] extern crate bitflags;
extern crate either;
extern crate phf;


use ::phf::Map;
use ::std::cmp::min;
use ::std::io;
use ::std::io::Write;
use ::std::mem::transmute;


fn main()
{
	write_out().unwrap();
}

fn write_out() -> io::Result<()>
{
	let mut instructions_writer = Vec::with_capacity(1024 * 32);
	
//	println!();
//	println!("#[derive(Debug, Clone)]");
//	println!("pub struct InstructionsWriter");
//	println!("{{");
	
	for (key, signatures) in Mnemonic::map()
	{
		assert_ne!(signatures.len(), 0, "No signatures");
		
		let mnemonic_name = *key;
		
		let rust_safe_mnemonic_name = match mnemonic_name
		{
			"loop" => "loop_",
			"in" => "in_",
			_ => key,
		};
		
		// If there is more than one signature, write out an enum to hold the different kinds of signature
		if signatures.len() != 1
		{
			let source_code_identifier = naive_title_case_prefixed_with(mnemonic_name, "Signatures");
			
			let source_code_file_name = format!("{}.rs", &source_code_identifier);
			let mut mnemonic_signatures_writer = Vec::with_capacity(1024 * 32);
			
			write_preamble(&mut mnemonic_signatures_writer)?;
			
			writeln!(mnemonic_signatures_writer, "/// Permutations of the parameters the mnemonic `{}` takes.", mnemonic_name.to_ascii_uppercase())?;
			writeln!(mnemonic_signatures_writer, "#[derive(Debug, Clone)]")?;
			writeln!(mnemonic_signatures_writer, "pub enum {}", &source_code_identifier)?;
			writeln!(mnemonic_signatures_writer, "{{")?;
			
			let mut signature_index = 0;
			for signature in signatures.iter()
			{
				let enum_variant_name = signature_index_to_enum_name(signature_index);
				
				if signature_index != 0
				{
					writeln!(mnemonic_signatures_writer)?;
				}
				writeln!(mnemonic_signatures_writer, "\t#[allow(missing_docs)]")?;
				write!(mnemonic_signatures_writer, "\t{}(", enum_variant_name)?;
				{
					let mut parameter_index = 0;
					for parameter in signature.parameters.iter()
					{
						//let parameter_name = parameter_index_to_parameter_name(parameter_index);
						if parameter_index != 0
						{
							writeln!(mnemonic_signatures_writer, ", ")?;
						}
						
						// TODO: convert parameter kind, size into some sort of object.
						write!("()");
						
						parameter_index += 1;
					}
				}
				writeln!(mnemonic_signatures_writer, "),")?;
				signature_index += 1;
			}
			
			writeln!(mnemonic_signatures_writer, "}}")?;
		}
		
		// instructions:   pub fn mov(&mut self, MovSignatures) -> &mut Self
		
		
		
	}
	
	Ok(())
}

#[inline(always)]
fn write_preamble(&mut writer: impl Writer) -> io::Result<()>
{
	const Preamble: &'static str = r"// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.";
	
	writeln!(mnemonic_signatures_writer, "{}", Preamble);
	writeln!(mnemonic_signatures_writer);
	writeln!(mnemonic_signatures_writer)
}

#[inline(always)]
fn naive_title_case_prefixed_with(mnemonic_name: &mut str, prefix: &str) -> String
{
	let mut clone = mnemonic_name.to_string();
	let first_byte_slice = &mut clone[0 .. 1];
	first_byte_slice.make_ascii_uppercase();
	mnemonic_name.push_str(prefix);
	clone
}

#[inline(always)]
fn signature_index_to_enum_name(signature_index: u8) -> &'static str
{
	match signature_index
	{
		0 => "A",
		1 => "B",
		2 => "C",
		3 => "D",
		4 => "E",
		5 => "F",
		6 => "G",
		7 => "H",
		8 => "I",
		9 => "J",
		10 => "K",
		11 => "L",
		12 => "M",
		13 => "N",
		14 => "O",
		15 => "P",
		16 => "Q",
		17 => "R",
		18 => "S",
		19 => "T",
		20 => "U",
		21 => "V",
		22 => "W",
		23 => "X",
		24 => "Y",
		25 => "Z",
		26 => "AA",
		27 => "BB",
		28 => "CC",
		29 => "DD",
		30 => "EE",
		31 => "FF",
		32 => "GG",
		33 => "HH",
		34 => "II",
		35 => "JJ",
		36 => "KK",
		37 => "LL",
		38 => "MM",
		39 => "NN",
		_ => panic!("signature_index was '{}', ie 40 or more", signature_index),
	}
}

#[inline(always)]
fn parameter_index_to_parameter_name(parameter_index: u8) -> &'static str
{
	match parameter_index
	{
		0 => "a",
		1 => "b",
		2 => "c",
		3 => "d",
		4 => "e",
		5 => "f",
		6 => "g",
		7 => "h",
		_ => panic!("signature_index was '{}', ie 8 or more", parameter_index),
	}
}


include!("CpuFeature.rs");
include!("InstructionFlag.rs");
include!("Mnemonic.rs");
include!("MnemonicDefinitionSignature.rs");
include!("MnemonicParameter.rs");
include!("MnemonicParameterKind.rs");
include!("MnemonicParameterSize.rs");
include!("Register.rs");
include!("RegisterFamily.rs");
include!("RegisterIdentifier.rs");
include!("Size.rs");

static COMMENTS: &'static [&'static str] =
[
]
