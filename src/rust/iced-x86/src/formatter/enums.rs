/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use core::fmt;
#[cfg(feature = "javascript")]
use wasm_bindgen::prelude::*;

// GENERATOR-BEGIN: NumberBase
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Number base
#[cfg_attr(feature = "javascript", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NumberBase {
	/// Hex numbers (base 16)
	Hexadecimal,
	/// Decimal numbers (base 10)
	Decimal,
	/// Octal numbers (base 8)
	Octal,
	/// Binary numbers (base 2)
	Binary,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_NUMBER_BASE: [&str; 4] = [
	"Hexadecimal",
	"Decimal",
	"Octal",
	"Binary",
];
impl fmt::Debug for NumberBase {
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_NUMBER_BASE[*self as usize])?;
		Ok(())
	}
}
impl Default for NumberBase {
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn default() -> Self {
		NumberBase::Hexadecimal
	}
}
// GENERATOR-END: NumberBase

// GENERATOR-BEGIN: MemorySizeOptions
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Memory size options used by the formatters
#[cfg_attr(feature = "javascript", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MemorySizeOptions {
	/// Show memory size if the assembler requires it, else don't show anything
	Default,
	/// Always show the memory size, even if the assembler doesn't need it
	Always,
	/// Show memory size if a human can't figure out the size of the operand
	Minimum,
	/// Never show memory size
	Never,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_MEMORY_SIZE_OPTIONS: [&str; 4] = [
	"Default",
	"Always",
	"Minimum",
	"Never",
];
impl fmt::Debug for MemorySizeOptions {
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_MEMORY_SIZE_OPTIONS[*self as usize])?;
		Ok(())
	}
}
impl Default for MemorySizeOptions {
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn default() -> Self {
		MemorySizeOptions::Default
	}
}
// GENERATOR-END: MemorySizeOptions

// GENERATOR-BEGIN: FormatMnemonicOptions
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Format mnemonic options
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub struct FormatMnemonicOptions;
impl FormatMnemonicOptions {
	/// No option is set
	pub const NONE: u32 = 0x0000_0000;
	/// Don't add any prefixes
	pub const NO_PREFIXES: u32 = 0x0000_0001;
	/// Don't add the mnemonic
	pub const NO_MNEMONIC: u32 = 0x0000_0002;
}
/// Format mnemonic options
#[cfg(feature = "javascript")]
#[cfg_attr(feature = "javascript", wasm_bindgen(js_name = "FormatMnemonicOptions"))]
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum FormatMnemonicOptionsJS {
	/// No option is set
	None = 0x0000_0000,
	/// Don't add any prefixes
	NoPrefixes = 0x0000_0001,
	/// Don't add the mnemonic
	NoMnemonic = 0x0000_0002,
}
// GENERATOR-END: FormatMnemonicOptions

// GENERATOR-BEGIN: PrefixKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Prefix
#[cfg_attr(feature = "javascript", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(missing_docs)]
pub enum PrefixKind {
	ES,
	CS,
	SS,
	DS,
	FS,
	GS,
	Lock,
	Rep,
	Repe,
	Repne,
	OperandSize,
	AddressSize,
	HintNotTaken,
	HintTaken,
	Bnd,
	Notrack,
	Xacquire,
	Xrelease,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_PREFIX_KIND: [&str; 18] = [
	"ES",
	"CS",
	"SS",
	"DS",
	"FS",
	"GS",
	"Lock",
	"Rep",
	"Repe",
	"Repne",
	"OperandSize",
	"AddressSize",
	"HintNotTaken",
	"HintTaken",
	"Bnd",
	"Notrack",
	"Xacquire",
	"Xrelease",
];
impl fmt::Debug for PrefixKind {
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_PREFIX_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for PrefixKind {
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn default() -> Self {
		PrefixKind::ES
	}
}
// GENERATOR-END: PrefixKind

// GENERATOR-BEGIN: DecoratorKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Decorator
#[cfg_attr(feature = "javascript", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
pub enum DecoratorKind {
	/// Broadcast decorator, eg. `{1to4}`
	Broadcast,
	/// Rounding control, eg. `{rd-sae}`
	RoundingControl,
	/// Suppress all exceptions: `{sae}`
	SuppressAllExceptions,
	/// Zeroing masking: `{z}`
	ZeroingMasking,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_DECORATOR_KIND: [&str; 4] = [
	"Broadcast",
	"RoundingControl",
	"SuppressAllExceptions",
	"ZeroingMasking",
];
impl fmt::Debug for DecoratorKind {
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_DECORATOR_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for DecoratorKind {
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn default() -> Self {
		DecoratorKind::Broadcast
	}
}
// GENERATOR-END: DecoratorKind

// GENERATOR-BEGIN: NumberKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Number kind
#[cfg_attr(feature = "javascript", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
#[allow(missing_docs)]
pub enum NumberKind {
	Int8,
	UInt8,
	Int16,
	UInt16,
	Int32,
	UInt32,
	Int64,
	UInt64,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_NUMBER_KIND: [&str; 8] = [
	"Int8",
	"UInt8",
	"Int16",
	"UInt16",
	"Int32",
	"UInt32",
	"Int64",
	"UInt64",
];
impl fmt::Debug for NumberKind {
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_NUMBER_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for NumberKind {
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn default() -> Self {
		NumberKind::Int8
	}
}
// GENERATOR-END: NumberKind

// GENERATOR-BEGIN: FormatterTextKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Formatter text kind
#[cfg_attr(feature = "javascript", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
pub enum FormatterTextKind {
	/// Normal text
	Text,
	/// Assembler directive
	Directive,
	/// Any prefix
	Prefix,
	/// Any mnemonic
	Mnemonic,
	/// Any keyword
	Keyword,
	/// Any operator
	Operator,
	/// Any punctuation
	Punctuation,
	/// Number
	Number,
	/// Any register
	Register,
	/// A decorator, eg. `sae` in `{sae}`
	Decorator,
	/// Selector value (eg. far `JMP`/`CALL`)
	SelectorValue,
	/// Label address (eg. `JE XXXXXX`)
	LabelAddress,
	/// Function address (eg. `CALL XXXXXX`)
	FunctionAddress,
	/// Data symbol
	Data,
	/// Label symbol
	Label,
	/// Function symbol
	Function,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_FORMATTER_TEXT_KIND: [&str; 16] = [
	"Text",
	"Directive",
	"Prefix",
	"Mnemonic",
	"Keyword",
	"Operator",
	"Punctuation",
	"Number",
	"Register",
	"Decorator",
	"SelectorValue",
	"LabelAddress",
	"FunctionAddress",
	"Data",
	"Label",
	"Function",
];
impl fmt::Debug for FormatterTextKind {
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_FORMATTER_TEXT_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for FormatterTextKind {
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn default() -> Self {
		FormatterTextKind::Text
	}
}
// GENERATOR-END: FormatterTextKind

// GENERATOR-BEGIN: SymbolFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Symbol flags
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub struct SymbolFlags;
impl SymbolFlags {
	/// No bit is set
	pub const NONE: u32 = 0x0000_0000;
	/// It's a symbol relative to a register, eg. a struct offset `[ebx+some_struct.field1]`. If this is cleared, it's the address of a symbol.
	pub const RELATIVE: u32 = 0x0000_0001;
	/// It's a signed symbol and it should be displayed as `-symbol` or `reg-symbol` instead of `symbol` or `reg+symbol`
	pub const SIGNED: u32 = 0x0000_0002;
}
/// Symbol flags
#[cfg(feature = "javascript")]
#[cfg_attr(feature = "javascript", wasm_bindgen(js_name = "SymbolFlags"))]
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum SymbolFlagsJS {
	/// No bit is set
	None = 0x0000_0000,
	/// It's a symbol relative to a register, eg. a struct offset `[ebx+some_struct.field1]`. If this is cleared, it's the address of a symbol.
	Relative = 0x0000_0001,
	/// It's a signed symbol and it should be displayed as `-symbol` or `reg-symbol` instead of `symbol` or `reg+symbol`
	Signed = 0x0000_0002,
}
// GENERATOR-END: SymbolFlags

// GENERATOR-BEGIN: PseudoOpsKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum PseudoOpsKind {
	cmpps,
	vcmpps,
	cmppd,
	vcmppd,
	cmpss,
	vcmpss,
	cmpsd,
	vcmpsd,
	pclmulqdq,
	vpclmulqdq,
	vpcomb,
	vpcomw,
	vpcomd,
	vpcomq,
	vpcomub,
	vpcomuw,
	vpcomud,
	vpcomuq,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_PSEUDO_OPS_KIND: [&str; 18] = [
	"cmpps",
	"vcmpps",
	"cmppd",
	"vcmppd",
	"cmpss",
	"vcmpss",
	"cmpsd",
	"vcmpsd",
	"pclmulqdq",
	"vpclmulqdq",
	"vpcomb",
	"vpcomw",
	"vpcomd",
	"vpcomq",
	"vpcomub",
	"vpcomuw",
	"vpcomud",
	"vpcomuq",
];
impl fmt::Debug for PseudoOpsKind {
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_PSEUDO_OPS_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for PseudoOpsKind {
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn default() -> Self {
		PseudoOpsKind::cmpps
	}
}
// GENERATOR-END: PseudoOpsKind

// GENERATOR-BEGIN: FormatterFlowControl
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) enum FormatterFlowControl {
	AlwaysShortBranch,
	ShortBranch,
	NearBranch,
	NearCall,
	FarBranch,
	FarCall,
	Xbegin,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_FORMATTER_FLOW_CONTROL: [&str; 7] = [
	"AlwaysShortBranch",
	"ShortBranch",
	"NearBranch",
	"NearCall",
	"FarBranch",
	"FarCall",
	"Xbegin",
];
impl fmt::Debug for FormatterFlowControl {
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_FORMATTER_FLOW_CONTROL[*self as usize])?;
		Ok(())
	}
}
impl Default for FormatterFlowControl {
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn default() -> Self {
		FormatterFlowControl::AlwaysShortBranch
	}
}
// GENERATOR-END: FormatterFlowControl
