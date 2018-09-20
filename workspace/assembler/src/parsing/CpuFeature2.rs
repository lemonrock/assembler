// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// CPU Features.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CpuFeature2
{
	/// Floating Point Unit (FPU).
	FPU,
	
	/// Original Multimedia extensions (MMX).
	MMX,
	
	/// Multimedia extensions (MMX) additions by Cyrix.
	MMX_CYRIX,
	
	/// AMD 3DNow! extensions (similar to MMX).
	TDNOW,
	
	/// AMD 3DNow! extensions (similar to MMX) additions by Cyrix.
	TDNOW_CYRIX,
	
	/// Streaming SIMD Extensions (SSE).
	SSE,
	
	/// Streaming SIMD Extensions 2 (SSE2).
	SSE2,
	
	/// Streaming SIMD Extensions 3 (SSE3).
	SSE3,
	
	/// Virtualization (eg Intel VT-x) instructions (VMX).
	VMX,
	
	/// Virtualization (eg AMD-V) instructions (VMX).
	VMX_AMD,
	
	/// Supplemental Streaming SIMD Extensions 3 (SSSE3).
	SSSE3,
	
	/// Streaming SIMD Extensions 4a (SSE 4a).
	SSE4A,
	
	/// Streaming SIMD Extensions 4a (SSE 4a) additions by AMD.
	SSE4A_AMD,
	
	/// Streaming SIMD Extensions 4.1 (SSE4.1).
	SSE41,
	
	/// Streaming SIMD Extensions 4.2 (SSE4.2).
	SSE42,
	
	/// Streaming SIMD Extensions 5 (SSE5).
	SSE5,
	
	/// Advanced Vector Extensions (AVX).
	AVX,
	
	/// Advanced Vector Extensions 2 (AVX2).
	AVX2,
	
	/// Fused Multiply Add (FMA).
	FMA,
	
	/// Bit Manipulation Instructions 1 (BMI1).
	BMI1,
	
	/// Bit Manipulation Instructions 2 (BMI2).
	BMI2,
	
	/// Trailing Bit Manipulation, introduced by AMD Piledriver as an instruction to BM1 but dropped from AMD Zen (TBM).
	TBM,
	
	/// Restricted Transaction Memory (RTM).
	RTM,
	
	/// Invalidate Process Context Identifier (INVPCID).
	///
	/// Used only by the OS Kernel.
	INVPCID,
	
	/// Memory Protection Extensions (MPX).
	MPX,
	
	/// SHA-1 and SHA-2 acceleration instructions (SHA).
	SHA,
	
	/// Some cache pre-fetching instructions.
	PREFETCHWT1,
	
	/// Cyrix only instructions.
	CYRIX,
	
	/// AMD only instructions.
	AMD,
}
