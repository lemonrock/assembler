// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Any bit memory (eg `Any256BitMemory`) used as a memory operand.
pub trait BitMemory: Memory
{
	#[doc(hidden)]
	#[inline(always)]
	fn statically_relative_address(instruction_stream: &InstructionStream, array_location_in_memory: InstructionPointer, index_register: Register64Bit, scale: IndexScale, base_register_holding_start_of_instructions_pointer: Register64Bit) -> Self
	{
		if array_location_in_memory <= ::std::i32::MAX as usize
		{
			Self::index_64_scale_displacement(index_register, scale, (array_location_in_memory as i32).into())
		}
		else
		{
			let image_base = instruction_stream.start_instruction_pointer();
			debug_assert!(image_base <= ::std::isize::MAX as usize, "self.start_instruction_pointer() 0x{:08X} is greater than ::std::isize::MAX", image_base);
			debug_assert!(array_location_in_memory <= ::std::isize::MAX as usize, "array_location_in_memory 0x{:08X} is greater than ::std::isize::MAX", array_location_in_memory);
			
			let address_of_array_relative_to_image_base = (array_location_in_memory as isize) - (image_base as isize);
			debug_assert!(address_of_array_relative_to_image_base <= ::std::i32::MAX as isize, "address_of_array_relative_to_image_base 0x{:08X} is greater than ::std::i32::MAX", address_of_array_relative_to_image_base);
			
			Self::base_64_index_64_scale_displacement(base_register_holding_start_of_instructions_pointer, index_register, scale, (address_of_array_relative_to_image_base as i32).into())
		}
	}
}
