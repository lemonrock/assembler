// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


use super::*;


/// Based on `syntax::ast::LitKind`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustLiteral
{
	/// Maps to `syntax::ast::LitKind::Byte`.
	Byte(u8),
	
	/// Maps to `syntax::ast::LitKind::Int`.
	Integer(u128, ()),
	
	#[doc(hidden)]
	#[allow(dead_code)]
	Other,
}

/// Based on `syntax::ast::UnOp`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustUnaryOpt
{
	/// Maps to `syntax::ast::UnOp::Neg`.
	#[allow(dead_code)]
	Negate,
	
	#[doc(hidden)]
	#[allow(dead_code)]
	Other,
}

/// Based on `syntax::ast::Lit`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RustLiteralNode
{
	node: RustLiteral,
}

/// Based on `syntax::ast::ExprKind`.
///
/// See <https://docs.rs/rustc-ap-syntax/224.0.0/syntax/ast/enum.ExprKind.html>.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum RustExpressionKind
{
	/// Based on `syntax::ast::ExprKind::Lit`.
	Literal(RustLiteral),
	
	/// Based on `syntax::ast::ExprKind::Unary`.
	Unary(RustUnaryOpt, Box<RustExpression>),
	
	#[doc(hidden)]
	#[allow(dead_code)]
	Other,
}

/// Based on `syntax::ptr::P<syntax::ast::Expr>`.
///
/// See <https://docs.rs/rustc-ap-syntax/224.0.0/syntax/ast/struct.Expr.html>.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RustExpression
{
	node: RustExpressionKind,
}

impl RustExpression
{
	#[inline(always)]
	pub(crate) fn zero() -> Self
	{
		Self
		{
			node: RustExpressionKind::Literal(RustLiteral::Integer(0, ()))
		}
	}
	
	#[inline(always)]
	pub(crate) fn literal_byte(byte: u8) -> Self
	{
		Self
		{
			node: RustExpressionKind::Literal(RustLiteral::Byte(byte))
		}
	}
	
	/// Returns an expression `self | (value & mask)`.
	#[inline(always)]
	pub(crate) fn or_with_masked_value(self, _value: Self, _mask: u8) -> Self
	{
		unimplemented!()
	}
	
	#[inline(always)]
	pub(crate) fn derive_size(&self) -> Option<Size>
	{
		use self::Size::*;
		use self::RustExpressionKind::*;
		use self::RustLiteral::*;
		use self::RustUnaryOpt::*;
		
		match self.node
		{
			Literal(ref literal) => match *literal
			{
				Byte(_) => Some(BYTE),
				
				Integer(value, _) => if value < 0x80
				{
					Some(BYTE)
				}
				else if value < 0x8000
				{
					Some(WORD)
				}
				else if value < 0x8000_0000
				{
					Some(DWORD)
				}
				else
				{
					Some(QWORD)
				},
				
				_ => None,
			},
			
			Unary(Negate, ref rust_expression) => match rust_expression.node
			{
				Literal(ref literal) => match *literal
				{
					Byte(_) => Some(BYTE),
					
					Integer(value, _) => if value >= 0x80
					{
						Some(BYTE)
					}
					else if value >= 0x8000
					{
						Some(WORD)
					}
					else if value >= 0x8000_0000
					{
						Some(DWORD)
					}
					else
					{
						Some(QWORD)
					},
					
					_ => None
				},
				
				_ => None,
			}
			
			_ => None,
		}
	}
}

// TODO: Revise.
/// `pub type Ident = Spanned<ast::Ident>`.
pub type RustIdent = String;
