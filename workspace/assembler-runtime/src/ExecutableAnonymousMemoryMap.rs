// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Represents an executable memory map that can be used to generate program code into.
#[derive(Debug)]
pub struct ExecutableAnonymousMemoryMap
{
	address: *mut u8,
	length: usize,
}

impl Drop for ExecutableAnonymousMemoryMap
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { munmap(self.address as *mut _, self.length) };
	}
}

impl ExecutableAnonymousMemoryMap
{
	/// Create a new instance.
	///
	/// `length` is rounded up to the nearest power of two, and is floored at the smallest page size (4Kb).
	#[inline(always)]
	pub fn new(length: usize) -> io::Result<Self>
	{
		const PageSize: usize = 4096;
		const NoFileDescriptor: i32 = -1;
		const NoOffset: i64 = 0;
		
		let aligned_length = if length < PageSize
		{
			PageSize
		}
		else
		{
			length.next_power_of_two()
		};
		
		let result = unsafe { mmap(null_mut(), aligned_length, PROT_NONE, MAP_ANON | MAP_SHARED, NoFileDescriptor, NoOffset) };
		if unlikely!(result == MAP_FAILED)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			Ok
			(
				Self
				{
					address: result as *mut _,
					length,
				}
			)
		}
	}
	
	/// Get an assembler to this anonymous map.
	#[inline(always)]
	pub fn assembler(&mut self, likely_number_of_labels_hint: usize) -> Assembler
	{
		Assembler::new(self, likely_number_of_labels_hint)
	}
	
	#[inline(always)]
	pub(crate) fn make_writable(&mut self)
	{
		self.mprotect(PROT_WRITE)
	}
	
	#[inline(always)]
	pub(crate) fn make_executable(&mut self)
	{
		self.mprotect(PROT_EXEC)
	}
	
	#[inline(always)]
	fn mprotect(&self, protection_flags: i32)
	{
		unsafe { mprotect(self.address as *mut _, self.length, protection_flags) };
	}
}
