// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Kind of the mnemonic parameter (operand).
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum MnemonicParameterKind
{
	/// `i`.
	immediate,
	
	/// `o`.
	instruction_offset,
	
	/// `A`
	rax,
	
	/// `B`
	rcx,
	
	/// `C`
	rdx,
	
	/// `D`
	rbx,
	
	/// `E`
	rsp,
	
	/// `F`
	rbp,
	
	/// `G`
	rsi,
	
	/// `H`
	rdi,
	
	/// `I`
	r8,
	
	/// `J`
	r9,
	
	/// `K`
	r10,
	
	/// `L`
	r11,
	
	/// `M`
	r12,
	
	/// `N`
	r13,
	
	/// `O`
	r14,
	
	/// `P`
	r15,
	
	/// `Q`
	///
	/// Matches segment register ES.
	es,
	
	/// `R`
	///
	/// Matches segment register CS.
	cs,
	
	/// `S`
	///
	/// Matches segment register SS.
	ss,
	
	/// `T`
	///
	/// Matches segment register DS.
	ds,
	
	/// `U`
	///
	/// Matches segment register FS.
	fs,
	
	/// `V`
	///
	/// Matches segment register GS.
	gs,
	
	/// `W`
	///
	/// Matches register CR8.
	cr8,
	
	/// `X`
	///
	/// Matches register ST0.
	st0,
	
	/// `r`.
	legacy_register,
	
	/// `f`.
	floating_pointer_register,
	
	/// `x`.
	mmx_register,
	
	/// `y`.
	xmm_or_ymm_register,
	
	/// `s`.
	segment_register,
	
	/// `c`.
	control_register,
	
	/// `d`.
	debug_register,
	
	/// `b`.
	bound_register,
	
	/// `m`.
	memory,
	
	/// `v` (`r` and `m`).
	legacy_register_and_memory,
	
	/// `u` (`x` and `m`).
	mmx_register_and_memory,
	
	/// `w` (`y` and `m`).
	xmm_or_ymm_register_and_memory,
	
	/// `k`.
	///
	/// Vector Scaled Index Byte addressing, 32 bit result, size determines xmm or ymm.
	vector_scaled_index_byte_32,
	
	/// `l`.
	///
	/// Vector Scaled Index Byte addressing, 64 bit result, size determines xmm or ymm.
	vector_scaled_index_byte_64,
}

impl MnemonicParameterKind
{
	#[inline(always)]
	pub(crate) fn is_encoded(self) -> bool
	{
		use self::MnemonicParameterKind::*;
		
		match self
		{
			immediate | instruction_offset | legacy_register | floating_pointer_register | mmx_register | xmm_or_ymm_register | segment_register | control_register | debug_register | bound_register | memory | legacy_register_and_memory | mmx_register_and_memory | xmm_or_ymm_register_and_memory | vector_scaled_index_byte_32 | vector_scaled_index_byte_64 => true,
			
			_ => false,
		}
	}
}
