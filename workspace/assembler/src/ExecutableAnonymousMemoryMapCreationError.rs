// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of assembler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Errors from trying to instantiate an `ExecutableAnonymousMemoryMap`.
#[derive(Debug)]
pub enum ExecutableAnonymousMemoryMapCreationError
{
	/// mmap failed for `usize` aligned bytes.
	MMapFailed(io::Error, usize),

	/// mlock failed for `usize` aligned bytes.
	MLockFailed(io::Error, usize),
}

impl Display for ExecutableAnonymousMemoryMapCreationError
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::ExecutableAnonymousMemoryMapCreationError::*;

		match *self
		{
			MMapFailed(ref error, bytes) => write!(f, "mmap of {} aligned bytes failed with '{}'", bytes, error),
			MLockFailed(ref error, bytes) => write!(f, "mlock of {} aligned bytes failed with '{}'", bytes, error),
		}
	}
}

impl Error for ExecutableAnonymousMemoryMapCreationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::ExecutableAnonymousMemoryMapCreationError::*;

		match *self
		{
			MMapFailed(ref error, _) => Some(error),
			MLockFailed(ref error, _) => Some(error),
		}
	}
}
