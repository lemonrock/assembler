// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A stateful assembler of the current stream of x64 instructions, labels and relocations.
#[derive(Debug)]
pub struct Assembler
{
	start_instructions_pointer: *mut u8,
	instructions_pointer: *mut u8,
	end_instructions_pointer: *mut u8,
	labels: HashMap<Label, AssemblyOffset>,
	protected_mode_label_relocations: Vec<(Label, ProtectedModeRelocationLocation)>,
	long_mode_label_relocations: Vec<(Label, LongModeRelocationLocation)>,
}

impl Assembler
{
	/// Creates a new assembler.
	///
	/// Should be passed a pointer to a memory map with Write permissions (mprotect).
	#[inline(always)]
	pub fn new(start_instructions_pointer: *mut u8, length: usize, likely_number_of_labels_hint: usize) -> Self
	{
		Self
		{
			start_instructions_pointer,
			instructions_pointer: start_instructions_pointer,
			end_instructions_pointer: unsafe { start_instructions_pointer.offset(length as isize) },
			labels: HashMap::with_capacity(likely_number_of_labels_hint),
			protected_mode_label_relocations: Vec::with_capacity(likely_number_of_labels_hint),
			long_mode_label_relocations: Vec::with_capacity(likely_number_of_labels_hint),
		}
	}
	
	/// Finishes assembly.
	#[inline(always)]
	pub fn finish(mut self)
	{
		self.patch_protected_mode_relocations();
		self.patch_long_mode_relocations();
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
	
	/// Uses `NOP`s to align to an alignment.
	///
	// TODO: Inefficient for alignments greater than 32; this may impact AVX-512 usage, which uses 64-bit alignment.
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
