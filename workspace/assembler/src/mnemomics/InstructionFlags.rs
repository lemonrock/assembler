// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


bitflags!
{
	/// Instruction flags.
	pub(crate) struct InstructionFlags: u32
	{
		/// Instruction has default encoding.
		const DEFAULT = 0x0000_0000;

		/// Instruction requires a `VEX` prefix to be encoded.
		const VEX_OP = 0x0000_0001;

		/// Instruction requires a `XOP` prefix to be encoded.
		const XOP_OP = 0x0000_0002;

		/// Instruction encodes the final opcode byte in the immediate position, eg 3DNow! extensions.
		const IMM_OP = 0x0000_0004;
		
		/// `AUTO_SIZE`, `AUTO_NO32`, `AUTO_REXW` and `AUTO_VEXL` are mutually exclusive; see `OneOfTheAutomaticFlagsIsSet` below.
		///
		/// For 16-bit, implies `OPSIZE` prefix.
		/// For 32-bit, implies no prefix.
		/// For 64-bit, implies `REX.W`, `VEX.W` or `XOP.W` prefix.
		const AUTO_SIZE = 0x0000_0008;

		/// For 16-bit, implies `OPSIZE` prefix.
		/// For 32-bit, implies no prefix.
		/// For 64-bit, implies no prefix.
		const AUTO_NO32 = 0x0000_0010;
		
		/// Illegal for 16-bit.
		/// For 64-bit implies no prefix.
		/// For 64-bit, implies `REX.W`, `VEX.W` or `XOP.W` prefix.
		const AUTO_REXW = 0x0000_0020;

		/// For 128-bit, implies no prefix.
		/// For 256-bit, implies `VEX.L` prefix.
		const AUTO_VEXL = 0x0000_0040;

		/// Implies `OPSIZE` prefix.
		const WORD_SIZE = 0x0000_0080;

		/// Implies `REX.W`, `VEX.W` or `XOP.W` prefix.
		const WITH_REXW = 0x0000_0100;

		/// Implies `VEX.L` or `XOP.L` prefixes.
		const WITH_VEXL = 0x0000_0200;

		/// Operands with unknown sizes cannot be assumed to match.
		const EXACT_SIZE = 0x0000_0400;

		/// Mandatory prefix (same as `WORD_SIZE`).
		const PREF_66 = Self::WORD_SIZE.bits;

		/// Mandatory prefix (same as `SMALL_ADDRESS`).
		const PREF_67 = 0x0000_0800;

		/// Mandatory prefix (same as `LOCK`).
		const PREF_F0 = 0x0000_1000;

		/// Mandatory prefix `REPNE`.
		const PREF_F2 = 0x0000_2000;

		/// Mandatory prefix `REP`.
		const PREF_F3 = 0x0000_4000;
		
		/// User `LOCK` prefix is valid with instruction.
		const LOCK = 0x0000_8000;

		/// User `REP` prefix is valid with instruction.
		const REP = 0x0001_0000;
		
		/// User `REP` prefix is valid with instruction.
		const REPE = 0x0002_0000;
		
		/// A register argument is encoded in the last byte of the opcode.
		const SHORT_ARG = 0x0004_0000;

		/// Select alternate argument encoding `MR`.
		const ENC_MR = 0x0008_0000;

		/// Select alternate argument encoding `VM`.
		const ENC_VM = 0x0010_0000;

		/// A special encoding using the SIB to specify an immediate and two registers.
		const ENC_MIB = 0x0020_0000;

		/// Instruction *not* available in Long mode.
		const X86_ONLY = 0x0040_0000;
		
		/// `AUTO_SIZE`, `AUTO_NO32`, `AUTO_REXW` or `AUTO_VEXL`.
		const OneOfTheAutomaticFlagsIsSet = Self::AUTO_SIZE.bits | Self::AUTO_NO32.bits | Self::AUTO_REXW.bits | Self::AUTO_VEXL.bits;
		
		/// `VEX_OP` or `XOP_OP`.
		const VexOperationOrXopOperation = Self::VEX_OP.bits | Self::XOP_OP.bits;
	}
}

impl Default for InstructionFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		InstructionFlags::DEFAULT
	}
}
