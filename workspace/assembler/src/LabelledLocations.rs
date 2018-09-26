// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct LabelledLocations
{
	pointer: NonNull<usize>,
	length: usize,
	layout: Layout,
	
	next_label_index: usize,
}

impl Drop for LabelledLocations
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { dealloc(self.mut_u8_pointer(), self.layout) }
	}
}

impl LabelledLocations
{
	/// Using a value of 0xFFFFFFFF_FFFFFFFF is preferable to 0; this is because it is possible to map anonymous memory at location 0x00 and get a label to it; the limitations of the x64 address model to 48-bit addresses make this larger value impossible.
	const UnlabelledSentinel: usize = ::std::usize::MAX;
	
	#[inline(always)]
	pub(crate) fn new(likely_number_of_labels_hint: usize) -> Self
	{
		let length = likely_number_of_labels_hint;
		let layout = Self::layout(length);
		let allocation = unsafe { alloc(layout) };
		
		let mut this = Self
		{
			pointer: Self::from_allocation(allocation),
			length,
			layout,
			
			next_label_index: 0,
		};
		
		this.initialize_newly_allocated_memory(0, length);
		
		this
	}
	
	#[inline(always)]
	pub(crate) fn create_label(&mut self) -> Label
	{
		if unlikely!(self.next_label_index == self.length)
		{
			self.resize()
		}
		
		let label_index = self.next_label_index;
		self.next_label_index += 1;
		Label(label_index)
	}
	
	#[inline(always)]
	pub(crate) fn set(&mut self, label: Label, instruction_pointer: InstructionPointer)
	{
		let instruction_pointer_pointer = self.pointer_at_index(label.0);
		
		debug_assert_eq!(unsafe { *instruction_pointer_pointer }, Self::UnlabelledSentinel, "Label '{:?}' has already been used to label", label);
		unsafe { *instruction_pointer_pointer = instruction_pointer };
	}
	
	#[inline(always)]
	pub(crate) fn potential_target_instruction_pointer(&self, label: Label) -> InstructionPointer
	{
		unsafe { *self.pointer_at_index(label.0) }
	}
	
	#[inline(always)]
	fn resize(&mut self)
	{
		const ResizeScalar: usize = 2;
		
		let old_length = self.length;
		let new_length = old_length * ResizeScalar;
		let new_size = self.layout.size() * ResizeScalar;
		let allocation = unsafe { realloc(self.mut_u8_pointer(), self.layout, new_size) };
		self.pointer = Self::from_allocation(allocation);
		self.layout = Self::layout(new_length);
		self.length = new_length;
		self.initialize_newly_allocated_memory(old_length, old_length)
	}
	
	#[inline(always)]
	fn initialize_newly_allocated_memory(&mut self, from_index_inclusive: usize, length: usize)
	{
		unsafe { self.pointer_at_index(from_index_inclusive).write_bytes(Self::UnlabelledSentinel as u8, length) }
	}
	
	#[inline(always)]
	fn pointer_at_index(&self, index: usize) -> *mut usize
	{
		unsafe { (self.pointer.as_ptr() as *mut usize).offset(index as isize) }
	}
	
	#[inline(always)]
	fn mut_u8_pointer(&self) -> *mut u8
	{
		self.pointer.as_ptr() as *mut u8
	}
	
	#[inline(always)]
	fn from_allocation(allocation: *mut u8) -> NonNull<usize>
	{
		assert!(!allocation.is_null(), "Could not (re)allocate memory for labelled locations");
		unsafe { NonNull::new_unchecked(allocation as *mut usize) }
	}
	
	#[inline(always)]
	fn layout(length: usize) -> Layout
	{
		unsafe { Layout::from_size_align_unchecked(size_of::<usize>() * length, align_of::<usize>()) }
	}
}
