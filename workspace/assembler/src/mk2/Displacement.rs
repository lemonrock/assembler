// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


pub(crate) trait Displacement
{
	#[inline(always)]
	fn write(self, byte_emitter: &mut ByteEmitter);
}

impl Displacement for u8
{
	#[inline(always)]
	fn write(self, byte_emitter: &mut ByteEmitter)
	{
		byte_emitter.emit_u8(self.to_le())
	}
}

impl Displacement for u16
{
	#[inline(always)]
	fn write(self, byte_emitter: &mut ByteEmitter)
	{
		byte_emitter.emit_u16(self)
	}
}

impl Displacement for u32
{
	#[inline(always)]
	fn write(self, byte_emitter: &mut ByteEmitter)
	{
		byte_emitter.emit_u32(self)
	}
}

impl Displacement for u64
{
	#[inline(always)]
	fn write(self, byte_emitter: &mut ByteEmitter)
	{
		byte_emitter.emit_u64(self)
	}
}
