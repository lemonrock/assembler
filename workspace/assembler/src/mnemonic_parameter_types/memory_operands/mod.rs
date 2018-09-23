// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


use super::*;


include!("Any8BitMemory.rs");
include!("Any16BitMemory.rs");
include!("Any32BitMemory.rs");
include!("Any64BitMemory.rs");
include!("Any128BitMemory.rs");
include!("Any256BitMemory.rs");
include!("Contiguous2ByteMemory.rs");
include!("Contiguous28ByteMemory.rs");
include!("Contiguous108ByteMemory.rs");
include!("Contiguous512ByteMemory.rs");
include!("FarPointer16BitTo16Bit.rs");
include!("FarPointer16BitTo32Bit.rs");
include!("FarPointer16BitTo64Bit.rs");
include!("IndexScale.rs");
include!("MemoryOperand.rs");
include!("X87BinaryCodedDecimal80BitMemory.rs");
include!("X87Float32BitMemory.rs");
include!("X87Float64BitMemory.rs");
include!("X87Float80BitMemory.rs");
include!("X87Integer16BitMemory.rs");
include!("X87Integer32BitMemory.rs");
include!("X87Integer64BitMemory.rs");
