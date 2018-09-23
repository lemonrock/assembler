// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A stateful assembler of the current stream of x64 instructions, labels and relocations.
#[derive(Debug)]
pub struct Assembler<'a>
{
	executable_anonymous_memory_map: &'a mut ExecutableAnonymousMemoryMap,
	start_instructions_pointer: *mut u8,
	instructions_pointer: *mut u8,
	end_instructions_pointer: *mut u8,
	labels: HashMap<Label, AssemblyOffset>,
	protected_mode_label_relocations: Vec<(Label, ProtectedModeRelocationLocation)>,
	long_mode_label_relocations: Vec<(Label, LongModeRelocationLocation)>,
}

impl<'a> Assembler<'a>
{
	/// Creates a new assembler.
	///
	/// Should be passed a pointer to a memory map with Write permissions (mprotect).
	#[inline(always)]
	pub(crate) fn new(executable_anonymous_memory_map: &'a mut ExecutableAnonymousMemoryMap, likely_number_of_labels_hint: usize) -> Self
	{
		executable_anonymous_memory_map.make_writable();
		
		let start_instructions_pointer = executable_anonymous_memory_map.address;
		let length = executable_anonymous_memory_map.length;
		
		Self
		{
			executable_anonymous_memory_map,
			start_instructions_pointer,
			instructions_pointer: start_instructions_pointer,
			end_instructions_pointer: unsafe { start_instructions_pointer.offset(length as isize) },
			labels: HashMap::with_capacity(likely_number_of_labels_hint),
			protected_mode_label_relocations: Vec::with_capacity(likely_number_of_labels_hint),
			long_mode_label_relocations: Vec::with_capacity(likely_number_of_labels_hint),
		}
	}
	
	/// Finishes assembly; at this point code is executable.
	#[inline(always)]
	pub fn finish(mut self)
	{
		self.patch_protected_mode_relocations();
		self.patch_long_mode_relocations();
		
		self.executable_anonymous_memory_map.make_executable();
	}
	
	#[inline(always)]
	fn patch_protected_mode_relocations(&mut self)
	{
		for (label, relocation_location) in self.protected_mode_label_relocations.iter()
		{
			let assembly_offset = self.labels.get(label).expect(&format!("Missing label '{:?}'", label));
			let target_address = *assembly_offset;
			relocation_location.set_relocation_value(self.start_instructions_pointer, target_address)
		}
		self.protected_mode_label_relocations.clear()
	}
	
	#[inline(always)]
	fn patch_long_mode_relocations(&mut self)
	{
		for (label, relocation_location) in self.long_mode_label_relocations.iter()
		{
			let assembly_offset = self.labels.get(label).expect(&format!("Missing label '{:?}'", label));
			let target_address = *assembly_offset;
			relocation_location.set_relocation_value(self.start_instructions_pointer, target_address)
		}
		self.long_mode_label_relocations.clear()
	}
	
	/// Creates a function pointer to the current offset that takes no arguments and returns a result of type `R`.
	///
	/// Resultant function may cause havoc until `self.finish()` is called.
	#[inline(always)]
	pub fn nullary_function_pointer<R>(&mut self) -> unsafe extern "C" fn() -> R
	{
		unsafe { transmute(self.instructions_pointer as usize) }
	}
	
	/// Creates a function pointer to the current offset that takes one argument of type `A` and returns a result of type `R`.
	///
	/// Resultant function may cause havoc until `self.finish()` is called.
	#[inline(always)]
	pub fn unary_function_pointer<R, A>(&mut self) -> unsafe extern "C" fn(A) -> R
	{
		unsafe { transmute(self.instructions_pointer as usize) }
	}
	
	/// Creates a function pointer to the current offset that takes two argument of types `A` and `B` and returns a result of type `R`.
	///
	/// Resultant function may cause havoc until `self.finish()` is called.
	#[inline(always)]
	pub fn binary_function_pointer<R, A, B>(&mut self) -> unsafe extern "C" fn(A, B) -> R
	{
		unsafe { transmute(self.instructions_pointer as usize) }
	}
	
	/// Creates a function pointer to the current offset that takes three argument of types `A`, `B` and `C` and returns a result of type `R`.
	///
	/// Resultant function may cause havoc until `self.finish()` is called.
	#[inline(always)]
	pub fn ternary_function_pointer<R, A, B, C>(&mut self) -> unsafe extern "C" fn(A, B, C) -> R
	{
		unsafe { transmute(self.instructions_pointer as usize) }
	}
	
	/// Creates a function pointer to the current offset that takes four argument of types `A`, `B`, `C` and `D` and returns a result of type `R`.
	///
	/// Resultant function may cause havoc until `self.finish()` is called.
	#[inline(always)]
	pub fn quaternary_function_pointer<R, A, B, C, D>(&mut self) -> unsafe extern "C" fn(A, B, C, D) -> R
	{
		unsafe { transmute(self.instructions_pointer as usize) }
	}
	
	/// Creates a function pointer to the current offset that takes five argument of types `A`, `B`, `C`, `D` and `E` and returns a result of type `R`.
	///
	/// Resultant function may cause havoc until `self.finish()` is called.
	#[inline(always)]
	pub fn quinary_function_pointer<R, A, B, C, D, E>(&mut self) -> unsafe extern "C" fn(A, B, C, D, E) -> R
	{
		unsafe { transmute(self.instructions_pointer as usize) }
	}
	
	/// Creates a function pointer to the current offset that takes six argument of types `A`, `B`, `C`, `D`, `E` and `F` and returns a result of type `R`.
	///
	/// Resultant function may cause havoc until `self.finish()` is called.
	#[inline(always)]
	pub fn senary_function_pointer<R, A, B, C, D, E, F>(&mut self) -> unsafe extern "C" fn(A, B, C, D, E, F) -> R
	{
		unsafe { transmute(self.instructions_pointer as usize) }
	}
	
	/// Pushes a label into the assembler at the current offset.
	#[inline(always)]
	pub fn push_label(&mut self, interned_label_prefix: u32, label_number: u32, offset: AssemblyOffset)
	{
		let label = Label
		{
			interned_label_prefix,
			label_number
		};
		
		let previous_value = self.labels.insert(label, offset);
		debug_assert!(previous_value.is_none(), "Label ({}, {}) has already been pushed", interned_label_prefix, label_number)
	}
	
	/// Pushes a labelled protected (32-bit) mode relocation (jump target) into the assembler at the current offset.
	#[inline(always)]
	pub fn push_labelled_protected_mode_relocation(&mut self, interned_label_prefix: u32, label_number: u32, relocation_offset: u8, relocation_size: ProtectedModeRelocationSize, relocation_kind: ProtectedModeRelocationKind)
	{
		let relocation_location = ProtectedModeRelocationLocation
		{
			assembly_offset: self.offset(),
			relocation: ProtectedModeRelocation
			{
				relocation_offset,
				relocation_size,
				relocation_kind,
			},
		};
		
		let label = Label
		{
			interned_label_prefix,
			label_number,
		};
		
		if let Some(assembly_offset) = self.labels.get(&label)
		{
			let target_address = *assembly_offset;
			relocation_location.set_relocation_value(self.start_instructions_pointer, target_address)
		}
		else
		{
			self.protected_mode_label_relocations.push((label, relocation_location))
		}
	}
	
	/// Pushes a labelled long (64-bit) mode relocation (jump target) into the assembler at the current offset.
	#[inline(always)]
	pub fn push_labelled_long_mode_relocation(&mut self, interned_label_prefix: u32, label_number: u32, relocation_offset: u8, relocation_size: LongModeRelocationSize)
	{
		let relocation_location = LongModeRelocationLocation
		{
			assembly_offset: self.offset(),
			relocation: LongModeRelocation
			{
				relocation_offset,
				relocation_size,
			},
		};
		
		let label = Label
		{
			interned_label_prefix,
			label_number,
		};
		
		if let Some(assembly_offset) = self.labels.get(&label)
		{
			let target_address = *assembly_offset;
			relocation_location.set_relocation_value(self.start_instructions_pointer, target_address)
		}
		else
		{
			self.long_mode_label_relocations.push((label, relocation_location))
		}
	}
	
	/// Pushes a bare protected (32-bit) mode relocation (jump target) into the assembler at the current offset.
	#[inline(always)]
	pub fn push_bare_protected_mode_relocation(&mut self, target_address: usize, relocation_offset: u8, relocation_size: ProtectedModeRelocationSize, relocation_kind: ProtectedModeRelocationKind)
	{
		let relocation_location = ProtectedModeRelocationLocation
		{
			assembly_offset: self.offset(),
			relocation: ProtectedModeRelocation
			{
				relocation_offset,
				relocation_size,
				relocation_kind,
			},
		};
		relocation_location.set_relocation_value(self.start_instructions_pointer, target_address);
	}
	
	/// Pushes a bare long (64-bit) mode relocation (jump target) into the assembler at the current offset.
	#[inline(always)]
	pub fn push_bare_long_mode_relocation(&mut self, target_address: usize, relocation_offset: u8, relocation_size: LongModeRelocationSize)
	{
		let relocation_location = LongModeRelocationLocation
		{
			assembly_offset: self.offset(),
			relocation: LongModeRelocation
			{
				relocation_offset,
				relocation_size,
			},
		};
		relocation_location.set_relocation_value(self.start_instructions_pointer, target_address);
	}
	
	/// Pushes value into the assembler at the current offset.
	#[inline(always)]
	pub fn push_value<T: PushValue>(&mut self, value: T)
	{
		let offset = size_of::<T>() as isize;
		
		debug_assert!(unsafe { self.instructions_pointer.offset(offset) as usize <= self.end_instructions_pointer as usize }, "Not enough space to push a value");
		unsafe
		{
			*(self.instructions_pointer as *mut T) = value;
			self.increment_instructions_pointer(offset)
		}
	}
	
	/// Pushes bytes into the assembler at the current offset.
	#[inline(always)]
	pub fn push_bytes(&mut self, bytes: &[u8])
	{
		let length = bytes.len();
		let offset = length as isize;
		
		debug_assert!(unsafe { self.instructions_pointer.offset(offset) as usize <= self.end_instructions_pointer as usize }, "Not enough space to push a value");
		unsafe
		{
			copy_nonoverlapping(bytes.as_ptr(), self.instructions_pointer, length);
			self.increment_instructions_pointer(offset)
		}
	}
	
	/// Uses `NOP`s (No Operation) opcodes to align to an alignment.
	///
	/// Efficient for alignments up to 64 (needed for AVX-512).
	#[inline(always)]
	pub fn push_alignment(&mut self, alignment: usize)
	{
		let offset = self.offset() % alignment;
		
		const NOP: u8 = 0x90;
		
		match offset
		{
			0 => (),
			
			1 => self.push_value(0x90u8),
			
			2 => self.push_value(0x9090u16),
			
			3 => self.push_bytes(&[NOP, NOP, NOP]),
			
			4 => self.push_value(0x90909090u32),
			
			5 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP]),
			
			6 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP]),
			
			7 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			8 => self.push_value(0x9090909090909090u64),
			
			9 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			10 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			11 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			12 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			13 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			14 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			15 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			16 => self.push_value(0x90909090909090909090909090909090u128),
			
			17 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			18 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			19 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			20 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			21 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			22 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			23 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			24 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			25 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			26 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			27 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			28 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			29 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			30 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			31 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			32 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			33 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			34 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			35 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			36 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			37 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			38 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			39 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			40 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			41 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			42 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			43 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			44 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			45 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			46 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			47 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			48 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			49 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			50 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			51 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			52 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			53 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			54 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			55 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			56 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			57 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			58 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			59 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			60 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			61 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			62 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			63 => self.push_bytes(&[NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP, NOP]),
			
			_ => for _ in 0 .. (alignment - offset)
			{
				self.push_value(0x90);
			},
		}
	}
	
	#[inline(always)]
	fn offset(&self) -> usize
	{
		(self.instructions_pointer as usize) - (self.start_instructions_pointer as usize)
	}
	
	#[inline(always)]
	unsafe fn increment_instructions_pointer(&mut self, offset: isize)
	{
		self.instructions_pointer = self.instructions_pointer.offset(offset)
	}
}
