// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A value that can be made little-endian; wraps use if `u8.to_le()` and related values that, in the Rust standard library, are not defined using a trait.
pub trait PushValue
{
	/// To little endian.
	#[inline(always)]
	fn to_little_endian(self) -> Self;
}

impl PushValue for u8
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

impl PushValue for u16
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

impl PushValue for u32
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

impl PushValue for u64
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

impl PushValue for u128
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

impl PushValue for i8
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

impl PushValue for i16
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

impl PushValue for i32
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

impl PushValue for i64
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

impl PushValue for i128
{
	#[inline(always)]
	fn to_little_endian(self) -> Self
	{
		self.to_le()
	}
}

