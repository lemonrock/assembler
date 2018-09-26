// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Implementations are function pointers with a known, absolute, virtual address.
pub trait FunctionPointer
{
	/// Absolute virtual address.
	#[inline(always)]
	fn absolute_virtual_address(self) -> usize;
}

impl FunctionPointer for usize
{
	#[inline(always)]
	fn absolute_virtual_address(self) -> usize
	{
		self
	}
}

impl<R> FunctionPointer for unsafe extern "C" fn() -> R
{
	#[inline(always)]
	fn absolute_virtual_address(self) -> usize
	{
		unsafe { transmute(self) }
	}
}

impl<R, A> FunctionPointer for unsafe extern "C" fn(A) -> R
{
	#[inline(always)]
	fn absolute_virtual_address(self) -> usize
	{
		unsafe { transmute(self) }
	}
}

impl<R, A, B> FunctionPointer for unsafe extern "C" fn(A, B) -> R
{
	#[inline(always)]
	fn absolute_virtual_address(self) -> usize
	{
		unsafe { transmute(self) }
	}
}

impl<R, A, B, C> FunctionPointer for unsafe extern "C" fn(A, B, C) -> R
{
	#[inline(always)]
	fn absolute_virtual_address(self) -> usize
	{
		unsafe { transmute(self) }
	}
}

impl<R, A, B, C, D> FunctionPointer for unsafe extern "C" fn(A, B, C, D) -> R
{
	#[inline(always)]
	fn absolute_virtual_address(self) -> usize
	{
		unsafe { transmute(self) }
	}
}

impl<R, A, B, C, D, E> FunctionPointer for unsafe extern "C" fn(A, B, C, D, E) -> R
{
	#[inline(always)]
	fn absolute_virtual_address(self) -> usize
	{
		unsafe { transmute(self) }
	}
}

impl<R, A, B, C, D, E, F> FunctionPointer for unsafe extern "C" fn(A, B, C, D, E, F) -> R
{
	#[inline(always)]
	fn absolute_virtual_address(self) -> usize
	{
		unsafe { transmute(self) }
	}
}
