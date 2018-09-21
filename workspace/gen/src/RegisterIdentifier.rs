// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A register identifier.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RegisterIdentifier
{
	/// General purpose register 0, although certain legacy instructions prefer it.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for:-
	///
	/// * Returning an integer value from a function call;
	/// * Passing the number of final variable arguments ('varargs', the `...` parameter) in a function call;
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rax`: 8 bytes wide (`QWORD`);
	/// * `eax`: 4 bytes wide (`DWORD`);
	/// * `ax`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `al`: 1 byte wide (`WORD`);
	///
	/// Bits 8 - 15 can be accessed using the pseudo-register `AH` (`A` 'high').
	///
	/// Was often used as an `accumulator` on the 8086, hence the `A` in the name.
	RAX = 0x00,
	
	/// General purpose register 1, although certain legacy instructions prefer it.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the fourth (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rcx`: 8 bytes wide (`QWORD`);
	/// * `ecx`: 4 bytes wide (`DWORD`);
	/// * `cx`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `cl`: 1 byte wide (`WORD`);
	///
	/// Bits 8 - 15 can be accessed using the pseudo-register `CH` (`C` 'high').
	///
	/// Was often used as a `counter` for loops on the 8086, hence the `C` in the name.
	RCX = 0x01,
	
	/// General purpose register 2, although certain legacy instructions prefer it.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the third (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rdx`: 8 bytes wide (`QWORD`);
	/// * `edx`: 4 bytes wide (`DWORD`);
	/// * `dx`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `dl`: 1 byte wide (`WORD`);
	///
	/// Bits 8 - 15 can be accessed using the pseudo-register `DH` (`D` 'high').
	///
	/// Introduced with the 8086.
	RDX = 0x02,
	
	/// General purpose register 3, although certain legacy instructions prefer it.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rbx`: 8 bytes wide (`QWORD`);
	/// * `ebx`: 4 bytes wide (`DWORD`);
	/// * `bx`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `bl`: 1 byte wide (`WORD`);
	///
	/// Bits 8 - 15 can be accessed using the pseudo-register `BH` (`B` 'high').
	///
	/// Introduced with the 8086.
	RBX = 0x03,
	
	/// General purpose register 4.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rsp`: 8 bytes wide (`QWORD`);
	/// * `esp`: 4 bytes wide (`DWORD`);
	/// * `sp`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `spl`: 1 byte wide (`WORD`) (final `l` is for 'lower');
	///
	/// Usually used as the Stack Pointer, hence the `SP` in the name.
	RSP = 0x04,
	
	/// General purpose register 5.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rbp`: 8 bytes wide (`QWORD`);
	/// * `ebp`: 4 bytes wide (`DWORD`);
	/// * `sb`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `bpl`: 1 byte wide (`WORD`) (final `l` is for 'lower');
	///
	/// Usually used as the Base Pointer, hence the `BP` in the name.
	RBP = 0x05,
	
	/// General purpose register 6.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the second (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rsi`: 8 bytes wide (`QWORD`);
	/// * `esi`: 4 bytes wide (`DWORD`);
	/// * `si`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `sil`: 1 byte wide (`WORD`) (final `l` is for 'lower');
	RSI = 0x06,
	
	/// General purpose register 7.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the first (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rdi`: 8 bytes wide (`QWORD`);
	/// * `edi`: 4 bytes wide (`DWORD`);
	/// * `di`: 2 bytes wide (`WORD`) (this encoding is inefficient);
	/// * `dil`: 1 byte wide (`WORD`) (final `l` is for 'lower');
	RDI = 0x07,
	
	/// General purpose register 8.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the fifth (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r8`: 8 bytes wide (`QWORD`);
	/// * `r8d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r8w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r8b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R8 = 0x08,
	
	/// General purpose register 9.
	///
	/// Used in the 64-bit AMD System V Application Binary Interface (ABI) for the sixth (one-based) function call argument.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r9`: 8 bytes wide (`QWORD`);
	/// * `r9d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r9w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r9b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R9 = 0x09,
	
	/// General purpose register 10.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r10`: 8 bytes wide (`QWORD`);
	/// * `r10d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r10w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r10b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R10 = 0x0A,
	
	/// General purpose register 11.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r11`: 8 bytes wide (`QWORD`);
	/// * `r11d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r11w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r11b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R11 = 0x0B,
	
	/// General purpose register 12.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r12`: 8 bytes wide (`QWORD`);
	/// * `r12d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r12w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r12b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R12 = 0x0C,
	
	/// General purpose register 13.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r13`: 8 bytes wide (`QWORD`);
	/// * `r13d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r13w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r13b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R13 = 0x0D,
	
	/// General purpose register 14.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r14`: 8 bytes wide (`QWORD`);
	/// * `r14d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r14w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r14b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R14 = 0x0E,
	
	/// General purpose register 15.
	///
	/// 1, 2, 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `r15`: 8 bytes wide (`QWORD`);
	/// * `r15d`: 4 bytes wide (`DWORD`) (final `d` is for `DWORD`);
	/// * `r15w`: 2 bytes wide (`WORD`) (final `w` is for `WORD`);
	/// * `r15b`: 1 byte wide (`WORD`) (final `b` is for `BYTE`);
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	R15 = 0x0F,
	
	/// Instruction Pointer register.
	///
	/// 4 or 8 bytes wide.
	///
	/// Encoded in assembler as:-
	///
	/// * `rip`: 8 bytes wide (`QWORD`);
	/// * `eip`: 4 bytes wide (`DWORD`);
	RIP = 0x15,
	
	/// Pseudo-register for bits 8-15 of general purpose register 0 (RAX).
	///
	/// One byte wide.
	///
	/// Encoded in assembler as `ah`.
	AH = 0x24,
	
	/// Pseudo-register for bits 8-15 of general purpose register 1 (RCX).
	///
	/// One byte wide.
	///
	/// Encoded in assembler as `ch`.
	CH = 0x25,
	
	/// Pseudo-register for bits 8-15 of general purpose register 2 (RDX).
	///
	/// One byte wide.
	///
	/// Encoded in assembler as `dh`.
	DH = 0x26,
	
	/// Pseudo-register for bits 8-15 of general purpose register 3 (RBX).
	///
	/// One byte wide.
	///
	/// Encoded in assembler as `bh`.
	BH = 0x27,
	
	/// x87 floating point register 0.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st0`.
	ST0 = 0x30,
	
	/// x87 floating point register 1.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st1`.
	ST1 = 0x31,
	
	/// x87 floating point register 2.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st2`.
	ST2 = 0x32,
	
	/// x87 floating point register 3.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st3`.
	ST3 = 0x33,
	
	/// x87 floating point register 4.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st4`.
	ST4 = 0x34,
	
	/// x87 floating point register 5.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st5`.
	ST5 = 0x35,
	
	/// x87 floating point register 6.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st6`.
	ST6 = 0x36,
	
	/// x87 floating point register 7.
	///
	/// 10 bytes (80-bits) wide.
	///
	/// Encoded in assembler as `st7`.
	ST7 = 0x37,
	
	/// Multimedia Extension Register 0; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx0`.
	MMX0 = 0x40,
	
	/// Multimedia Extension Register 1; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx1`.
	MMX1 = 0x41,
	
	/// Multimedia Extension Register 2; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx2`.
	MMX2 = 0x42,
	
	/// Multimedia Extension Register 3; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx3`.
	MMX3 = 0x43,
	
	/// Multimedia Extension Register 4; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx4`.
	MMX4 = 0x44,
	
	/// Multimedia Extension Register 5; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx5`.
	MMX5 = 0x45,
	
	/// Multimedia Extension Register 6; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx6`.
	MMX6 = 0x46,
	
	/// Multimedia Extension Register 7; alternative encoding exists.
	///
	/// 8 bytes wide.
	///
	/// Encoded in assembler as `mmx7`.
	MMX7 = 0x47,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 0; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM0  = 0x50,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 1; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM1  = 0x51,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 2; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM2  = 0x52,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 3; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM3  = 0x53,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 4; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM4  = 0x54,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 5; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM5  = 0x55,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 6; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM6  = 0x56,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 7; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM7  = 0x57,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 8; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM8  = 0x58,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 9; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM9  = 0x59,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 10; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM10 = 0x5A,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 11; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM11 = 0x5B,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 12; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM12 = 0x5C,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 13; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM13 = 0x5D,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 14; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM14 = 0x5E,
	
	/// XMM, YMM or ZMM Multimedia Extension Register 15; alternative encoding exists.
	///
	/// 16 or 32 bytes wide.
	XMM15 = 0x5F,
	
	/// Segment register 0; alternative encoding exits.
	///
	/// 2 bytes wide.
	ES = 0x60,
	
	/// Segment register 1; alternative encoding exits.
	///
	/// 2 bytes wide.
	CS = 0x61,
	
	/// Segment register 2; alternative encoding exits.
	///
	/// 2 bytes wide.
	SS = 0x62,
	
	/// Segment register 3; alternative encoding exits.
	///
	/// 2 bytes wide.
	DS = 0x63,
	
	/// Segment register 4; alternative encoding exits.
	///
	/// 2 bytes wide.
	FS = 0x64,
	
	/// Segment register 5; alternative encoding exits.
	///
	/// 2 bytes wide.
	GS = 0x65,
	
	/// Control register 0.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Has various control flags (bits) that modify the basic operation of the processor.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/Control_register).
	CR0  = 0x70,
	
	/// Control register 1.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	CR1  = 0x71,
	
	/// Control register 2.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Contains the Page Fault Linear Address (PFLA); when a page fault occurs, this is the memory address that a program attempted to access.
	CR2  = 0x72,
	
	/// Control register 3.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Interpretation of value depends on setting in control register 0 and control register 4.
	CR3  = 0x73,
	
	
	/// Control register 4.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Has various control flags (bits) that modify the operation of the processor.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/Control_register).
	CR4  = 0x74,
	
	/// Control register 5.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	CR5  = 0x75,
	
	/// Control register 6.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	CR6  = 0x76,
	
	/// Control register 7.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	CR7  = 0x77,
	
	/// Control register 8.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used to prioritize external interrupts and is referred to as the task-priority register (TPR).
	/// See [Wikipedia](https://en.wikipedia.org/wiki/Control_register).
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR8  = 0x78,
	
	/// Control register 9.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR9  = 0x79,
	
	/// Control register 10.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR10 = 0x7A,
	
	/// Control register 11.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR11 = 0x7B,
	
	/// Control register 12.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR12 = 0x7C,
	
	/// Control register 13.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR13 = 0x7D,
	
	/// Control register 14.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR14 = 0x7E,
	
	/// Control register 15.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form always requires a prefix, so its usage creates less compact machine code.
	CR15 = 0x7F,
	
	/// Debug register 0.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for a debug breakpoint address in conjunction with debug register 7.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR0  = 0x80,
	
	/// Debug register 1.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for a debug breakpoint address in conjunction with debug register 7.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR1  = 0x81,
	
	/// Debug register 2.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for a debug breakpoint address in conjunction with debug register 7.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR2  = 0x82,
	
	/// Debug register 3.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for a debug breakpoint address in conjunction with debug register 7.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR3  = 0x83,
	
	/// Debug register 4.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	DR4  = 0x84,
	
	/// Debug register 5.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	DR5  = 0x85,
	
	/// Debug register 6.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for debug status.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR6  = 0x86,
	
	/// Debug register 7.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Used for debug control.
	/// See [Wikipedia](https://en.wikipedia.org/wiki/X86_debug_register).
	DR7  = 0x87,
	
	/// Debug register 8.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR8  = 0x88,
	
	/// Debug register 9.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR9  = 0x89,
	
	/// Debug register 10.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR10 = 0x8A,
	
	/// Debug register 11.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR11 = 0x8B,
	
	/// Debug register 12.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR12 = 0x8C,
	
	/// Debug register 13.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR13 = 0x8D,
	
	/// Debug register 14.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR14 = 0x8E,
	
	/// Debug register 15.
	///
	/// 4 bytes wide (although notionally 8 bytes wide in long mode, it has not effect).
	///
	/// Reserved.
	///
	/// Encoded form ?may? require a prefix, so its usage creates less compact machine code.
	DR15 = 0x8F,
	
	/// Bound register 0.
	///
	/// 16 bytes wide.
	///
	/// Encoded in assembler as `bnd0`.
	BND0 = 0x90,
	
	/// Bound register 1.
	///
	/// 16 bytes wide.
	///
	/// Encoded in assembler as `bnd1`.
	BND1 = 0x91,
	
	/// Bound register 2.
	///
	/// 16 bytes wide.
	///
	/// Encoded in assembler as `bnd2`.
	BND2 = 0x92,
	
	/// Bound register 3.
	///
	/// 16 bytes wide.
	///
	/// Encoded in assembler as `bnd3`.
	BND3 = 0x93
}

impl RegisterIdentifier
{
	#[inline(always)]
	pub(crate) fn code_and_8_then_invert(&self) -> u8
	{
		!self.code_and_8()
	}
	
	#[inline(always)]
	pub(crate) fn code_and_8(&self) -> u8
	{
		self.code() & 8
	}
	
	#[inline(always)]
	pub(crate) fn code_and_7(&self) -> u8
	{
		self.code() & 7
	}
	
	#[inline(always)]
	pub(crate) fn code(self) -> u8
	{
		(self as u8) & 0x0F
	}
	
	#[inline(always)]
	pub(crate) fn family(self) -> RegisterFamily
	{
		unsafe { transmute((self as u8) >> 4) }
	}
}
