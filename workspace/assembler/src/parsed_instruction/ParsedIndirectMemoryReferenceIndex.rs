// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An index.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct ParsedIndirectMemoryReferenceIndex
{
	register: Register,
	scale: isize,
	expression: Option<RustExpression>,
}

impl ParsedIndirectMemoryReferenceIndex
{
	#[inline(always)]
	pub(crate) fn new_for_base_register(register: Register) -> Self
	{
		Self
		{
			register,
			scale: 1,
			expression: None,
		}
	}
	
	#[inline(always)]
	pub(crate) fn addressing_mode_is_vector_scaled_index_byte(index: &Option<Self>) -> bool
	{
		match index.as_ref()
		{
			None => false,
			Some(&ParsedIndirectMemoryReferenceIndex { register, .. }) => register.is_xmm(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_suitable_for_base_register(&self) -> bool
	{
		self.scale == 1 && self.expression.is_none()
	}
	
	#[inline(always)]
	pub(crate) fn does_not_use_xmm_register(index: &Option<Self>) -> bool
	{
		match *index
		{
			None => true,
			Some(ref this) => !this.register.is_xmm()
		}
	}
	
	#[inline(always)]
	pub(crate) fn scale_is_invalid(scale: isize) -> bool
	{
		Self::encode_scale_internal(scale).is_none()
	}
	
	#[inline(always)]
	pub(crate) fn encode_scale(scale: isize) -> u8
	{
		Self::encode_scale_internal(scale).expect("should already have been validated")
	}
	
	#[inline(always)]
	fn encode_scale_internal(scale: isize) -> Option<u8>
	{
		match scale
		{
			1 => Some(0),
			2 => Some(1),
			4 => Some(2),
			8 => Some(3),
			_ => None
		}
	}
}
