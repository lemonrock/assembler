// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct InstructionStream<W: Write>
{
	writer: W,
	runtime_assembler_instance_variable_name: String,
	accumulated_bytes_so_far: Vec<u8>,
	length_written_so_far: usize,
	next_interned_label_prefix: u32,
	interned_label_prefixes: HashMap<String, u32>,
}

impl<W: Write> InstructionStream<W>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(writer: W, runtime_assembler_instance_variable_name: String) -> Self
	{
		Self
		{
			writer,
			runtime_assembler_instance_variable_name,
			accumulated_bytes_so_far: Vec::with_capacity(4096),
			length_written_so_far: 0,
			next_interned_label_prefix: 0,
			interned_label_prefixes: HashMap::default(),
		}
	}
	
	/// Finishes writing.
	#[inline(always)]
	pub fn finish(&mut self) -> io::Result<()>
	{
		self.flush_bytes()?;
		
		self.writer.flush()
	}
	
	#[inline(always)]
	pub(crate) fn push_u8_if_some(&mut self, value: Option<u8>)
	{
		if let Some(value) = value
		{
			self.push_u8(value)
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_u8_if(&mut self, condition: bool, value: u8)
	{
		if condition
		{
			self.push_u8(value)
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_unsigned_constant(&mut self, value: u64, size: Size)
	{
		use self::Size::*;
		
		match size
		{
			BYTE =>
			{
				debug_assert!(value <= ::std::u8::MAX as u64, "value is larger than an u8");
				self.push_u8(value as u8)
			}
		
			WORD =>
			{
				debug_assert!(value <= ::std::u16::MAX as u64, "value is larger than an u16");
				self.push_u16(value as u16)
			}
			
			DWORD =>
			{
				debug_assert!(value <= ::std::u32::MAX as u64, "value is larger than an u32");
				self.push_u32(value as u32)
			}
			
			QWORD => self.push_u64(value),
			
			_ => panic!("size '{:?}' can not be bigger than a QWORD (64-bit)", size),
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_u64(&mut self, value: u64)
	{
		let values: [u8; 8] = unsafe { transmute(value.to_le()) };
		self.push_u8s(&values[..])
	}
	
	#[inline(always)]
	pub(crate) fn push_u32(&mut self, value: u32)
	{
		let values: [u8; 4] = unsafe { transmute(value.to_le()) };
		self.push_u8s(&values[..])
	}
	
	#[inline(always)]
	pub(crate) fn push_u16(&mut self, value: u16)
	{
		let values: [u8; 2] = unsafe { transmute(value.to_le()) };
		self.push_u8s(&values[..])
	}
	
	#[inline(always)]
	pub(crate) fn push_u8(&mut self, value: u8)
	{
		self.accumulated_bytes_so_far.push(value)
	}
	
	#[inline(always)]
	pub(crate) fn push_u8s(&mut self, values: &[u8])
	{
		for value in values.iter()
		{
			self.push_u8(*value)
		}
	}
	
	/// `label PREFIX NUMBER` where `PREFIX` is a constant UTF-8 string and `NUMBER` is a rust expression that resolves to a 64-bit label number.
	#[inline(always)]
	pub(crate) fn push_label(&mut self, label_prefix: &str, label_number_u32: RustExpression) -> io::Result<()>
	{
		let offset = self.length_written_so_far + self.accumulated_bytes_so_far.len();
		let interned_label_prefix = self.intern_label_prefix_string(label_prefix);
		writeln!(self.writer, "// push_label {} => {}", label_prefix, interned_label_prefix)?;
		writeln!(self.writer, "{}.push_label({}, ({}) as u32, {});", self.runtime_assembler_instance_variable_name, interned_label_prefix, label_number_u32.to_rust_code(), offset)
	}
	
	#[inline(always)]
	pub(crate) fn push_labelled_jump_target_reference(&mut self, offset: u8, size: Size, protected_mode_relocation_kind: RelocationKind, mode: SupportedOperationalMode, label_prefix: String, label_number_u32: RustExpression) -> io::Result<()>
	{
		self.flush_bytes()?;
		
		use self::RelocationKind::*;
		use self::Size::*;
		use self::SupportedOperationalMode::*;
		
		let interned_label_prefix = self.intern_label_prefix_string(&label_prefix);
		
		match mode
		{
			Protected =>
			{
				let relocation_size = match size
				{
					BYTE => "BYTE",
					WORD => "WORD",
					DWORD => "DWORD",
					_ => panic!("Relocations in protected mode must be for a size of BYTE, WORD, or DWORD"),
				};
				
				let relocation_kind = match protected_mode_relocation_kind
				{
					Relative => "Relative",
					Absolute => "Absolute",
					Extern => "Extern",
				};
				
				writeln!(self.writer, "{}.push_labelled_protected_mode_relocation({}, ({}) as u32, {}, ProtectedModeRelocationSize::{}, ProtectedModeRelocationKind::{});", self.runtime_assembler_instance_variable_name, interned_label_prefix, label_number_u32.to_rust_code(), offset, relocation_size, relocation_kind)
			}
			
			Long =>
			{
				debug_assert_eq!(protected_mode_relocation_kind, Relative, "Long mode does not support anything other than relative relocations");
				
				let relocation_size = match size
				{
					BYTE => "BYTE",
					WORD => "WORD",
					DWORD => "DWORD",
					QWORD => "QWORD",
					_ => panic!("Relocations in long mode must be for a size of BYTE, WORD, DWORD or QWORD"),
				};
				
				writeln!(self.writer, "{}.push_labelled_long_mode_relocation({}, ({}) as u32, {}, LongModeRelocationSize::{});", self.runtime_assembler_instance_variable_name, interned_label_prefix, label_number_u32.to_rust_code(), offset, relocation_size)
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_bare_jump_target_reference(&mut self, offset: u8, size: Size, protected_mode_relocation_kind: RelocationKind, mode: SupportedOperationalMode, target_address_usize: RustExpression) -> io::Result<()>
	{
		self.flush_bytes()?;
		
		use self::RelocationKind::*;
		use self::Size::*;
		use self::SupportedOperationalMode::*;
		
		match mode
		{
			Protected =>
			{
				let relocation_size = match size
				{
					BYTE => "BYTE",
					WORD => "WORD",
					DWORD => "DWORD",
					_ => panic!("Relocations in protected mode must be for a size of BYTE, WORD, or DWORD"),
				};
				
				let relocation_kind = match protected_mode_relocation_kind
				{
					Relative => "Relative",
					Absolute => "Absolute",
					Extern => "Extern",
				};
				
				writeln!(self.writer, "{}.push_bare_protected_mode_relocation(({}) as usize, {}, ProtectedModeRelocationSize::{}, ProtectedModeRelocationKind::{});", target_address_usize.to_rust_code(), self.runtime_assembler_instance_variable_name, offset, relocation_size, relocation_kind)
			}
			
			Long =>
			{
				debug_assert_eq!(protected_mode_relocation_kind, Relative, "Long mode does not support anything other than relative relocations");
				
				let relocation_size = match size
				{
					BYTE => "BYTE",
					WORD => "WORD",
					DWORD => "DWORD",
					QWORD => "QWORD",
					_ => panic!("Relocations in long mode must be for a size of BYTE, WORD, DWORD or QWORD"),
				};
				
				writeln!(self.writer, "{}.push_bare_long_mode_relocation(({}) as usize, {}, LongModeRelocationSize::{});", target_address_usize.to_rust_code(), self.runtime_assembler_instance_variable_name, offset, relocation_size)
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn push_unsigned_expression(&mut self, value: RustExpression, size: Size) -> io::Result<()>
	{
		self.push_value_from_expression(value, size, "u8", "u16", "u32", "u64", "u128")
	}
	
	#[inline(always)]
	pub(crate) fn push_signed_expression(&mut self, value: RustExpression, size: Size) -> io::Result<()>
	{
		self.push_value_from_expression(value, size, "i8", "i16", "i32", "i64", "i128")
	}
	
	#[inline(always)]
	fn push_scaled_index_byte_with_scale_calculated_by_expression(&mut self, scale: isize, expression: RustExpression, reg1: RegisterIdentifier, reg2: RegisterIdentifier) -> io::Result<()>
	{
		writeln!(self.writer);
		writeln!(self.writer, "// push_scaled_index_byte_with_scale_calculated_by_expression.");
		// See push_mod_rm_byte_or_scaled_index_byte() for reasoning behind this logic.
		// scaled_index_byte = ((scale * expression) & 0x02) << 6 | reg1.code_and_7() << 3 | reg2.code_and_7();
		writeln!(self.writer, "*instructions_pointer = (({} * (({} as u8)) & 0x02) << 6 | {} | {};", scale as u8, expression.to_rust_code(), reg1.code_and_7() << 3, reg2.code_and_7());
		writeln!(self.writer, "instructions_pointer = instructions_pointer.offset(1);");
		
		self.length_written_so_far += 1;
		
		Ok(())
	}
	
	#[inline(always)]
	fn push_value_from_expression(&mut self, value: RustExpression, size: Size, byte: &str, word: &str, dword: &str, qword: &str, oword: &str) -> io::Result<()>
	{
		self.flush_bytes()?;
		
		writeln!(self.writer);
		writeln!(self.writer, "// push_value_from_expression.");
		
		use self::Size::*;
		let (length, type_name) = match size
		{
			BYTE => (1, byte),
			
			WORD => (2, word),
			
			DWORD => (4, dword),
			
			QWORD => (8, qword),
			
			OWORD => (16, qword),
			
			_ => panic!("size '{:?}' is not supported; it must be one of BYTE, WORD, DWORD, QWORD or OWORD", size),
		};
		
		self.length_written_so_far += length;
		
		writeln!(self.writer, "{}.push_value(({}) as {});", self.runtime_assembler_instance_variable_name, value.to_rust_code(), type_name)
	}
	
	#[inline(always)]
	fn flush_bytes(&mut self) -> io::Result<()>
	{
		let length = self.accumulated_bytes_so_far.len();
		
		if length == 0
		{
			return Ok(())
		}
		
		self.length_written_so_far += length;
		
		writeln!(self.writer);
		writeln!(self.writer, "// flush_bytes ({}).", length);
		
		match length
		{
			1 => writeln!(self.writer, "{}.push_value(0x{:02X}u8);", self.runtime_assembler_instance_variable_name, unsafe { self.accumulated_bytes_so_far.get_unchecked(0) })?,
			
			// eg `assembler.push_value(0x44332211u32.to_le());` where the byte 0x11 is the first byte to be written.
			2 | 4 | 8 =>
			{
				write!(self.writer, "{}.push_value(0x", self.runtime_assembler_instance_variable_name)?;
				
				for index in (0 .. length).rev()
				{
					let byte = unsafe { self.accumulated_bytes_so_far.get_unchecked(index) };
					write!(self.writer, "{:02X}", byte)?;
				}
				
				writeln!(self.writer, "u{}.to_le());", length * 8)?;
			}
			
			length @ _ =>
			{
				write!(self.writer, "{}.push_bytes(&[", self.runtime_assembler_instance_variable_name)?;
				
				let iterator = self.accumulated_bytes_so_far.iter();
				let first = *iterator.next().unwrap();
				write!(self.writer, "0x{:02X}", first)?;
				for byte in iterator
				{
					write!(self.writer, ", 0x{:02X}", *byte)?;
				}
				writeln!(self.writer, "]);")?;
			},
		}
		
		self.accumulated_bytes_so_far.clear();
		
		Ok(())
	}
	
	#[inline(always)]
	fn intern_label_prefix_string(&mut self, prefix: &str) -> u32
	{
		let next_interned_label_prefix = self.next_interned_label_prefix;
		let interned_label_prefix = self.interned_label_prefixes.entry(prefix.to_string()).or_insert(next_interned_label_prefix);
		self.next_interned_label_prefix += 1;
		
		*interned_label_prefix
	}
}
