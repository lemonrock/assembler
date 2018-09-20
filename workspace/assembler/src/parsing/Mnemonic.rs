// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


pub(crate) struct Mnemonic;

impl Mnemonic
{
	pub(crate) fn mnemonic_tag_to_signature<'a>(potential_mnemonic: &str) -> Option<&'a &'static [MnemonicDefinitionSignature]>
	{
		use self::CpuFeature2::*;
		use self::MnemonicParameterKind::*;
		use self::MnemonicParameterSize::*;
		use self::OpcodeBytes::*;
		use self::RegisterIdentifier::*;
		use self::Size::*;
		
		#[inline(always)]
		const fn parameter(kind: MnemonicParameterKind, size: MnemonicParameterSize) -> MnemonicParameter
		{
			MnemonicParameter::new(kind, size)
		}
		
		#[inline(always)]
		const fn signature(parameters: &'static [MnemonicParameter], opcode_bytes: OpcodeBytes, register_identifier: Option<RegisterIdentifier>, instruction_flags: u32, cpu_feature_required: Option<CpuFeature2>) -> MnemonicDefinitionSignature
		{
			MnemonicDefinitionSignature::new(parameters, opcode_bytes, register_identifier, instruction_flags, cpu_feature_required)
		}
		
		macro_rules! signatures
		{
			($($item: expr, )*) =>
			{
				{
					static SLICE: &'static [MnemonicDefinitionSignature] = &[$($item, )*];
					SLICE
				}
			}
		}
		
		macro_rules! parameters
		{
			() =>
			{
				{
					static SLICE: &'static [MnemonicParameter] = &[];
					SLICE
				}
			};
			
			($first: expr$(, $item: expr)*) =>
			{
				{
					static SLICE: &'static [MnemonicParameter] = &[$first$(, $item)*];
					SLICE
				}
			};
		}
		
		macro_rules! flags
		{
			() =>
			{
				0x00
			};
			
			($a: tt) =>
			{
				InstructionFlag2::$a.bits
			};
			
			($a: tt, $b: tt) =>
			{
				InstructionFlag2::$a.bits | InstructionFlag2::$b.bits
			};
			
			($a: tt, $b: tt, $c: tt) =>
			{
				InstructionFlag2::$a.bits | InstructionFlag2::$b.bits | InstructionFlag2::$c.bits
			};
			
			($a: tt, $b: tt, $c: tt, $d:tt) =>
			{
				InstructionFlag2::$a.bits | InstructionFlag2::$b.bits | InstructionFlag2::$c.bits | InstructionFlag2::$d.bits
			}
		}
		
		static MnemonicsToSignatures: Map<&'static str, &'static [MnemonicDefinitionSignature]> = phf_map!
		{
			"aaa" => signatures!
			[
				signature(parameters![], _1(0x37), None, flags!(x86_only), None),
			],

			"aad" => signatures!
			[
				signature(parameters![], _2(0xD5, 0x0A), None, flags!(x86_only), None),
			],

			"aam" => signatures!
			[
				signature(parameters![], _2(0xD4, 0x0A), None, flags!(x86_only), None),
			],

			"aas" => signatures!
			[
				signature(parameters![], _1(0x3F), None, flags!(x86_only), None),
			],

			"adc" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x14), None, flags!(), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RDX), flags!(lock), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x10), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RDX), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x10), None, flags!(enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0x12), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RDX), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(rax, AllPossible), parameter(immediate, AllPossible)], _1(0x15), None, flags!(auto_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RDX), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RDX), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x11), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RDX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _1(0x11), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _1(0x13), None, flags!(auto_size), None),
			],

			"adcx" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(legacy_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0xF6), None, flags!(with_rex_w, pref_66), None),
			],

			"add" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x04), None, flags!(), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RAX), flags!(lock), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x00), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RAX), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x00), None, flags!(enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0x02), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RAX), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(rax, AllPossible), parameter(immediate, AllPossible)], _1(0x05), None, flags!(auto_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RAX), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RAX), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x01), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RAX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _1(0x01), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _1(0x03), None, flags!(auto_size), None),
			],

			"addpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x58), None, flags!(pref_66), Some(SSE2)),
			],

			"addps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x58), None, flags!(), Some(SSE)),
			],

			"addsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x58), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x58), None, flags!(pref_F2), Some(SSE2)),
			],

			"addss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x58), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x58), None, flags!(pref_F3), Some(SSE)),
			],

			"addsubpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xD0), None, flags!(pref_66), Some(SSE3)),
			],

			"addsubps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xD0), None, flags!(pref_F2), Some(SSE3)),
			],

			"adox" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(legacy_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0xF6), None, flags!(with_rex_w, pref_F3), None),
			],

			"aesdec" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xDE), None, flags!(pref_66), Some(SSE)),
			],

			"aesdeclast" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xDF), None, flags!(pref_66), Some(SSE)),
			],

			"aesenc" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xDC), None, flags!(pref_66), Some(SSE)),
			],

			"aesenclast" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xDD), None, flags!(pref_66), Some(SSE)),
			],

			"aesimc" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xDB), None, flags!(pref_66), Some(SSE)),
			],

			"aeskeygenassist" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0xDF), None, flags!(pref_66), Some(SSE)),
			],

			"and" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x24), None, flags!(), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RSP), flags!(lock), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x20), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RSP), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x20), None, flags!(enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0x22), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RSP), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(rax, AllPossible), parameter(immediate, AllPossible)], _1(0x25), None, flags!(auto_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RSP), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RSP), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x21), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RSP), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _1(0x21), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _1(0x23), None, flags!(auto_size), None),
			],

			"andn" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x02, 0xF2), None, flags!(vex_op, auto_rex_w), Some(BMI1)),
			],

			"andnpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x55), None, flags!(pref_66), Some(SSE2)),
			],

			"andnps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x55), None, flags!(), Some(SSE)),
			],

			"andpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x54), None, flags!(pref_66), Some(SSE2)),
			],

			"andps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x54), None, flags!(), Some(SSE)),
			],

			"arpl" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(WORD)), parameter(legacy_register, Fixed(WORD))], _1(0x63), None, flags!(x86_only), None),
			],

			"bextr" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(DWORD))], _2(0x10, 0x10), None, flags!(xop_op, auto_rex_w), Some(TBM)),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x02, 0xF7), None, flags!(vex_op, auto_rex_w, enc_mr), Some(BMI1)),
			],

			"blcfill" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x09, 0x01), Some(RCX), flags!(xop_op, auto_rex_w, enc_vm), Some(TBM)),
			],

			"blci" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x09, 0x02), Some(RSI), flags!(xop_op, auto_rex_w, enc_vm), Some(TBM)),
			],

			"blcic" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x09, 0x01), Some(RBP), flags!(xop_op, auto_rex_w, enc_vm), Some(TBM)),
			],

			"blcmsk" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x09, 0x02), Some(RCX), flags!(xop_op, auto_rex_w, enc_vm), Some(TBM)),
			],

			"blcs" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x09, 0x01), Some(RBX), flags!(xop_op, auto_rex_w, enc_vm), Some(TBM)),
			],

			"blendpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0D), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0D), None, flags!(pref_66), Some(SSE41)),
			],

			"blendps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0C), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0C), None, flags!(pref_66), Some(SSE41)),
			],

			"blendvpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x15), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x15), None, flags!(pref_66), Some(SSE41)),
			],

			"blendvps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x14), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x14), None, flags!(pref_66), Some(SSE41)),
			],

			"blsfill" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x09, 0x01), Some(RDX), flags!(xop_op, auto_rex_w, enc_vm), Some(TBM)),
			],

			"blsi" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x02, 0xF3), Some(RBX), flags!(vex_op, auto_rex_w, enc_vm), Some(BMI1)),
			],

			"blsic" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x09, 0x01), Some(RSI), flags!(xop_op, auto_rex_w, enc_vm), Some(TBM)),
			],

			"blsmsk" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x02, 0xF3), Some(RDX), flags!(vex_op, auto_rex_w, enc_vm), Some(BMI1)),
			],

			"blsr" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x02, 0xF3), Some(RCX), flags!(vex_op, auto_rex_w, enc_vm), Some(BMI1)),
			],

			"bndcl" => signatures!
			[
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(memory, NoSize)], _2(0x0F, 0x1A), None, flags!(pref_F3), Some(MPX)),
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0x1A), None, flags!(pref_F3), Some(MPX)),
			],

			"bndcn" => signatures!
			[
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(memory, NoSize)], _2(0x0F, 0x1B), None, flags!(pref_F2), Some(MPX)),
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0x1B), None, flags!(pref_F2), Some(MPX)),
			],

			"bndcu" => signatures!
			[
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(memory, NoSize)], _2(0x0F, 0x1A), None, flags!(pref_F2), Some(MPX)),
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0x1A), None, flags!(pref_F2), Some(MPX)),
			],

			"bndldx" => signatures!
			[
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(memory, NoSize)], _2(0x0F, 0x1A), None, flags!(enc_mib), Some(MPX)),
			],

			"bndmk" => signatures!
			[
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(memory, NoSize)], _2(0x0F, 0x1B), None, flags!(enc_mib, pref_F3), Some(MPX)),
			],

			"bndmov" => signatures!
			[
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(bound_register, Fixed(OWORD))], _2(0x0F, 0x1A), None, flags!(pref_66), Some(MPX)),
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(bound_register, Fixed(OWORD))], _2(0x0F, 0x1B), None, flags!(enc_mr, pref_66), Some(MPX)),
				signature(parameters![parameter(bound_register, Fixed(OWORD)), parameter(memory, NoSize)], _2(0x0F, 0x1A), None, flags!(pref_66), Some(MPX)),
				signature(parameters![parameter(memory, NoSize), parameter(bound_register, Fixed(OWORD))], _2(0x0F, 0x1B), None, flags!(enc_mr, pref_66), Some(MPX)),
			],

			"bndstx" => signatures!
			[
				signature(parameters![parameter(memory, NoSize), parameter(bound_register, Fixed(OWORD))], _2(0x0F, 0x1B), None, flags!(enc_mr, enc_mib), Some(MPX)),
			],

			"bound" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, NoSize)], _1(0x62), None, flags!(auto_size, x86_only), None),
			],

			"bsf" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0xBC), None, flags!(auto_size), None),
			],

			"bsr" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0xBD), None, flags!(auto_size), None),
			],

			"bswap" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible)], _2(0x0F, 0xC8), None, flags!(auto_rex_w, short_arg), None),
			],

			"bt" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xBA), Some(RSP), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xA3), None, flags!(auto_size, enc_mr), None),
			],

			"btc" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xBA), Some(RDI), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xBA), Some(RDI), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xBB), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xBB), None, flags!(auto_size, enc_mr), None),
			],

			"btr" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xBA), Some(RSI), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xBA), Some(RSI), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xB3), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xB3), None, flags!(auto_size, enc_mr), None),
			],

			"bts" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xBA), Some(RBP), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xBA), Some(RBP), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xAB), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xAB), None, flags!(auto_size, enc_mr), None),
			],

			"bzhi" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x02, 0xF5), None, flags!(vex_op, auto_rex_w, enc_mr), Some(BMI2)),
			],

			"call" => signatures!
			[
				signature(parameters![parameter(immediate, Fixed(WORD)), parameter(immediate, Fixed(WORD))], _1(0x9A), None, flags!(x86_only, word_size, exact_size), None),
				signature(parameters![parameter(immediate, Fixed(DWORD)), parameter(immediate, Fixed(WORD))], _1(0x9A), None, flags!(x86_only), None),
				signature(parameters![parameter(memory, Fixed(FWORD))], _1(0xFF), Some(RBX), flags!(x86_only, exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _1(0xE8), None, flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible)], _1(0xFF), Some(RDX), flags!(auto_no_32), None),
			],

			"callf" => signatures!
			[
				signature(parameters![parameter(immediate, Fixed(WORD)), parameter(immediate, Fixed(WORD))], _1(0x9A), None, flags!(x86_only, word_size, exact_size), None),
				signature(parameters![parameter(immediate, Fixed(DWORD)), parameter(immediate, Fixed(WORD))], _1(0x9A), None, flags!(x86_only), None),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xFF), Some(RBX), flags!(x86_only, word_size, exact_size), None),
				signature(parameters![parameter(memory, Fixed(FWORD))], _1(0xFF), Some(RBX), flags!(x86_only), None),
			],

			"cbw" => signatures!
			[
				signature(parameters![], _1(0x98), None, flags!(word_size), None),
			],

			"cdq" => signatures!
			[
				signature(parameters![], _1(0x99), None, flags!(), None),
			],

			"cdqe" => signatures!
			[
				signature(parameters![], _1(0x98), None, flags!(with_rex_w), None),
			],

			"clac" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xCA), None, flags!(), None),
			],

			"clc" => signatures!
			[
				signature(parameters![], _1(0xF8), None, flags!(), None),
			],

			"cld" => signatures!
			[
				signature(parameters![], _1(0xFC), None, flags!(), None),
			],

			"clflush" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _2(0x0F, 0xAE), Some(RDI), flags!(), Some(SSE2)),
			],

			"clgi" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xDD), None, flags!(), Some(VMX_AMD)),
			],

			"cli" => signatures!
			[
				signature(parameters![], _1(0xFA), None, flags!(), None),
			],

			"clts" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x06), None, flags!(), None),
			],

			"clzero" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xFC), None, flags!(), Some(AMD)),
			],

			"cmc" => signatures!
			[
				signature(parameters![], _1(0xF5), None, flags!(), None),
			],

			"cmova" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x47), None, flags!(auto_size), None),
			],

			"cmovae" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x43), None, flags!(auto_size), None),
			],

			"cmovb" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x42), None, flags!(auto_size), None),
			],

			"cmovbe" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x46), None, flags!(auto_size), None),
			],

			"cmovc" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x42), None, flags!(auto_size), None),
			],

			"cmove" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x44), None, flags!(auto_size), None),
			],

			"cmovg" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4F), None, flags!(auto_size), None),
			],

			"cmovge" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4D), None, flags!(auto_size), None),
			],

			"cmovl" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4C), None, flags!(auto_size), None),
			],

			"cmovle" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4E), None, flags!(auto_size), None),
			],

			"cmovna" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x46), None, flags!(auto_size), None),
			],

			"cmovnae" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x42), None, flags!(auto_size), None),
			],

			"cmovnb" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x43), None, flags!(auto_size), None),
			],

			"cmovnbe" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x47), None, flags!(auto_size), None),
			],

			"cmovnc" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x43), None, flags!(auto_size), None),
			],

			"cmovne" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x45), None, flags!(auto_size), None),
			],

			"cmovng" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4E), None, flags!(auto_size), None),
			],

			"cmovnge" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4C), None, flags!(auto_size), None),
			],

			"cmovnl" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4D), None, flags!(auto_size), None),
			],

			"cmovnle" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4F), None, flags!(auto_size), None),
			],

			"cmovno" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x41), None, flags!(auto_size), None),
			],

			"cmovnp" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4B), None, flags!(auto_size), None),
			],

			"cmovns" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x49), None, flags!(auto_size), None),
			],

			"cmovnz" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x45), None, flags!(auto_size), None),
			],

			"cmovo" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x40), None, flags!(auto_size), None),
			],

			"cmovp" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4A), None, flags!(auto_size), None),
			],

			"cmovpe" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4A), None, flags!(auto_size), None),
			],

			"cmovpo" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x4B), None, flags!(auto_size), None),
			],

			"cmovs" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x48), None, flags!(auto_size), None),
			],

			"cmovz" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x44), None, flags!(auto_size), None),
			],

			"cmp" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x3C), None, flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0x3A), None, flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RDI), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x38), None, flags!(enc_mr), None),
				signature(parameters![parameter(rax, AllPossible), parameter(immediate, AllPossible)], _1(0x3D), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _1(0x3B), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RDI), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RDI), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x39), None, flags!(auto_size, enc_mr), None),
			],

			"cmpeqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x00), None, flags!(pref_66, imm_op), Some(SSE2)),
			],

			"cmpeqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x00), None, flags!(imm_op), Some(SSE)),
			],

			"cmpeqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0xC2, 0x00), None, flags!(pref_F2, imm_op), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x00), None, flags!(pref_F2, imm_op), Some(SSE2)),
			],

			"cmpeqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0xC2, 0x00), None, flags!(pref_F3, imm_op), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x00), None, flags!(pref_F3, imm_op), Some(SSE)),
			],

			"cmplepd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x02), None, flags!(imm_op, pref_66), Some(SSE2)),
			],

			"cmpleps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x02), None, flags!(imm_op), Some(SSE)),
			],

			"cmplesd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0xC2, 0x02), None, flags!(pref_F2, imm_op), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x02), None, flags!(pref_F2, imm_op), Some(SSE2)),
			],

			"cmpless" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0xC2, 0x02), None, flags!(pref_F3, imm_op), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x02), None, flags!(pref_F3, imm_op), Some(SSE)),
			],

			"cmpltpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x01), None, flags!(imm_op, pref_66), Some(SSE2)),
			],

			"cmpltps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x01), None, flags!(imm_op), Some(SSE)),
			],

			"cmpltsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0xC2, 0x01), None, flags!(pref_F2, imm_op), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x01), None, flags!(pref_F2, imm_op), Some(SSE2)),
			],

			"cmpltss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0xC2, 0x01), None, flags!(imm_op, pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x01), None, flags!(imm_op, pref_F3), Some(SSE)),
			],

			"cmpneqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x04), None, flags!(pref_66, imm_op), Some(SSE2)),
			],

			"cmpneqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x04), None, flags!(imm_op), Some(SSE)),
			],

			"cmpneqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0xC2, 0x04), None, flags!(pref_F2, imm_op), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x04), None, flags!(pref_F2, imm_op), Some(SSE2)),
			],

			"cmpneqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0xC2, 0x04), None, flags!(imm_op, pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x04), None, flags!(imm_op, pref_F3), Some(SSE)),
			],

			"cmpnlepd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x06), None, flags!(imm_op, pref_66), Some(SSE2)),
			],

			"cmpnleps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x06), None, flags!(imm_op), Some(SSE)),
			],

			"cmpnlesd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0xC2, 0x06), None, flags!(pref_F2, imm_op), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x06), None, flags!(pref_F2, imm_op), Some(SSE2)),
			],

			"cmpnless" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0xC2, 0x06), None, flags!(imm_op, pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x06), None, flags!(imm_op, pref_F3), Some(SSE)),
			],

			"cmpnltpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x05), None, flags!(pref_66, imm_op), Some(SSE2)),
			],

			"cmpnltps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x05), None, flags!(imm_op), Some(SSE)),
			],

			"cmpnltsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0xC2, 0x05), None, flags!(imm_op, pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x05), None, flags!(imm_op, pref_F2), Some(SSE2)),
			],

			"cmpnltss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0xC2, 0x05), None, flags!(pref_F3, imm_op), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x05), None, flags!(pref_F3, imm_op), Some(SSE)),
			],

			"cmpordpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x07), None, flags!(imm_op, pref_66), Some(SSE2)),
			],

			"cmpordps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x07), None, flags!(imm_op), Some(SSE)),
			],

			"cmpordsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0xC2, 0x07), None, flags!(pref_F2, imm_op), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x07), None, flags!(pref_F2, imm_op), Some(SSE2)),
			],

			"cmpordss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0xC2, 0x07), None, flags!(imm_op, pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x07), None, flags!(imm_op, pref_F3), Some(SSE)),
			],

			"cmppd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC2), None, flags!(pref_66), Some(SSE2)),
			],

			"cmpps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, NoSize), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC2), None, flags!(), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC2), None, flags!(), Some(SSE)),
			],

			"cmpsb" => signatures!
			[
				signature(parameters![], _1(0xA6), None, flags!(repe), None),
			],

			"cmpsd" => signatures!
			[
				signature(parameters![], _1(0xA7), None, flags!(repe), None),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC2), None, flags!(pref_F2), Some(SSE2)),
			],

			"cmpsq" => signatures!
			[
				signature(parameters![], _1(0xA7), None, flags!(repe, with_rex_w), None),
			],

			"cmpss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, NoSize), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC2), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC2), None, flags!(pref_F3), Some(SSE)),
			],

			"cmpsw" => signatures!
			[
				signature(parameters![], _1(0xA7), None, flags!(repe, word_size), None),
			],

			"cmpunordpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x03), None, flags!(pref_66, imm_op), Some(SSE2)),
			],

			"cmpunordps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0xC2, 0x03), None, flags!(imm_op), Some(SSE)),
			],

			"cmpunordsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0xC2, 0x03), None, flags!(pref_F2, imm_op), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x03), None, flags!(pref_F2, imm_op), Some(SSE2)),
			],

			"cmpunordss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0xC2, 0x03), None, flags!(pref_F3, imm_op), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0xC2, 0x03), None, flags!(pref_F3, imm_op), Some(SSE)),
			],

			"cmpxchg" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _2(0x0F, 0xB0), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _2(0x0F, 0xB0), None, flags!(enc_mr), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xB1), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xB1), None, flags!(auto_size, enc_mr), None),
			],

			"cmpxchg16b" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(OWORD))], _2(0x0F, 0xC7), Some(RCX), flags!(lock, with_rex_w), None),
			],

			"cmpxchg8b" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD))], _2(0x0F, 0xC7), Some(RCX), flags!(lock), None),
			],

			"comisd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x2F), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2F), None, flags!(pref_66), Some(SSE2)),
			],

			"comiss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x2F), None, flags!(), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2F), None, flags!(), Some(SSE)),
			],

			"cpu_read" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x3D), None, flags!(), Some(CYRIX)),
			],

			"cpu_write" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x3C), None, flags!(), Some(CYRIX)),
			],

			"cpuid" => signatures!
			[
				signature(parameters![], _2(0x0F, 0xA2), None, flags!(), None),
			],

			"cqo" => signatures!
			[
				signature(parameters![], _1(0x99), None, flags!(with_rex_w), None),
			],

			"crc32" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, Fixed(BYTE))], _3(0x0F, 0x38, 0xF0), None, flags!(auto_rex_w, pref_F2, exact_size), None),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(legacy_register_and_memory, Fixed(WORD))], _3(0x0F, 0x38, 0xF1), None, flags!(word_size, pref_F2, exact_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _3(0x0F, 0x38, 0xF1), None, flags!(auto_rex_w, pref_F2, exact_size), None),
			],

			"cvtdq2pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0xE6), None, flags!(pref_F3), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0xE6), None, flags!(pref_F3), Some(SSE2)),
			],

			"cvtdq2ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5B), None, flags!(), Some(SSE2)),
			],

			"cvtpd2dq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE6), None, flags!(pref_F2), Some(SSE2)),
			],

			"cvtpd2pi" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x2D), None, flags!(pref_66), Some(SSE2)),
			],

			"cvtpd2ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5A), None, flags!(pref_66), Some(SSE2)),
			],

			"cvtpi2pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x2A), None, flags!(pref_66), Some(SSE2)),
			],

			"cvtpi2ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x2A), None, flags!(), Some(MMX)),
			],

			"cvtps2dq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5B), None, flags!(pref_66), Some(SSE2)),
			],

			"cvtps2pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5A), None, flags!(), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5A), None, flags!(), Some(SSE2)),
			],

			"cvtps2pi" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x2D), None, flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2D), None, flags!(), Some(MMX)),
			],

			"cvtsd2si" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x2D), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2D), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x2D), None, flags!(with_rex_w, pref_F2), Some(SSE2)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2D), None, flags!(with_rex_w, pref_F2), Some(SSE2)),
			],

			"cvtsd2ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5A), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5A), None, flags!(pref_F2), Some(SSE2)),
			],

			"cvtsi2sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(DWORD))], _2(0x0F, 0x2A), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x2A), None, flags!(with_rex_w, pref_F2), Some(SSE2)),
			],

			"cvtsi2ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(DWORD))], _2(0x0F, 0x2A), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x2A), None, flags!(with_rex_w, pref_F3), Some(SSE)),
			],

			"cvtss2sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x5A), None, flags!(pref_F3), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5A), None, flags!(pref_F3), Some(SSE2)),
			],

			"cvtss2si" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x2D), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2D), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x2D), None, flags!(with_rex_w, pref_F3), Some(SSE)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2D), None, flags!(with_rex_w, pref_F3), Some(SSE)),
			],

			"cvttpd2dq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE6), None, flags!(pref_66), Some(SSE2)),
			],

			"cvttpd2pi" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x2C), None, flags!(pref_66), Some(SSE2)),
			],

			"cvttps2dq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5B), None, flags!(pref_F3), Some(SSE2)),
			],

			"cvttps2pi" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x2C), None, flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2C), None, flags!(), Some(MMX)),
			],

			"cvttsd2si" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x2C), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2C), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x2C), None, flags!(with_rex_w, pref_F2), Some(SSE2)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2C), None, flags!(with_rex_w, pref_F2), Some(SSE2)),
			],

			"cvttss2si" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x2C), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2C), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x2C), None, flags!(with_rex_w, pref_F3), Some(SSE)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2C), None, flags!(with_rex_w, pref_F3), Some(SSE)),
			],

			"cwd" => signatures!
			[
				signature(parameters![], _1(0x99), None, flags!(word_size), None),
			],

			"cwde" => signatures!
			[
				signature(parameters![], _1(0x98), None, flags!(), None),
			],

			"daa" => signatures!
			[
				signature(parameters![], _1(0x27), None, flags!(x86_only), None),
			],

			"das" => signatures!
			[
				signature(parameters![], _1(0x2F), None, flags!(x86_only), None),
			],

			"dec" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _1(0xFE), Some(RCX), flags!(lock), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE))], _1(0xFE), Some(RCX), flags!(), None),
				signature(parameters![parameter(memory, AllPossible)], _1(0xFF), Some(RCX), flags!(auto_size, lock), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _1(0x48), Some(RAX), flags!(x86_only, short_arg), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _1(0xFF), Some(RCX), flags!(auto_size), None),
			],

			"div" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0xF6), Some(RSI), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible)], _1(0xF7), Some(RSI), flags!(auto_size), None),
			],

			"divpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5E), None, flags!(pref_66), Some(SSE2)),
			],

			"divps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5E), None, flags!(), Some(SSE)),
			],

			"divsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5E), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5E), None, flags!(pref_F2), Some(SSE2)),
			],

			"divss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x5E), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5E), None, flags!(pref_F3), Some(SSE)),
			],

			"dmint" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x39), None, flags!(), Some(CYRIX)),
			],

			"dppd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x41), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x41), None, flags!(pref_66), Some(SSE41)),
			],

			"dpps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x40), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x40), None, flags!(pref_66), Some(SSE41)),
			],

			"emms" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x77), None, flags!(), Some(MMX)),
			],

			"enter" => signatures!
			[
				signature(parameters![parameter(immediate, Fixed(WORD)), parameter(immediate, Fixed(BYTE))], _1(0xC8), None, flags!(), None),
			],

			"extractps" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x17), None, flags!(with_rex_w, enc_mr, pref_66), Some(SSE41)),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x17), None, flags!(enc_mr, pref_66), Some(SSE41)),
			],

			"extrq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x78), Some(RAX), flags!(pref_66), Some(SSE4A)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x79), None, flags!(pref_66), Some(SSE4A)),
			],

			"f2xm1" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF0), None, flags!(), Some(FPU)),
			],

			"fabs" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xE1), None, flags!(), Some(FPU)),
			],

			"fadd" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xC1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xC0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xC0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xC0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xC0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD8), Some(RAX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDC), Some(RAX), flags!(exact_size), Some(FPU)),
			],

			"faddp" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xC1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDE, 0xC0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDE, 0xC0), None, flags!(short_arg), Some(FPU)),
			],

			"fbld" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _1(0xDF), Some(RSP), flags!(), Some(FPU)),
			],

			"fbstp" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _1(0xDF), Some(RSI), flags!(), Some(FPU)),
			],

			"fchs" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xE0), None, flags!(), Some(FPU)),
			],

			"fclex" => signatures!
			[
				signature(parameters![], _3(0x9B, 0xDB, 0xE2), None, flags!(), Some(FPU)),
			],

			"fcmovb" => signatures!
			[
				signature(parameters![], _2(0xDA, 0xC1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDA, 0xC0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDA, 0xC0), None, flags!(short_arg), Some(FPU)),
			],

			"fcmovbe" => signatures!
			[
				signature(parameters![], _2(0xDA, 0xD1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDA, 0xD0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDA, 0xD0), None, flags!(short_arg), Some(FPU)),
			],

			"fcmove" => signatures!
			[
				signature(parameters![], _2(0xDA, 0xC9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDA, 0xC8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDA, 0xC8), None, flags!(short_arg), Some(FPU)),
			],

			"fcmovnb" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xC1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xC0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xC0), None, flags!(short_arg), Some(FPU)),
			],

			"fcmovnbe" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xD1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xD0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xD0), None, flags!(short_arg), Some(FPU)),
			],

			"fcmovne" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xC9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xC8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xC8), None, flags!(short_arg), Some(FPU)),
			],

			"fcmovnu" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xD9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xD8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xD8), None, flags!(short_arg), Some(FPU)),
			],

			"fcmovu" => signatures!
			[
				signature(parameters![], _2(0xDA, 0xD9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDA, 0xD8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDA, 0xD8), None, flags!(short_arg), Some(FPU)),
			],

			"fcom" => signatures!
			[
				signature(parameters![], _2(0xD8, 0xD1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xD0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xD0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD8), Some(RDX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDC), Some(RDX), flags!(exact_size), Some(FPU)),
			],

			"fcomi" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xF1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xF0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xF0), None, flags!(short_arg), Some(FPU)),
			],

			"fcomip" => signatures!
			[
				signature(parameters![], _2(0xDF, 0xF1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDF, 0xF0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDF, 0xF0), None, flags!(short_arg), Some(FPU)),
			],

			"fcomp" => signatures!
			[
				signature(parameters![], _2(0xD8, 0xD9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xD8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xD8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD8), Some(RBX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDC), Some(RBX), flags!(exact_size), Some(FPU)),
			],

			"fcompp" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xD9), None, flags!(), Some(FPU)),
			],

			"fcos" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xFF), None, flags!(), Some(FPU)),
			],

			"fdecstp" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF6), None, flags!(), Some(FPU)),
			],

			"fdisi" => signatures!
			[
				signature(parameters![], _3(0x9B, 0xDB, 0xE1), None, flags!(), Some(FPU)),
			],

			"fdiv" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xF9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xF0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xF0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xF8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xF8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD8), Some(RSI), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDC), Some(RSI), flags!(exact_size), Some(FPU)),
			],

			"fdivp" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xF9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDE, 0xF8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDE, 0xF8), None, flags!(short_arg), Some(FPU)),
			],

			"fdivr" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xF1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xF8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xF8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xF0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xF0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD8), Some(RDI), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDC), Some(RDI), flags!(exact_size), Some(FPU)),
			],

			"fdivrp" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xF1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDE, 0xF0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDE, 0xF0), None, flags!(short_arg), Some(FPU)),
			],

			"femms" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x0E), None, flags!(), Some(TDNOW)),
			],

			"feni" => signatures!
			[
				signature(parameters![], _3(0x9B, 0xDB, 0xE0), None, flags!(), Some(FPU)),
			],

			"ffree" => signatures!
			[
				signature(parameters![], _2(0xDD, 0xC1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDD, 0xC0), None, flags!(short_arg), Some(FPU)),
			],

			"fiadd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDA), Some(RAX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDE), Some(RAX), flags!(), Some(FPU)),
			],

			"ficom" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDA), Some(RDX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDE), Some(RDX), flags!(), Some(FPU)),
			],

			"ficomp" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDA), Some(RBX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDE), Some(RBX), flags!(), Some(FPU)),
			],

			"fidiv" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDA), Some(RSI), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDE), Some(RSI), flags!(), Some(FPU)),
			],

			"fidivr" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDA), Some(RDI), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDE), Some(RDI), flags!(), Some(FPU)),
			],

			"fild" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDB), Some(RAX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDF), Some(RBP), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDF), Some(RAX), flags!(), Some(FPU)),
			],

			"fimul" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDA), Some(RCX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDE), Some(RCX), flags!(), Some(FPU)),
			],

			"fincstp" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF7), None, flags!(), Some(FPU)),
			],

			"finit" => signatures!
			[
				signature(parameters![], _3(0x9B, 0xDB, 0xE3), None, flags!(), Some(FPU)),
			],

			"fist" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDB), Some(RDX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDF), Some(RDX), flags!(), Some(FPU)),
			],

			"fistp" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDB), Some(RBX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDF), Some(RDI), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDF), Some(RBX), flags!(), Some(FPU)),
			],

			"fisttp" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDB), Some(RCX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDD), Some(RCX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDF), Some(RCX), flags!(), Some(FPU)),
			],

			"fisub" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDA), Some(RSP), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDE), Some(RSP), flags!(), Some(FPU)),
			],

			"fisubr" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xDA), Some(RBP), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDE), Some(RBP), flags!(), Some(FPU)),
			],

			"fld" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xC1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD9, 0xC0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD9), Some(RAX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(PWORD))], _1(0xDB), Some(RBP), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDD), Some(RAX), flags!(exact_size), Some(FPU)),
			],

			"fld1" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xE8), None, flags!(), Some(FPU)),
			],

			"fldcw" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xD9), Some(RBP), flags!(), Some(FPU)),
			],

			"fldenv" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _1(0xD9), Some(RSP), flags!(), Some(FPU)),
			],

			"fldl2e" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xEA), None, flags!(), Some(FPU)),
			],

			"fldl2t" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xE9), None, flags!(), Some(FPU)),
			],

			"fldlg2" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xEC), None, flags!(), Some(FPU)),
			],

			"fldln2" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xED), None, flags!(), Some(FPU)),
			],

			"fldpi" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xEB), None, flags!(), Some(FPU)),
			],

			"fldz" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xEE), None, flags!(), Some(FPU)),
			],

			"fmul" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xC9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xC8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xC8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xC8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xC8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD8), Some(RCX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDC), Some(RCX), flags!(exact_size), Some(FPU)),
			],

			"fmulp" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xC9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDE, 0xC8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDE, 0xC8), None, flags!(short_arg), Some(FPU)),
			],

			"fnclex" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xE2), None, flags!(), Some(FPU)),
			],

			"fndisi" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xE1), None, flags!(), Some(FPU)),
			],

			"fneni" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xE0), None, flags!(), Some(FPU)),
			],

			"fninit" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xE3), None, flags!(), Some(FPU)),
			],

			"fnop" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xD0), None, flags!(), Some(FPU)),
			],

			"fnsave" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _1(0xDD), Some(RSI), flags!(), Some(FPU)),
			],

			"fnstcw" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xD9), Some(RDI), flags!(), Some(FPU)),
			],

			"fnstenv" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _1(0xD9), Some(RSI), flags!(), Some(FPU)),
			],

			"fnstsw" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(WORD))], _2(0xDF, 0xE0), None, flags!(), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _1(0xDD), Some(RDI), flags!(), Some(FPU)),
			],

			"fpatan" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF3), None, flags!(), Some(FPU)),
			],

			"fprem" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF8), None, flags!(), Some(FPU)),
			],

			"fprem1" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF5), None, flags!(), Some(FPU)),
			],

			"fptan" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF2), None, flags!(), Some(FPU)),
			],

			"frndint" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xFC), None, flags!(), Some(FPU)),
			],

			"frstor" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _1(0xDD), Some(RSP), flags!(), Some(FPU)),
			],

			"fsave" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x9B, 0xDD), Some(RSI), flags!(), Some(FPU)),
			],

			"fscale" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xFD), None, flags!(), Some(FPU)),
			],

			"fsetpm" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xE4), None, flags!(), Some(FPU)),
			],

			"fsin" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xFE), None, flags!(), Some(FPU)),
			],

			"fsincos" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xFB), None, flags!(), Some(FPU)),
			],

			"fsqrt" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xFA), None, flags!(), Some(FPU)),
			],

			"fst" => signatures!
			[
				signature(parameters![], _2(0xDD, 0xD1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDD, 0xD0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD9), Some(RDX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDD), Some(RDX), flags!(exact_size), Some(FPU)),
			],

			"fstcw" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(WORD))], _2(0x9B, 0xD9), Some(RDI), flags!(), Some(FPU)),
			],

			"fstenv" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x9B, 0xD9), Some(RSI), flags!(), Some(FPU)),
			],

			"fstp" => signatures!
			[
				signature(parameters![], _2(0xDD, 0xD9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDD, 0xD8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD9), Some(RBX), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(PWORD))], _1(0xDB), Some(RDI), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDD), Some(RBX), flags!(exact_size), Some(FPU)),
			],

			"fstsw" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(WORD))], _3(0x9B, 0xDF, 0xE0), None, flags!(), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(WORD))], _2(0x9B, 0xDD), Some(RDI), flags!(), Some(FPU)),
			],

			"fsub" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xE9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xE0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xE0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xE8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xE8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD8), Some(RSP), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDC), Some(RSP), flags!(exact_size), Some(FPU)),
			],

			"fsubp" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xE9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDE, 0xE8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDE, 0xE8), None, flags!(short_arg), Some(FPU)),
			],

			"fsubr" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xE1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xE8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD8, 0xE8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xE0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDC, 0xE0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xD8), Some(RBP), flags!(exact_size), Some(FPU)),
				signature(parameters![parameter(memory, Fixed(QWORD))], _1(0xDC), Some(RBP), flags!(exact_size), Some(FPU)),
			],

			"fsubrp" => signatures!
			[
				signature(parameters![], _2(0xDE, 0xE1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDE, 0xE0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xDE, 0xE0), None, flags!(short_arg), Some(FPU)),
			],

			"ftst" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xE4), None, flags!(), Some(FPU)),
			],

			"fucom" => signatures!
			[
				signature(parameters![], _2(0xDD, 0xE1), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDD, 0xE0), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDD, 0xE0), None, flags!(short_arg), Some(FPU)),
			],

			"fucomi" => signatures!
			[
				signature(parameters![], _2(0xDB, 0xE9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xE8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDB, 0xE8), None, flags!(short_arg), Some(FPU)),
			],

			"fucomip" => signatures!
			[
				signature(parameters![], _2(0xDF, 0xE9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDF, 0xE8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDF, 0xE8), None, flags!(short_arg), Some(FPU)),
			],

			"fucomp" => signatures!
			[
				signature(parameters![], _2(0xDD, 0xE9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDD, 0xE8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xDD, 0xE8), None, flags!(short_arg), Some(FPU)),
			],

			"fucompp" => signatures!
			[
				signature(parameters![], _2(0xDA, 0xE9), None, flags!(), Some(FPU)),
			],

			"fwait" => signatures!
			[
				signature(parameters![], _1(0x9B), None, flags!(), None),
			],

			"fxam" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xE5), None, flags!(), Some(FPU)),
			],

			"fxch" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xC9), None, flags!(), Some(FPU)),
				signature(parameters![parameter(st0, Fixed(PWORD)), parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD9, 0xC8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD))], _2(0xD9, 0xC8), None, flags!(short_arg), Some(FPU)),
				signature(parameters![parameter(floating_pointer_register, Fixed(PWORD)), parameter(st0, Fixed(PWORD))], _2(0xD9, 0xC8), None, flags!(short_arg), Some(FPU)),
			],

			"fxrstor" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xAE), Some(RCX), flags!(), Some(SSE)),
			],

			"fxrstor64" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xAE), Some(RCX), flags!(with_rex_w), Some(SSE)),
			],

			"fxsave" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xAE), Some(RAX), flags!(), Some(SSE)),
			],

			"fxsave64" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xAE), Some(RAX), flags!(with_rex_w), Some(SSE)),
			],

			"fxtract" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF4), None, flags!(), Some(FPU)),
			],

			"fyl2x" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF1), None, flags!(), Some(FPU)),
			],

			"fyl2xp1" => signatures!
			[
				signature(parameters![], _2(0xD9, 0xF9), None, flags!(), Some(FPU)),
			],

			"getsec" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x37), None, flags!(), None),
			],

			"haddpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x7C), None, flags!(pref_66), Some(SSE3)),
			],

			"haddps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x7C), None, flags!(pref_F2), Some(SSE3)),
			],

			"hlt" => signatures!
			[
				signature(parameters![], _1(0xF4), None, flags!(), None),
			],

			"hsubpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x7D), None, flags!(pref_66), Some(SSE3)),
			],

			"hsubps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x7D), None, flags!(pref_F2), Some(SSE3)),
			],

			"icebp" => signatures!
			[
				signature(parameters![], _1(0xF1), None, flags!(), None),
			],

			"idiv" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0xF6), Some(RDI), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible)], _1(0xF7), Some(RDI), flags!(auto_size), None),
			],

			"imul" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, AllPossible)], _1(0xF7), Some(RBP), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0xF6), Some(RBP), flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0xAF), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x6B), None, flags!(auto_size, exact_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(immediate, AllPossible)], _1(0x69), None, flags!(auto_size), None),
			],

			"in" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xE4), None, flags!(), None),
				signature(parameters![parameter(rax, Fixed(WORD)), parameter(immediate, Fixed(BYTE))], _1(0xE5), None, flags!(word_size), None),
				signature(parameters![parameter(rax, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _1(0xE5), None, flags!(), None),
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(rdx, Fixed(WORD))], _1(0xEC), None, flags!(), None),
				signature(parameters![parameter(rax, Fixed(WORD)), parameter(rdx, Fixed(WORD))], _1(0xED), None, flags!(word_size), None),
				signature(parameters![parameter(rax, Fixed(DWORD)), parameter(rdx, Fixed(WORD))], _1(0xED), None, flags!(), None),
			],

			"inc" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _1(0xFE), Some(RAX), flags!(lock), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE))], _1(0xFE), Some(RAX), flags!(), None),
				signature(parameters![parameter(memory, AllPossible)], _1(0xFF), Some(RAX), flags!(auto_size, lock), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _1(0x40), Some(RAX), flags!(x86_only, short_arg), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _1(0xFF), Some(RAX), flags!(auto_size), None),
			],

			"insb" => signatures!
			[
				signature(parameters![], _1(0x6C), None, flags!(rep), None),
			],

			"insd" => signatures!
			[
				signature(parameters![], _1(0x6D), None, flags!(rep), None),
			],

			"insertps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x21), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x21), None, flags!(pref_66), Some(SSE41)),
			],

			"insertq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x79), None, flags!(pref_F2), Some(SSE4A)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x78), None, flags!(pref_F2), Some(SSE4A)),
			],

			"insw" => signatures!
			[
				signature(parameters![], _1(0x6D), None, flags!(word_size, rep), None),
			],

			"int" => signatures!
			[
				signature(parameters![parameter(immediate, Fixed(BYTE))], _1(0xCD), None, flags!(), None),
			],

			"int01" => signatures!
			[
				signature(parameters![], _1(0xF1), None, flags!(), None),
			],

			"int03" => signatures!
			[
				signature(parameters![], _1(0xCC), None, flags!(), None),
			],

			"int1" => signatures!
			[
				signature(parameters![], _1(0xF1), None, flags!(), None),
			],

			"int3" => signatures!
			[
				signature(parameters![], _1(0xCC), None, flags!(), None),
			],

			"into" => signatures!
			[
				signature(parameters![], _1(0xCE), None, flags!(x86_only), None),
			],

			"invd" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x08), None, flags!(), None),
			],

			"invept" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(memory, Fixed(OWORD))], _3(0x0F, 0x38, 0x80), None, flags!(pref_66), Some(VMX)),
			],

			"invlpg" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x01), Some(RDI), flags!(), None),
			],

			"invlpga" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xDF), None, flags!(), Some(AMD)),
				signature(parameters![parameter(rax, Fixed(QWORD)), parameter(rcx, Fixed(DWORD))], _3(0x0F, 0x01, 0xDF), None, flags!(), Some(AMD)),
			],

			"invpcid" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(memory, Fixed(OWORD))], _3(0x0F, 0x38, 0x82), None, flags!(pref_66), Some(INVPCID)),
			],

			"invvpid" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(memory, Fixed(OWORD))], _3(0x0F, 0x38, 0x81), None, flags!(pref_66), Some(VMX)),
			],

			"iret" => signatures!
			[
				signature(parameters![], _1(0xCF), None, flags!(), None),
			],

			"iretd" => signatures!
			[
				signature(parameters![], _1(0xCF), None, flags!(), None),
			],

			"iretq" => signatures!
			[
				signature(parameters![], _1(0xCF), None, flags!(with_rex_w), None),
			],

			"iretw" => signatures!
			[
				signature(parameters![], _1(0xCF), None, flags!(word_size), None),
			],

			"ja" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x77), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x87), None, flags!(), None),
			],

			"jae" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x73), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x83), None, flags!(), None),
			],

			"jb" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x72), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x82), None, flags!(), None),
			],

			"jbe" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x76), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x86), None, flags!(), None),
			],

			"jc" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x72), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x82), None, flags!(), None),
			],

			"je" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x74), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x84), None, flags!(), None),
			],

			"jecxz" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0xE3), None, flags!(pref_67), None),
			],

			"jg" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7F), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8F), None, flags!(), None),
			],

			"jge" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7D), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8D), None, flags!(), None),
			],

			"jl" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7C), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8C), None, flags!(), None),
			],

			"jle" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7E), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8E), None, flags!(), None),
			],

			"jmp" => signatures!
			[
				signature(parameters![parameter(immediate, Fixed(WORD)), parameter(immediate, Fixed(WORD))], _1(0x9A), None, flags!(x86_only, word_size, exact_size), None),
				signature(parameters![parameter(immediate, Fixed(DWORD)), parameter(immediate, Fixed(WORD))], _1(0xEA), None, flags!(x86_only), None),
				signature(parameters![parameter(memory, Fixed(FWORD))], _1(0xFF), Some(RBP), flags!(x86_only, exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0xEB), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _1(0xE9), None, flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible)], _1(0xFF), Some(RSP), flags!(auto_no_32), None),
			],

			"jmpf" => signatures!
			[
				signature(parameters![parameter(immediate, Fixed(WORD)), parameter(immediate, Fixed(WORD))], _1(0x9A), None, flags!(x86_only, word_size, exact_size), None),
				signature(parameters![parameter(immediate, Fixed(DWORD)), parameter(immediate, Fixed(WORD))], _1(0xEA), None, flags!(x86_only), None),
				signature(parameters![parameter(memory, Fixed(DWORD))], _1(0xFF), Some(RBP), flags!(x86_only, word_size, exact_size), None),
				signature(parameters![parameter(memory, Fixed(FWORD))], _1(0xFF), Some(RBP), flags!(x86_only), None),
			],

			"jna" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x76), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x86), None, flags!(), None),
			],

			"jnae" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x72), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x82), None, flags!(), None),
			],

			"jnb" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x73), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x83), None, flags!(), None),
			],

			"jnbe" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x77), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x87), None, flags!(), None),
			],

			"jnc" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x73), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x83), None, flags!(), None),
			],

			"jne" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x75), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x85), None, flags!(), None),
			],

			"jng" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7E), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8E), None, flags!(), None),
			],

			"jnge" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7C), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8C), None, flags!(), None),
			],

			"jnl" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7D), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8D), None, flags!(), None),
			],

			"jnle" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7F), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8F), None, flags!(), None),
			],

			"jno" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x71), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x81), None, flags!(), None),
			],

			"jnp" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7B), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8B), None, flags!(), None),
			],

			"jns" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x79), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x89), None, flags!(), None),
			],

			"jnz" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x75), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x85), None, flags!(), None),
			],

			"jo" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x70), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x80), None, flags!(), None),
			],

			"jp" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7A), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8A), None, flags!(), None),
			],

			"jpe" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7A), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8A), None, flags!(), None),
			],

			"jpo" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x7B), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x8B), None, flags!(), None),
			],

			"jrcxz" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0xE3), None, flags!(), None),
			],

			"js" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x78), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x88), None, flags!(), None),
			],

			"jz" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0x74), None, flags!(exact_size), None),
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0x0F, 0x84), None, flags!(), None),
			],

			"lahf" => signatures!
			[
				signature(parameters![], _1(0x9F), None, flags!(), None),
			],

			"lar" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, Fixed(WORD))], _2(0x0F, 0x02), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0x02), None, flags!(auto_size), None),
			],

			"lddqu" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(OWORD))], _2(0x0F, 0xF0), None, flags!(pref_F2), Some(SSE3)),
			],

			"ldmxcsr" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _2(0x0F, 0xAE), Some(RDX), flags!(), Some(SSE)),
			],

			"lds" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, NoSize)], _1(0xC5), None, flags!(auto_size, x86_only), None),
			],

			"lea" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, NoSize)], _1(0x8D), None, flags!(auto_size), None),
			],

			"leave" => signatures!
			[
				signature(parameters![], _1(0xC9), None, flags!(), None),
			],

			"les" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, NoSize)], _1(0xC4), None, flags!(auto_size, x86_only), None),
			],

			"lfence" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xAE, 0xE8), None, flags!(), Some(AMD)),
			],

			"lfs" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, NoSize)], _2(0x0F, 0xB4), None, flags!(auto_size), None),
			],

			"lgdt" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x01), Some(RDX), flags!(), None),
			],

			"lgs" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, NoSize)], _2(0x0F, 0xB5), None, flags!(auto_size), None),
			],

			"lidt" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x01), Some(RBX), flags!(), None),
			],

			"lldt" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x00), Some(RDX), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(WORD))], _2(0x0F, 0x00), Some(RDX), flags!(), None),
			],

			"llwpcb" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible)], _2(0x09, 0x12), Some(RAX), flags!(xop_op, auto_rex_w), Some(AMD)),
			],

			"lmsw" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x01), Some(RSI), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(WORD))], _2(0x0F, 0x01), Some(RSI), flags!(), None),
			],

			"lodsb" => signatures!
			[
				signature(parameters![], _1(0xAC), None, flags!(rep), None),
			],

			"lodsd" => signatures!
			[
				signature(parameters![], _1(0xAD), None, flags!(rep), None),
			],

			"lodsq" => signatures!
			[
				signature(parameters![], _1(0xAD), None, flags!(with_rex_w, rep), None),
			],

			"lodsw" => signatures!
			[
				signature(parameters![], _1(0xAD), None, flags!(word_size, rep), None),
			],

			"loop" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0xE2), None, flags!(), None),
			],

			"loope" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0xE1), None, flags!(), None),
			],

			"loopne" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0xE0), None, flags!(), None),
			],

			"loopnz" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0xE0), None, flags!(), None),
			],

			"loopz" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(BYTE))], _1(0xE1), None, flags!(), None),
			],

			"lsl" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, Fixed(WORD))], _2(0x0F, 0x03), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0x03), None, flags!(auto_size), None),
			],

			"lss" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, NoSize)], _2(0x0F, 0xB2), None, flags!(auto_size), None),
			],

			"ltr" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x00), Some(RBX), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(WORD))], _2(0x0F, 0x00), Some(RBX), flags!(), None),
			],

			"lwpins" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(DWORD))], _2(0x10, 0x12), Some(RAX), flags!(xop_op, auto_rex_w, enc_vm), Some(AMD)),
			],

			"lwpval" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(DWORD))], _2(0x10, 0x12), Some(RCX), flags!(xop_op, auto_rex_w, enc_vm), Some(AMD)),
			],

			"lzcnt" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0xBD), None, flags!(auto_size, pref_F3), Some(AMD)),
			],

			"maskmovdqu" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0xF7), None, flags!(pref_66), Some(SSE2)),
			],

			"maskmovq" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register, Fixed(QWORD))], _2(0x0F, 0xF7), None, flags!(), Some(MMX)),
			],

			"maxpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5F), None, flags!(pref_66), Some(SSE2)),
			],

			"maxps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5F), None, flags!(), Some(SSE)),
			],

			"maxsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5F), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5F), None, flags!(pref_F2), Some(SSE2)),
			],

			"maxss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x5F), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5F), None, flags!(pref_F3), Some(SSE)),
			],

			"mfence" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xAE, 0xF0), None, flags!(), Some(AMD)),
			],

			"minpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5D), None, flags!(pref_66), Some(SSE2)),
			],

			"minps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5D), None, flags!(), Some(SSE)),
			],

			"minsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5D), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5D), None, flags!(pref_F2), Some(SSE2)),
			],

			"minss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x5D), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5D), None, flags!(pref_F3), Some(SSE)),
			],

			"monitor" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xC8), None, flags!(), None),
				signature(parameters![parameter(rax, Fixed(QWORD)), parameter(rcx, Fixed(DWORD)), parameter(rdx, Fixed(DWORD))], _3(0x0F, 0x01, 0xC8), None, flags!(), None),
			],

			"monitorx" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xFA), None, flags!(), Some(AMD)),
				signature(parameters![parameter(rax, AllPossible), parameter(rcx, Fixed(DWORD)), parameter(rdx, Fixed(DWORD))], _3(0x0F, 0x01, 0xFA), None, flags!(), Some(AMD)),
			],

			"montmul" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xA6, 0xC0), None, flags!(pref_F3), Some(CYRIX)),
			],

			"mov" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x89), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x88), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _1(0x8B), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0x8A), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(segment_register, Fixed(WORD))], _1(0x8C), None, flags!(auto_size), None),
				signature(parameters![parameter(memory, Fixed(WORD)), parameter(segment_register, Fixed(WORD))], _1(0x8C), None, flags!(), None),
				signature(parameters![parameter(segment_register, Fixed(WORD)), parameter(memory, Fixed(WORD))], _1(0x8C), None, flags!(), None),
				signature(parameters![parameter(segment_register, Fixed(WORD)), parameter(legacy_register, Fixed(WORD))], _1(0x8C), None, flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xB0), None, flags!(short_arg), None),
				signature(parameters![parameter(legacy_register, Fixed(WORD)), parameter(immediate, Fixed(WORD))], _1(0xB8), None, flags!(word_size, short_arg), None),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(immediate, Fixed(DWORD))], _1(0xB8), None, flags!(short_arg), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, AllPossible)], _1(0xC7), Some(RAX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xC6), Some(RAX), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(immediate, Fixed(QWORD))], _1(0xB8), None, flags!(with_rex_w, short_arg), None),
				signature(parameters![parameter(control_register, Fixed(DWORD)), parameter(legacy_register, Fixed(DWORD))], _2(0x0F, 0x22), None, flags!(), None),
				signature(parameters![parameter(control_register, Fixed(QWORD)), parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0x22), None, flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(control_register, Fixed(DWORD))], _2(0x0F, 0x20), None, flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(control_register, Fixed(QWORD))], _2(0x0F, 0x20), None, flags!(), None),
				signature(parameters![parameter(cr8, Fixed(DWORD)), parameter(legacy_register, Fixed(DWORD))], _2(0x0F, 0x22), Some(RAX), flags!(pref_F0), None),
				signature(parameters![parameter(cr8, Fixed(QWORD)), parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0x22), Some(RAX), flags!(pref_F0), None),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(cr8, Fixed(DWORD))], _2(0x0F, 0x22), Some(RAX), flags!(pref_F0), None),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(cr8, Fixed(QWORD))], _2(0x0F, 0x22), Some(RAX), flags!(pref_F0), None),
				signature(parameters![parameter(debug_register, Fixed(DWORD)), parameter(legacy_register, Fixed(DWORD))], _2(0x0F, 0x23), None, flags!(), None),
				signature(parameters![parameter(debug_register, Fixed(QWORD)), parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0x23), None, flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(debug_register, Fixed(DWORD))], _2(0x0F, 0x21), None, flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(debug_register, Fixed(QWORD))], _2(0x0F, 0x21), None, flags!(), None),
			],

			"movabs" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(QWORD))], _1(0xA0), None, flags!(), None),
				signature(parameters![parameter(rax, Fixed(WORD)), parameter(immediate, Fixed(QWORD))], _1(0xA1), None, flags!(word_size), None),
				signature(parameters![parameter(rax, Fixed(DWORD)), parameter(immediate, Fixed(QWORD))], _1(0xA1), None, flags!(), None),
				signature(parameters![parameter(rax, Fixed(QWORD)), parameter(immediate, Fixed(QWORD))], _1(0xA1), None, flags!(with_rex_w), None),
				signature(parameters![parameter(immediate, Fixed(QWORD)), parameter(rax, Fixed(BYTE))], _1(0xA2), None, flags!(), None),
				signature(parameters![parameter(immediate, Fixed(QWORD)), parameter(rax, Fixed(WORD))], _1(0xA3), None, flags!(word_size), None),
				signature(parameters![parameter(immediate, Fixed(QWORD)), parameter(rax, Fixed(DWORD))], _1(0xA3), None, flags!(), None),
				signature(parameters![parameter(immediate, Fixed(QWORD)), parameter(rax, Fixed(QWORD))], _1(0xA3), None, flags!(with_rex_w), None),
			],

			"movapd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x29), None, flags!(enc_mr, pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(OWORD))], _2(0x0F, 0x28), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x28), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x29), None, flags!(enc_mr, pref_66), Some(SSE2)),
			],

			"movaps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x28), None, flags!(), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x29), None, flags!(enc_mr), Some(SSE)),
			],

			"movbe" => signatures!
			[
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _3(0x0F, 0x38, 0xF1), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, AllPossible)], _3(0x0F, 0x38, 0xF0), None, flags!(auto_size), None),
			],

			"movd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x7E), None, flags!(enc_mr, pref_66), Some(SSE2)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(legacy_register_and_memory, Fixed(DWORD))], _2(0x0F, 0x6E), None, flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(legacy_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x6E), None, flags!(with_rex_w), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x6E), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(DWORD))], _2(0x0F, 0x6E), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(DWORD)), parameter(mmx_register, Fixed(QWORD))], _2(0x0F, 0x7E), None, flags!(enc_mr), Some(MMX)),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x7E), None, flags!(enc_mr, pref_66), Some(SSE2)),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(QWORD)), parameter(mmx_register, Fixed(QWORD))], _2(0x0F, 0x7E), None, flags!(with_rex_w, enc_mr), Some(MMX)),
			],

			"movddup" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x12), None, flags!(pref_F2), Some(SSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x12), None, flags!(pref_F2), Some(SSE3)),
			],

			"movdq2q" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0xD6), None, flags!(pref_F2), Some(SSE2)),
			],

			"movdqa" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x7F), None, flags!(enc_mr, pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(OWORD))], _2(0x0F, 0x6F), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x6F), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x7F), None, flags!(enc_mr, pref_66), Some(SSE2)),
			],

			"movdqu" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x7F), None, flags!(enc_mr, pref_F3), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(OWORD))], _2(0x0F, 0x6F), None, flags!(pref_F3), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x6F), None, flags!(pref_F3), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x7F), None, flags!(enc_mr, pref_F3), Some(SSE2)),
			],

			"movhlps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x12), None, flags!(), Some(SSE)),
			],

			"movhpd" => signatures!
			[
				signature(parameters![parameter(memory, NoSize), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x17), None, flags!(enc_mr, pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, NoSize)], _2(0x0F, 0x16), None, flags!(pref_66), Some(SSE2)),
			],

			"movhps" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x17), None, flags!(enc_mr), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x16), None, flags!(), Some(SSE)),
			],

			"movlhps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x16), None, flags!(), Some(SSE)),
			],

			"movlpd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x13), None, flags!(enc_mr, pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x12), None, flags!(pref_66), Some(SSE2)),
			],

			"movlps" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x13), None, flags!(enc_mr), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x12), None, flags!(), Some(SSE)),
			],

			"movmskpd" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x50), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x50), None, flags!(with_rex_w, pref_66), Some(SSE2)),
			],

			"movmskps" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x50), None, flags!(), Some(SSE)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x50), None, flags!(with_rex_w), Some(SSE)),
			],

			"movntdq" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0xE7), None, flags!(enc_mr, pref_66), Some(SSE2)),
			],

			"movntdqa" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(OWORD))], _3(0x0F, 0x38, 0x2A), None, flags!(pref_66), Some(SSE41)),
			],

			"movnti" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD)), parameter(legacy_register, Fixed(DWORD))], _2(0x0F, 0xC3), None, flags!(enc_mr), None),
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0xC3), None, flags!(with_rex_w, enc_mr), None),
			],

			"movntpd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2B), None, flags!(enc_mr, pref_66), Some(SSE2)),
			],

			"movntps" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2B), None, flags!(enc_mr), Some(SSE)),
			],

			"movntq" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(mmx_register, Fixed(QWORD))], _2(0x0F, 0xE7), None, flags!(enc_mr), Some(MMX)),
			],

			"movntsd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2B), None, flags!(enc_mr, pref_F2), Some(SSE4A_AMD)),
			],

			"movntss" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2B), None, flags!(enc_mr, pref_F3), Some(SSE4A)),
			],

			"movq" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0xD6), None, flags!(enc_mr, pref_66), Some(SSE2)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x6F), None, flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(legacy_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x6E), None, flags!(with_rex_w), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x7E), None, flags!(pref_F3), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x6E), None, flags!(with_rex_w, pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x7E), None, flags!(pref_F3), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0xD6), None, flags!(enc_mr, pref_66), Some(SSE2)),
				signature(parameters![parameter(mmx_register_and_memory, Fixed(QWORD)), parameter(mmx_register, Fixed(QWORD))], _2(0x0F, 0x7F), None, flags!(enc_mr), Some(MMX)),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(QWORD)), parameter(mmx_register, Fixed(QWORD))], _2(0x0F, 0x7E), None, flags!(with_rex_w, enc_mr), Some(MMX)),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x7E), None, flags!(with_rex_w, enc_mr, pref_66), Some(SSE2)),
			],

			"movq2dq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(mmx_register, Fixed(QWORD))], _2(0x0F, 0xD6), None, flags!(pref_F3), Some(SSE2)),
			],

			"movsb" => signatures!
			[
				signature(parameters![], _1(0xA4), None, flags!(rep), None),
			],

			"movsd" => signatures!
			[
				signature(parameters![], _1(0xA5), None, flags!(rep), None),
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x11), None, flags!(enc_mr, pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x10), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x10), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x11), None, flags!(enc_mr, pref_F2), Some(SSE2)),
			],

			"movshdup" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x16), None, flags!(pref_F3), Some(SSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x16), None, flags!(pref_F3), Some(SSE3)),
			],

			"movsldup" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x12), None, flags!(pref_F3), Some(SSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x12), None, flags!(pref_F3), Some(SSE3)),
			],

			"movsq" => signatures!
			[
				signature(parameters![], _1(0xA5), None, flags!(with_rex_w, rep), None),
			],

			"movss" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x11), None, flags!(enc_mr, pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x10), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x10), None, flags!(pref_F3), Some(SSE)),
			],

			"movsw" => signatures!
			[
				signature(parameters![], _1(0xA5), None, flags!(word_size, rep), None),
			],

			"movsx" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(legacy_register_and_memory, Fixed(DWORD))], _1(0x63), None, flags!(with_rex_w), None),
				signature(parameters![parameter(legacy_register, Fixed(WORD)), parameter(memory, Fixed(BYTE))], _2(0x0F, 0xBE), None, flags!(word_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0xBE), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, Fixed(WORD))], _2(0x0F, 0xBF), None, flags!(auto_rex_w, exact_size), None),
			],

			"movsxd" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(legacy_register_and_memory, Fixed(DWORD))], _1(0x63), None, flags!(with_rex_w), None),
			],

			"movupd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x11), None, flags!(enc_mr, pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(OWORD))], _2(0x0F, 0x10), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x10), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x11), None, flags!(enc_mr, pref_66), Some(SSE2)),
			],

			"movups" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x10), None, flags!(), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x11), None, flags!(enc_mr), Some(SSE)),
			],

			"movzx" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(WORD)), parameter(memory, Fixed(BYTE))], _2(0x0F, 0xB6), None, flags!(word_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0xB6), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, Fixed(WORD))], _2(0x0F, 0xB7), None, flags!(auto_rex_w, exact_size), None),
			],

			"mpsadbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x42), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x42), None, flags!(pref_66), Some(SSE41)),
			],

			"mul" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0xF6), Some(RSP), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible)], _1(0xF7), Some(RSP), flags!(auto_size), None),
			],

			"mulpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x59), None, flags!(pref_66), Some(SSE2)),
			],

			"mulps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x59), None, flags!(), Some(SSE)),
			],

			"mulsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x59), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x59), None, flags!(pref_F2), Some(SSE2)),
			],

			"mulss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x59), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x59), None, flags!(pref_F3), Some(SSE)),
			],

			"mulx" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x02, 0xF6), None, flags!(vex_op, auto_rex_w, pref_F2), Some(BMI2)),
			],

			"mwait" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xC9), None, flags!(), None),
				signature(parameters![parameter(rax, Fixed(DWORD)), parameter(rcx, Fixed(DWORD))], _3(0x0F, 0x01, 0xC9), None, flags!(), None),
			],

			"mwaitx" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xFB), None, flags!(), Some(AMD)),
				signature(parameters![parameter(rax, Fixed(DWORD)), parameter(rcx, Fixed(DWORD))], _3(0x0F, 0x01, 0xFB), None, flags!(), Some(AMD)),
			],

			"neg" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _1(0xF6), Some(RBX), flags!(lock), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE))], _1(0xF6), Some(RBX), flags!(), None),
				signature(parameters![parameter(memory, AllPossible)], _1(0xF7), Some(RBX), flags!(auto_size, lock), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _1(0xF7), Some(RBX), flags!(auto_size), None),
			],

			"nop" => signatures!
			[
				signature(parameters![], _1(0x90), None, flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0x1F), Some(RAX), flags!(auto_size), None),
			],

			"not" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _1(0xF6), Some(RDX), flags!(lock), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE))], _1(0xF6), Some(RDX), flags!(), None),
				signature(parameters![parameter(memory, AllPossible)], _1(0xF7), Some(RDX), flags!(auto_size, lock), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _1(0xF7), Some(RDX), flags!(auto_size), None),
			],

			"or" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x0C), None, flags!(), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RCX), flags!(lock), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x08), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RCX), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x08), None, flags!(enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0x0A), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RCX), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(rax, AllPossible), parameter(immediate, AllPossible)], _1(0x0D), None, flags!(auto_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RCX), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RCX), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x09), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RCX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _1(0x09), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _1(0x0B), None, flags!(auto_size), None),
			],

			"orpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x56), None, flags!(pref_66), Some(SSE2)),
			],

			"orps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x56), None, flags!(), Some(SSE)),
			],

			"out" => signatures!
			[
				signature(parameters![parameter(immediate, Fixed(BYTE)), parameter(rax, Fixed(BYTE))], _1(0xE6), None, flags!(), None),
				signature(parameters![parameter(immediate, Fixed(BYTE)), parameter(rax, Fixed(WORD))], _1(0xE7), None, flags!(), None),
				signature(parameters![parameter(immediate, Fixed(BYTE)), parameter(rax, Fixed(DWORD))], _1(0xE7), None, flags!(), None),
				signature(parameters![parameter(rdx, Fixed(WORD)), parameter(rax, Fixed(BYTE))], _1(0xEE), None, flags!(), None),
				signature(parameters![parameter(rdx, Fixed(WORD)), parameter(rax, Fixed(WORD))], _1(0xEF), None, flags!(word_size), None),
				signature(parameters![parameter(rdx, Fixed(WORD)), parameter(rax, Fixed(DWORD))], _1(0xEF), None, flags!(), None),
			],

			"outsb" => signatures!
			[
				signature(parameters![], _1(0x6E), None, flags!(rep), None),
			],

			"outsd" => signatures!
			[
				signature(parameters![], _1(0x6F), None, flags!(rep), None),
			],

			"outsw" => signatures!
			[
				signature(parameters![], _1(0x6F), None, flags!(word_size, rep), None),
			],

			"pabsb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x1C), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x1C), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x1C), None, flags!(pref_66), Some(SSSE3)),
			],

			"pabsd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x1E), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x1E), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x1E), None, flags!(pref_66), Some(SSSE3)),
			],

			"pabsw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x1D), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x1D), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x1D), None, flags!(pref_66), Some(SSSE3)),
			],

			"packssdw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x6B), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x6B), None, flags!(pref_66), Some(SSE2)),
			],

			"packsswb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x63), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x63), None, flags!(pref_66), Some(SSE2)),
			],

			"packusdw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x2B), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x2B), None, flags!(pref_66), Some(SSE41)),
			],

			"packuswb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x67), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x67), None, flags!(pref_66), Some(SSE2)),
			],

			"paddb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xFC), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xFC), None, flags!(pref_66), Some(SSE2)),
			],

			"paddd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xFE), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xFE), None, flags!(pref_66), Some(SSE2)),
			],

			"paddq" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xD4), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xD4), None, flags!(pref_66), Some(SSE2)),
			],

			"paddsb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xEC), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xEC), None, flags!(pref_66), Some(SSE2)),
			],

			"paddsiw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x51), None, flags!(), Some(MMX_CYRIX)),
			],

			"paddsw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xED), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xED), None, flags!(pref_66), Some(SSE2)),
			],

			"paddusb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xDC), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xDC), None, flags!(pref_66), Some(SSE2)),
			],

			"paddusw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xDD), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xDD), None, flags!(pref_66), Some(SSE2)),
			],

			"paddw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xFD), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xFD), None, flags!(pref_66), Some(SSE2)),
			],

			"palignr" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0F), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0F), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0F), None, flags!(pref_66), Some(SSSE3)),
			],

			"pand" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xDB), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xDB), None, flags!(pref_66), Some(SSE2)),
			],

			"pandn" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xDF), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xDF), None, flags!(pref_66), Some(SSE2)),
			],

			"pause" => signatures!
			[
				signature(parameters![], _1(0x90), None, flags!(pref_F3), None),
			],

			"paveb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x50), None, flags!(), Some(MMX_CYRIX)),
			],

			"pavgb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xE0), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE0), None, flags!(pref_66), Some(SSE2)),
			],

			"pavgusb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xBF), None, flags!(imm_op), Some(TDNOW)),
			],

			"pavgw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xE3), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE3), None, flags!(pref_66), Some(SSE2)),
			],

			"pblendvb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x10), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x10), None, flags!(pref_66), Some(SSE41)),
			],

			"pblendw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0E), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0E), None, flags!(pref_66), Some(SSE41)),
			],

			"pclmulhqhqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _4(0x0F, 0x3A, 0x44, 0x11), None, flags!(imm_op, pref_66), Some(SSE)),
			],

			"pclmulhqlqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _4(0x0F, 0x3A, 0x44, 0x01), None, flags!(pref_66, imm_op), Some(SSE)),
			],

			"pclmullqhqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _4(0x0F, 0x3A, 0x44, 0x10), None, flags!(pref_66, imm_op), Some(SSE)),
			],

			"pclmullqlqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _4(0x0F, 0x3A, 0x44, 0x00), None, flags!(pref_66, imm_op), Some(SSE)),
			],

			"pclmulqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x44), None, flags!(pref_66), Some(SSE)),
			],

			"pcmpeqb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x74), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x74), None, flags!(pref_66), Some(SSE2)),
			],

			"pcmpeqd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x76), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x76), None, flags!(pref_66), Some(SSE2)),
			],

			"pcmpeqq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x29), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x29), None, flags!(pref_66), Some(SSE41)),
			],

			"pcmpeqw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x75), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x75), None, flags!(pref_66), Some(SSE2)),
			],

			"pcmpestri" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x61), None, flags!(pref_66), Some(SSE42)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x61), None, flags!(pref_66), Some(SSE42)),
			],

			"pcmpestrm" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x60), None, flags!(pref_66), Some(SSE42)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x60), None, flags!(pref_66), Some(SSE42)),
			],

			"pcmpgtb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x64), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x64), None, flags!(pref_66), Some(SSE2)),
			],

			"pcmpgtd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x66), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x66), None, flags!(pref_66), Some(SSE2)),
			],

			"pcmpgtq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x37), None, flags!(pref_66), Some(SSE42)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x37), None, flags!(pref_66), Some(SSE42)),
			],

			"pcmpgtw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x65), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x65), None, flags!(pref_66), Some(SSE2)),
			],

			"pcmpistri" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x63), None, flags!(pref_66), Some(SSE42)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x63), None, flags!(pref_66), Some(SSE42)),
			],

			"pcmpistrm" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x62), None, flags!(pref_66), Some(SSE42)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x62), None, flags!(pref_66), Some(SSE42)),
			],

			"pdep" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x02, 0xF5), None, flags!(vex_op, auto_rex_w, pref_F2), Some(BMI2)),
			],

			"pdistib" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x54), None, flags!(), Some(MMX_CYRIX)),
			],

			"pext" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x02, 0xF5), None, flags!(vex_op, auto_rex_w, pref_F3), Some(BMI2)),
			],

			"pextrb" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x14), None, flags!(enc_mr, pref_66), Some(SSE41)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x14), None, flags!(enc_mr, pref_66), Some(SSE41)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x14), None, flags!(with_rex_w, enc_mr, pref_66), Some(SSE41)),
			],

			"pextrd" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x16), None, flags!(enc_mr, pref_66), Some(SSE41)),
			],

			"pextrq" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x16), None, flags!(with_rex_w, enc_mr, pref_66), Some(SSE41)),
			],

			"pextrw" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(WORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x15), None, flags!(enc_mr, pref_66), Some(SSE41)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(mmx_register, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC5), None, flags!(), Some(MMX)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC5), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x15), None, flags!(enc_mr, pref_66), Some(SSE41)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x15), None, flags!(with_rex_w, enc_mr, pref_66), Some(SSE41)),
			],

			"pf2id" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x1D), None, flags!(imm_op), Some(TDNOW)),
			],

			"pf2iw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x1C), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfacc" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xAE), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfadd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x9E), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfcmpeq" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xB0), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfcmpge" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x90), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfcmpgt" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xA0), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfmax" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xA4), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfmin" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x94), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfmul" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xB4), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfnacc" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x8A), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfpnacc" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x8E), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfrcp" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x96), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfrcpit1" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xA6), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfrcpit2" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xB6), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfrcpv" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x86), None, flags!(imm_op), Some(TDNOW_CYRIX)),
			],

			"pfrsqit1" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xA7), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfrsqrt" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x97), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfrsqrtv" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x87), None, flags!(imm_op), Some(TDNOW_CYRIX)),
			],

			"pfsub" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x9A), None, flags!(imm_op), Some(TDNOW)),
			],

			"pfsubr" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xAA), None, flags!(imm_op), Some(TDNOW)),
			],

			"phaddd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x02), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x02), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x02), None, flags!(pref_66), Some(SSSE3)),
			],

			"phaddsw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x03), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x03), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x03), None, flags!(pref_66), Some(SSSE3)),
			],

			"phaddw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x01), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x01), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x01), None, flags!(pref_66), Some(SSSE3)),
			],

			"phminposuw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x41), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x41), None, flags!(pref_66), Some(SSE41)),
			],

			"phsubd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x06), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x06), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x06), None, flags!(pref_66), Some(SSSE3)),
			],

			"phsubsw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x07), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x07), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x07), None, flags!(pref_66), Some(SSSE3)),
			],

			"phsubw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x05), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x05), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x05), None, flags!(pref_66), Some(SSSE3)),
			],

			"pi2fd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x0D), None, flags!(imm_op), Some(TDNOW)),
			],

			"pi2fw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0x0C), None, flags!(imm_op), Some(TDNOW)),
			],

			"pinsrb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, NoSize), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x20), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x20), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x20), None, flags!(pref_66), Some(SSE41)),
			],

			"pinsrd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, NoSize), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x22), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x22), None, flags!(pref_66), Some(SSE41)),
			],

			"pinsrq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, NoSize), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x22), None, flags!(with_rex_w, pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x22), None, flags!(with_rex_w, pref_66), Some(SSE41)),
			],

			"pinsrw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(memory, NoSize), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC4), None, flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(legacy_register, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC4), None, flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(legacy_register_and_memory, Fixed(WORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC4), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, NoSize), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC4), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(WORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC4), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC4), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register, Fixed(WORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC4), None, flags!(pref_66), Some(SSE2)),
			],

			"pmachriw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5E), None, flags!(), Some(MMX_CYRIX)),
			],

			"pmaddubsw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x04), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x04), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x04), None, flags!(pref_66), Some(SSSE3)),
			],

			"pmaddwd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xF5), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xF5), None, flags!(pref_66), Some(SSE2)),
			],

			"pmagw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x52), None, flags!(), Some(MMX_CYRIX)),
			],

			"pmaxsb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x3C), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x3C), None, flags!(pref_66), Some(SSE41)),
			],

			"pmaxsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x3D), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x3D), None, flags!(pref_66), Some(SSE41)),
			],

			"pmaxsw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xEE), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xEE), None, flags!(pref_66), Some(SSE2)),
			],

			"pmaxub" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xDE), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xDE), None, flags!(pref_66), Some(SSE2)),
			],

			"pmaxud" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x3F), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x3F), None, flags!(pref_66), Some(SSE41)),
			],

			"pmaxuw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x3E), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x3E), None, flags!(pref_66), Some(SSE41)),
			],

			"pminsb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x38), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x38), None, flags!(pref_66), Some(SSE41)),
			],

			"pminsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x39), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x39), None, flags!(pref_66), Some(SSE41)),
			],

			"pminsw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xEA), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xEA), None, flags!(pref_66), Some(SSE2)),
			],

			"pminub" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xDA), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xDA), None, flags!(pref_66), Some(SSE2)),
			],

			"pminud" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x3B), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x3B), None, flags!(pref_66), Some(SSE41)),
			],

			"pminuw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x3A), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x3A), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovmskb" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(mmx_register, Fixed(QWORD))], _2(0x0F, 0xD7), None, flags!(), Some(MMX)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0xD7), None, flags!(pref_66), Some(SSE2)),
			],

			"pmovsxbd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0x38, 0x21), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x21), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovsxbq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(WORD))], _3(0x0F, 0x38, 0x22), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x22), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovsxbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x20), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x20), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovsxdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x25), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x25), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovsxwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x23), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x23), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovsxwq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0x38, 0x24), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x24), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovzxbd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0x38, 0x31), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x31), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovzxbq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(WORD))], _3(0x0F, 0x38, 0x32), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x32), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovzxbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x30), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x30), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovzxdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x35), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x35), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovzxwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x33), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x33), None, flags!(pref_66), Some(SSE41)),
			],

			"pmovzxwq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _3(0x0F, 0x38, 0x34), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x34), None, flags!(pref_66), Some(SSE41)),
			],

			"pmuldq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x28), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x28), None, flags!(pref_66), Some(SSE41)),
			],

			"pmulhriw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x5D), None, flags!(), Some(MMX_CYRIX)),
			],

			"pmulhrsw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x0B), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x0B), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x0B), None, flags!(pref_66), Some(SSSE3)),
			],

			"pmulhrwa" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xB7), None, flags!(imm_op), Some(TDNOW)),
			],

			"pmulhrwc" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x59), None, flags!(), Some(MMX_CYRIX)),
			],

			"pmulhuw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xE4), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE4), None, flags!(pref_66), Some(SSE2)),
			],

			"pmulhw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xE5), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE5), None, flags!(pref_66), Some(SSE2)),
			],

			"pmulld" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x40), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x40), None, flags!(pref_66), Some(SSE41)),
			],

			"pmullw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xD5), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xD5), None, flags!(pref_66), Some(SSE2)),
			],

			"pmuludq" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xF4), None, flags!(), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xF4), None, flags!(pref_66), Some(SSE2)),
			],

			"pmvgezb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5C), None, flags!(), Some(MMX_CYRIX)),
			],

			"pmvlzb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5B), None, flags!(), Some(MMX_CYRIX)),
			],

			"pmvnzb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5A), None, flags!(), Some(MMX_CYRIX)),
			],

			"pmvzb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x58), None, flags!(), Some(MMX_CYRIX)),
			],

			"pop" => signatures!
			[
				signature(parameters![parameter(es, Fixed(WORD))], _1(0x07), None, flags!(x86_only), None),
				signature(parameters![parameter(ss, Fixed(WORD))], _1(0x17), None, flags!(x86_only), None),
				signature(parameters![parameter(ds, Fixed(WORD))], _1(0x1F), None, flags!(x86_only), None),
				signature(parameters![parameter(fs, Fixed(WORD))], _2(0x0F, 0xA1), None, flags!(), None),
				signature(parameters![parameter(gs, Fixed(WORD))], _2(0x0F, 0xA9), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _1(0x58), None, flags!(auto_no_32, short_arg), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible)], _1(0x8F), Some(RAX), flags!(auto_no_32), None),
			],

			"popa" => signatures!
			[
				signature(parameters![], _1(0x61), None, flags!(x86_only, word_size), None),
			],

			"popad" => signatures!
			[
				signature(parameters![], _1(0x61), None, flags!(x86_only), None),
			],

			"popcnt" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0xB8), None, flags!(auto_size, pref_F3), None),
			],

			"popf" => signatures!
			[
				signature(parameters![], _1(0x9D), None, flags!(), None),
			],

			"popfq" => signatures!
			[
				signature(parameters![], _1(0x9D), None, flags!(), None),
			],

			"popfw" => signatures!
			[
				signature(parameters![], _1(0x9D), None, flags!(word_size), None),
			],

			"por" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xEB), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xEB), None, flags!(pref_66), Some(SSE2)),
			],

			"prefetch" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD))], _2(0x0F, 0x0D), Some(RAX), flags!(), Some(TDNOW)),
			],

			"prefetchnta" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _2(0x0F, 0x18), Some(RAX), flags!(), None),
			],

			"prefetcht0" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _2(0x0F, 0x18), Some(RCX), flags!(), None),
			],

			"prefetcht1" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _2(0x0F, 0x18), Some(RDX), flags!(), None),
			],

			"prefetcht2" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _2(0x0F, 0x18), Some(RBX), flags!(), None),
			],

			"prefetchw" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD))], _2(0x0F, 0x0D), Some(RCX), flags!(), Some(TDNOW)),
			],

			"prefetchwt1" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE))], _2(0x0F, 0x0D), Some(RDX), flags!(), Some(PREFETCHWT1)),
			],

			"psadbw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xF6), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xF6), None, flags!(pref_66), Some(SSE2)),
			],

			"pshufb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x00), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x00), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x00), None, flags!(pref_66), Some(SSSE3)),
			],

			"pshufd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x70), None, flags!(pref_66), Some(SSE2)),
			],

			"pshufhw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x70), None, flags!(pref_F3), Some(SSE2)),
			],

			"pshuflw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x70), None, flags!(pref_F2), Some(SSE2)),
			],

			"pshufw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x70), None, flags!(), Some(MMX)),
			],

			"psignb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x08), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x08), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x08), None, flags!(pref_66), Some(SSSE3)),
			],

			"psignd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x0A), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x0A), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x0A), None, flags!(pref_66), Some(SSSE3)),
			],

			"psignw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x09), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x09), None, flags!(pref_66), Some(SSSE3)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x09), None, flags!(pref_66), Some(SSSE3)),
			],

			"pslld" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x72), Some(RSI), flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xF2), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x72), Some(RSI), flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xF2), None, flags!(pref_66), Some(SSE2)),
			],

			"pslldq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x73), Some(RDI), flags!(pref_66), Some(SSE2)),
			],

			"psllq" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x73), Some(RSI), flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xF3), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x73), Some(RSI), flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xF3), None, flags!(pref_66), Some(SSE2)),
			],

			"psllw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x71), Some(RSI), flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xF1), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x71), Some(RSI), flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xF1), None, flags!(pref_66), Some(SSE2)),
			],

			"psrad" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x72), Some(RSP), flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xE2), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x72), Some(RSP), flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE2), None, flags!(pref_66), Some(SSE2)),
			],

			"psraw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x71), Some(RSP), flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xE1), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x71), Some(RSP), flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE1), None, flags!(pref_66), Some(SSE2)),
			],

			"psrld" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x72), Some(RDX), flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xD2), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x72), Some(RDX), flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xD2), None, flags!(pref_66), Some(SSE2)),
			],

			"psrldq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x73), Some(RBX), flags!(pref_66), Some(SSE2)),
			],

			"psrlq" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x73), Some(RDX), flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xD3), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x73), Some(RDX), flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xD3), None, flags!(pref_66), Some(SSE2)),
			],

			"psrlw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x71), Some(RDX), flags!(), Some(MMX)),
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xD1), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0x71), Some(RDX), flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xD1), None, flags!(pref_66), Some(SSE2)),
			],

			"psubb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xF8), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xF8), None, flags!(pref_66), Some(SSE2)),
			],

			"psubd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xFA), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xFA), None, flags!(pref_66), Some(SSE2)),
			],

			"psubq" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xFB), None, flags!(), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xFB), None, flags!(pref_66), Some(SSE2)),
			],

			"psubsb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xE8), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE8), None, flags!(pref_66), Some(SSE2)),
			],

			"psubsiw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x55), None, flags!(), Some(MMX_CYRIX)),
			],

			"psubsw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xE9), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xE9), None, flags!(pref_66), Some(SSE2)),
			],

			"psubusb" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xD8), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xD8), None, flags!(pref_66), Some(SSE2)),
			],

			"psubusw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xD9), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xD9), None, flags!(pref_66), Some(SSE2)),
			],

			"psubw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xF9), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xF9), None, flags!(pref_66), Some(SSE2)),
			],

			"pswapd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _3(0x0F, 0x0F, 0xBB), None, flags!(imm_op), Some(TDNOW)),
			],

			"ptest" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x0F, 0x38, 0x17), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x0F, 0x38, 0x17), None, flags!(pref_66), Some(SSE41)),
			],

			"punpckhbw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x68), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x68), None, flags!(pref_66), Some(SSE2)),
			],

			"punpckhdq" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x6A), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x6A), None, flags!(pref_66), Some(SSE2)),
			],

			"punpckhqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x6D), None, flags!(pref_66), Some(SSE2)),
			],

			"punpckhwd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x69), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x69), None, flags!(pref_66), Some(SSE2)),
			],

			"punpcklbw" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x60), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x60), None, flags!(pref_66), Some(SSE2)),
			],

			"punpckldq" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x62), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x62), None, flags!(pref_66), Some(SSE2)),
			],

			"punpcklqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x6C), None, flags!(pref_66), Some(SSE2)),
			],

			"punpcklwd" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x61), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x61), None, flags!(pref_66), Some(SSE2)),
			],

			"push" => signatures!
			[
				signature(parameters![parameter(es, Fixed(WORD))], _1(0x06), None, flags!(x86_only), None),
				signature(parameters![parameter(cs, Fixed(WORD))], _1(0x0E), None, flags!(x86_only), None),
				signature(parameters![parameter(ss, Fixed(WORD))], _1(0x16), None, flags!(x86_only), None),
				signature(parameters![parameter(ds, Fixed(WORD))], _1(0x1E), None, flags!(x86_only), None),
				signature(parameters![parameter(fs, Fixed(WORD))], _2(0x0F, 0xA0), None, flags!(), None),
				signature(parameters![parameter(gs, Fixed(WORD))], _2(0x0F, 0xA8), None, flags!(), None),
				signature(parameters![parameter(immediate, Fixed(BYTE))], _1(0x6A), None, flags!(exact_size), None),
				signature(parameters![parameter(immediate, Fixed(WORD))], _1(0x68), None, flags!(exact_size, word_size), None),
				signature(parameters![parameter(immediate, Fixed(DWORD))], _1(0x68), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _1(0x50), None, flags!(auto_no_32, short_arg), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible)], _1(0xFF), Some(RSI), flags!(auto_no_32), None),
			],

			"pusha" => signatures!
			[
				signature(parameters![], _1(0x60), None, flags!(x86_only, word_size), None),
			],

			"pushad" => signatures!
			[
				signature(parameters![], _1(0x60), None, flags!(x86_only), None),
			],

			"pushf" => signatures!
			[
				signature(parameters![], _1(0x9C), None, flags!(), None),
			],

			"pushfq" => signatures!
			[
				signature(parameters![], _1(0x9C), None, flags!(), None),
			],

			"pushfw" => signatures!
			[
				signature(parameters![], _1(0x9C), None, flags!(word_size), None),
			],

			"pxor" => signatures!
			[
				signature(parameters![parameter(mmx_register, Fixed(QWORD)), parameter(mmx_register_and_memory, Fixed(QWORD))], _2(0x0F, 0xEF), None, flags!(), Some(MMX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0xEF), None, flags!(pref_66), Some(SSE2)),
			],

			"rcl" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(rcx, Fixed(BYTE))], _1(0xD2), Some(RDX), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xC0), Some(RDX), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(rcx, Fixed(BYTE))], _1(0xD3), Some(RDX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0xC1), Some(RDX), flags!(auto_size), None),
			],

			"rcpps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x53), None, flags!(), Some(SSE)),
			],

			"rcpss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x53), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x53), None, flags!(pref_F3), Some(SSE)),
			],

			"rcr" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(rcx, Fixed(BYTE))], _1(0xD2), Some(RBX), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xC0), Some(RBX), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(rcx, Fixed(BYTE))], _1(0xD3), Some(RBX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0xC1), Some(RBX), flags!(auto_size), None),
			],

			"rdfsbase" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD))], _2(0x0F, 0xAE), Some(RAX), flags!(pref_F3), None),
				signature(parameters![parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0xAE), Some(RAX), flags!(with_rex_w, pref_F3), None),
			],

			"rdgsbase" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD))], _2(0x0F, 0xAE), Some(RCX), flags!(pref_F3), None),
				signature(parameters![parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0xAE), Some(RCX), flags!(with_rex_w, pref_F3), None),
			],

			"rdm" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x3A), None, flags!(), Some(CYRIX)),
			],

			"rdmsr" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x32), None, flags!(), None),
			],

			"rdpid" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0xC7), Some(RDI), flags!(pref_F3), None),
			],

			"rdpkru" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xEE), None, flags!(), None),
			],

			"rdpmc" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x33), None, flags!(), None),
			],

			"rdrand" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0xC7), Some(RSI), flags!(with_rex_w), None),
			],

			"rdseed" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0xC7), Some(RDI), flags!(with_rex_w), None),
			],

			"rdshr" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(DWORD))], _2(0x0F, 0x36), Some(RAX), flags!(), Some(CYRIX)),
			],

			"rdtsc" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x31), None, flags!(), None),
			],

			"rdtscp" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xF9), None, flags!(), None),
			],

			"ret" => signatures!
			[
				signature(parameters![], _1(0xC3), None, flags!(), None),
				signature(parameters![parameter(immediate, Fixed(WORD))], _1(0xC2), None, flags!(), None),
			],

			"retf" => signatures!
			[
				signature(parameters![], _1(0xCB), None, flags!(), None),
				signature(parameters![parameter(immediate, Fixed(WORD))], _1(0xCA), None, flags!(), None),
			],

			"retn" => signatures!
			[
				signature(parameters![], _1(0xC3), None, flags!(), None),
				signature(parameters![parameter(immediate, Fixed(WORD))], _1(0xC2), None, flags!(), None),
			],

			"rol" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(rcx, Fixed(BYTE))], _1(0xD2), Some(RAX), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xC0), Some(RAX), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(rcx, Fixed(BYTE))], _1(0xD3), Some(RAX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0xC1), Some(RAX), flags!(auto_size), None),
			],

			"ror" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(rcx, Fixed(BYTE))], _1(0xD2), Some(RCX), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xC0), Some(RCX), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(rcx, Fixed(BYTE))], _1(0xD3), Some(RCX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0xC1), Some(RCX), flags!(auto_size), None),
			],

			"rorx" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0xF0), None, flags!(vex_op, auto_rex_w, pref_F2), Some(BMI2)),
			],

			"roundpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x09), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x09), None, flags!(pref_66), Some(SSE41)),
			],

			"roundps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x08), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x08), None, flags!(pref_66), Some(SSE41)),
			],

			"roundsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0B), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0B), None, flags!(pref_66), Some(SSE41)),
			],

			"roundss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0A), None, flags!(pref_66), Some(SSE41)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0x0A), None, flags!(pref_66), Some(SSE41)),
			],

			"rsdc" => signatures!
			[
				signature(parameters![parameter(segment_register, Fixed(WORD)), parameter(memory, Fixed(PWORD))], _2(0x0F, 0x79), None, flags!(exact_size), Some(CYRIX)),
			],

			"rsldt" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(PWORD))], _2(0x0F, 0x7B), Some(RAX), flags!(exact_size), Some(CYRIX)),
			],

			"rsm" => signatures!
			[
				signature(parameters![], _2(0x0F, 0xAA), None, flags!(), None),
			],

			"rsqrtps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x52), None, flags!(), Some(SSE)),
			],

			"rsqrtss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x52), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x52), None, flags!(pref_F3), Some(SSE)),
			],

			"rsts" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(PWORD))], _2(0x0F, 0x7D), Some(RAX), flags!(exact_size), Some(CYRIX)),
			],

			"sahf" => signatures!
			[
				signature(parameters![], _1(0x9E), None, flags!(), None),
			],

			"sal" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(rcx, Fixed(BYTE))], _1(0xD2), Some(RSP), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xC0), Some(RSP), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(rcx, Fixed(BYTE))], _1(0xD3), Some(RSP), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0xC1), Some(RSP), flags!(auto_size), None),
			],

			"sar" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(rcx, Fixed(BYTE))], _1(0xD2), Some(RDI), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xC0), Some(RDI), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(rcx, Fixed(BYTE))], _1(0xD3), Some(RDI), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0xC1), Some(RDI), flags!(auto_size), None),
			],

			"sarx" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x02, 0xF7), None, flags!(vex_op, auto_rex_w, enc_mr, pref_F3), Some(BMI2)),
			],

			"sbb" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x1C), None, flags!(), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RBX), flags!(lock), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x18), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RBX), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x18), None, flags!(enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0x1A), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RBX), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(rax, AllPossible), parameter(immediate, AllPossible)], _1(0x1D), None, flags!(auto_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RBX), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RBX), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x19), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RBX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _1(0x19), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _1(0x1B), None, flags!(auto_size), None),
			],

			"scasb" => signatures!
			[
				signature(parameters![], _1(0xAE), None, flags!(repe), None),
			],

			"scasd" => signatures!
			[
				signature(parameters![], _1(0xAF), None, flags!(repe), None),
			],

			"scasq" => signatures!
			[
				signature(parameters![], _1(0xAF), None, flags!(repe, with_rex_w), None),
			],

			"scasw" => signatures!
			[
				signature(parameters![], _1(0xAF), None, flags!(repe, word_size), None),
			],

			"seta" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x97), Some(RAX), flags!(), None),
			],

			"setae" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x93), Some(RAX), flags!(), None),
			],

			"setb" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x92), Some(RAX), flags!(), None),
			],

			"setbe" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x96), Some(RAX), flags!(), None),
			],

			"setc" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x92), Some(RAX), flags!(), None),
			],

			"sete" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x94), Some(RAX), flags!(), None),
			],

			"setg" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9F), Some(RAX), flags!(), None),
			],

			"setge" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9D), Some(RAX), flags!(), None),
			],

			"setl" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9C), Some(RAX), flags!(), None),
			],

			"setle" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9E), Some(RAX), flags!(), None),
			],

			"setna" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x96), Some(RAX), flags!(), None),
			],

			"setnae" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x92), Some(RAX), flags!(), None),
			],

			"setnb" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x93), Some(RAX), flags!(), None),
			],

			"setnbe" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x97), Some(RAX), flags!(), None),
			],

			"setnc" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x93), Some(RAX), flags!(), None),
			],

			"setne" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x95), Some(RAX), flags!(), None),
			],

			"setng" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9E), Some(RAX), flags!(), None),
			],

			"setnge" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9C), Some(RAX), flags!(), None),
			],

			"setnl" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9D), Some(RAX), flags!(), None),
			],

			"setnle" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9F), Some(RAX), flags!(), None),
			],

			"setno" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x91), Some(RAX), flags!(), None),
			],

			"setnp" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9B), Some(RAX), flags!(), None),
			],

			"setns" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x99), Some(RAX), flags!(), None),
			],

			"setnz" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x95), Some(RAX), flags!(), None),
			],

			"seto" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x90), Some(RAX), flags!(), None),
			],

			"setp" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9A), Some(RAX), flags!(), None),
			],

			"setpe" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9A), Some(RAX), flags!(), None),
			],

			"setpo" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x9B), Some(RAX), flags!(), None),
			],

			"sets" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x98), Some(RAX), flags!(), None),
			],

			"setz" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE))], _2(0x0F, 0x94), Some(RAX), flags!(), None),
			],

			"sfence" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xAE, 0xF8), None, flags!(), Some(AMD)),
			],

			"sgdt" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x01), Some(RAX), flags!(), None),
			],

			"sha1msg1" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xC9), None, flags!(), Some(SHA)),
			],

			"sha1msg2" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xCA), None, flags!(), Some(SHA)),
			],

			"sha1nexte" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xC8), None, flags!(), Some(SHA)),
			],

			"sha1rnds4" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _3(0x0F, 0x3A, 0xCC), None, flags!(), Some(SHA)),
			],

			"sha256msg1" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xCC), None, flags!(), Some(SHA)),
			],

			"sha256msg2" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xCD), None, flags!(), Some(SHA)),
			],

			"sha256rnds2" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x0F, 0x38, 0xCB), None, flags!(), Some(SHA)),
			],

			"shl" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(rcx, Fixed(BYTE))], _1(0xD2), Some(RSP), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xC0), Some(RSP), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(rcx, Fixed(BYTE))], _1(0xD3), Some(RSP), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0xC1), Some(RSP), flags!(auto_size), None),
			],

			"shld" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible), parameter(rcx, Fixed(BYTE))], _2(0x0F, 0xA5), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xA4), None, flags!(auto_size, enc_mr), None),
			],

			"shlx" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x02, 0xF7), None, flags!(vex_op, auto_rex_w, enc_mr, pref_66), Some(BMI2)),
			],

			"shr" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(rcx, Fixed(BYTE))], _1(0xD2), Some(RBP), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xC0), Some(RBP), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(rcx, Fixed(BYTE))], _1(0xD3), Some(RBP), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0xC1), Some(RBP), flags!(auto_size), None),
			],

			"shrd" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible), parameter(rcx, Fixed(BYTE))], _2(0x0F, 0xAD), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xAC), None, flags!(auto_size, enc_mr), None),
			],

			"shrx" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x02, 0xF7), None, flags!(vex_op, auto_rex_w, enc_mr, pref_F2), Some(BMI2)),
			],

			"shufpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC6), None, flags!(pref_66), Some(SSE2)),
			],

			"shufps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x0F, 0xC6), None, flags!(), Some(SSE)),
			],

			"sidt" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x01), Some(RCX), flags!(), None),
			],

			"skinit" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xDE), None, flags!(), None),
			],

			"sldt" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x00), Some(RAX), flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _2(0x0F, 0x00), Some(RAX), flags!(auto_size), None),
			],

			"slwpcb" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible)], _2(0x09, 0x12), Some(RCX), flags!(xop_op, auto_rex_w), Some(AMD)),
			],

			"smint" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x38), None, flags!(), Some(CYRIX)),
			],

			"smsw" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x01), Some(RSP), flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _2(0x0F, 0x01), Some(RSP), flags!(auto_size), None),
			],

			"sqrtpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x51), None, flags!(pref_66), Some(SSE2)),
			],

			"sqrtps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x51), None, flags!(), Some(SSE)),
			],

			"sqrtsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x51), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x51), None, flags!(pref_F2), Some(SSE2)),
			],

			"sqrtss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x51), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x51), None, flags!(pref_F3), Some(SSE)),
			],

			"stac" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xCB), None, flags!(), None),
			],

			"stc" => signatures!
			[
				signature(parameters![], _1(0xF9), None, flags!(), None),
			],

			"std" => signatures!
			[
				signature(parameters![], _1(0xFD), None, flags!(), None),
			],

			"stgi" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xDC), None, flags!(), Some(VMX_AMD)),
			],

			"sti" => signatures!
			[
				signature(parameters![], _1(0xFB), None, flags!(), None),
			],

			"stmxcsr" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _2(0x0F, 0xAE), Some(RBX), flags!(), Some(SSE)),
			],

			"stosb" => signatures!
			[
				signature(parameters![], _1(0xAA), None, flags!(rep), None),
			],

			"stosd" => signatures!
			[
				signature(parameters![], _1(0xAB), None, flags!(rep), None),
			],

			"stosq" => signatures!
			[
				signature(parameters![], _1(0xAB), None, flags!(with_rex_w, rep), None),
			],

			"stosw" => signatures!
			[
				signature(parameters![], _1(0xAB), None, flags!(word_size, rep), None),
			],

			"str" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x00), Some(RCX), flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible)], _2(0x0F, 0x00), Some(RCX), flags!(auto_size), None),
			],

			"sub" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x2C), None, flags!(), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RBP), flags!(lock), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x28), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RBP), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x28), None, flags!(enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0x2A), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RBP), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(rax, AllPossible), parameter(immediate, AllPossible)], _1(0x2D), None, flags!(auto_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RBP), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RBP), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x29), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RBP), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _1(0x29), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _1(0x2B), None, flags!(auto_size), None),
			],

			"subpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5C), None, flags!(pref_66), Some(SSE2)),
			],

			"subps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x5C), None, flags!(), Some(SSE)),
			],

			"subsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x5C), None, flags!(pref_F2), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5C), None, flags!(pref_F2), Some(SSE2)),
			],

			"subss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x5C), None, flags!(pref_F3), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x5C), None, flags!(pref_F3), Some(SSE)),
			],

			"svdc" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(PWORD)), parameter(segment_register, Fixed(WORD))], _2(0x0F, 0x78), None, flags!(enc_mr, exact_size), Some(CYRIX)),
			],

			"svldt" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(PWORD))], _2(0x0F, 0x7A), Some(RAX), flags!(exact_size), Some(CYRIX)),
			],

			"svts" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(PWORD))], _2(0x0F, 0x7C), Some(RAX), flags!(exact_size), Some(CYRIX)),
			],

			"swapgs" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xF8), None, flags!(), None),
			],

			"syscall" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x05), None, flags!(), Some(AMD)),
			],

			"sysenter" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x34), None, flags!(x86_only), None),
			],

			"sysexit" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x35), None, flags!(x86_only), None),
			],

			"sysret" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x07), None, flags!(), Some(AMD)),
			],

			"t1mskc" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x09, 0x01), Some(RDI), flags!(xop_op, auto_rex_w, enc_vm), Some(TBM)),
			],

			"test" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xA8), None, flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(memory, Fixed(BYTE))], _1(0x84), None, flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0xF6), Some(RAX), flags!(), None),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x84), None, flags!(enc_mr), None),
				signature(parameters![parameter(rax, AllPossible), parameter(immediate, AllPossible)], _1(0xA9), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, AllPossible)], _1(0x85), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(immediate, AllPossible)], _1(0xF7), Some(RAX), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register_and_memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x85), None, flags!(auto_size, enc_mr), None),
			],

			"tzcnt" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x0F, 0xBC), None, flags!(auto_size, pref_F3), Some(BMI1)),
			],

			"tzmsk" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _2(0x09, 0x01), Some(RSP), flags!(xop_op, auto_rex_w, enc_vm), Some(TBM)),
			],

			"ucomisd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x0F, 0x2E), None, flags!(pref_66), Some(SSE2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2E), None, flags!(pref_66), Some(SSE2)),
			],

			"ucomiss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x0F, 0x2E), None, flags!(), Some(SSE)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x0F, 0x2E), None, flags!(), Some(SSE)),
			],

			"ud2" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x0B), None, flags!(), None),
			],

			"ud2a" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x0B), None, flags!(), None),
			],

			"unpckhpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x15), None, flags!(pref_66), Some(SSE2)),
			],

			"unpckhps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x15), None, flags!(), Some(SSE)),
			],

			"unpcklpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x14), None, flags!(pref_66), Some(SSE2)),
			],

			"unpcklps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x14), None, flags!(), Some(SSE)),
			],

			"vaddpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x58), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vaddps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x58), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vaddsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x58), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x58), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vaddss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x58), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x58), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vaddsubpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xD0), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vaddsubps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xD0), None, flags!(vex_op, auto_vex_l, pref_F2), Some(AVX)),
			],

			"vaesdec" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0xDE), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vaesdeclast" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0xDF), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vaesenc" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0xDC), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vaesenclast" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0xDD), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vaesimc" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0xDB), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vaeskeygenassist" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0xDF), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vandnpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x55), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vandnps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x55), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vandpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x54), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vandps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x54), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vblendpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x0D), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vblendps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x0C), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vblendvpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x4B), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vblendvps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x4A), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vbroadcastf128" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(memory, Fixed(OWORD))], _2(0x02, 0x1A), None, flags!(with_vex_l, vex_op, pref_66), Some(AVX)),
			],

			"vbroadcasti128" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(memory, Fixed(OWORD))], _2(0x02, 0x5A), None, flags!(with_vex_l, vex_op, pref_66), Some(AVX2)),
			],

			"vbroadcastsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x19), None, flags!(vex_op, with_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x19), None, flags!(vex_op, with_vex_l, pref_66), Some(AVX)),
			],

			"vbroadcastss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(memory, Fixed(DWORD))], _2(0x02, 0x18), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x18), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vcmpeq_ospd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x10), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x10), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpeq_osps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x10), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpeq_ossd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x10), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x10), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpeq_osss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x10), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x10), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpeq_uqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x08), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x08), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmpeq_uqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x08), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpeq_uqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x08), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x08), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpeq_uqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x08), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x08), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpeq_uspd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x18), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x18), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmpeq_usps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x18), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpeq_ussd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x18), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x18), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpeq_usss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x18), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x18), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpeqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x00), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpeqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x00), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpeqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x00), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x00), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpeqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x00), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x00), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpfalse_oqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpfalse_oqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpfalse_oqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpfalse_oqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpfalse_ospd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1B), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmpfalse_osps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1B), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpfalse_ossd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1B), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1B), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpfalse_osss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1B), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1B), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpfalsepd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpfalseps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpfalsesd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpfalsess" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0B), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpge_oqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x1D), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x1D), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpge_oqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1D), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpge_oqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1D), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1D), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpge_oqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1D), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1D), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpge_ospd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmpge_osps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpge_ossd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpge_osss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpgepd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpgeps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpgesd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpgess" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0D), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpgt_oqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1E), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpgt_oqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1E), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpgt_oqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1E), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1E), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpgt_oqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1E), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1E), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpgt_ospd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpgt_osps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpgt_ossd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpgt_osss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpgtpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpgtps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpgtsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpgtss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0E), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmple_oqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x12), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x12), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmple_oqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x12), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmple_oqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x12), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x12), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmple_oqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x12), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x12), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmple_ospd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmple_osps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmple_ossd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmple_osss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmplepd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x02), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmpleps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmplesd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpless" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x02), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmplt_oqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x11), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmplt_oqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x11), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmplt_oqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x11), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x11), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmplt_oqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x11), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x11), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmplt_ospd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x01), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmplt_osps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmplt_ossd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmplt_osss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpltpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x01), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmpltps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpltsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpltss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x01), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpneq_oqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x0C), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x0C), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmpneq_oqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0C), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpneq_oqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0C), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0C), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpneq_oqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0C), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0C), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpneq_ospd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x1C), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x1C), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpneq_osps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1C), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpneq_ossd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1C), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1C), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpneq_osss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1C), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1C), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpneq_uqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmpneq_uqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpneq_uqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpneq_uqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpneq_uspd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x14), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x14), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmpneq_usps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x14), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpneq_ussd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x14), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x14), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpneq_usss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x14), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x14), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpneqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x04), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpneqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpneqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpneqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x04), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpnge_uqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x19), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmpnge_uqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x19), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpnge_uqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x19), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x19), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpnge_uqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x19), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x19), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpnge_uspd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpnge_usps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpnge_ussd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpnge_usss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpngepd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x09), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpngeps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpngesd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpngess" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x09), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpngt_uqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1A), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmpngt_uqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1A), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpngt_uqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1A), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1A), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpngt_uqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1A), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1A), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpngt_uspd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmpngt_usps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpngt_ussd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpngt_usss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpngtpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpngtps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpngtsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpngtss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0A), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpnle_uqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x16), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmpnle_uqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x16), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpnle_uqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x16), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x16), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpnle_uqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x16), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x16), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpnle_uspd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x06), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmpnle_usps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpnle_ussd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpnle_usss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpnlepd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpnleps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpnlesd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpnless" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x06), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpnlt_uqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x15), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x15), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmpnlt_uqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x15), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpnlt_uqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x15), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x15), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpnlt_uqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x15), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x15), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpnlt_uspd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpnlt_usps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpnlt_ussd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpnlt_usss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpnltpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x05), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpnltps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpnltsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpnltss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x05), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpord_qpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x07), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpord_qps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpord_qsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpord_qss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpord_spd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x17), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x17), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpord_sps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x17), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpord_ssd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x17), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x17), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpord_sss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x17), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x17), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpordpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x07), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmpordps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpordsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpordss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x07), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmppd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC2), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vcmpps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC2), None, flags!(vex_op, auto_vex_l, enc_mr), Some(AVX)),
			],

			"vcmpsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC2), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC2), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vcmpss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC2), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC2), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vcmptrue_uqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(with_vex_l, vex_op, imm_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vcmptrue_uqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmptrue_uqsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmptrue_uqss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmptrue_uspd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1F), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmptrue_usps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x1F), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmptrue_ussd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1F), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1F), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmptrue_usss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x1F), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x1F), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmptruepd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmptrueps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmptruesd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmptruess" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x0F), None, flags!(vex_op, pref_F3, imm_op), Some(AVX)),
			],

			"vcmpunord_qpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, auto_vex_l, pref_66, imm_op), Some(AVX)),
			],

			"vcmpunord_qps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpunord_qsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, imm_op, pref_F2), Some(AVX)),
			],

			"vcmpunord_qss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpunord_spd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x13), None, flags!(vex_op, auto_vex_l, imm_op, pref_66), Some(AVX)),
			],

			"vcmpunord_sps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x13), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpunord_ssd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x13), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x13), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpunord_sss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x13), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x13), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcmpunordpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _3(0x01, 0xC2, 0x03), None, flags!(with_vex_l, vex_op, pref_66, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vcmpunordps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, auto_vex_l, imm_op), Some(AVX)),
			],

			"vcmpunordsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, pref_F2, imm_op), Some(AVX)),
			],

			"vcmpunordss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _3(0x01, 0xC2, 0x03), None, flags!(vex_op, imm_op, pref_F3), Some(AVX)),
			],

			"vcomisd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x2F), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x2F), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vcomiss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x2F), None, flags!(vex_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x2F), None, flags!(vex_op), Some(AVX)),
			],

			"vcvtdq2pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0xE6), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0xE6), None, flags!(vex_op, auto_vex_l, pref_F3), Some(AVX)),
			],

			"vcvtdq2ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5B), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vcvtpd2dq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, AllPossible)], _2(0x01, 0xE6), None, flags!(vex_op, auto_vex_l, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, AllPossible)], _2(0x01, 0xE6), None, flags!(vex_op, auto_vex_l, pref_F2), Some(AVX)),
			],

			"vcvtpd2ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, AllPossible)], _2(0x01, 0x5A), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, AllPossible)], _2(0x01, 0x5A), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vcvtph2ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x13), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0x13), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vcvtps2dq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5B), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vcvtps2pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x5A), None, flags!(vex_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0x5A), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vcvtps2ph" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x1D), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x1D), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vcvtsd2si" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, Fixed(QWORD))], _2(0x01, 0x2D), None, flags!(vex_op, auto_rex_w, pref_F2), Some(AVX)),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x2D), None, flags!(vex_op, auto_rex_w, pref_F2), Some(AVX)),
			],

			"vcvtsd2ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x5A), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5A), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vcvtsi2sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, AllPossible)], _2(0x01, 0x2A), None, flags!(vex_op, auto_rex_w, pref_F2), Some(AVX)),
			],

			"vcvtsi2ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, AllPossible)], _2(0x01, 0x2A), None, flags!(vex_op, auto_rex_w, pref_F3), Some(AVX)),
			],

			"vcvtss2sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x5A), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5A), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vcvtss2si" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, Fixed(DWORD))], _2(0x01, 0x2D), None, flags!(vex_op, auto_rex_w, pref_F3), Some(AVX)),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x2D), None, flags!(vex_op, auto_rex_w, pref_F3), Some(AVX)),
			],

			"vcvttpd2dq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, AllPossible)], _2(0x01, 0xE6), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, AllPossible)], _2(0x01, 0xE6), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vcvttps2dq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5B), None, flags!(vex_op, auto_vex_l, pref_F3), Some(AVX)),
			],

			"vcvttsd2si" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, Fixed(QWORD))], _2(0x01, 0x2C), None, flags!(vex_op, auto_rex_w, pref_F2), Some(AVX)),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x2C), None, flags!(vex_op, auto_rex_w, pref_F2), Some(AVX)),
			],

			"vcvttss2si" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, Fixed(DWORD))], _2(0x01, 0x2C), None, flags!(vex_op, auto_rex_w, pref_F3), Some(AVX)),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x2C), None, flags!(vex_op, auto_rex_w, pref_F3), Some(AVX)),
			],

			"vdivpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5E), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vdivps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5E), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vdivsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x5E), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5E), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vdivss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x5E), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5E), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vdppd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x41), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vdpps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x40), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"verr" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x00), Some(RSP), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(WORD))], _2(0x0F, 0x00), Some(RSP), flags!(), None),
			],

			"verw" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0x00), Some(RBP), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(WORD))], _2(0x0F, 0x00), Some(RBP), flags!(), None),
			],

			"vextractf128" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x19), None, flags!(with_vex_l, vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vextracti128" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x39), None, flags!(with_vex_l, vex_op, enc_mr, pref_66), Some(AVX2)),
			],

			"vextractps" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x17), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vfmadd123pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA8), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd123ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA8), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd123sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xA9), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xA9), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmadd123ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xA9), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xA9), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmadd132pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x98), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd132ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x98), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd132sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x99), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x99), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmadd132ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x99), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x99), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmadd213pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA8), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd213ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA8), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd213sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xA9), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xA9), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmadd213ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xA9), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xA9), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmadd231pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB8), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd231ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB8), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd231sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xB9), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xB9), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmadd231ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xB9), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xB9), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmadd312pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x98), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd312ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x98), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd312sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x99), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x99), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmadd312ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x99), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x99), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmadd321pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB8), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd321ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB8), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmadd321sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xB9), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xB9), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmadd321ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xB9), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xB9), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmaddpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x69), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x69), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfmaddps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x68), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x68), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfmaddsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x6B), None, flags!(vex_op, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x03, 0x6B), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x6B), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
			],

			"vfmaddss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x6A), None, flags!(vex_op, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x03, 0x6A), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x6A), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
			],

			"vfmaddsub123pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA6), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub123ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA6), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub132pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x96), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub132ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x96), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub213pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA6), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub213ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA6), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub231pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB6), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub231ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB6), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub312pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x96), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub312ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x96), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub321pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB6), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsub321ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB6), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmaddsubpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x5D), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x5D), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfmaddsubps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x5C), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x5C), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfmsub123pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAA), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub123ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAA), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub123sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xAB), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAB), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmsub123ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xAB), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAB), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmsub132pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9A), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub132ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9A), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub132sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x9B), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9B), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmsub132ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x9B), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9B), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmsub213pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAA), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub213ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAA), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub213sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xAB), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAB), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmsub213ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xAB), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAB), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmsub231pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBA), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub231ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBA), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub231sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xBB), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBB), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmsub231ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xBB), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBB), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmsub312pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9A), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub312ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9A), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub312sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x9B), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9B), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmsub312ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x9B), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9B), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmsub321pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBA), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub321ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBA), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsub321sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xBB), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBB), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfmsub321ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xBB), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBB), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfmsubadd123pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA7), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd123ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA7), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd132pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x97), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd132ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x97), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd213pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA7), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd213ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xA7), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd231pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB7), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd231ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB7), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd312pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x97), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd312ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x97), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd321pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB7), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubadd321ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xB7), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfmsubaddpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x5F), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x5F), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfmsubaddps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x5E), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x5E), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfmsubpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x6D), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x6D), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfmsubps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x6C), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x6C), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfmsubsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x6F), None, flags!(vex_op, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x03, 0x6F), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x6F), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
			],

			"vfmsubss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x6E), None, flags!(vex_op, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x03, 0x6E), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x6E), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
			],

			"vfnmadd123pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAC), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd123ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAC), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd123sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xAD), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAD), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmadd123ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xAD), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAD), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmadd132pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9C), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd132ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9C), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd132sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x9D), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9D), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmadd132ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x9D), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9D), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmadd213pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAC), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd213ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAC), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd213sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xAD), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAD), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmadd213ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xAD), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAD), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmadd231pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBC), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd231ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBC), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd231sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xBD), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBD), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmadd231ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xBD), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBD), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmadd312pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9C), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd312ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9C), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd312sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x9D), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9D), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmadd312ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x9D), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9D), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmadd321pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBC), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd321ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBC), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmadd321sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xBD), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBD), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmadd321ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xBD), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBD), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmaddpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x79), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x79), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfnmaddps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x78), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x78), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfnmaddsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x7B), None, flags!(vex_op, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x03, 0x7B), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x7B), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
			],

			"vfnmaddss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x7A), None, flags!(vex_op, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x03, 0x7A), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x7A), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
			],

			"vfnmsub123pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAE), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub123ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAE), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub123sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xAF), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAF), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmsub123ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xAF), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAF), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmsub132pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9E), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub132ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9E), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub132sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x9F), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9F), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmsub132ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x9F), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9F), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmsub213pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAE), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub213ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xAE), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub213sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xAF), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAF), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmsub213ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xAF), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xAF), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmsub231pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBE), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub231ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBE), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub231sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xBF), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBF), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmsub231ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xBF), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBF), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmsub312pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9E), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub312ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x9E), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub312sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x9F), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9F), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmsub312ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x9F), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x9F), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmsub321pd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBE), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub321ps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0xBE), None, flags!(vex_op, auto_vex_l, pref_66), Some(FMA)),
			],

			"vfnmsub321sd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0xBF), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBF), None, flags!(vex_op, with_rex_w, pref_66), Some(FMA)),
			],

			"vfnmsub321ss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0xBF), None, flags!(vex_op, pref_66), Some(FMA)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0xBF), None, flags!(vex_op, pref_66), Some(FMA)),
			],

			"vfnmsubpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x7D), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x7D), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfnmsubps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x03, 0x7C), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x7C), None, flags!(vex_op, auto_vex_l, pref_66), Some(SSE5)),
			],

			"vfnmsubsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x7F), None, flags!(vex_op, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x03, 0x7F), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x7F), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
			],

			"vfnmsubss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x7E), None, flags!(vex_op, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x03, 0x7E), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x03, 0x7E), None, flags!(vex_op, with_rex_w, pref_66), Some(SSE5)),
			],

			"vfrczpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x09, 0x81), None, flags!(xop_op, auto_vex_l), Some(SSE5)),
			],

			"vfrczps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x09, 0x80), None, flags!(xop_op, auto_vex_l), Some(SSE5)),
			],

			"vfrczsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x09, 0x83), None, flags!(xop_op), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x83), None, flags!(xop_op), Some(SSE5)),
			],

			"vfrczss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x09, 0x82), None, flags!(xop_op), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x82), None, flags!(xop_op), Some(SSE5)),
			],

			"vgatherdpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(vector_scaled_index_byte_64, Fixed(OWORD)), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x92), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX2)),
			],

			"vgatherdps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(vector_scaled_index_byte_32, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x92), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX2)),
			],

			"vgatherqpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(vector_scaled_index_byte_64, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x93), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX2)),
			],

			"vgatherqps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(vector_scaled_index_byte_32, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x93), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX2)),
			],

			"vhaddpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x7C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vhaddps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x7C), None, flags!(vex_op, auto_vex_l, pref_F2), Some(AVX)),
			],

			"vhsubpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x7D), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vhsubps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x7D), None, flags!(vex_op, auto_vex_l, pref_F2), Some(AVX)),
			],

			"vinsertf128" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x18), None, flags!(with_vex_l, vex_op, pref_66), Some(AVX)),
			],

			"vinserti128" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x38), None, flags!(with_vex_l, vex_op, pref_66), Some(AVX2)),
			],

			"vinsertps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x21), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x21), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vlddqu" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(memory, AllPossible)], _2(0x01, 0xF0), None, flags!(vex_op, auto_vex_l, pref_F2), Some(AVX)),
			],

			"vldmxcsr" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _2(0x01, 0xAE), Some(RDX), flags!(vex_op), Some(AVX)),
			],

			"vldqqu" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(memory, Fixed(HWORD))], _2(0x01, 0xF0), None, flags!(with_vex_l, vex_op, pref_F2), Some(AVX)),
			],

			"vmaskmovdqu" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0xF7), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vmaskmovpd" => signatures!
			[
				signature(parameters![parameter(memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x2F), None, flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(memory, AllPossible)], _2(0x02, 0x2D), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vmaskmovps" => signatures!
			[
				signature(parameters![parameter(memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x2E), None, flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(memory, AllPossible)], _2(0x02, 0x2C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vmaxpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5F), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vmaxps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5F), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vmaxsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x5F), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5F), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vmaxss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x5F), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5F), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vmcall" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xC1), None, flags!(), Some(VMX)),
			],

			"vmclear" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xC7), Some(RSI), flags!(pref_66), Some(VMX)),
			],

			"vmfunc" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xD4), None, flags!(), Some(VMX)),
			],

			"vminpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5D), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vminps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5D), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vminsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x5D), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5D), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vminss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x5D), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5D), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vmlaunch" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xC2), None, flags!(), Some(VMX)),
			],

			"vmload" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xDA), None, flags!(), Some(VMX_AMD)),
			],

			"vmmcall" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xD9), None, flags!(), Some(VMX_AMD)),
			],

			"vmovapd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x28), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD))], _2(0x01, 0x29), None, flags!(vex_op, with_vex_l, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x29), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vmovaps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x28), None, flags!(vex_op, auto_vex_l), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD))], _2(0x01, 0x29), None, flags!(vex_op, with_vex_l, enc_mr), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x29), None, flags!(vex_op, enc_mr), Some(AVX)),
			],

			"vmovd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(DWORD))], _2(0x01, 0x6E), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x7E), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vmovddup" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x12), None, flags!(vex_op, auto_vex_l, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x12), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vmovdqa" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x6F), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD))], _2(0x01, 0x7F), None, flags!(vex_op, with_vex_l, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x7F), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vmovdqu" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x6F), None, flags!(vex_op, auto_vex_l, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD))], _2(0x01, 0x7F), None, flags!(vex_op, with_vex_l, enc_mr, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x7F), None, flags!(vex_op, enc_mr, pref_F3), Some(AVX)),
			],

			"vmovhlps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x12), None, flags!(vex_op), Some(AVX)),
			],

			"vmovhpd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x17), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x16), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vmovhps" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x17), None, flags!(vex_op, enc_mr), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x16), None, flags!(vex_op), Some(AVX)),
			],

			"vmovlhps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x16), None, flags!(vex_op), Some(AVX)),
			],

			"vmovlpd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x13), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x12), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vmovlps" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x13), None, flags!(vex_op, enc_mr), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x12), None, flags!(vex_op), Some(AVX)),
			],

			"vmovmskpd" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x01, 0x50), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vmovmskps" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x01, 0x50), None, flags!(vex_op), Some(AVX)),
			],

			"vmovntdq" => signatures!
			[
				signature(parameters![parameter(memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x01, 0xE7), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vmovntdqa" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(memory, AllPossible)], _2(0x02, 0x2A), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vmovntpd" => signatures!
			[
				signature(parameters![parameter(memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x01, 0x2B), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vmovntps" => signatures!
			[
				signature(parameters![parameter(memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x01, 0x2B), None, flags!(vex_op, auto_vex_l, enc_mr), Some(AVX)),
			],

			"vmovntqq" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD))], _2(0x01, 0xE7), None, flags!(with_vex_l, vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vmovq" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0xD6), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x7E), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(QWORD))], _2(0x01, 0x6E), None, flags!(with_rex_w, vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x7E), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0xD6), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x7E), None, flags!(with_rex_w, vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vmovqqa" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _2(0x01, 0x6F), None, flags!(with_vex_l, vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD))], _2(0x01, 0x7F), None, flags!(with_vex_l, vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vmovqqu" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _2(0x01, 0x6F), None, flags!(with_vex_l, vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD))], _2(0x01, 0x7F), None, flags!(with_vex_l, vex_op, enc_mr, pref_F3), Some(AVX)),
			],

			"vmovsd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x11), None, flags!(vex_op, enc_mr, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x10), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x10), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x11), None, flags!(vex_op, enc_vm, pref_F2), Some(AVX)),
			],

			"vmovshdup" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x16), None, flags!(vex_op, auto_vex_l, pref_F3), Some(AVX)),
			],

			"vmovsldup" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x12), None, flags!(vex_op, auto_vex_l, pref_F3), Some(AVX)),
			],

			"vmovss" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x11), None, flags!(vex_op, enc_mr, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x10), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x10), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x11), None, flags!(vex_op, enc_vm, pref_F3), Some(AVX)),
			],

			"vmovupd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x10), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD))], _2(0x01, 0x11), None, flags!(vex_op, with_vex_l, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x11), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vmovups" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x10), None, flags!(vex_op, auto_vex_l), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD))], _2(0x01, 0x11), None, flags!(vex_op, with_vex_l, enc_mr), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x11), None, flags!(vex_op, enc_mr), Some(AVX)),
			],

			"vmpsadbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x42), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vmptrld" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xC7), Some(RSI), flags!(), Some(VMX)),
			],

			"vmptrst" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xC7), Some(RDI), flags!(), Some(VMX)),
			],

			"vmread" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(QWORD)), parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0x78), None, flags!(enc_mr), Some(VMX)),
			],

			"vmresume" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xC3), None, flags!(), Some(VMX)),
			],

			"vmrun" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xD8), None, flags!(), Some(VMX_AMD)),
			],

			"vmsave" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xDB), None, flags!(), Some(VMX_AMD)),
			],

			"vmulpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x59), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vmulps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x59), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vmulsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x59), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x59), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vmulss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x59), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x59), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vmwrite" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(legacy_register_and_memory, Fixed(QWORD))], _2(0x0F, 0x79), None, flags!(), Some(VMX)),
			],

			"vmxoff" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xC4), None, flags!(), Some(VMX)),
			],

			"vmxon" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xC7), Some(RSI), flags!(pref_F3), Some(VMX)),
			],

			"vorpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x56), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vorps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x56), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vpabsb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x1C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpabsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x1E), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpabsw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x1D), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpackssdw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x6B), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpacksswb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x63), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpackusdw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x2B), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpackuswb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x67), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpaddb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xFC), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpaddd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xFE), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpaddq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xD4), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpaddsb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xEC), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpaddsw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xED), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpaddusb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xDC), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpaddusw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xDD), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpaddw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xFD), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpalignr" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x0F), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vpand" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xDB), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpandn" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xDF), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpavgb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xE0), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpavgw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xE3), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpblendd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x02), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX2)),
			],

			"vpblendvb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x03, 0x4C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpblendw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x0E), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vpbroadcastb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(memory, Fixed(BYTE))], _2(0x02, 0x78), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x78), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
			],

			"vpbroadcastd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(memory, Fixed(DWORD))], _2(0x02, 0x58), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x58), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
			],

			"vpbroadcastq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x59), None, flags!(vex_op, with_vex_l, pref_66), Some(AVX2)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x59), None, flags!(vex_op, pref_66), Some(AVX2)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x59), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
			],

			"vpbroadcastw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(memory, Fixed(WORD))], _2(0x02, 0x79), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x79), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
			],

			"vpclmulhqhqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x03, 0x44, 0x11), None, flags!(vex_op, pref_66, imm_op), Some(AVX)),
			],

			"vpclmulhqlqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x03, 0x44, 0x01), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vpclmullqhqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x03, 0x44, 0x10), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vpclmullqlqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _3(0x03, 0x44, 0x00), None, flags!(vex_op, imm_op, pref_66), Some(AVX)),
			],

			"vpclmulqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x44), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vpcmov" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x08, 0xA2), None, flags!(xop_op, auto_vex_l), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x08, 0xA2), None, flags!(xop_op, auto_vex_l), Some(SSE5)),
			],

			"vpcmpeqb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x74), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpcmpeqd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x76), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpcmpeqq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x29), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpcmpeqw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x75), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpcmpestri" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x61), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vpcmpestrm" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x60), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vpcmpgtb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x64), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpcmpgtd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x66), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpcmpgtq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x37), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpcmpgtw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x65), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpcmpistri" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x63), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vpcmpistrm" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x62), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vpcomb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xCC), None, flags!(xop_op), Some(SSE5)),
			],

			"vpcomd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xCE), None, flags!(xop_op), Some(SSE5)),
			],

			"vpcomq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xCF), None, flags!(xop_op), Some(SSE5)),
			],

			"vpcomub" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xEC), None, flags!(xop_op), Some(SSE5)),
			],

			"vpcomud" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xEE), None, flags!(xop_op), Some(SSE5)),
			],

			"vpcomuq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xEF), None, flags!(xop_op), Some(SSE5)),
			],

			"vpcomuw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xED), None, flags!(xop_op), Some(SSE5)),
			],

			"vpcomw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xCD), None, flags!(xop_op), Some(SSE5)),
			],

			"vperm2f128" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x06), None, flags!(with_vex_l, vex_op, pref_66), Some(AVX)),
			],

			"vperm2i128" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x46), None, flags!(with_vex_l, vex_op, pref_66), Some(AVX2)),
			],

			"vpermd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _2(0x02, 0x36), None, flags!(with_vex_l, vex_op, pref_66), Some(AVX2)),
			],

			"vpermilpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x0D), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x05), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpermilps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x0C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x04), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpermpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x01), None, flags!(with_vex_l, with_rex_w, vex_op, pref_66), Some(AVX2)),
			],

			"vpermps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD))], _2(0x02, 0x16), None, flags!(with_vex_l, vex_op, pref_66), Some(AVX2)),
			],

			"vpermq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(HWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(HWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x00), None, flags!(with_vex_l, with_rex_w, vex_op, pref_66), Some(AVX2)),
			],

			"vpextrb" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x14), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x14), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x14), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vpextrd" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x16), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(legacy_register_and_memory, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x16), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vpextrq" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x16), None, flags!(with_rex_w, vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vpextrw" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(WORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x15), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC5), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(legacy_register, Fixed(DWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x15), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC5), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(legacy_register, Fixed(QWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x15), None, flags!(vex_op, enc_mr, pref_66), Some(AVX)),
			],

			"vpgatherdd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(vector_scaled_index_byte_32, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x90), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX2)),
			],

			"vpgatherdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(vector_scaled_index_byte_64, Fixed(OWORD)), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x90), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX2)),
			],

			"vpgatherqd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(vector_scaled_index_byte_32, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x91), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX2)),
			],

			"vpgatherqq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(vector_scaled_index_byte_64, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x91), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX2)),
			],

			"vphaddbd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xC2), None, flags!(xop_op), Some(SSE5)),
			],

			"vphaddbq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xC3), None, flags!(xop_op), Some(SSE5)),
			],

			"vphaddbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xC1), None, flags!(xop_op), Some(SSE5)),
			],

			"vphaddd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x02), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vphadddq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xCB), None, flags!(xop_op), Some(SSE5)),
			],

			"vphaddsw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x03), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vphaddubd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xD2), None, flags!(xop_op), Some(SSE5)),
			],

			"vphaddubq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xD3), None, flags!(xop_op), Some(SSE5)),
			],

			"vphaddubw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xD1), None, flags!(xop_op), Some(SSE5)),
			],

			"vphaddudq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xDB), None, flags!(xop_op), Some(SSE5)),
			],

			"vphadduwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xD6), None, flags!(xop_op), Some(SSE5)),
			],

			"vphadduwq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xD7), None, flags!(xop_op), Some(SSE5)),
			],

			"vphaddw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x01), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vphaddwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xC6), None, flags!(xop_op), Some(SSE5)),
			],

			"vphaddwq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xC7), None, flags!(xop_op), Some(SSE5)),
			],

			"vphminposuw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0x41), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vphsubbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xE1), None, flags!(xop_op), Some(SSE5)),
			],

			"vphsubd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x06), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vphsubdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xE3), None, flags!(xop_op), Some(SSE5)),
			],

			"vphsubsw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x07), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vphsubw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x05), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vphsubwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0xE2), None, flags!(xop_op), Some(SSE5)),
			],

			"vpinsrb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x20), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x20), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vpinsrd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x22), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vpinsrq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x22), None, flags!(vex_op, with_rex_w, pref_66), Some(AVX)),
			],

			"vpinsrw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC4), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(legacy_register_and_memory, Fixed(WORD)), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC4), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vpmacsdd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x9E), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmacsdqh" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x9F), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmacsdql" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x97), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmacssdd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x8E), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmacssdqh" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x8F), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmacssdql" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x87), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmacsswd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x86), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmacssww" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x85), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmacswd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x96), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmacsww" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0x95), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmadcsswd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0xA6), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmadcswd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0xB6), None, flags!(xop_op), Some(SSE5)),
			],

			"vpmaddubsw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x04), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmaddwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xF5), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmaskmovd" => signatures!
			[
				signature(parameters![parameter(memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x8E), None, flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX2)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(memory, AllPossible)], _2(0x02, 0x8C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
			],

			"vpmaskmovq" => signatures!
			[
				signature(parameters![parameter(memory, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x02, 0x8E), None, flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX2)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(memory, AllPossible)], _2(0x02, 0x8C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
			],

			"vpmaxsb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x3C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmaxsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x3D), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmaxsw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xEE), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmaxub" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xDE), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmaxud" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x3F), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmaxuw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x3E), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpminsb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x38), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpminsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x39), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpminsw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xEA), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpminub" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xDA), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpminud" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x3B), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpminuw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x3A), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovmskb" => signatures!
			[
				signature(parameters![parameter(legacy_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible)], _2(0x01, 0xD7), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vpmovsxbd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x21), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x21), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovsxbq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(memory, Fixed(WORD))], _2(0x02, 0x22), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x22), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovsxbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x20), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0x20), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovsxdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x25), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0x25), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovsxwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x23), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0x23), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovsxwq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x24), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x24), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovzxbd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x31), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x31), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovzxbq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(memory, Fixed(WORD))], _2(0x02, 0x32), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x32), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovzxbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x30), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0x30), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovzxdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x35), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0x35), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovzxwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x02, 0x33), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x02, 0x33), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmovzxwq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x02, 0x34), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x02, 0x34), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmuldq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x28), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmulhrsw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x0B), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmulhuw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xE4), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmulhw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xE5), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmulld" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x40), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmullw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xD5), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpmuludq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xF4), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpor" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xEB), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpperm" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x08, 0xA3), None, flags!(xop_op), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x08, 0xA3), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vprotb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xC0), None, flags!(xop_op), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x90), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x90), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vprotd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xC2), None, flags!(xop_op), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x92), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x92), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vprotq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xC3), None, flags!(xop_op), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x93), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x93), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vprotw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x08, 0xC1), None, flags!(xop_op), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x91), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x91), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vpsadbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xF6), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpshab" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x98), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x98), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vpshad" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x9A), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x9A), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vpshaq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x9B), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x9B), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vpshaw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x99), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x99), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vpshlb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x94), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x94), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vpshld" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x96), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x96), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vpshlq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x97), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x97), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vpshlw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x09, 0x95), None, flags!(xop_op, enc_mr), Some(SSE5)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x09, 0x95), None, flags!(with_rex_w, xop_op), Some(SSE5)),
			],

			"vpshufb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x00), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpshufd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x70), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpshufhw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x70), None, flags!(vex_op, auto_vex_l, pref_F3), Some(AVX)),
			],

			"vpshuflw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x70), None, flags!(vex_op, auto_vex_l, pref_F2), Some(AVX)),
			],

			"vpsignb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x08), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsignd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x0A), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsignw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x09), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpslld" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x72), Some(RSI), flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0xF2), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpslldq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x73), Some(RDI), flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
			],

			"vpsllq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x73), Some(RSI), flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0xF3), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsllvd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x47), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
			],

			"vpsllvq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x47), None, flags!(vex_op, auto_vex_l, with_rex_w, pref_66), Some(AVX2)),
			],

			"vpsllw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x71), Some(RSI), flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0xF1), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsrad" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x72), Some(RSP), flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0xE2), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsravd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x46), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
			],

			"vpsraw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x71), Some(RSP), flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0xE1), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsrld" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x72), Some(RDX), flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0xD2), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsrldq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x73), Some(RBX), flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
			],

			"vpsrlq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x73), Some(RDX), flags!(vex_op, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0xD3), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsrlvd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x45), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX2)),
			],

			"vpsrlvq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x45), None, flags!(vex_op, auto_vex_l, with_rex_w, pref_66), Some(AVX2)),
			],

			"vpsrlw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0x71), Some(RDX), flags!(vex_op, auto_vex_l, enc_vm, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x01, 0xD1), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsubb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xF8), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsubd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xFA), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsubq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xFB), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsubsb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xE8), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsubsw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xE9), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsubusb" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xD8), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsubusw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xD9), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpsubw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xF9), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vptest" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x17), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpunpckhbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x68), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpunpckhdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x6A), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpunpckhqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x6D), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpunpckhwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x69), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpunpcklbw" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x60), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpunpckldq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x62), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpunpcklqdq" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x6C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpunpcklwd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x61), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vpxor" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0xEF), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vrcpps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x53), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vrcpss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x53), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x53), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vroundpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x09), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vroundps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x08), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vroundsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x0B), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x0B), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vroundss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x0A), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(immediate, Fixed(BYTE))], _2(0x03, 0x0A), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vrsqrtps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x52), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vrsqrtss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x52), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x52), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vshufpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC6), None, flags!(vex_op, auto_vex_l, enc_mr, pref_66), Some(AVX)),
			],

			"vshufps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible), parameter(immediate, Fixed(BYTE))], _2(0x01, 0xC6), None, flags!(vex_op, auto_vex_l, enc_mr), Some(AVX)),
			],

			"vsqrtpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x51), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vsqrtps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x51), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vsqrtsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x51), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x51), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vsqrtss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x51), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x51), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vstmxcsr" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(DWORD))], _2(0x01, 0xAE), Some(RBX), flags!(vex_op), Some(AVX)),
			],

			"vsubpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5C), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vsubps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x5C), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vsubsd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x5C), None, flags!(vex_op, pref_F2), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5C), None, flags!(vex_op, pref_F2), Some(AVX)),
			],

			"vsubss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x5C), None, flags!(vex_op, pref_F3), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x5C), None, flags!(vex_op, pref_F3), Some(AVX)),
			],

			"vtestpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x0F), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vtestps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x02, 0x0E), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vucomisd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(QWORD))], _2(0x01, 0x2E), None, flags!(vex_op, pref_66), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x2E), None, flags!(vex_op, pref_66), Some(AVX)),
			],

			"vucomiss" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(memory, Fixed(DWORD))], _2(0x01, 0x2E), None, flags!(vex_op), Some(AVX)),
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register, Fixed(OWORD))], _2(0x01, 0x2E), None, flags!(vex_op), Some(AVX)),
			],

			"vunpckhpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x15), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vunpckhps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x15), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vunpcklpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x14), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vunpcklps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x14), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vxorpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x57), None, flags!(vex_op, auto_vex_l, pref_66), Some(AVX)),
			],

			"vxorps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register, AllPossible), parameter(xmm_or_ymm_register_and_memory, AllPossible)], _2(0x01, 0x57), None, flags!(vex_op, auto_vex_l), Some(AVX)),
			],

			"vzeroall" => signatures!
			[
				signature(parameters![], _2(0x01, 0x77), None, flags!(with_vex_l, vex_op), Some(AVX)),
			],

			"vzeroupper" => signatures!
			[
				signature(parameters![], _2(0x01, 0x77), None, flags!(vex_op), Some(AVX)),
			],

			"wbinvd" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x09), None, flags!(), None),
			],

			"wrfsbase" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD))], _2(0x0F, 0xAE), Some(RDX), flags!(pref_F3), None),
				signature(parameters![parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0xAE), Some(RDX), flags!(with_rex_w, pref_F3), None),
			],

			"wrgsbase" => signatures!
			[
				signature(parameters![parameter(legacy_register, Fixed(DWORD))], _2(0x0F, 0xAE), Some(RBX), flags!(pref_F3), None),
				signature(parameters![parameter(legacy_register, Fixed(QWORD))], _2(0x0F, 0xAE), Some(RBX), flags!(with_rex_w, pref_F3), None),
			],

			"wrmsr" => signatures!
			[
				signature(parameters![], _2(0x0F, 0x30), None, flags!(), None),
			],

			"wrpkru" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xEF), None, flags!(), None),
			],

			"wrshr" => signatures!
			[
				signature(parameters![parameter(legacy_register_and_memory, Fixed(DWORD))], _2(0x0F, 0x37), Some(RAX), flags!(), Some(CYRIX)),
			],

			"xabort" => signatures!
			[
				signature(parameters![parameter(immediate, Fixed(BYTE))], _2(0xC6, 0xF8), None, flags!(), Some(RTM)),
			],

			"xadd" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _2(0x0F, 0xC0), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _2(0x0F, 0xC0), None, flags!(enc_mr), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xC1), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _2(0x0F, 0xC1), None, flags!(auto_size, enc_mr), None),
			],

			"xbegin" => signatures!
			[
				signature(parameters![parameter(instruction_offset, Fixed(DWORD))], _2(0xC7, 0xF8), None, flags!(), Some(RTM)),
			],

			"xchg" => signatures!
			[
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x86), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(memory, Fixed(BYTE))], _1(0x86), None, flags!(lock), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x86), None, flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x86), None, flags!(enc_mr), None),
				signature(parameters![parameter(rax, AllPossible), parameter(legacy_register, AllPossible)], _1(0x90), None, flags!(auto_size, short_arg), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x87), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(rax, AllPossible)], _1(0x90), None, flags!(auto_size, short_arg), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(memory, AllPossible)], _1(0x87), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _1(0x87), None, flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _1(0x87), None, flags!(auto_size, enc_mr), None),
			],

			"xcryptcbc" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xA7, 0xD0), None, flags!(pref_F3), Some(CYRIX)),
			],

			"xcryptcfb" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xA7, 0xE0), None, flags!(pref_F3), Some(CYRIX)),
			],

			"xcryptctr" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xA7, 0xD8), None, flags!(pref_F3), Some(CYRIX)),
			],

			"xcryptecb" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xA7, 0xC8), None, flags!(pref_F3), Some(CYRIX)),
			],

			"xcryptofb" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xA7, 0xE8), None, flags!(pref_F3), Some(CYRIX)),
			],

			"xend" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xD5), None, flags!(), Some(RTM)),
			],

			"xgetbv" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xD0), None, flags!(), None),
			],

			"xlat" => signatures!
			[
				signature(parameters![], _1(0xD7), None, flags!(), None),
			],

			"xlatb" => signatures!
			[
				signature(parameters![], _1(0xD7), None, flags!(), None),
			],

			"xor" => signatures!
			[
				signature(parameters![parameter(rax, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x34), None, flags!(), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RSI), flags!(lock), None),
				signature(parameters![parameter(memory, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x30), None, flags!(lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(immediate, Fixed(BYTE))], _1(0x80), Some(RSI), flags!(), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register, Fixed(BYTE))], _1(0x30), None, flags!(enc_mr), None),
				signature(parameters![parameter(legacy_register, Fixed(BYTE)), parameter(legacy_register_and_memory, Fixed(BYTE))], _1(0x32), None, flags!(), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RSI), flags!(auto_size, exact_size), None),
				signature(parameters![parameter(rax, AllPossible), parameter(immediate, AllPossible)], _1(0x35), None, flags!(auto_size), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RSI), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(immediate, Fixed(BYTE))], _1(0x83), Some(RSI), flags!(auto_size, lock), None),
				signature(parameters![parameter(memory, AllPossible), parameter(legacy_register, AllPossible)], _1(0x31), None, flags!(auto_size, lock, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(immediate, AllPossible)], _1(0x81), Some(RSI), flags!(auto_size), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register, AllPossible)], _1(0x31), None, flags!(auto_size, enc_mr), None),
				signature(parameters![parameter(legacy_register, AllPossible), parameter(legacy_register_and_memory, AllPossible)], _1(0x33), None, flags!(auto_size), None),
			],

			"xorpd" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x57), None, flags!(pref_66), Some(SSE2)),
			],

			"xorps" => signatures!
			[
				signature(parameters![parameter(xmm_or_ymm_register, Fixed(OWORD)), parameter(xmm_or_ymm_register_and_memory, Fixed(OWORD))], _2(0x0F, 0x57), None, flags!(), Some(SSE)),
			],

			"xrstor" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xAE), Some(RBP), flags!(), None),
			],

			"xrstor64" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xAE), Some(RBP), flags!(with_rex_w), None),
			],

			"xrstors64" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xC7), Some(RBX), flags!(with_rex_w), None),
			],

			"xsave" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xAE), Some(RSP), flags!(), None),
			],

			"xsave64" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xAE), Some(RSP), flags!(with_rex_w), None),
			],

			"xsavec64" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xC7), Some(RSP), flags!(with_rex_w), None),
			],

			"xsaveopt64" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xAE), Some(RSI), flags!(with_rex_w), None),
			],

			"xsaves64" => signatures!
			[
				signature(parameters![parameter(memory, NoSize)], _2(0x0F, 0xC7), Some(RBP), flags!(with_rex_w), None),
			],

			"xsetbv" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xD1), None, flags!(), None),
			],

			"xsha1" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xA6, 0xC8), None, flags!(pref_F3), Some(CYRIX)),
			],

			"xsha256" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xA6, 0xD0), None, flags!(pref_F3), Some(CYRIX)),
			],

			"xstore" => signatures!
			[
				signature(parameters![], _3(0x0F, 0xA7, 0xC0), None, flags!(), Some(CYRIX)),
			],

			"xtest" => signatures!
			[
				signature(parameters![], _3(0x0F, 0x01, 0xD6), None, flags!(), Some(RTM)),
			],
		};
		
		MnemonicsToSignatures.get(potential_mnemonic.to_lowercase().as_str())
	}
}
