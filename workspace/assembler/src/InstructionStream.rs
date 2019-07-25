// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An instruction stream.
///
/// Has functions for writing x64 instructions, organized by mnemonic and the parameters they need.
///
/// When finished, call `finish()`.
///
/// When writing 8-bit `Jcc` (`JMP` and conditional `JMP` instructions), a `ShortJmpResult` is returned in error if the target effective address could be resolved and its displacement exceeds the size of an `i8`. In this case, the instruction stream is rolled back to point to just before where the instruction started to be emitted. Use this result to try to make a 8-bit `JMP` and then replace it with a 32-bit one if an error occurs.
///
/// Note that unresolved labels (ie those yet to be attached to a location in the instruction stream) will not produce such an error. Instead a panic (in debug builds) or silent error will occur when `finish()` is called.
#[derive(Debug)]
pub struct InstructionStream<'a>
{
	byte_emitter: ByteEmitter,
	executable_anonymous_memory_map: &'a mut ExecutableAnonymousMemoryMap,
	labelled_locations: LabelledLocations,
	instruction_pointers_to_replace_labels_with_8_bit_displacements: Vec<(Label, InstructionPointer)>,
	instruction_pointers_to_replace_labels_with_32_bit_displacements: Vec<(Label, InstructionPointer)>,
	emitted_labels: Vec<(Label, InstructionPointer)>,
}

impl<'a> InstructionStream<'a>
{
	/// `REX.W` prefix.
	const REX_W: u8 = 0x48;
	
	/// `REX.R` prefix.
	pub(crate) const REX_R: u8 = 0x44;
	
	/// `REX.X` prefix.
	pub(crate) const REX_X: u8 = 0x42;
	
	/// `REX.B` prefix.
	pub(crate) const REX_B: u8 = 0x41;
	
	/// `REX` prefix.
	pub(crate) const REX: u8 = 0x40;
	
	#[inline(always)]
	pub(crate) fn new(executable_anonymous_memory_map: &'a mut ExecutableAnonymousMemoryMap, instruction_stream_hints: &InstructionStreamHints) -> Self
	{
		executable_anonymous_memory_map.make_writable();
		
		Self
		{
			byte_emitter: ByteEmitter::new(executable_anonymous_memory_map),
			executable_anonymous_memory_map,
			labelled_locations: LabelledLocations::new(instruction_stream_hints.number_of_labels),
			instruction_pointers_to_replace_labels_with_8_bit_displacements: Vec::with_capacity(instruction_stream_hints.number_of_8_bit_jumps),
			instruction_pointers_to_replace_labels_with_32_bit_displacements: Vec::with_capacity(instruction_stream_hints.number_of_32_bit_jumps),
			emitted_labels: Vec::with_capacity(instruction_stream_hints.number_of_emitted_labels),
		}
	}
	
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	fn attempt_to_resize_in_place(&mut self) -> io::Result<()>
	{
		let new_length = self.executable_anonymous_memory_map.attempt_to_resize_in_place_whilst_writing()?;
		self.byte_emitter.end_instruction_pointer += new_length;
		Ok(())
	}
	
	#[cfg(not(any(target_os = "android", target_os = "linux")))]
	#[inline(always)]
	fn attempt_to_resize_in_place(&mut self) -> io::Result<()>
	{
		Err(io::Error::new(io::ErrorKind::Other, "Could not resize in place"))
	}
	
	/// Returns `(number_of_labels, number_of_8_bit_jumps, number_of_32_bit_jumps)` which can be used as input to tweak the next version.
	#[inline(always)]
	fn hints_for_next_instance(&self) -> InstructionStreamHints
	{
		InstructionStreamHints
		{
			number_of_labels: self.labelled_locations.next_label_index,
			number_of_8_bit_jumps: self.instruction_pointers_to_replace_labels_with_8_bit_displacements.len(),
			number_of_32_bit_jumps: self.instruction_pointers_to_replace_labels_with_32_bit_displacements.len(),
			number_of_emitted_labels: self.emitted_labels.len(),
		}
	}
	
	/// Resolves all remaining labels and makes code executable.
	///
	/// Will panic in debug builds if labels can not be resolved, 8-bit JMPs are too far away or 32-bit JMPs have displacements of more than 2Gb!
	///
	/// Returns a slice containing just the instructions encoded; useful for testing or for dumping to a file; and hints to use for the next instance.
	#[inline(always)]
	pub fn finish(mut self) -> (&'a [u8], InstructionStreamHints)
	{
		let hints = self.hints_for_next_instance();
		
		for (label, insert_at_instruction_pointer) in self.instruction_pointers_to_replace_labels_with_8_bit_displacements.iter()
		{
			let target_instruction_pointer = self.valid_target_instruction_pointer(*label);
			
			let result = self.byte_emitter.insert_8_bit_effective_address_displacement(*insert_at_instruction_pointer, target_instruction_pointer);
			
			debug_assert!(result.is_ok(), "8-bit JMP for {:?} was too far", label)
		}
		
		for (label, insert_at_instruction_pointer) in self.instruction_pointers_to_replace_labels_with_32_bit_displacements.iter()
		{
			let target_instruction_pointer = self.valid_target_instruction_pointer(*label);
			
			let result = self.byte_emitter.insert_32_bit_effective_address_displacement(*insert_at_instruction_pointer, target_instruction_pointer);
			
			debug_assert!(result.is_ok(), "32-bit JMP for {:?} was too far", label)
		}
		
		for (label, insert_at_instruction_pointer) in self.emitted_labels.iter()
		{
			let target_instruction_pointer = self.valid_target_instruction_pointer(*label);
			
			self.byte_emitter.emit_u64_at(target_instruction_pointer as u64, *insert_at_instruction_pointer)
		}
		
		self.executable_anonymous_memory_map.make_executable();
		
		let length = self.instruction_pointer() - self.start_instruction_pointer();
		let slice = unsafe { from_raw_parts(self.start_instruction_pointer() as *const u8, length) };
		(slice, hints)
	}
	
	#[inline(always)]
	fn target_instruction_pointer(&self, label: Label) -> InstructionPointer
	{
		self.labelled_locations.potential_target_instruction_pointer(label)
	}
	
	#[inline(always)]
	fn valid_target_instruction_pointer(&self, label: Label) -> InstructionPointer
	{
		let target_instruction_pointer = self.target_instruction_pointer(label);
		debug_assert!(target_instruction_pointer.is_valid(), "unresolved {:?}'", label);
		target_instruction_pointer
	}
	
	/// Creates an unique label and uses it to label the current location.
	#[inline(always)]
	pub fn create_and_attach_label(&mut self) -> Label
	{
		let label = self.create_label();
		self.attach_label(label);
		label
	}
	
	/// Creates an unique label, scoped to this instance of the Instruction Stream.
	///
	/// The label is created unattached.
	#[inline(always)]
	pub fn create_label(&mut self) -> Label
	{
		self.labelled_locations.create_label()
	}
	
	/// Labels the current location.
	///
	/// It is an error to use the same label to label more than one location (or to label the current location with the same label twice or more).
	///
	/// This only checked for in debug builds where it causes a runtime panic.
	///
	/// Labels should be created using `self.create_label()`; no checks are made for labels created with another instance and attached to this one.
	#[inline(always)]
	pub fn attach_label(&mut self, label: Label)
	{
		let instruction_pointer = self.instruction_pointer();
		self.labelled_locations.set(label, instruction_pointer)
	}
	
	/// Emits the 64-bit value of a label at the current location.
	///
	/// Typically used when build jump tables.
	///
	/// It is an error to use the same label to label more than one location (or to label the current location with the same label twice or more).
	///
	/// This only checked for in debug builds where it causes a runtime panic.
	///
	/// Labels should be created using `self.create_label()`; no checks are made for labels created with another instance and attached to this one.
	#[inline(always)]
	pub fn emit_label(&mut self, label: Label)
	{
		let target_instruction_pointer = self.target_instruction_pointer(label);
		if target_instruction_pointer.is_valid()
		{
			self.emit_quad_word(target_instruction_pointer as u64)
		}
		else
		{
			let instruction_pointer = self.instruction_pointer();
			self.emitted_labels.push((label, instruction_pointer));
			self.skip_quad_word();
		}
	}
	
	/// Emits a non-leaf function prologue suitable for both the System V Application Binary Interface for AMD64 and the Microsoft x64 Calling Convention.
	#[inline(always)]
	pub fn push_stack_frame(&mut self)
	{
		use self::Register64Bit::RBP;
		use self::Register64Bit::RSP;
		
		self.push_Register64Bit_r64(RBP);
		self.mov_Register64Bit_Register64Bit_rm64_r64(RBP, RSP);
	}
	
	/// Emits a non-leaf function epilogue (which returns) suitable for both the System V Application Binary Interface for AMD64 and the Microsoft x64 Calling Convention.
	#[inline(always)]
	pub fn pop_stack_frame_and_return(&mut self)
	{
		use self::Register64Bit::RBP;
		use self::Register64Bit::RSP;
		
		self.mov_Register64Bit_Register64Bit_rm64_r64(RSP, RBP);
		self.pop_Register64Bit_r64(RBP);
		self.ret();
	}
	
	/// Zeroes the `RAX` register using the most efficient code (`XOR RAX, RAX`, although could just as easily be `SUB RAX, RAX`).
	///
	/// Also equivalent to a C _Bool's false value.
	#[inline(always)]
	pub fn zero_RAX(&mut self)
	{
		use self::Register32Bit::EAX;
		
		self.xor_Register32Bit_Register32Bit(EAX, EAX);
	}
	
	/// Sets the `RAX` register to be equivalent to a C _Bool's false value using the most efficient code (`MOV AL, 0x00`).
	///
	/// Note that this is not necessarily `0x00000000_00000001`, just that the bottom eight bits of `RAX` are `0x00`.
	#[inline(always)]
	pub fn set_RAX_to_c_bool_false(&mut self)
	{
		use self::Register8Bit::AL;
		
		self.mov_Register8Bit_Immediate8Bit(AL, Immediate8Bit::Zero);
	}
	
	/// Sets the `RAX` register to be equivalent to a C _Bool's true value using the most efficient code (`MOV AL, 0x01`).
	///
	/// Note that this is not necessarily `0x00000000_00000001`, just that the bottom eight bits of `RAX` are `0x01`.
	#[inline(always)]
	pub fn set_RAX_to_c_bool_true(&mut self)
	{
		use self::Register8Bit::AL;
		
		self.mov_Register8Bit_Immediate8Bit(AL, Immediate8Bit::One);
	}
	
	/// Creates a function pointer to the current location that takes no arguments and returns a result of type `R`.
	///
	/// Resultant function will not execute (and in all likelihood cause an uncaught signal to occur) until `self.finish()` is called.
	#[inline(always)]
	pub fn nullary_function_pointer<R>(&self) -> unsafe extern "C" fn() -> R
	{
		unsafe { transmute(self.instruction_pointer()) }
	}
	
	/// Creates a function pointer to the current location that takes one argument of type `A` and returns a result of type `R`.
	///
	/// Resultant function will not execute (and in all likelihood cause an uncaught signal to occur) until `self.finish()` is called.
	#[inline(always)]
	pub fn unary_function_pointer<R, A>(&self) -> unsafe extern "C" fn(A) -> R
	{
		unsafe { transmute(self.instruction_pointer()) }
	}
	
	/// Creates a function pointer to the current location that takes two argument of types `A` and `B` and returns a result of type `R`.
	///
	/// Resultant function will not execute (and in all likelihood cause an uncaught signal to occur) until `self.finish()` is called.
	#[inline(always)]
	pub fn binary_function_pointer<R, A, B>(&self) -> unsafe extern "C" fn(A, B) -> R
	{
		unsafe { transmute(self.instruction_pointer()) }
	}
	
	/// Creates a function pointer to the current location that takes three argument of types `A`, `B` and `C` and returns a result of type `R`.
	///
	/// Resultant function will not execute (and in all likelihood cause an uncaught signal to occur) until `self.finish()` is called.
	#[inline(always)]
	pub fn ternary_function_pointer<R, A, B, C>(&self) -> unsafe extern "C" fn(A, B, C) -> R
	{
		unsafe { transmute(self.instruction_pointer()) }
	}
	
	/// Creates a function pointer to the current location that takes four argument of types `A`, `B`, `C` and `D` and returns a result of type `R`.
	///
	/// Resultant function will not execute (and in all likelihood cause an uncaught signal to occur) until `self.finish()` is called.
	#[inline(always)]
	pub fn quaternary_function_pointer<R, A, B, C, D>(&self) -> unsafe extern "C" fn(A, B, C, D) -> R
	{
		unsafe { transmute(self.instruction_pointer()) }
	}
	
	/// Creates a function pointer to the current location that takes five argument of types `A`, `B`, `C`, `D` and `E` and returns a result of type `R`.
	///
	/// Resultant function will not execute (and in all likelihood cause an uncaught signal to occur) until `self.finish()` is called.
	#[inline(always)]
	pub fn quinary_function_pointer<R, A, B, C, D, E>(&self) -> unsafe extern "C" fn(A, B, C, D, E) -> R
	{
		unsafe { transmute(self.instruction_pointer()) }
	}
	
	/// Creates a function pointer to the current location that takes six argument of types `A`, `B`, `C`, `D`, `E` and `F` and returns a result of type `R`.
	///
	/// Resultant function will not execute (and in all likelihood cause an uncaught signal to occur) until `self.finish()` is called.
	#[inline(always)]
	pub fn senary_function_pointer<R, A, B, C, D, E, F>(&self) -> unsafe extern "C" fn(A, B, C, D, E, F) -> R
	{
		unsafe { transmute(self.instruction_pointer()) }
	}
	
	/// Emits (pushes) a byte into the instruction stream at the current location.
	///
	/// The byte can be data or instructions.
	#[inline(always)]
	pub fn emit_byte(&mut self, byte: u8)
	{
		self.reserve_space(1);
		self.byte_emitter.emit_u8(byte)
	}
	
	/// Emits (pushes) a word (2 bytes) into the instruction stream at the current location.
	///
	/// The word can be data or instructions.
	///
	/// The word will be swapped into the little endian form (a no-op on x64 platforms).
	#[inline(always)]
	pub fn emit_word(&mut self, word: u16)
	{
		self.reserve_space(2);
		self.byte_emitter.emit_u16(word)
	}
	
	/// Emits (pushes) a double word (4 bytes) into the instruction stream at the current location.
	///
	/// The word can be data or instructions.
	///
	/// The word will be swapped into the little endian form (a no-op on x64 platforms).
	#[inline(always)]
	pub fn emit_double_word(&mut self, double_word: u32)
	{
		self.reserve_space(4);
		self.byte_emitter.emit_u32(double_word)
	}
	
	/// Emits (pushes) a quad word (8 bytes) into the instruction stream at the current location.
	///
	/// The word can be data or instructions.
	///
	/// The word will be swapped into the little endian form (a no-op on x64 platforms).
	#[inline(always)]
	pub fn emit_quad_word(&mut self, quad_word: u64)
	{
		self.reserve_space(8);
		self.byte_emitter.emit_u64(quad_word)
	}
	
	/// Emits (pushes) a double quad word (16 bytes) into the instruction stream at the current location.
	///
	/// The word can be data or instructions.
	///
	/// The word will be swapped into the little endian form (a no-op on x64 platforms).
	#[inline(always)]
	pub fn emit_double_quad_word(&mut self, double_quad_word: u128)
	{
		self.reserve_space(16);
		self.byte_emitter.emit_u128(double_quad_word)
	}
	
	/// Emits (pushes) zero or more bytes into the instruction stream at the current location.
	///
	/// Bytes can be data or instructions.
	#[inline(always)]
	pub fn emit_bytes(&mut self, bytes: &[u8])
	{
		self.reserve_space(bytes.len());
		self.byte_emitter.emit_bytes(bytes)
	}
	
	/// Rewinds by the length of a byte (1 byte) and then emits `byte`.
	#[inline(always)]
	pub fn rewind_to_emit_byte(&mut self, byte: u8)
	{
		let instruction_pointer = self.instruction_pointer();
		self.byte_emitter.emit_u8_at(byte, instruction_pointer - 1)
	}
	
	/// Rewinds by the length of a double word (4 bytes) and then emits `double_word`.
	#[inline(always)]
	pub fn rewind_to_emit_double_word(&mut self, double_word: u32)
	{
		let instruction_pointer = self.instruction_pointer();
		self.byte_emitter.emit_u32_at(double_word, instruction_pointer - 4)
	}
	
	/// Skips over a byte in the instruction stream at the current location.
	#[inline(always)]
	pub(crate) fn skip_byte(&mut self)
	{
		self.reserve_space(1);
		self.byte_emitter.skip_u8()
	}
	
	/// Skips over a double word (4 bytes) in the instruction stream at the current location.
	///
	/// The byte can be data or instructions.
	#[inline(always)]
	pub(crate) fn skip_double_word(&mut self)
	{
		self.reserve_space(4);
		self.byte_emitter.skip_u32()
	}
	
	/// Skips over a quad word (8 bytes) in the instruction stream at the current location.
	///
	/// The byte can be data or instructions.
	#[inline(always)]
	pub(crate) fn skip_quad_word(&mut self)
	{
		self.reserve_space(8);
		self.byte_emitter.skip_u64()
	}
	
	/// Skips over zero or more `count` bytes in the instruction stream at the current location.
	#[inline(always)]
	pub fn skip_bytes(&mut self, count: usize)
	{
		self.reserve_space(count);
		self.byte_emitter.skip_bytes(count)
	}
	
	/// Emits (pushes) `NOP`s (No Operation) opcodes into the instruction stream at the current location to ensure the desired `alignment`.
	///
	/// Efficient for alignments up to 32 (needed for AVX-2).
	#[inline(always)]
	pub fn emit_nops(&mut self, count: usize)
	{
		match count
		{
			0 => (),
			
			1 => self.nop_1(),
			
			2 => self.nop_2(),
			
			3 => self.nop_3(),
			
			4 => self.nop_4(),
			
			5 => self.nop_5(),
			
			6 => self.nop_6(),
			
			7 => self.nop_7(),
			
			8 => self.nop_8(),
			
			9 => self.nop_9(),
			
			10 =>
			{
				self.nop_9();
				self.nop_1()
			}
			
			11 =>
			{
				self.nop_9();
				self.nop_2()
			}
			
			12 =>
			{
				self.nop_9();
				self.nop_3()
			}
			
			13 =>
			{
				self.nop_9();
				self.nop_4()
			}
			
			14 =>
			{
				self.nop_9();
				self.nop_5()
			}
			
			15 =>
			{
				self.nop_9();
				self.nop_6()
			}
			
			16 =>
			{
				self.nop_9();
				self.nop_7()
			}
			
			17 =>
			{
				self.nop_9();
				self.nop_8()
			}
			
			18 =>
			{
				self.nop_9();
				self.nop_9()
			}
			
			19 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_1()
			}
			
			20 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_2()
			}
			
			21 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_3()
			}
			
			22 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_4()
			}
			
			23 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_5()
			}
			
			24 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_6()
			}
			
			25 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_7()
			}
			
			26 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_8()
			}
			
			27 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9()
			}
			
			28 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_1()
			}
			
			29 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_2()
			}
			
			30 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_3()
			}
			
			31 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_4()
			}
			
			32 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_5()
			}
			
			33 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_6()
			}
			
			34 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_7()
			}
			
			35 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_8()
			}
			
			36 =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_9()
			}
			
			_ =>
			{
				self.nop_9();
				self.nop_9();
				self.nop_9();
				self.nop_9();
				
				let nop_count = count - 36;
				let nop_9s = nop_count / 9;
				for _ in 0 .. nop_9s
				{
					self.nop_9()
				}
				
				match nop_count % 9
				{
					0 => (),
					
					1 => self.nop_1(),
					
					2 => self.nop_2(),
					
					3 => self.nop_3(),
					
					4 => self.nop_4(),
					
					5 => self.nop_5(),
					
					6 => self.nop_6(),
					
					7 => self.nop_7(),
					
					8 => self.nop_8(),
					
					_ => unreachable!(),
				}
			}
		}
	}
	
	/// Emits (pushes) `NOP`s (No Operation) opcodes into the instruction stream at the current location to ensure the desired `alignment`.
	///
	/// Efficient for alignments up to 32 (needed for AVX-2).
	#[inline(always)]
	pub fn emit_alignment(&mut self, alignment: usize)
	{
		let offset = self.instruction_pointer() % alignment;
		
		if offset == 0
		{
			return
		}
		
		let count = alignment - offset;
		
		self.emit_nops(count);
	}
	
	#[inline(always)]
	fn nop_1(&mut self)
	{
		const NOP: u8 = 0x90;
		self.emit_byte(NOP)
	}
	
	#[inline(always)]
	fn nop_2(&mut self)
	{
		self.emit_word(0x6690)
	}
	
	#[inline(always)]
	fn nop_3(&mut self)
	{
		self.emit_bytes(&[0x0F, 0x1F, 0x00])
	}
	
	#[inline(always)]
	fn nop_4(&mut self)
	{
		self.emit_double_word(0x0F1F4000)
	}
	
	#[inline(always)]
	fn nop_5(&mut self)
	{
		self.emit_bytes(&[0x0F, 0x1F, 0x44, 0x00, 0x00])
	}
	
	#[inline(always)]
	fn nop_6(&mut self)
	{
		self.emit_bytes(&[0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00])
	}
	
	#[inline(always)]
	fn nop_7(&mut self)
	{
		self.emit_bytes(&[0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00])
	}
	
	#[inline(always)]
	fn nop_8(&mut self)
	{
		self.emit_quad_word(0x0F1F840000000000)
	}
	
	#[inline(always)]
	fn nop_9(&mut self)
	{
		self.emit_bytes(&[0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00])
	}
	
	#[inline(always)]
	fn reserve_space(&mut self, length: usize)
	{
		let remaining_space = self.byte_emitter.remaining_space();
		if remaining_space < length
		{
			if self.attempt_to_resize_in_place().is_err()
			{
				panic!("There is no more space in the buffer and remap failed")
			}
		}
	}
	
	#[inline(always)]
	fn reserve_space_for_instruction(&mut self)
	{
		const MaximumOpcodeLength: usize = 15;
		self.reserve_space(MaximumOpcodeLength)
	}
	
	#[inline(always)]
	fn bookmark(&mut self)
	{
		self.byte_emitter.store_bookmark()
	}
	
	#[inline(always)]
	fn reset_to_bookmark(&mut self)
	{
		self.byte_emitter.reset_to_bookmark()
	}
	
	/// The current instruction pointer.
	#[inline(always)]
	pub fn instruction_pointer(&self) -> InstructionPointer
	{
		self.byte_emitter.instruction_pointer
	}
	
	/// The location of the start of instructions.
	#[inline(always)]
	pub fn start_instruction_pointer(&self) -> InstructionPointer
	{
		self.byte_emitter.start_instruction_pointer
	}
	
	// See Figure 2-9, Intel Manual Volume 2A Section 2-15 (May 2018).
	#[inline(always)]
	fn vex_7(&mut self, mmmmm: u8, L: u8, pp: u8, W: u8, vvvv: impl Register, rm: impl MemoryOrRegister, r: impl Register)
	{
		rm.emit_vex_prefix(&mut self.byte_emitter, mmmmm, L, pp, W, vvvv, r)
	}
	
	// See Figure 2-9, Intel Manual Volume 2A Section 2-15 (May 2018).
	#[inline(always)]
	fn vex_5(&mut self, mmmmm: u8, L: u8, pp: u8, W: u8, vvvv: impl Register)
	{
		if mmmmm == 0x01 && W == 0
		{
			self.byte_emitter.emit_2_byte_vex_prefix(0x80, vvvv, L, pp)
		}
		else
		{
			self.byte_emitter.emit_3_byte_vex_prefix(0x80, 0x40, 0x20, mmmmm, W, vvvv, L, pp)
		}
	}
	
	#[inline(always)]
	fn prefix_fwait(&mut self, byte: u8)
	{
		self.byte_emitter.emit_u8(byte)
	}
	
	#[inline(always)]
	fn prefix_group1(&mut self, byte: u8)
	{
		self.byte_emitter.emit_u8(byte)
	}
	
	#[inline(always)]
	fn prefix_group2(&mut self, memory_operand_or_branch_hint: impl PrefixGroup2)
	{
		memory_operand_or_branch_hint.emit_prefix_group2(&mut self.byte_emitter)
	}
	
	#[inline(always)]
	fn prefix_group3(&mut self)
	{
		self.byte_emitter.emit_u8(0x66)
	}
	
	#[inline(always)]
	fn prefix_group4(&mut self, memory: impl HasMemoryOperand)
	{
		self.prefix_group4_if_address_override(memory.memory_operand().address_override_for_32_bit)
	}
	
	#[inline(always)]
	fn prefix_group4_if_address_override(&mut self, address_override_for_32_bit: bool)
	{
		if address_override_for_32_bit
		{
			const AddressOverridePrefix: u8 = 0x67;
			self.byte_emitter.emit_u8(AddressOverridePrefix)
		}
	}
	
	#[inline(always)]
	fn rex_3(&mut self, rm: impl MemoryOrRegister, r: impl Register, byte: u8)
	{
		rm.emit_rex_3(&mut self.byte_emitter, r, byte)
	}
	
	#[inline(always)]
	fn rex_2(&mut self, rm: impl MemoryOrRegister, byte: u8)
	{
		rm.emit_rex_2(&mut self.byte_emitter, byte)
	}
	
	#[inline(always)]
	fn rex_1(&mut self, byte: u8)
	{
		self.byte_emitter.emit_u8_if_not_zero(byte)
	}
	
	#[inline(always)]
	pub(crate) fn opcode_1(&mut self, opcode: u8)
	{
		self.byte_emitter.emit_u8(opcode)
	}
	
	#[inline(always)]
	fn opcode_2(&mut self, opcode1: u8, rcode: impl OpcodeEncoding)
	{
		rcode.emit_2(self, opcode1)
	}
	
	#[inline(always)]
	fn opcode_3(&mut self, opcode1: u8, opcode2: u8, rcode: impl OpcodeEncoding)
	{
		rcode.emit_3(self, opcode1, opcode2)
	}
	
	#[inline(always)]
	fn mod_rm_sib(&mut self, rm: impl MemoryOrRegister, reg: impl Register)
	{
		rm.emit_mod_rm_sib(&mut self.byte_emitter, reg)
	}
	
	#[inline(always)]
	fn displacement_immediate_1(&mut self, displacement: impl AsDisplacement)
	{
		displacement.displacement().emit(&mut self.byte_emitter)
	}
	
	#[inline(always)]
	fn displacement_immediate_2(&mut self, displacement1: Immediate8Bit, displacement2: Immediate16Bit)
	{
		// NOTE: This order is correct, with the second displacement emitted before the first.
		self.displacement_immediate_1(displacement2);
		self.displacement_immediate_1(displacement1);
	}
	
	/// Returns an error if displacement would exceed 8 bits.
	#[inline(always)]
	fn displacement_label_8bit(&mut self, label: Label) -> ShortJmpResult
	{
		let insert_at_instruction_pointer = self.instruction_pointer();
		self.skip_byte();
		
		let target_instruction_pointer = self.target_instruction_pointer(label);
		
		if target_instruction_pointer.is_valid()
		{
			match self.byte_emitter.insert_8_bit_effective_address_displacement(insert_at_instruction_pointer, target_instruction_pointer)
			{
				Ok(()) => Ok(()),
				Err(()) =>
				{
					self.reset_to_bookmark();
					Err(())
				}
			}
		}
		else
		{
			self.instruction_pointers_to_replace_labels_with_8_bit_displacements.push((label, insert_at_instruction_pointer));
			Ok(())
		}
	}
	
	/// Does not return an error if displacement would exceed 32 bits, but panics in debug builds.
	///
	/// Errors are very unlikely indeed for such overly large displacements, are almost certainly a mistake and can not realistically be recovered from, in any event.
	#[inline(always)]
	fn displacement_label_32bit(&mut self, label: Label)
	{
		let insert_at_instruction_pointer = self.instruction_pointer();
		self.skip_double_word();
		
		let target_instruction_pointer = self.target_instruction_pointer(label);
		
		if target_instruction_pointer.is_valid()
		{
			self.byte_emitter.insert_32_bit_effective_address_displacement(insert_at_instruction_pointer, target_instruction_pointer).expect("32-bit JMP was too far")
		}
		else
		{
			self.instruction_pointers_to_replace_labels_with_32_bit_displacements.push((label, insert_at_instruction_pointer));
		}
	}
	
	/// Calculates a relative address from an absolute address, such as a function pointer or static constant pointer.
	///
	/// **WARNING**: Be very careful using the resultant value for CALL, JMP, etc, as it would need correcting for the size of the emitted opcode sequence including displacement.
	///
	/// **WARNING**: The location of emitted code may be such that if it is more than 2Gb away from common library function calls (eg `printf`); it may be preferrable to use an absolute address indirectly in this case.
	///
	/// **WARNING**: No checks are made for addresses that would exceed the boundaries of signed integers...
	///
	/// **WARNING**: In Kernel-model code, addresses are in the top half of the address space and so the casts below to `isize` will be invalid.
	#[inline(always)]
	fn relative_address_32bit(&self, absolute_address: impl FunctionPointer, offset_to_end_of_opcode_encoding: usize) -> RelativeAddress32Bit
	{
		RelativeAddress32Bit(((absolute_address.absolute_virtual_address() as isize) - ((self.instruction_pointer() + offset_to_end_of_opcode_encoding) as isize)) as i32)
	}
	
	/// Attempts to calculate a Jump destination which uses an index register and scale but an absolute offset from address 0.
	///
	/// Typically used for when building jump tables and for other uses of 'computed jumps' (also known as indirect branches, indirect jumps and register-indirect jumps).
	///
	/// Will try to use a form that does not need a register first (See Agner Fog's Optimizing Volume 2 (Optimizing subroutines in assembly language) § 3.3 Addressing Modes - Addressing static arrays in 64 bit mode, Example 3.11b).
	///
	/// If this is not possible, then falls back to Agner Fog's Optimizing Volume 2 (Optimizing subroutines in assembly language) § 3.3 Addressing Modes - Addressing static arrays in 64 bit mode, Example 3.11c using the value in `base_register_holding_start_of_instructions_pointer`.
	///
	/// A typically register to use for `base_register_holding_start_of_instructions_pointer` would be `RBX`.
	///
	/// The fallback approach will panic if `base_register_holding_start_of_instructions_pointer` is None, or if the required Jump is more than 2Gb relatively (unlikely).
	///
	/// Ideally, make sure the argument `allocate_in_first_2Gb` to `ExecutableAnonymousMemoryMap::new()` is `true` (or test that the start of instructions is below 0x80000000 (2^31 bytes)) and then `base_register_holding_start_of_instructions_pointer` can be None safely.
	#[inline(always)]
	pub fn jmp_Any64BitMemory_statically_relative_address(&mut self, index_register: Register64Bit, scale: IndexScale, base_register_holding_start_of_instructions_pointer: Register64Bit)
	{
		const ArtificallyLargeDisplacementPlaceholder: Immediate32Bit = Immediate32Bit::Maximum;
		
		self.bookmark();
		
		// Firstly, try to use absolute addressing within the first 2Gb.
		let memory_destination = Any64BitMemory::index_64_scale_displacement(index_register, scale, ArtificallyLargeDisplacementPlaceholder);
		self.jmp_Any64BitMemory(memory_destination);
		
		let instruction_pointer = self.instruction_pointer();
		
		if instruction_pointer <= ::std::i32::MAX as usize
		{
			self.rewind_to_emit_double_word(instruction_pointer as u32);
			return
		}
		
		self.reset_to_bookmark();
		
		// Fallback to using an image-relative addressing.
		let memory_destination = Any64BitMemory::base_64_index_64_scale_displacement(base_register_holding_start_of_instructions_pointer, index_register, scale, ArtificallyLargeDisplacementPlaceholder);
		self.jmp_Any64BitMemory(memory_destination);
		
		let image_base = self.start_instruction_pointer();
		let displacement = self.instruction_pointer() - image_base;
		debug_assert!(displacement <= ::std::i32::MAX as usize, "Jumps of more than 2Gb are not supported");
		self.rewind_to_emit_double_word(displacement as u32)
	}
	
	/// Typically used for when trying to reference static (global) arrays in memory using an index with instructions such as `MOV` or `VPTEST`.
	///
	/// Can be used with `jmp_Any64BitMemory()`, but only if the start of the jump table is known in advance.
	///
	/// If it is not, use the method `jmp_Any64BitMemory_accounting_for_first_2Gb()`.
	///
	/// The result is only valid as long as an instruction is not written into this instruction stream.
	///
	/// Will try to use a form that does not need a register first (See Agner Fog's Optimizing Volume 2 (Optimizing subroutines in assembly language) § 3.3 Addressing Modes - Addressing static arrays in 64 bit mode, Example 3.11b).
	///
	/// If this is not possible, then falls back to Agner Fog's Optimizing Volume 2 (Optimizing subroutines in assembly language) § 3.3 Addressing Modes - Addressing static arrays in 64 bit mode, Example 3.11c using the value in `base_register_holding_start_of_instructions_pointer`.
	///
	/// A typically register to use for `base_register_holding_start_of_instructions_pointer` would be `RBX`.
	///
	/// The fallback approach will panic if the required Jump is more than 2Gb relatively (unlikely).
	///
	/// If the result is `None`, one can apply Agner Fog's Optimizing Volume 2 (Optimizing subroutines in assembly language) § 3.3 Addressing Modes - Addressing static arrays in 64 bit mode, Example 3.11e (this will require a `LEA Register, [array_location_in_memory]` prior to this instruction).
	///
	/// Ideally, make sure the argument `allocate_in_first_2Gb` to `ExecutableAnonymousMemoryMap::new()` is `true` (or test that the start of instructions is below 0x80000000 (2^31 bytes)) and then `base_register_holding_start_of_instructions_pointer` can be None safely.
	#[inline(always)]
	pub fn statically_relative_address<BM: BitMemory>(&self, array_location_in_memory: InstructionPointer, index_register: Register64Bit, scale: IndexScale, base_register_holding_start_of_instructions_pointer: Register64Bit) -> BM
	{
		BM::statically_relative_address(self, array_location_in_memory, index_register, scale, base_register_holding_start_of_instructions_pointer)
	}
	
	/// Use as follows with `Memory::relative_instruction_pointer_relative()`:-
	///
	/// ```
	/// instruction_stream.vmovdqa_YMM_Any256BitMemory(ymm_register, Any256BitMemory::relative_instruction_pointer_relative());
	/// instruction_stream.overwrite_last_displacement_with_relative_address_to(absolute_address);
	/// ```
	///
	/// Will panic in debug builds if the required displacement is more than 2Gb (such a displacement is extremely unlikely).
	#[inline(always)]
	pub fn overwrite_last_32bit_displacement_with_relative_address_to(&mut self, location_in_memory: InstructionPointer)
	{
		debug_assert!(location_in_memory <= ::std::isize::MAX as usize, "location_in_memory is larger than ::std::isize::MAX");
		
		let instruction_pointer = self.instruction_pointer();
		debug_assert!(instruction_pointer <= ::std::isize::MAX as usize, "instruction_pointer is larger than ::std::isize::MAX");
		
		let offset = (location_in_memory as isize) - (instruction_pointer as isize);
		debug_assert!(offset <= (::std::i32::MAX as isize) && offset >= (::std::i32::MIN as isize), "offset to location_in_memory is bigger than a 32-bit displacement can hold");
		
		self.rewind_to_emit_double_word(offset as i32 as u32);
	}
	
	/// Emits a block of a fixed size (blocks are padded to the desired size).
	///
	/// Panics in debug builds if the block is too large.
	///
	/// Typically used for jump table blocks.
	///
	/// Size is actually specified as a scale (power of two); use a `u8` or `IndexScale`.
	#[inline(always)]
	pub fn emit_fixed_size_block<R>(&mut self, scale: impl Into<u8>, mut emit_instructions: impl FnMut(&mut Self) -> R) -> R
	{
		let scale = scale.into();
		let desired_block_size = 1 << (scale as usize);
		
		let start = self.instruction_pointer();
		
		let result = emit_instructions(self);
		
		let block_size = self.instruction_pointer() - start;
		debug_assert!(block_size <= desired_block_size, "block_size '{}' exceeds desired_block_size '{}' (scale '{})", block_size, desired_block_size, scale);
		self.skip_bytes(desired_block_size - block_size);
		
		result
	}
}

include!("InstructionStream.instructions.rs");
