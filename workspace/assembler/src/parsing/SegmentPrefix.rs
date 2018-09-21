// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum SegmentPrefix
{
	/// Segment `SS`.
	ss = 0x36,
	
	/// Segment `CS`.
	cs = 0x2E,
	
	/// Segment `DS`.
	ds = 0x3E,
	
	/// Segment `ES`.
	es = 0x26,
	
	/// Segment `FS`.
	fs = 0x64,
	
	/// Segment `GS`.
	gs = 0x65,
}

impl SegmentPrefix
{
	#[inline(always)]
	pub(crate) fn parse_prefix(potential_prefix_tag_already_in_ascii_lower_case: &str) -> Option<SegmentPrefix>
	{
		use self::SegmentPrefix::*;
		
		static KnownPrefixes: Map<&'static str, SegmentPrefix> = phf_map!
		{
			"ss" => ss,
			"cs" => cs,
			"ds" => ds,
			"es" => es,
			"fs" => fs,
			"gs" => gs,
		};
		
		KnownPrefixes.get(potential_prefix_tag_already_in_ascii_lower_case).map(|reference| *reference)
	}
	
	#[inline(always)]
	pub(crate) fn assign(this: Option<Self>) -> Option<u8>
	{
		match this
		{
			None => None,
			Some(segment_prefix) => Some(segment_prefix as u8),
		}
	}
}
