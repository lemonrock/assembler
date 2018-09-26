// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


macro_rules! impl_immediate
{
	($name: tt, $signed_size: tt, $unsigned_size: tt) =>
	{
		/// An immediate argument, typically used for a displacement.
		#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct $name(pub $signed_size);
		
		impl AsDisplacement for $name
		{
			type D = $unsigned_size;
			
			#[inline(always)]
			fn displacement(self) -> Self::D
			{
				self.0 as $unsigned_size
			}
		}
		
		impl Immediate for $name
		{
			const Zero: Self = $name(0);
			
			const One: Self = $name(1);
			
			const Minimum: Self = $name(::std::$signed_size::MIN);
			
			const Maximum: Self = $name(::std::$signed_size::MAX);
			
			type SignedInteger = $signed_size;
			
			#[inline(always)]
			fn value(self) -> Self::SignedInteger
			{
				self.0
			}
		}
		
		impl Into<$signed_size> for $name
		{
			#[inline(always)]
			fn into(self) -> $signed_size
			{
				self.0
			}
		}
		
		impl Into<$unsigned_size> for $name
		{
			#[inline(always)]
			fn into(self) -> $unsigned_size
			{
				self.0 as $unsigned_size
			}
		}
		
		impl From<$unsigned_size> for $name
		{
			#[inline(always)]
			fn from(immediate: $unsigned_size) -> Self
			{
				$name(immediate as $signed_size)
			}
		}
		
		impl From<$signed_size> for $name
		{
			#[inline(always)]
			fn from(immediate: $signed_size) -> Self
			{
				$name(immediate)
			}
		}

		impl Add<$name> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn add(self, rhs: Self) -> Self::Output
			{
				$name(self.0 + rhs.0)
			}
		}
		
		impl Add<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn add(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 + rhs)
			}
		}
		
		impl AddAssign for $name
		{
			#[inline(always)]
			fn add_assign(&mut self, rhs: Self)
			{
				self.0 += rhs.0
			}
		}
		
		impl AddAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn add_assign(&mut self, rhs: $signed_size)
			{
				self.0 += rhs
			}
		}
		
		impl BitAnd<$name> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn bitand(self, rhs: Self) -> Self::Output
			{
				$name(self.0 & rhs.0)
			}
		}
		
		impl BitAnd<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn bitand(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 & rhs)
			}
		}
		
		impl BitAndAssign for $name
		{
			#[inline(always)]
			fn bitand_assign(&mut self, rhs: Self)
			{
				self.0 &= rhs.0
			}
		}
		
		impl BitAndAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn bitand_assign(&mut self, rhs: $signed_size)
			{
				self.0 &= rhs
			}
		}
		
		impl BitOr<$name> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn bitor(self, rhs: Self) -> Self::Output
			{
				$name(self.0 | rhs.0)
			}
		}
		
		impl BitOr<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn bitor(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 | rhs)
			}
		}
		
		impl BitOrAssign for $name
		{
			#[inline(always)]
			fn bitor_assign(&mut self, rhs: Self)
			{
				self.0 |= rhs.0
			}
		}
		
		impl BitOrAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn bitor_assign(&mut self, rhs: $signed_size)
			{
				self.0 |= rhs
			}
		}
		
		impl BitXor<$name> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn bitxor(self, rhs: Self) -> Self::Output
			{
				$name(self.0 ^ rhs.0)
			}
		}
		
		impl BitXor<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn bitxor(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 ^ rhs)
			}
		}
		
		impl BitXorAssign for $name
		{
			#[inline(always)]
			fn bitxor_assign(&mut self, rhs: Self)
			{
				self.0 ^= rhs.0
			}
		}
		
		impl BitXorAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn bitxor_assign(&mut self, rhs: $signed_size)
			{
				self.0 ^= rhs
			}
		}
		
		impl Div<$name> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn div(self, rhs: Self) -> Self::Output
			{
				$name(self.0 / rhs.0)
			}
		}
		
		impl Div<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn div(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 / rhs)
			}
		}
		
		impl DivAssign for $name
		{
			#[inline(always)]
			fn div_assign(&mut self, rhs: Self)
			{
				self.0 /= rhs.0
			}
		}
		
		impl DivAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn div_assign(&mut self, rhs: $signed_size)
			{
				self.0 /= rhs
			}
		}
		
		impl Mul<$name> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn mul(self, rhs: Self) -> Self::Output
			{
				$name(self.0 * rhs.0)
			}
		}
		
		impl Mul<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn mul(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 * rhs)
			}
		}
		
		impl MulAssign for $name
		{
			#[inline(always)]
			fn mul_assign(&mut self, rhs: Self)
			{
				self.0 *= rhs.0
			}
		}
		
		impl MulAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn mul_assign(&mut self, rhs: $signed_size)
			{
				self.0 *= rhs
			}
		}
		
		impl Neg for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn neg(self) -> Self::Output
			{
				$name(self.0.neg())
			}
		}
		
		impl Not for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn not(self) -> Self::Output
			{
				$name(!self.0)
			}
		}
		
		impl Rem<$name> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn rem(self, rhs: Self) -> Self::Output
			{
				$name(self.0 % rhs.0)
			}
		}
		
		impl Rem<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn rem(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 % rhs)
			}
		}
		
		impl RemAssign for $name
		{
			#[inline(always)]
			fn rem_assign(&mut self, rhs: Self)
			{
				self.0 %= rhs.0
			}
		}
		
		impl RemAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn rem_assign(&mut self, rhs: $signed_size)
			{
				self.0 %= rhs
			}
		}
		
		impl Shl for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn shl(self, rhs: Self) -> Self::Output
			{
				$name(self.0 << rhs.0)
			}
		}
		
		impl Shl<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn shl(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 << rhs)
			}
		}
		
		impl ShlAssign for $name
		{
			#[inline(always)]
			fn shl_assign(&mut self, rhs: Self)
			{
				self.0 = self.0 << rhs.0
			}
		}
		
		impl ShlAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn shl_assign(&mut self, rhs: $signed_size)
			{
				self.0 = self.0 << rhs
			}
		}
		
		impl Shr for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn shr(self, rhs: Self) -> Self::Output
			{
				$name(self.0 >> rhs.0)
			}
		}
		
		impl Shr<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn shr(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 >> rhs)
			}
		}
		
		impl ShrAssign for $name
		{
			#[inline(always)]
			fn shr_assign(&mut self, rhs: Self)
			{
				self.0 = self.0 >> rhs.0
			}
		}
		
		impl ShrAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn shr_assign(&mut self, rhs: $signed_size)
			{
				self.0 = self.0 >> rhs
			}
		}
		
		impl Sub<$name> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn sub(self, rhs: Self) -> Self::Output
			{
				$name(self.0 - rhs.0)
			}
		}
		
		impl Sub<$signed_size> for $name
		{
			type Output = Self;
			
			#[inline(always)]
			fn sub(self, rhs: $signed_size) -> Self::Output
			{
				$name(self.0 - rhs)
			}
		}
		
		impl SubAssign for $name
		{
			#[inline(always)]
			fn sub_assign(&mut self, rhs: Self)
			{
				self.0 -= rhs.0
			}
		}
		
		impl SubAssign<$signed_size> for $name
		{
			#[inline(always)]
			fn sub_assign(&mut self, rhs: $signed_size)
			{
				self.0 -= rhs
			}
		}

	}
}
