// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


bitflags!
{
	/// CPU 'features', effectively sets of supported additional instructions and registers.
	pub struct CpuFeatures: u32
	{
		/// CpuFeatures implicitly present in x86-64; effectively a default.
		const X64_IMPLICIT = 0x0000_0000;
		
		/// Floating Point Unit (FPU).
		const FPU = 0x0000_0001;
		
		/// Original Multimedia extensions (MMX).
		const MMX = 0x0000_0002;
		
		/// AMD 3DNow! extensions (similar to MMX).
		const TDNOW = 0x0000_0004;
		
		/// Streaming SIMD Extensions (SSE).
		const SSE = 0x0000_0008;
		
		/// Streaming SIMD Extensions 2 (SSE2).
		const SSE2 = 0x0000_0010;
		
		/// Streaming SIMD Extensions 3 (SSE3).
		const SSE3 = 0x0000_0020;
		
		/// Virtualization (eg Intel VT-x) instructions (VMX).
		const VMX = 0x0000_0040;
		
		/// Supplemental Streaming SIMD Extensions 3 (SSSE3).
		const SSSE3 = 0x0000_0080;
		
		/// Streaming SIMD Extensions 4a (SSE 4a).
		const SSE4A = 0x0000_0100;
		
		/// Streaming SIMD Extensions 4.1 (SSE4.1).
		const SSE41 = 0x0000_0200;
		
		/// Streaming SIMD Extensions 4.2 (SSE4.2).
		const SSE42 = 0x0000_0400;
		
		/// Streaming SIMD Extensions 5 (SSE5).
		const SSE5 = 0x0000_0800;
		
		/// Advanced Vector Extensions (AVX).
		const AVX = 0x0000_1000;
		
		/// Advanced Vector Extensions 2 (AVX2).
		const AVX2 = 0x0000_2000;
		
		/// Fused Multiply Add (FMA).
		const FMA = 0x0000_4000;
		
		/// Bit Manipulation Instructions 1 (BMI1).
		const BMI1 = 0x0000_8000;
		
		/// Bit Manipulation Instructions 2 (BMI2).
		const BMI2 = 0x0001_0000;
		
		/// Trailing Bit Manipulation, introduced by AMD Piledriver as an instruction to BM1 but dropped from AMD Zen (TBM).
		const TBM = 0x0002_0000;
		
		/// Restricted Transaction Memory (RTM).
		const RTM = 0x0004_0000;
		
		/// Invalidate Process Context Identifier (INVPCID).
		///
		/// Used only by the OS Kernel.
		const INVPCID = 0x0008_0000;
		
		/// Memory Protection Extensions (MPX).
		const MPX = 0x0010_0000;
		
		/// SHA-1 and SHA-2 acceleration instructions (SHA).
		const SHA = 0x0020_0000;
		
		/// Some cache pre-fetching instructions.
		const PREFETCHWT1 = 0x0040_0000;
		
		/// Cyrix only instructions.
		const CYRIX = 0x0080_0000;
		
		/// AMD only instructions.
		const AMD = 0x0100_0000;
		
		/// All.
		const All = Self::X64_IMPLICIT.bits | Self::FPU.bits | Self::MMX.bits | Self::TDNOW.bits | Self::SSE.bits | Self::SSE2.bits | Self::SSE3.bits | Self::VMX.bits | Self::SSSE3.bits | Self::SSE4A.bits | Self::SSE41.bits | Self::SSE42.bits | Self::SSE5.bits | Self::AVX.bits | Self::AVX2.bits | Self::FMA.bits | Self::BMI1.bits | Self::BMI2.bits | Self::TBM.bits | Self::RTM.bits | Self::INVPCID.bits | Self::MPX.bits | Self::SHA.bits | Self::PREFETCHWT1.bits | Self::CYRIX.bits | Self::AMD.bits;
	}
}

impl Default for CpuFeatures
{
	#[inline(always)]
	fn default() -> Self
	{
		CpuFeatures::X64_IMPLICIT
	}
}
