// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Represents an executable memory map that can be used to generate program code into.
#[derive(Debug)]
pub struct ExecutableAnonymousMemoryMap
{
	address: *mut u8,
	length: usize,
	ignore_mlock_failure: bool,
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
	///
	/// On Linux, `allocate_in_first_2Gb` should ideally be `true`.
	///
	/// On Linux, there are historic bugs in `mlock` which may require `ignore_mlock_failure` to be `true`.
	///
	/// Memory is created using an anonymous, shared mmap with no access rights (not even read) which is then locked (`mlock`'d).
	#[cfg_attr(not(any(target_os = "android", target_os = "linux")), allow(unused_variables))]
	#[inline(always)]
	pub fn new(length: usize, allocate_in_first_2Gb: bool, ignore_mlock_failure: bool) -> Result<Self, ExecutableAnonymousMemoryMapCreationError>
	{
		use self::ExecutableAnonymousMemoryMapCreationError::*;

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
		
		
		let flags =
		{
			#[cfg(any(target_os = "android", target_os = "linux"))]
			{
				let mut flags = MAP_ANON | MAP_SHARED;
				if allocate_in_first_2Gb
				{
					flags |= MAP_32BIT
				}
				flags
			}
			
			#[cfg(not(any(target_os = "android", target_os = "linux")))]
			{
				MAP_ANON | MAP_SHARED
			}
		};
		
		let result = unsafe { mmap(null_mut(), aligned_length, PROT_NONE, flags, NoFileDescriptor, NoOffset) };
		if result == MAP_FAILED
		{
			Err(MMapFailed(io::Error::last_os_error(), aligned_length))
		}
		else
		{
			let address = result;
			let result = unsafe { mlock(address, length) };
			if !ignore_mlock_failure && result != 0
			{
				if result == -1
				{
					return Err(MLockFailed(io::Error::last_os_error(), aligned_length))
				}
				else
				{
					panic!("Unexpected result code from mlock (new) '{}'", result)
				}
			}

			Ok
			(
				Self
				{
					address: address as *mut _,
					length,
					ignore_mlock_failure,
				}
			)
		}
	}
	
	/// Get an assembler to this anonymous map.
	#[inline(always)]
	pub fn instruction_stream(&mut self, instruction_stream_hints: &InstructionStreamHints) -> InstructionStream
	{
		InstructionStream::new(self, &instruction_stream_hints)
	}
	
	#[inline(always)]
	pub(crate) fn make_writable(&mut self)
	{
		self.mprotect(self.address, self.length, PROT_WRITE)
	}
	
	#[inline(always)]
	pub(crate) fn make_executable(&mut self)
	{
		self.mprotect(self.address, self.length, PROT_EXEC)
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub(crate) fn attempt_to_resize_in_place_whilst_writing(&mut self) -> io::Result<usize>
	{
		const NoFlags: i32 = 0;
		let old_length = self.length;
		let new_length = self.length * 2;
		let old_address = self.address;
		let new_address = unsafe { mremap(old_address as *mut _, old_length, new_length, NoFlags) };
		if unlikely!(new_address == MAP_FAILED)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			debug_assert_eq!(new_address as *mut u8, old_address, "address has changed");
			
			let new_memory_address = unsafe { old_address.add(old_length) };
			
			let result = unsafe { mlock(new_memory_address as *mut _, old_length) };
			if unlikely!(!self.ignore_mlock_failure && result != 0)
			{
				if likely!(result == -1)
				{
					return Err(io::Error::last_os_error())
				}
				else
				{
					panic!("Unexpected result code from mlock (attempt_to_resize_in_place_whilst_writing) '{}'", result)
				}
			}

			self.mprotect(new_memory_address, old_length, PROT_WRITE);
			self.length = new_length;
			Ok(new_length)
		}
	}
	
	#[inline(always)]
	fn mprotect(&self, address: *mut u8, length: usize, protection_flags: i32)
	{
		unsafe { mprotect(address as *mut _, length, protection_flags) };
	}
}
