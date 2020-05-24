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

// GENERATOR-BEGIN: NumberBase
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Number base
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NumberBase {
	/// Hex numbers (base 16)
	Hexadecimal = 0,
	/// Decimal numbers (base 10)
	Decimal = 1,
	/// Octal numbers (base 8)
	Octal = 2,
	/// Binary numbers (base 2)
	Binary = 3,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_NUMBER_BASE: [&str; 4] = [
	"Hexadecimal",
	"Decimal",
	"Octal",
	"Binary",
];
impl fmt::Debug for NumberBase {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_NUMBER_BASE[*self as usize])?;
		Ok(())
	}
}
impl Default for NumberBase {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		NumberBase::Hexadecimal
	}
}
// GENERATOR-END: NumberBase

// GENERATOR-BEGIN: MemorySizeOptions
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Memory size options used by the formatters
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MemorySizeOptions {
	/// Show memory size if the assembler requires it, else don't show anything
	Default = 0,
	/// Always show the memory size, even if the assembler doesn't need it
	Always = 1,
	/// Show memory size if a human can't figure out the size of the operand
	Minimum = 2,
	/// Never show memory size
	Never = 3,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_MEMORY_SIZE_OPTIONS: [&str; 4] = [
	"Default",
	"Always",
	"Minimum",
	"Never",
];
impl fmt::Debug for MemorySizeOptions {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_MEMORY_SIZE_OPTIONS[*self as usize])?;
		Ok(())
	}
}
impl Default for MemorySizeOptions {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
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
// GENERATOR-END: FormatMnemonicOptions

// GENERATOR-BEGIN: PrefixKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Prefix
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
#[allow(missing_docs)]
pub enum PrefixKind {
	ES = 0,
	CS = 1,
	SS = 2,
	DS = 3,
	FS = 4,
	GS = 5,
	Lock = 6,
	Rep = 7,
	Repe = 8,
	Repne = 9,
	OperandSize = 10,
	AddressSize = 11,
	HintNotTaken = 12,
	HintTaken = 13,
	Bnd = 14,
	Notrack = 15,
	Xacquire = 16,
	Xrelease = 17,
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
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_PREFIX_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for PrefixKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		PrefixKind::ES
	}
}
// GENERATOR-END: PrefixKind

// GENERATOR-BEGIN: DecoratorKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Decorator
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
pub enum DecoratorKind {
	/// Broadcast decorator, eg. `{1to4}`
	Broadcast = 0,
	/// Rounding control, eg. `{rd-sae}`
	RoundingControl = 1,
	/// Suppress all exceptions: `{sae}`
	SuppressAllExceptions = 2,
	/// Zeroing masking: `{z}`
	ZeroingMasking = 3,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_DECORATOR_KIND: [&str; 4] = [
	"Broadcast",
	"RoundingControl",
	"SuppressAllExceptions",
	"ZeroingMasking",
];
impl fmt::Debug for DecoratorKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_DECORATOR_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for DecoratorKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		DecoratorKind::Broadcast
	}
}
// GENERATOR-END: DecoratorKind

// GENERATOR-BEGIN: NumberKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Number kind
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
#[allow(missing_docs)]
pub enum NumberKind {
	Int8 = 0,
	UInt8 = 1,
	Int16 = 2,
	UInt16 = 3,
	Int32 = 4,
	UInt32 = 5,
	Int64 = 6,
	UInt64 = 7,
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
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_NUMBER_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for NumberKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		NumberKind::Int8
	}
}
// GENERATOR-END: NumberKind

// GENERATOR-BEGIN: FormatterTextKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Formatter text kind
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
pub enum FormatterTextKind {
	/// Normal text
	Text = 0,
	/// Assembler directive
	Directive = 1,
	/// Any prefix
	Prefix = 2,
	/// Any mnemonic
	Mnemonic = 3,
	/// Any keyword
	Keyword = 4,
	/// Any operator
	Operator = 5,
	/// Any punctuation
	Punctuation = 6,
	/// Number
	Number = 7,
	/// Any register
	Register = 8,
	/// A decorator, eg. `sae` in `{sae}`
	Decorator = 9,
	/// Selector value (eg. far `JMP`/`CALL`)
	SelectorValue = 10,
	/// Label address (eg. `JE XXXXXX`)
	LabelAddress = 11,
	/// Function address (eg. `CALL XXXXXX`)
	FunctionAddress = 12,
	/// Data symbol
	Data = 13,
	/// Label symbol
	Label = 14,
	/// Function symbol
	Function = 15,
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
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_FORMATTER_TEXT_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for FormatterTextKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
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
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_PSEUDO_OPS_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for PseudoOpsKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
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
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_FORMATTER_FLOW_CONTROL[*self as usize])?;
		Ok(())
	}
}
impl Default for FormatterFlowControl {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		FormatterFlowControl::AlwaysShortBranch
	}
}
// GENERATOR-END: FormatterFlowControl

// GENERATOR-BEGIN: CC_b
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JB` / `JC` / `JNAE`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_b {
	/// `JB`, `CMOVB`, `SETB`
	b = 0,
	/// `JC`, `CMOVC`, `SETC`
	c = 1,
	/// `JNAE`, `CMOVNAE`, `SETNAE`
	nae = 2,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_B: [&str; 3] = [
	"b",
	"c",
	"nae",
];
impl fmt::Debug for CC_b {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_B[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_b {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_b::b
	}
}
// GENERATOR-END: CC_b

// GENERATOR-BEGIN: CC_ae
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JAE` / `JNB` / `JNC`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_ae {
	/// `JAE`, `CMOVAE`, `SETAE`
	ae = 0,
	/// `JNB`, `CMOVNB`, `SETNB`
	nb = 1,
	/// `JNC`, `CMOVNC`, `SETNC`
	nc = 2,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_AE: [&str; 3] = [
	"ae",
	"nb",
	"nc",
];
impl fmt::Debug for CC_ae {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_AE[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_ae {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_ae::ae
	}
}
// GENERATOR-END: CC_ae

// GENERATOR-BEGIN: CC_e
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JE` / `JZ`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_e {
	/// `JE`, `CMOVE`, `SETE`, `LOOPE`, `REPE`
	e = 0,
	/// `JZ`, `CMOVZ`, `SETZ`, `LOOPZ`, `REPZ`
	z = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_E: [&str; 2] = [
	"e",
	"z",
];
impl fmt::Debug for CC_e {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_E[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_e {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_e::e
	}
}
// GENERATOR-END: CC_e

// GENERATOR-BEGIN: CC_ne
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JNE` / `JNZ`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_ne {
	/// `JNE`, `CMOVNE`, `SETNE`, `LOOPNE`, `REPNE`
	ne = 0,
	/// `JNZ`, `CMOVNZ`, `SETNZ`, `LOOPNZ`, `REPNZ`
	nz = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_NE: [&str; 2] = [
	"ne",
	"nz",
];
impl fmt::Debug for CC_ne {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_NE[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_ne {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_ne::ne
	}
}
// GENERATOR-END: CC_ne

// GENERATOR-BEGIN: CC_be
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JBE` / `JNA`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_be {
	/// `JBE`, `CMOVBE`, `SETBE`
	be = 0,
	/// `JNA`, `CMOVNA`, `SETNA`
	na = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_BE: [&str; 2] = [
	"be",
	"na",
];
impl fmt::Debug for CC_be {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_BE[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_be {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_be::be
	}
}
// GENERATOR-END: CC_be

// GENERATOR-BEGIN: CC_a
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JA` / `JNBE`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_a {
	/// `JA`, `CMOVA`, `SETA`
	a = 0,
	/// `JNBE`, `CMOVNBE`, `SETNBE`
	nbe = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_A: [&str; 2] = [
	"a",
	"nbe",
];
impl fmt::Debug for CC_a {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_A[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_a {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_a::a
	}
}
// GENERATOR-END: CC_a

// GENERATOR-BEGIN: CC_p
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JP` / `JPE`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_p {
	/// `JP`, `CMOVP`, `SETP`
	p = 0,
	/// `JPE`, `CMOVPE`, `SETPE`
	pe = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_P: [&str; 2] = [
	"p",
	"pe",
];
impl fmt::Debug for CC_p {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_P[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_p {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_p::p
	}
}
// GENERATOR-END: CC_p

// GENERATOR-BEGIN: CC_np
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JNP` / `JPO`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_np {
	/// `JNP`, `CMOVNP`, `SETNP`
	np = 0,
	/// `JPO`, `CMOVPO`, `SETPO`
	po = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_NP: [&str; 2] = [
	"np",
	"po",
];
impl fmt::Debug for CC_np {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_NP[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_np {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_np::np
	}
}
// GENERATOR-END: CC_np

// GENERATOR-BEGIN: CC_l
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JL` / `JNGE`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_l {
	/// `JL`, `CMOVL`, `SETL`
	l = 0,
	/// `JNGE`, `CMOVNGE`, `SETNGE`
	nge = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_L: [&str; 2] = [
	"l",
	"nge",
];
impl fmt::Debug for CC_l {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_L[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_l {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_l::l
	}
}
// GENERATOR-END: CC_l

// GENERATOR-BEGIN: CC_ge
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JGE` / `JNL`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_ge {
	/// `JGE`, `CMOVGE`, `SETGE`
	ge = 0,
	/// `JNL`, `CMOVNL`, `SETNL`
	nl = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_GE: [&str; 2] = [
	"ge",
	"nl",
];
impl fmt::Debug for CC_ge {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_GE[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_ge {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_ge::ge
	}
}
// GENERATOR-END: CC_ge

// GENERATOR-BEGIN: CC_le
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JLE` / `JNG`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_le {
	/// `JLE`, `CMOVLE`, `SETLE`
	le = 0,
	/// `JNG`, `CMOVNG`, `SETNG`
	ng = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_LE: [&str; 2] = [
	"le",
	"ng",
];
impl fmt::Debug for CC_le {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_LE[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_le {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_le::le
	}
}
// GENERATOR-END: CC_le

// GENERATOR-BEGIN: CC_g
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JG` / `JNLE`)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum CC_g {
	/// `JG`, `CMOVG`, `SETG`
	g = 0,
	/// `JNLE`, `CMOVNLE`, `SETNLE`
	nle = 1,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CC_G: [&str; 2] = [
	"g",
	"nle",
];
impl fmt::Debug for CC_g {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CC_G[*self as usize])?;
		Ok(())
	}
}
impl Default for CC_g {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CC_g::g
	}
}
// GENERATOR-END: CC_g
