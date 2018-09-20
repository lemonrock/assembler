// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


bitflags!
{
	/// Instruction flags.
    #[derive(Default)]
	pub(crate) struct InstructionFlag2: u32
	{
		/// Instruction requires a `VEX` prefix to be encoded.
		const vex_op = 0x0000_0001;

		/// Instruction requires a `XOP` prefix to be encoded.
		const xop_op = 0x0000_0002;

		/// Instruction encodes the final opcode byte in the immediate position, eg 3DNow! extensions.
		const imm_op = 0x0000_0004;
		
		/// `auto_size`, `auto_no_32`, `auto_rex_w` and `auto_vex_l` are mutually exclusive; see `OneOfTheAutomaticFlagsIsSet` below.
		///
		/// For 16-bit, implies `OPSIZE` prefix.
		/// For 32-bit, implies no prefix.
		/// For 64-bit, implies `REX.W`, `VEX.W` or `XOP.W` prefix.
		const auto_size = 0x0000_0008;

		/// For 16-bit, implies `OPSIZE` prefix.
		/// For 32-bit, implies no prefix.
		/// For 64-bit, implies no prefix.
		const auto_no_32 = 0x0000_0010;
		
		/// Illegal for 16-bit.
		/// For 64-bit implies no prefix.
		/// For 64-bit, implies `REX.W`, `VEX.W` or `XOP.W` prefix.
		const auto_rex_w = 0x0000_0020;

		/// For 128-bit, implies no prefix.
		/// For 256-bit, implies `VEX.L` prefix.
		const auto_vex_l = 0x0000_0040;

		/// Implies `OPSIZE` prefix.
		const word_size = 0x0000_0080;

		/// Implies `REX.W`, `VEX.W` or `XOP.W` prefix.
		const with_rex_w = 0x0000_0100;

		/// Implies `VEX.L` or `XOP.L` prefixes.
		const with_vex_l = 0x0000_0200;

		/// Operands with unknown sizes cannot be assumed to match.
		const exact_size = 0x0000_0400;

		/// Mandatory prefix (same as `word_size`).
		const pref_66 = Self::word_size.bits;

		/// Mandatory prefix ?(same as `SMALL_ADDRESS`)?
		const pref_67 = 0x0000_0800;

		/// Mandatory prefix (same as `lock`).
		const pref_F0 = 0x0000_1000;

		/// Mandatory prefix `REPNE`.
		const pref_F2 = 0x0000_2000;

		/// Mandatory prefix `REP`.
		const pref_F3 = 0x0000_4000;
		
		/// User `LOCK` prefix is valid with instruction.
		const lock = 0x0000_8000;

		/// User `REP` prefix is valid with instruction.
		const rep = 0x0001_0000;
		
		/// User `REP` prefix is valid with instruction.
		const repe = 0x0002_0000;
		
		/// A register argument is encoded in the last byte of the opcode.
		const short_arg = 0x0004_0000;

		/// Select alternate argument encoding `MR`.
		const enc_mr = 0x0008_0000;

		/// Select alternate argument encoding `VM`.
		const enc_vm = 0x0010_0000;

		/// A special encoding using the Scaled Index Byte (SIB) to specify an immediate and two registers.
		const enc_mib = 0x0020_0000;

		/// Instruction *not* available in Long mode.
		const x86_only = 0x0040_0000;
		
		/// `auto_size`, `auto_no_32`, `auto_rex_w` or `auto_vex_l`.
		const OneOfTheAutomaticFlagsIsSet = Self::auto_size.bits | Self::auto_no_32.bits | Self::auto_rex_w.bits | Self::auto_vex_l.bits;
		
		/// `vex_op` or `xop_op`.
		const VexOperationOrXopOperation = Self::vex_op.bits | Self::xop_op.bits;
	}
}
