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

use super::cc::{
	cc_a_to_iced, cc_ae_to_iced, cc_b_to_iced, cc_be_to_iced, cc_e_to_iced, cc_g_to_iced, cc_ge_to_iced, cc_l_to_iced, cc_le_to_iced, cc_ne_to_iced,
	cc_np_to_iced, cc_p_to_iced, iced_to_cc_a, iced_to_cc_ae, iced_to_cc_b, iced_to_cc_be, iced_to_cc_e, iced_to_cc_g, iced_to_cc_ge, iced_to_cc_l,
	iced_to_cc_le, iced_to_cc_ne, iced_to_cc_np, iced_to_cc_p, CC_a, CC_ae, CC_b, CC_be, CC_e, CC_g, CC_ge, CC_l, CC_le, CC_ne, CC_np, CC_p,
};
use super::format_mnemonic_options::FormatMnemonicOptions;
use super::instruction::Instruction;
use super::memory_size_options::{iced_to_memory_size_options, memory_size_options_to_iced, MemorySizeOptions};
#[cfg(feature = "instr_info")]
use super::op_access::{iced_to_op_access, OpAccess};
#[cfg(feature = "instr_api")]
use super::register::{register_to_iced, Register};
use wasm_bindgen::prelude::*;

/// Formatter syntax (GNU Assembler, Intel XED, masm, nasm)
#[wasm_bindgen]
pub enum FormatterSyntax {
	/// GNU Assembler (AT&T)
	Gas,
	/// Intel XED
	Intel,
	/// masm
	Masm,
	/// nasm
	Nasm,
}

/// X86 formatter that supports GNU Assembler, Intel XED, masm and nasm syntax
#[wasm_bindgen]
pub struct Formatter {
	formatter: Box<dyn iced_x86_rust::Formatter>,
}

#[wasm_bindgen]
impl Formatter {
	/// Creates an x86 formatter
	///
	/// # Arguments
	///
	/// * `syntax`: Formatter syntax, see [`FormatterSyntax`]
	///
	/// # Examples
	///
	/// ```js
	/// const assert = require("assert").strict;
	/// const { Decoder, DecoderOptions, Formatter, FormatterSyntax } = require("iced-x86");
	///
	/// const bytes = new Uint8Array([0x62, 0xF2, 0x4F, 0xDD, 0x72, 0x50, 0x01]);
	/// const decoder = new Decoder(64, bytes, DecoderOptions.None);
	/// const instr = decoder.decode();
	///
	/// const formatter = new Formatter(FormatterSyntax.Masm);
	/// formatter.uppercaseMnemonics = true;
	/// const disasm = formatter.format(instr);
	/// assert.equal(disasm, "VCVTNE2PS2BF16 zmm2{k5}{z},zmm6,dword bcst [rax+4]");
	///
	/// // Free wasm memory
	/// decoder.free();
	/// instr.free();
	/// formatter.free();
	/// ```
	///
	/// [`FormatterSyntax`]: enum.FormatterSyntax.html
	#[wasm_bindgen(constructor)]
	pub fn new(syntax: FormatterSyntax) -> Self {
		let formatter: Box<dyn iced_x86_rust::Formatter> = match syntax {
			#[cfg(feature = "gas")]
			FormatterSyntax::Gas => Box::new(iced_x86_rust::GasFormatter::new()),
			#[cfg(not(feature = "gas"))]
			FormatterSyntax::Gas => panic!(),
			#[cfg(feature = "intel")]
			FormatterSyntax::Intel => Box::new(iced_x86_rust::IntelFormatter::new()),
			#[cfg(not(feature = "intel"))]
			FormatterSyntax::Intel => panic!(),
			#[cfg(feature = "masm")]
			FormatterSyntax::Masm => Box::new(iced_x86_rust::MasmFormatter::new()),
			#[cfg(not(feature = "masm"))]
			FormatterSyntax::Masm => panic!(),
			#[cfg(feature = "nasm")]
			FormatterSyntax::Nasm => Box::new(iced_x86_rust::NasmFormatter::new()),
			#[cfg(not(feature = "nasm"))]
			FormatterSyntax::Nasm => panic!(),
		};
		Self { formatter }
	}

	/// Formats the whole instruction: prefixes, mnemonic, operands
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	#[wasm_bindgen]
	pub fn format(&mut self, instruction: &Instruction) -> String {
		let mut output = String::new();
		self.formatter.format(&instruction.0, &mut output);
		output
	}

	/// Formats the mnemonic and any prefixes
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	#[wasm_bindgen(js_name = "formatMnemonic")]
	pub fn format_mnemonic(&mut self, instruction: &Instruction) -> String {
		let mut output = String::new();
		self.formatter.format_mnemonic(&instruction.0, &mut output);
		output
	}

	/// Formats the mnemonic and any prefixes
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	/// - `options`: Options, see [`FormatMnemonicOptions`]
	///
	/// [`FormatMnemonicOptions`]: enum.FormatMnemonicOptions.html
	#[wasm_bindgen(js_name = "formatMnemonicOptions")]
	pub fn format_mnemonic_options(&mut self, instruction: &Instruction, options: u32 /*flags: FormatMnemonicOptions*/) -> String {
		// It's not part of the method sig so make sure it's still compiled by referencing it here
		const_assert_eq!(0, FormatMnemonicOptions::None as u32);
		let mut output = String::new();
		self.formatter.format_mnemonic_options(&instruction.0, &mut output, options);
		output
	}

	/// Gets the number of operands that will be formatted. A formatter can add and remove operands
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	#[wasm_bindgen(js_name = "operandCount")]
	pub fn operand_count(&mut self, instruction: &Instruction) -> u32 {
		self.formatter.operand_count(&instruction.0)
	}

	/// Returns the operand access ([`OpAccess`]) but only if it's an operand added by the formatter. If it's an
	/// operand that is part of [`Instruction`], you should call eg. [`InstructionInfoFactory.info()`].
	///
	/// # Throws
	///
	/// Throws if `operand` is invalid
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	/// - `operand`: Operand number, 0-based. This is a formatter operand and isn't necessarily the same as an instruction operand. See [`operandCount`]
	///
	/// [`OpAccess`]: enum.OpAccess.html
	/// [`Instruction`]: struct.Instruction.html
	/// [`InstructionInfoFactory.info()`]: struct.InstructionInfoFactory.html#method.info
	/// [`operandCount`]: #method.operand_count
	#[cfg(feature = "instr_info")]
	#[wasm_bindgen(js_name = "opAccess")]
	pub fn op_access(&mut self, instruction: &Instruction, operand: u32) -> Option<OpAccess> {
		self.formatter.op_access(&instruction.0, operand).unwrap().map(iced_to_op_access)
	}

	/// Converts a formatter operand index to an instruction operand index. Returns `undefined` if it's an operand added by the formatter
	///
	/// # Throws
	///
	/// Throws if `operand` is invalid
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	/// - `operand`: Operand number, 0-based. This is a formatter operand and isn't necessarily the same as an instruction operand. See [`operandCount`]
	///
	/// [`operandCount`]: #method.operand_count
	#[wasm_bindgen(js_name = "getInstructionOperand")]
	pub fn get_instruction_operand(&mut self, instruction: &Instruction, operand: u32) -> Option<u32> {
		self.formatter.get_instruction_operand(&instruction.0, operand).unwrap()
	}

	/// Converts an instruction operand index to a formatter operand index. Returns `undefined` if the instruction operand isn't used by the formatter
	///
	/// # Throws
	///
	/// Throws if `instructionOperand` is invalid
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	/// - `instructionOperand`: Instruction operand
	#[wasm_bindgen(js_name = "getFormatterOperand")]
	pub fn get_formatter_operand(&mut self, instruction: &Instruction, #[allow(non_snake_case)] instructionOperand: u32) -> Option<u32> {
		self.formatter.get_formatter_operand(&instruction.0, instructionOperand).unwrap()
	}

	/// Formats an operand. This is a formatter operand and not necessarily a real instruction operand.
	/// A formatter can add and remove operands.
	///
	/// # Throws
	///
	/// Throws if `operand` is invalid
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	/// - `operand`: Operand number, 0-based. This is a formatter operand and isn't necessarily the same as an instruction operand. See [`operandCount`]
	///
	/// [`operandCount`]: #method.operand_count
	#[wasm_bindgen(js_name = "formatOperand")]
	pub fn format_operand(&mut self, instruction: &Instruction, operand: u32) -> String {
		let mut output = String::new();
		self.formatter.format_operand(&instruction.0, &mut output, operand).unwrap();
		output
	}

	/// Formats an operand separator
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	#[wasm_bindgen(js_name = "formatOperandSeparator")]
	pub fn format_operand_separator(&mut self, instruction: &Instruction) -> String {
		let mut output = String::new();
		self.formatter.format_operand_separator(&instruction.0, &mut output);
		output
	}

	/// Formats all operands
	///
	/// # Arguments
	///
	/// - `instruction`: Instruction
	#[wasm_bindgen(js_name = "formatAllOperands")]
	pub fn format_all_operands(&mut self, instruction: &Instruction) -> String {
		let mut output = String::new();
		self.formatter.format_all_operands(&instruction.0, &mut output);
		output
	}

	/// Formats a register
	///
	/// # Arguments
	///
	/// - `register`: Register (a [`Register`] enum value)
	///
	/// [`Register`]: enum.Register.html
	#[wasm_bindgen(js_name = "formatRegister")]
	// This adds the Register enum to the js/ts files, but this API won't be called often so disable it by default.
	#[cfg(feature = "instr_api")]
	pub fn format_register(&mut self, register: Register) -> String {
		self.formatter.format_register(register_to_iced(register)).to_owned()
	}

	/// Formats a `i8`
	///
	/// # Arguments
	///
	/// - `value`: Value
	#[wasm_bindgen(js_name = "formatI8")]
	pub fn format_i8(&mut self, value: i8) -> String {
		self.formatter.format_i8(value).to_owned()
	}

	/// Formats a `i16`
	///
	/// # Arguments
	///
	/// - `value`: Value
	#[wasm_bindgen(js_name = "formatI16")]
	pub fn format_i16(&mut self, value: i16) -> String {
		self.formatter.format_i16(value).to_owned()
	}

	/// Formats a `i32`
	///
	/// # Arguments
	///
	/// - `value`: Value
	#[wasm_bindgen(js_name = "formatI32")]
	pub fn format_i32(&mut self, value: i32) -> String {
		self.formatter.format_i32(value).to_owned()
	}

	/// Formats a `i64`.
	///
	/// Enable the `bigint` feature to use APIs with 64-bit numbers (requires `BigInt`).
	///
	/// # Arguments
	///
	/// - `hi`: High 32 bits of `i64` value
	/// - `lo`: Low 32 bits of `i64` value
	#[wasm_bindgen(js_name = "formatI64")]
	#[cfg(not(feature = "bigint"))]
	pub fn format_i64(&mut self, hi: u32, lo: u32) -> String {
		let value = (((hi as u64) << 32) | (lo as u64)) as i64;
		self.formatter.format_i64(value).to_owned()
	}

	/// Formats a `i64`
	///
	/// # Arguments
	///
	/// - `value`: Value
	#[wasm_bindgen(js_name = "formatI64")]
	#[cfg(feature = "bigint")]
	pub fn format_i64(&mut self, value: i64) -> String {
		self.formatter.format_i64(value).to_owned()
	}

	/// Formats a `u8`
	///
	/// # Arguments
	///
	/// - `value`: Value
	#[wasm_bindgen(js_name = "formatU8")]
	pub fn format_u8(&mut self, value: u8) -> String {
		self.formatter.format_u8(value).to_owned()
	}

	/// Formats a `u16`
	///
	/// # Arguments
	///
	/// - `value`: Value
	#[wasm_bindgen(js_name = "formatU16")]
	pub fn format_u16(&mut self, value: u16) -> String {
		self.formatter.format_u16(value).to_owned()
	}

	/// Formats a `u32`
	///
	/// # Arguments
	///
	/// - `value`: Value
	#[wasm_bindgen(js_name = "formatU32")]
	pub fn format_u32(&mut self, value: u32) -> String {
		self.formatter.format_u32(value).to_owned()
	}

	/// Formats a `u64`.
	///
	/// Enable the `bigint` feature to use APIs with 64-bit numbers (requires `BigInt`).
	///
	/// # Arguments
	///
	/// - `hi`: High 32 bits of `u64` value
	/// - `lo`: Low 32 bits of `u64` value
	#[wasm_bindgen(js_name = "formatU64")]
	#[cfg(not(feature = "bigint"))]
	pub fn format_u64(&mut self, hi: u32, lo: u32) -> String {
		let value = ((hi as u64) << 32) | (lo as u64);
		self.formatter.format_u64(value).to_owned()
	}

	/// Formats a `u64`
	///
	/// # Arguments
	///
	/// - `value`: Value
	#[wasm_bindgen(js_name = "formatU64")]
	#[cfg(feature = "bigint")]
	pub fn format_u64(&mut self, value: u64) -> String {
		self.formatter.format_u64(value).to_owned()
	}

	// NOTE: These tables must render correctly by `cargo doc` and inside of IDEs, eg. VSCode.

	/// Prefixes are upper cased
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `REP stosd`
	/// ✔️ | `false` | `rep stosd`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "uppercasePrefixes")]
	pub fn uppercase_prefixes(&self) -> bool {
		self.formatter.options().uppercase_prefixes()
	}

	/// Prefixes are upper cased
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `REP stosd`
	/// ✔️ | `false` | `rep stosd`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "uppercasePrefixes")]
	pub fn set_uppercase_prefixes(&mut self, value: bool) {
		self.formatter.options_mut().set_uppercase_prefixes(value);
	}

	/// Mnemonics are upper cased
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `MOV rcx,rax`
	/// ✔️ | `false` | `mov rcx,rax`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "uppercaseMnemonics")]
	pub fn uppercase_mnemonics(&self) -> bool {
		self.formatter.options().uppercase_mnemonics()
	}

	/// Mnemonics are upper cased
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `MOV rcx,rax`
	/// ✔️ | `false` | `mov rcx,rax`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "uppercaseMnemonics")]
	pub fn set_uppercase_mnemonics(&mut self, value: bool) {
		self.formatter.options_mut().set_uppercase_mnemonics(value);
	}

	/// Registers are upper cased
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov RCX,[RAX+RDX*8]`
	/// ✔️ | `false` | `mov rcx,[rax+rdx*8]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "uppercaseRegisters")]
	pub fn uppercase_registers(&self) -> bool {
		self.formatter.options().uppercase_registers()
	}

	/// Registers are upper cased
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov RCX,[RAX+RDX*8]`
	/// ✔️ | `false` | `mov rcx,[rax+rdx*8]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "uppercaseRegisters")]
	pub fn set_uppercase_registers(&mut self, value: bool) {
		self.formatter.options_mut().set_uppercase_registers(value);
	}

	/// Keywords are upper cased (eg. `BYTE PTR`, `SHORT`)
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov BYTE PTR [rcx],12h`
	/// ✔️ | `false` | `mov byte ptr [rcx],12h`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "uppercaseKeywords")]
	pub fn uppercase_keywords(&self) -> bool {
		self.formatter.options().uppercase_keywords()
	}

	/// Keywords are upper cased (eg. `BYTE PTR`, `SHORT`)
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov BYTE PTR [rcx],12h`
	/// ✔️ | `false` | `mov byte ptr [rcx],12h`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "uppercaseKeywords")]
	pub fn set_uppercase_keywords(&mut self, value: bool) {
		self.formatter.options_mut().set_uppercase_keywords(value);
	}

	/// Upper case decorators, eg. `{z}`, `{sae}`, `{rd-sae}` (but not op mask registers: `{k1}`)
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `vunpcklps xmm2{k5}{Z},xmm6,dword bcst [rax+4]`
	/// ✔️ | `false` | `vunpcklps xmm2{k5}{z},xmm6,dword bcst [rax+4]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "uppercaseDecorators")]
	pub fn uppercase_decorators(&self) -> bool {
		self.formatter.options().uppercase_decorators()
	}

	/// Upper case decorators, eg. `{z}`, `{sae}`, `{rd-sae}` (but not op mask registers: `{k1}`)
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `vunpcklps xmm2{k5}{Z},xmm6,dword bcst [rax+4]`
	/// ✔️ | `false` | `vunpcklps xmm2{k5}{z},xmm6,dword bcst [rax+4]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "uppercaseDecorators")]
	pub fn set_uppercase_decorators(&mut self, value: bool) {
		self.formatter.options_mut().set_uppercase_decorators(value);
	}

	/// Everything is upper cased, except numbers and their prefixes/suffixes
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `MOV EAX,GS:[RCX*4+0ffh]`
	/// ✔️ | `false` | `mov eax,gs:[rcx*4+0ffh]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "uppercaseAll")]
	pub fn uppercase_all(&self) -> bool {
		self.formatter.options().uppercase_all()
	}

	/// Everything is upper cased, except numbers and their prefixes/suffixes
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `MOV EAX,GS:[RCX*4+0ffh]`
	/// ✔️ | `false` | `mov eax,gs:[rcx*4+0ffh]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "uppercaseAll")]
	pub fn set_uppercase_all(&mut self, value: bool) {
		self.formatter.options_mut().set_uppercase_all(value);
	}

	/// Character index (0-based) where the first operand is formatted. Can be set to 0 to format it immediately after the mnemonic.
	/// At least one space or tab is always added between the mnemonic and the first operand.
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `0` | `mov•rcx,rbp`
	/// &nbsp; | `8` | `mov•••••rcx,rbp`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "firstOperandCharIndex")]
	pub fn first_operand_char_index(&self) -> u32 {
		self.formatter.options().first_operand_char_index()
	}

	/// Character index (0-based) where the first operand is formatted. Can be set to 0 to format it immediately after the mnemonic.
	/// At least one space or tab is always added between the mnemonic and the first operand.
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `0` | `mov•rcx,rbp`
	/// &nbsp; | `8` | `mov•••••rcx,rbp`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "firstOperandCharIndex")]
	pub fn set_first_operand_char_index(&mut self, value: u32) {
		self.formatter.options_mut().set_first_operand_char_index(value);
	}

	/// Size of a tab character or 0 to use spaces
	///
	/// - Default: `0`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "tabSize")]
	pub fn tab_size(&self) -> u32 {
		self.formatter.options().tab_size()
	}

	/// Size of a tab character or 0 to use spaces
	///
	/// - Default: `0`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "tabSize")]
	pub fn set_tab_size(&mut self, value: u32) {
		self.formatter.options_mut().set_tab_size(value);
	}

	/// Add a space after the operand separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov rax, rcx`
	/// ✔️ | `false` | `mov rax,rcx`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "spaceAfterOperandSeparator")]
	pub fn space_after_operand_separator(&self) -> bool {
		self.formatter.options().space_after_operand_separator()
	}

	/// Add a space after the operand separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov rax, rcx`
	/// ✔️ | `false` | `mov rax,rcx`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "spaceAfterOperandSeparator")]
	pub fn set_space_after_operand_separator(&mut self, value: bool) {
		self.formatter.options_mut().set_space_after_operand_separator(value);
	}

	/// Add a space between the memory expression and the brackets
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[ rcx+rdx ]`
	/// ✔️ | `false` | `mov eax,[rcx+rdx]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "spaceAfterMemoryBracket")]
	pub fn space_after_memory_bracket(&self) -> bool {
		self.formatter.options().space_after_memory_bracket()
	}

	/// Add a space between the memory expression and the brackets
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[ rcx+rdx ]`
	/// ✔️ | `false` | `mov eax,[rcx+rdx]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "spaceAfterMemoryBracket")]
	pub fn set_space_after_memory_bracket(&mut self, value: bool) {
		self.formatter.options_mut().set_space_after_memory_bracket(value);
	}

	/// Add spaces between memory operand `+` and `-` operators
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rcx + rdx*8 - 80h]`
	/// ✔️ | `false` | `mov eax,[rcx+rdx*8-80h]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "spaceBetweenMemoryAddOperators")]
	pub fn space_between_memory_add_operators(&self) -> bool {
		self.formatter.options().space_between_memory_add_operators()
	}

	/// Add spaces between memory operand `+` and `-` operators
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rcx + rdx*8 - 80h]`
	/// ✔️ | `false` | `mov eax,[rcx+rdx*8-80h]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "spaceBetweenMemoryAddOperators")]
	pub fn set_space_between_memory_add_operators(&mut self, value: bool) {
		self.formatter.options_mut().set_space_between_memory_add_operators(value);
	}

	/// Add spaces between memory operand `*` operator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rcx+rdx * 8-80h]`
	/// ✔️ | `false` | `mov eax,[rcx+rdx*8-80h]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "spaceBetweenMemoryMulOperators")]
	pub fn space_between_memory_mul_operators(&self) -> bool {
		self.formatter.options().space_between_memory_mul_operators()
	}

	/// Add spaces between memory operand `*` operator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rcx+rdx * 8-80h]`
	/// ✔️ | `false` | `mov eax,[rcx+rdx*8-80h]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "spaceBetweenMemoryMulOperators")]
	pub fn set_space_between_memory_mul_operators(&mut self, value: bool) {
		self.formatter.options_mut().set_space_between_memory_mul_operators(value);
	}

	/// Show memory operand scale value before the index register
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[8*rdx]`
	/// ✔️ | `false` | `mov eax,[rdx*8]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "scaleBeforeIndex")]
	pub fn scale_before_index(&self) -> bool {
		self.formatter.options().scale_before_index()
	}

	/// Show memory operand scale value before the index register
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[8*rdx]`
	/// ✔️ | `false` | `mov eax,[rdx*8]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "scaleBeforeIndex")]
	pub fn set_scale_before_index(&mut self, value: bool) {
		self.formatter.options_mut().set_scale_before_index(value);
	}

	/// Always show the scale value even if it's `*1`
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rbx+rcx*1]`
	/// ✔️ | `false` | `mov eax,[rbx+rcx]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "alwaysShowScale")]
	pub fn always_show_scale(&self) -> bool {
		self.formatter.options().always_show_scale()
	}

	/// Always show the scale value even if it's `*1`
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rbx+rcx*1]`
	/// ✔️ | `false` | `mov eax,[rbx+rcx]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "alwaysShowScale")]
	pub fn set_always_show_scale(&mut self, value: bool) {
		self.formatter.options_mut().set_always_show_scale(value);
	}

	/// Always show the effective segment register. If the option is `false`, only show the segment register if
	/// there's a segment override prefix.
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,ds:[ecx]`
	/// ✔️ | `false` | `mov eax,[ecx]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "alwaysShowSegmentRegister")]
	pub fn always_show_segment_register(&self) -> bool {
		self.formatter.options().always_show_segment_register()
	}

	/// Always show the effective segment register. If the option is `false`, only show the segment register if
	/// there's a segment override prefix.
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,ds:[ecx]`
	/// ✔️ | `false` | `mov eax,[ecx]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "alwaysShowSegmentRegister")]
	pub fn set_always_show_segment_register(&mut self, value: bool) {
		self.formatter.options_mut().set_always_show_segment_register(value);
	}

	/// Show zero displacements
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rcx*2+0]`
	/// ✔️ | `false` | `mov eax,[rcx*2]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "showZeroDisplacements")]
	pub fn show_zero_displacements(&self) -> bool {
		self.formatter.options().show_zero_displacements()
	}

	/// Show zero displacements
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rcx*2+0]`
	/// ✔️ | `false` | `mov eax,[rcx*2]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "showZeroDisplacements")]
	pub fn set_show_zero_displacements(&mut self, value: bool) {
		self.formatter.options_mut().set_show_zero_displacements(value);
	}

	/// Hex number prefix or an empty string, eg. `"0x"`
	///
	/// - Default: `""` (masm/nasm/intel), `"0x"` (gas)
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "hexPrefix")]
	pub fn hex_prefix(&self) -> String {
		self.formatter.options().hex_prefix().to_owned()
	}

	/// Hex number prefix or an empty string, eg. `"0x"`
	///
	/// - Default: `""` (masm/nasm/intel), `"0x"` (gas)
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "hexPrefix")]
	pub fn set_hex_prefix(&mut self, value: String) {
		self.formatter.options_mut().set_hex_prefix_string(value);
	}

	/// Hex number suffix or an empty string, eg. `"h"`
	///
	/// - Default: `"h"` (masm/nasm/intel), `""` (gas)
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "hexSuffix")]
	pub fn hex_suffix(&self) -> String {
		self.formatter.options().hex_suffix().to_owned()
	}

	/// Hex number suffix or an empty string, eg. `"h"`
	///
	/// - Default: `"h"` (masm/nasm/intel), `""` (gas)
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "hexSuffix")]
	pub fn set_hex_suffix(&mut self, value: String) {
		self.formatter.options_mut().set_hex_suffix_string(value);
	}

	/// Size of a digit group, see also [`digitSeparator`]
	///
	/// [`digitSeparator`]: #method.digit_separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `0` | `0x12345678`
	/// ✔️ | `4` | `0x1234_5678`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "hexDigitGroupSize")]
	pub fn hex_digit_group_size(&self) -> u32 {
		self.formatter.options().hex_digit_group_size()
	}

	/// Size of a digit group, see also [`digitSeparator`]
	///
	/// [`digitSeparator`]: #method.digit_separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `0` | `0x12345678`
	/// ✔️ | `4` | `0x1234_5678`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "hexDigitGroupSize")]
	pub fn set_hex_digit_group_size(&mut self, value: u32) {
		self.formatter.options_mut().set_hex_digit_group_size(value);
	}

	/// Decimal number prefix or an empty string
	///
	/// - Default: `""`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "decimalPrefix")]
	pub fn decimal_prefix(&self) -> String {
		self.formatter.options().decimal_prefix().to_owned()
	}

	/// Decimal number prefix or an empty string
	///
	/// - Default: `""`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "decimalPrefix")]
	pub fn set_decimal_prefix(&mut self, value: String) {
		self.formatter.options_mut().set_decimal_prefix_string(value);
	}

	/// Decimal number suffix or an empty string
	///
	/// - Default: `""`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "decimalSuffix")]
	pub fn decimal_suffix(&self) -> String {
		self.formatter.options().decimal_suffix().to_owned()
	}

	/// Decimal number suffix or an empty string
	///
	/// - Default: `""`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "decimalSuffix")]
	pub fn set_decimal_suffix(&mut self, value: String) {
		self.formatter.options_mut().set_decimal_suffix_string(value);
	}

	/// Size of a digit group, see also [`digitSeparator`]
	///
	/// [`digitSeparator`]: #method.digit_separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `0` | `12345678`
	/// ✔️ | `3` | `12_345_678`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "decimalDigitGroupSize")]
	pub fn decimal_digit_group_size(&self) -> u32 {
		self.formatter.options().decimal_digit_group_size()
	}

	/// Size of a digit group, see also [`digitSeparator`]
	///
	/// [`digitSeparator`]: #method.digit_separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `0` | `12345678`
	/// ✔️ | `3` | `12_345_678`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "decimalDigitGroupSize")]
	pub fn set_decimal_digit_group_size(&mut self, value: u32) {
		self.formatter.options_mut().set_decimal_digit_group_size(value);
	}

	/// Octal number prefix or an empty string
	///
	/// - Default: `""` (masm/nasm/intel), `"0"` (gas)
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "octalPrefix")]
	pub fn octal_prefix(&self) -> String {
		self.formatter.options().octal_prefix().to_owned()
	}

	/// Octal number prefix or an empty string
	///
	/// - Default: `""` (masm/nasm/intel), `"0"` (gas)
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "octalPrefix")]
	pub fn set_octal_prefix(&mut self, value: String) {
		self.formatter.options_mut().set_octal_prefix_string(value);
	}

	/// Octal number suffix or an empty string
	///
	/// - Default: `"o"` (masm/nasm/intel), `""` (gas)
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "octalSuffix")]
	pub fn octal_suffix(&self) -> String {
		self.formatter.options().octal_suffix().to_owned()
	}

	/// Octal number suffix or an empty string
	///
	/// - Default: `"o"` (masm/nasm/intel), `""` (gas)
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "octalSuffix")]
	pub fn set_octal_suffix(&mut self, value: String) {
		self.formatter.options_mut().set_octal_suffix_string(value);
	}

	/// Size of a digit group, see also [`digitSeparator`]
	///
	/// [`digitSeparator`]: #method.digit_separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `0` | `12345670`
	/// ✔️ | `4` | `1234_5670`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "octalDigitGroupSize")]
	pub fn octal_digit_group_size(&self) -> u32 {
		self.formatter.options().octal_digit_group_size()
	}

	/// Size of a digit group, see also [`digitSeparator`]
	///
	/// [`digitSeparator`]: #method.digit_separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `0` | `12345670`
	/// ✔️ | `4` | `1234_5670`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "octalDigitGroupSize")]
	pub fn set_octal_digit_group_size(&mut self, value: u32) {
		self.formatter.options_mut().set_octal_digit_group_size(value);
	}

	/// Binary number prefix or an empty string
	///
	/// - Default: `""` (masm/nasm/intel), `"0b"` (gas)
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "binaryPrefix")]
	pub fn binary_prefix(&self) -> String {
		self.formatter.options().binary_prefix().to_owned()
	}

	/// Binary number prefix or an empty string
	///
	/// - Default: `""` (masm/nasm/intel), `"0b"` (gas)
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "binaryPrefix")]
	pub fn set_binary_prefix(&mut self, value: String) {
		self.formatter.options_mut().set_binary_prefix_string(value);
	}

	/// Binary number suffix or an empty string
	///
	/// - Default: `"b"` (masm/nasm/intel), `""` (gas)
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "binarySuffix")]
	pub fn binary_suffix(&self) -> String {
		self.formatter.options().binary_suffix().to_owned()
	}

	/// Binary number suffix or an empty string
	///
	/// - Default: `"b"` (masm/nasm/intel), `""` (gas)
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "binarySuffix")]
	pub fn set_binary_suffix(&mut self, value: String) {
		self.formatter.options_mut().set_binary_suffix_string(value);
	}

	/// Size of a digit group, see also [`digitSeparator`]
	///
	/// [`digitSeparator`]: #method.digit_separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `0` | `11010111`
	/// ✔️ | `4` | `1101_0111`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "binaryDigitGroupSize")]
	pub fn binary_digit_group_size(&self) -> u32 {
		self.formatter.options().binary_digit_group_size()
	}

	/// Size of a digit group, see also [`digitSeparator`]
	///
	/// [`digitSeparator`]: #method.digit_separator
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `0` | `11010111`
	/// ✔️ | `4` | `1101_0111`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "binaryDigitGroupSize")]
	pub fn set_binary_digit_group_size(&mut self, value: u32) {
		self.formatter.options_mut().set_binary_digit_group_size(value);
	}

	/// Digit separator or an empty string. See also eg. [`hexDigitGroupSize`]
	///
	/// [`hexDigitGroupSize`]: #method.hex_digit_group_size
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `""` | `0x12345678`
	/// &nbsp; | `"_"` | `0x1234_5678`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "digitSeparator")]
	pub fn digit_separator(&self) -> String {
		self.formatter.options().digit_separator().to_owned()
	}

	/// Digit separator or an empty string. See also eg. [`hexDigitGroupSize`]
	///
	/// [`hexDigitGroupSize`]: #method.hex_digit_group_size
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `""` | `0x12345678`
	/// &nbsp; | `"_"` | `0x1234_5678`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "digitSeparator")]
	pub fn set_digit_separator(&mut self, value: String) {
		self.formatter.options_mut().set_digit_separator_string(value);
	}

	/// Add leading zeroes to hexadecimal/octal/binary numbers.
	/// This option has no effect on branch targets and displacements, use [`branchLeadingZeroes`]
	/// and [`displacementLeadingZeroes`].
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `0x0000000A`/`0000000Ah`
	/// ✔️ | `false` | `0xA`/`0Ah`
	///
	/// [`branchLeadingZeroes`]: #method.branch_leading_zeroes
	/// [`displacementLeadingZeroes`]: #method.displacement_leading_zeroes
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "leadingZeroes")]
	pub fn leading_zeroes(&self) -> bool {
		self.formatter.options().leading_zeroes()
	}

	/// Add leading zeroes to hexadecimal/octal/binary numbers.
	/// This option has no effect on branch targets and displacements, use [`branchLeadingZeroes`]
	/// and [`displacementLeadingZeroes`].
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `0x0000000A`/`0000000Ah`
	/// ✔️ | `false` | `0xA`/`0Ah`
	///
	/// [`branchLeadingZeroes`]: #method.branch_leading_zeroes
	/// [`displacementLeadingZeroes`]: #method.displacement_leading_zeroes
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "leadingZeroes")]
	pub fn set_leading_zeroes(&mut self, value: bool) {
		self.formatter.options_mut().set_leading_zeroes(value);
	}

	/// Use upper case hex digits
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `0xFF`
	/// &nbsp; | `false` | `0xff`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "uppercaseHex")]
	pub fn uppercase_hex(&self) -> bool {
		self.formatter.options().uppercase_hex()
	}

	/// Use upper case hex digits
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `0xFF`
	/// &nbsp; | `false` | `0xff`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "uppercaseHex")]
	pub fn set_uppercase_hex(&mut self, value: bool) {
		self.formatter.options_mut().set_uppercase_hex(value);
	}

	/// Small hex numbers (-9 .. 9) are shown in decimal
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `9`
	/// &nbsp; | `false` | `0x9`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "smallHexNumbersInDecimal")]
	pub fn small_hex_numbers_in_decimal(&self) -> bool {
		self.formatter.options().small_hex_numbers_in_decimal()
	}

	/// Small hex numbers (-9 .. 9) are shown in decimal
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `9`
	/// &nbsp; | `false` | `0x9`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "smallHexNumbersInDecimal")]
	pub fn set_small_hex_numbers_in_decimal(&mut self, value: bool) {
		self.formatter.options_mut().set_small_hex_numbers_in_decimal(value);
	}

	/// Add a leading zero to hex numbers if there's no prefix and the number starts with hex digits `A-F`
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `0FFh`
	/// &nbsp; | `false` | `FFh`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "addLeadingZeroToHexNumbers")]
	pub fn add_leading_zero_to_hex_numbers(&self) -> bool {
		self.formatter.options().add_leading_zero_to_hex_numbers()
	}

	/// Add a leading zero to hex numbers if there's no prefix and the number starts with hex digits `A-F`
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `0FFh`
	/// &nbsp; | `false` | `FFh`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "addLeadingZeroToHexNumbers")]
	pub fn set_add_leading_zero_to_hex_numbers(&mut self, value: bool) {
		self.formatter.options_mut().set_add_leading_zero_to_hex_numbers(value);
	}

	/// Number base (`2`, `8`, `10`, `16`)
	///
	/// - Default: `16`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "numberBase")]
	pub fn number_base(&self) -> u32 {
		match self.formatter.options().number_base() {
			iced_x86_rust::NumberBase::Binary => 2,
			iced_x86_rust::NumberBase::Octal => 8,
			iced_x86_rust::NumberBase::Decimal => 10,
			iced_x86_rust::NumberBase::Hexadecimal => 16,
		}
	}

	/// Number base (`2`, `8`, `10`, `16`)
	///
	/// - Default: `16`
	///
	/// # Throws
	///
	/// Throws if `value` is not `2`, `8`, `10`, `16`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "numberBase")]
	pub fn set_number_base(&mut self, value: u32) {
		let base = match value {
			2 => iced_x86_rust::NumberBase::Binary,
			8 => iced_x86_rust::NumberBase::Octal,
			10 => iced_x86_rust::NumberBase::Decimal,
			16 => iced_x86_rust::NumberBase::Hexadecimal,
			_ => panic!(),
		};
		self.formatter.options_mut().set_number_base(base);
	}

	/// Add leading zeroes to branch offsets. Used by `CALL NEAR`, `CALL FAR`, `JMP NEAR`, `JMP FAR`, `Jcc`, `LOOP`, `LOOPcc`, `XBEGIN`
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `je 00000123h`
	/// &nbsp; | `false` | `je 123h`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "branchLeadingZeroes")]
	pub fn branch_leading_zeroes(&self) -> bool {
		self.formatter.options().branch_leading_zeroes()
	}

	/// Add leading zeroes to branch offsets. Used by `CALL NEAR`, `CALL FAR`, `JMP NEAR`, `JMP FAR`, `Jcc`, `LOOP`, `LOOPcc`, `XBEGIN`
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `je 00000123h`
	/// &nbsp; | `false` | `je 123h`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "branchLeadingZeroes")]
	pub fn set_branch_leading_zeroes(&mut self, value: bool) {
		self.formatter.options_mut().set_branch_leading_zeroes(value);
	}

	/// Show immediate operands as signed numbers
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,-1`
	/// ✔️ | `false` | `mov eax,FFFFFFFF`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "signedImmediateOperands")]
	pub fn signed_immediate_operands(&self) -> bool {
		self.formatter.options().signed_immediate_operands()
	}

	/// Show immediate operands as signed numbers
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,-1`
	/// ✔️ | `false` | `mov eax,FFFFFFFF`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "signedImmediateOperands")]
	pub fn set_signed_immediate_operands(&mut self, value: bool) {
		self.formatter.options_mut().set_signed_immediate_operands(value);
	}

	/// Displacements are signed numbers
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `mov al,[eax-2000h]`
	/// &nbsp; | `false` | `mov al,[eax+0FFFFE000h]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "signedMemoryDisplacements")]
	pub fn signed_memory_displacements(&self) -> bool {
		self.formatter.options().signed_memory_displacements()
	}

	/// Displacements are signed numbers
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `mov al,[eax-2000h]`
	/// &nbsp; | `false` | `mov al,[eax+0FFFFE000h]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "signedMemoryDisplacements")]
	pub fn set_signed_memory_displacements(&mut self, value: bool) {
		self.formatter.options_mut().set_signed_memory_displacements(value);
	}

	/// Add leading zeroes to displacements
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov al,[eax+00000012h]`
	/// ✔️ | `false` | `mov al,[eax+12h]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "displacementLeadingZeroes")]
	pub fn displacement_leading_zeroes(&self) -> bool {
		self.formatter.options().displacement_leading_zeroes()
	}

	/// Add leading zeroes to displacements
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov al,[eax+00000012h]`
	/// ✔️ | `false` | `mov al,[eax+12h]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "displacementLeadingZeroes")]
	pub fn set_displacement_leading_zeroes(&mut self, value: bool) {
		self.formatter.options_mut().set_displacement_leading_zeroes(value);
	}

	/// Options (a [`MemorySizeOptions`] flags value) that control if the memory size (eg. `DWORD PTR`) is shown or not.
	/// This is ignored by the gas (AT&T) formatter.
	///
	/// - Default: [`Default`]
	///
	/// [`MemorySizeOptions`]: enum.MemorySizeOptions.html
	/// [`Default`]: enum.MemorySizeOptions.html#variant.Default
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "memorySizeOptions")]
	pub fn memory_size_options(&self) -> MemorySizeOptions {
		iced_to_memory_size_options(self.formatter.options().memory_size_options())
	}

	/// Options (a [`MemorySizeOptions`] flags value) that control if the memory size (eg. `DWORD PTR`) is shown or not.
	/// This is ignored by the gas (AT&T) formatter.
	///
	/// - Default: [`Default`]
	///
	/// [`MemorySizeOptions`]: enum.MemorySizeOptions.html
	/// [`Default`]: enum.MemorySizeOptions.html#variant.Default
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "memorySizeOptions")]
	pub fn set_memory_size_options(&mut self, value: MemorySizeOptions) {
		self.formatter.options_mut().set_memory_size_options(memory_size_options_to_iced(value));
	}

	/// Show `RIP+displ` or the virtual address
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rip+12345678h]`
	/// ✔️ | `false` | `mov eax,[1029384756AFBECDh]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "ripRelativeAddresses")]
	pub fn rip_relative_addresses(&self) -> bool {
		self.formatter.options().rip_relative_addresses()
	}

	/// Show `RIP+displ` or the virtual address
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[rip+12345678h]`
	/// ✔️ | `false` | `mov eax,[1029384756AFBECDh]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "ripRelativeAddresses")]
	pub fn set_rip_relative_addresses(&mut self, value: bool) {
		self.formatter.options_mut().set_rip_relative_addresses(value);
	}

	/// Show `NEAR`, `SHORT`, etc if it's a branch instruction
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `je short 1234h`
	/// &nbsp; | `false` | `je 1234h`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "showBranchSize")]
	pub fn show_branch_size(&self) -> bool {
		self.formatter.options().show_branch_size()
	}

	/// Show `NEAR`, `SHORT`, etc if it's a branch instruction
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `je short 1234h`
	/// &nbsp; | `false` | `je 1234h`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "showBranchSize")]
	pub fn set_show_branch_size(&mut self, value: bool) {
		self.formatter.options_mut().set_show_branch_size(value);
	}

	/// Use pseudo instructions
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `vcmpnltsd xmm2,xmm6,xmm3`
	/// &nbsp; | `false` | `vcmpsd xmm2,xmm6,xmm3,5`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "usePseudoOps")]
	pub fn use_pseudo_ops(&self) -> bool {
		self.formatter.options().use_pseudo_ops()
	}

	/// Use pseudo instructions
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `vcmpnltsd xmm2,xmm6,xmm3`
	/// &nbsp; | `false` | `vcmpsd xmm2,xmm6,xmm3,5`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "usePseudoOps")]
	pub fn set_use_pseudo_ops(&mut self, value: bool) {
		self.formatter.options_mut().set_use_pseudo_ops(value);
	}

	/// Show the original value after the symbol name
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[myfield (12345678)]`
	/// ✔️ | `false` | `mov eax,[myfield]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "showSymbolAddress")]
	pub fn show_symbol_address(&self) -> bool {
		self.formatter.options().show_symbol_address()
	}

	/// Show the original value after the symbol name
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,[myfield (12345678)]`
	/// ✔️ | `false` | `mov eax,[myfield]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "showSymbolAddress")]
	pub fn set_show_symbol_address(&mut self, value: bool) {
		self.formatter.options_mut().set_show_symbol_address(value);
	}

	/// (gas only): If `true`, the formatter doesn't add `%` to registers
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,ecx`
	/// ✔️ | `false` | `mov %eax,%ecx`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "gasNakedRegisters")]
	pub fn gas_naked_registers(&self) -> bool {
		self.formatter.options().gas_naked_registers()
	}

	/// (gas only): If `true`, the formatter doesn't add `%` to registers
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `mov eax,ecx`
	/// ✔️ | `false` | `mov %eax,%ecx`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "gasNakedRegisters")]
	pub fn set_gas_naked_registers(&mut self, value: bool) {
		self.formatter.options_mut().set_gas_naked_registers(value);
	}

	/// (gas only): Shows the mnemonic size suffix even when not needed
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `movl %eax,%ecx`
	/// ✔️ | `false` | `mov %eax,%ecx`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "gasShowMnemonicSizeSuffix")]
	pub fn gas_show_mnemonic_size_suffix(&self) -> bool {
		self.formatter.options().gas_show_mnemonic_size_suffix()
	}

	/// (gas only): Shows the mnemonic size suffix even when not needed
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `movl %eax,%ecx`
	/// ✔️ | `false` | `mov %eax,%ecx`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "gasShowMnemonicSizeSuffix")]
	pub fn set_gas_show_mnemonic_size_suffix(&mut self, value: bool) {
		self.formatter.options_mut().set_gas_show_mnemonic_size_suffix(value);
	}

	/// (gas only): Add a space after the comma if it's a memory operand
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `(%eax, %ecx, 2)`
	/// ✔️ | `false` | `(%eax,%ecx,2)`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "gasSpaceAfterMemoryOperandComma")]
	pub fn gas_space_after_memory_operand_comma(&self) -> bool {
		self.formatter.options().gas_space_after_memory_operand_comma()
	}

	/// (gas only): Add a space after the comma if it's a memory operand
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `(%eax, %ecx, 2)`
	/// ✔️ | `false` | `(%eax,%ecx,2)`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "gasSpaceAfterMemoryOperandComma")]
	pub fn set_gas_space_after_memory_operand_comma(&mut self, value: bool) {
		self.formatter.options_mut().set_gas_space_after_memory_operand_comma(value);
	}

	/// (masm only): Add a `DS` segment override even if it's not present. Used if it's 16/32-bit code and mem op is a displ
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `mov eax,ds:[12345678]`
	/// &nbsp; | `false` | `mov eax,[12345678]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "masmAddDsPrefix32")]
	pub fn masm_add_ds_prefix32(&self) -> bool {
		self.formatter.options().masm_add_ds_prefix32()
	}

	/// (masm only): Add a `DS` segment override even if it's not present. Used if it's 16/32-bit code and mem op is a displ
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `mov eax,ds:[12345678]`
	/// &nbsp; | `false` | `mov eax,[12345678]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "masmAddDsPrefix32")]
	pub fn set_masm_add_ds_prefix32(&mut self, value: bool) {
		self.formatter.options_mut().set_masm_add_ds_prefix32(value);
	}

	/// (masm only): Show symbols in brackets
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `[ecx+symbol]` / `[symbol]`
	/// &nbsp; | `false` | `symbol[ecx]` / `symbol`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "masmSymbolDisplInBrackets")]
	pub fn masm_symbol_displ_in_brackets(&self) -> bool {
		self.formatter.options().masm_symbol_displ_in_brackets()
	}

	/// (masm only): Show symbols in brackets
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `[ecx+symbol]` / `[symbol]`
	/// &nbsp; | `false` | `symbol[ecx]` / `symbol`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "masmSymbolDisplInBrackets")]
	pub fn set_masm_symbol_displ_in_brackets(&mut self, value: bool) {
		self.formatter.options_mut().set_masm_symbol_displ_in_brackets(value);
	}

	/// (masm only): Show displacements in brackets
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `[ecx+1234h]`
	/// &nbsp; | `false` | `1234h[ecx]`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "masmDisplInBrackets")]
	pub fn masm_displ_in_brackets(&self) -> bool {
		self.formatter.options().masm_displ_in_brackets()
	}

	/// (masm only): Show displacements in brackets
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// ✔️ | `true` | `[ecx+1234h]`
	/// &nbsp; | `false` | `1234h[ecx]`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "masmDisplInBrackets")]
	pub fn set_masm_displ_in_brackets(&mut self, value: bool) {
		self.formatter.options_mut().set_masm_displ_in_brackets(value);
	}

	/// (nasm only): Shows `BYTE`, `WORD`, `DWORD` or `QWORD` if it's a sign extended immediate operand value
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `or rcx,byte -1`
	/// ✔️ | `false` | `or rcx,-1`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "nasmShowSignExtendedImmediateSize")]
	pub fn nasm_show_sign_extended_immediate_size(&self) -> bool {
		self.formatter.options().nasm_show_sign_extended_immediate_size()
	}

	/// (nasm only): Shows `BYTE`, `WORD`, `DWORD` or `QWORD` if it's a sign extended immediate operand value
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `or rcx,byte -1`
	/// ✔️ | `false` | `or rcx,-1`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "nasmShowSignExtendedImmediateSize")]
	pub fn set_nasm_show_sign_extended_immediate_size(&mut self, value: bool) {
		self.formatter.options_mut().set_nasm_show_sign_extended_immediate_size(value);
	}

	/// Use `st(0)` instead of `st` if `st` can be used. Ignored by the nasm formatter.
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `fadd st(0),st(3)`
	/// ✔️ | `false` | `fadd st,st(3)`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "preferSt0")]
	pub fn prefer_st0(&self) -> bool {
		self.formatter.options().prefer_st0()
	}

	/// Use `st(0)` instead of `st` if `st` can be used. Ignored by the nasm formatter.
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `fadd st(0),st(3)`
	/// ✔️ | `false` | `fadd st,st(3)`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "preferSt0")]
	pub fn set_prefer_st0(&mut self, value: bool) {
		self.formatter.options_mut().set_prefer_st0(value);
	}

	/// Show useless prefixes. If it has useless prefixes, it could be data and not code.
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `es rep add eax,ecx`
	/// ✔️ | `false` | `add eax,ecx`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "showUselessPrefixes")]
	pub fn show_useless_prefixes(&self) -> bool {
		self.formatter.options().show_useless_prefixes()
	}

	/// Show useless prefixes. If it has useless prefixes, it could be data and not code.
	///
	/// Default | Value | Example
	/// --------|-------|--------
	/// &nbsp; | `true` | `es rep add eax,ecx`
	/// ✔️ | `false` | `add eax,ecx`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "showUselessPrefixes")]
	pub fn set_show_useless_prefixes(&mut self, value: bool) {
		self.formatter.options_mut().set_show_useless_prefixes(value)
	}

	/// Mnemonic condition code selector (eg. `JB` / `JC` / `JNAE`)
	///
	/// The value is a [`CC_b`] enum value.
	///
	/// [`CC_b`]: enum.CC_b.html
	///
	/// Default: `JB`, `CMOVB`, `SETB`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_b")]
	pub fn cc_b(&self) -> CC_b {
		iced_to_cc_b(self.formatter.options().cc_b())
	}

	/// Mnemonic condition code selector (eg. `JB` / `JC` / `JNAE`)
	///
	/// The value is a [`CC_b`] enum value.
	///
	/// [`CC_b`]: enum.CC_b.html
	///
	/// Default: `JB`, `CMOVB`, `SETB`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_b")]
	pub fn set_cc_b(&mut self, value: CC_b) {
		self.formatter.options_mut().set_cc_b(cc_b_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JAE` / `JNB` / `JNC`)
	///
	/// The value is a [`CC_ae`] enum value.
	///
	/// [`CC_ae`]: enum.CC_ae.html
	///
	/// Default: `JAE`, `CMOVAE`, `SETAE`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_ae")]
	pub fn cc_ae(&self) -> CC_ae {
		iced_to_cc_ae(self.formatter.options().cc_ae())
	}

	/// Mnemonic condition code selector (eg. `JAE` / `JNB` / `JNC`)
	///
	/// The value is a [`CC_ae`] enum value.
	///
	/// [`CC_ae`]: enum.CC_ae.html
	///
	/// Default: `JAE`, `CMOVAE`, `SETAE`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_ae")]
	pub fn set_cc_ae(&mut self, value: CC_ae) {
		self.formatter.options_mut().set_cc_ae(cc_ae_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JE` / `JZ`)
	///
	/// The value is a [`CC_e`] enum value.
	///
	/// [`CC_e`]: enum.CC_e.html
	///
	/// Default: `JE`, `CMOVE`, `SETE`, `LOOPE`, `REPE`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_e")]
	pub fn cc_e(&self) -> CC_e {
		iced_to_cc_e(self.formatter.options().cc_e())
	}

	/// Mnemonic condition code selector (eg. `JE` / `JZ`)
	///
	/// The value is a [`CC_e`] enum value.
	///
	/// [`CC_e`]: enum.CC_e.html
	///
	/// Default: `JE`, `CMOVE`, `SETE`, `LOOPE`, `REPE`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_e")]
	pub fn set_cc_e(&mut self, value: CC_e) {
		self.formatter.options_mut().set_cc_e(cc_e_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JNE` / `JNZ`)
	///
	/// The value is a [`CC_ne`] enum value.
	///
	/// [`CC_ne`]: enum.CC_ne.html
	///
	/// Default: `JNE`, `CMOVNE`, `SETNE`, `LOOPNE`, `REPNE`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_ne")]
	pub fn cc_ne(&self) -> CC_ne {
		iced_to_cc_ne(self.formatter.options().cc_ne())
	}

	/// Mnemonic condition code selector (eg. `JNE` / `JNZ`)
	///
	/// The value is a [`CC_ne`] enum value.
	///
	/// [`CC_ne`]: enum.CC_ne.html
	///
	/// Default: `JNE`, `CMOVNE`, `SETNE`, `LOOPNE`, `REPNE`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_ne")]
	pub fn set_cc_ne(&mut self, value: CC_ne) {
		self.formatter.options_mut().set_cc_ne(cc_ne_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JBE` / `JNA`)
	///
	/// The value is a [`CC_be`] enum value.
	///
	/// [`CC_be`]: enum.CC_be.html
	///
	/// Default: `JBE`, `CMOVBE`, `SETBE`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_be")]
	pub fn cc_be(&self) -> CC_be {
		iced_to_cc_be(self.formatter.options().cc_be())
	}

	/// Mnemonic condition code selector (eg. `JBE` / `JNA`)
	///
	/// The value is a [`CC_be`] enum value.
	///
	/// [`CC_be`]: enum.CC_be.html
	///
	/// Default: `JBE`, `CMOVBE`, `SETBE`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_be")]
	pub fn set_cc_be(&mut self, value: CC_be) {
		self.formatter.options_mut().set_cc_be(cc_be_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JA` / `JNBE`)
	///
	/// The value is a [`CC_a`] enum value.
	///
	/// [`CC_a`]: enum.CC_a.html
	///
	/// Default: `JA`, `CMOVA`, `SETA`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_a")]
	pub fn cc_a(&self) -> CC_a {
		iced_to_cc_a(self.formatter.options().cc_a())
	}

	/// Mnemonic condition code selector (eg. `JA` / `JNBE`)
	///
	/// The value is a [`CC_a`] enum value.
	///
	/// [`CC_a`]: enum.CC_a.html
	///
	/// Default: `JA`, `CMOVA`, `SETA`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_a")]
	pub fn set_cc_a(&mut self, value: CC_a) {
		self.formatter.options_mut().set_cc_a(cc_a_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JP` / `JPE`)
	///
	/// The value is a [`CC_p`] enum value.
	///
	/// [`CC_p`]: enum.CC_p.html
	///
	/// Default: `JP`, `CMOVP`, `SETP`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_p")]
	pub fn cc_p(&self) -> CC_p {
		iced_to_cc_p(self.formatter.options().cc_p())
	}

	/// Mnemonic condition code selector (eg. `JP` / `JPE`)
	///
	/// The value is a [`CC_p`] enum value.
	///
	/// [`CC_p`]: enum.CC_p.html
	///
	/// Default: `JP`, `CMOVP`, `SETP`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_p")]
	pub fn set_cc_p(&mut self, value: CC_p) {
		self.formatter.options_mut().set_cc_p(cc_p_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JNP` / `JPO`)
	///
	/// The value is a [`CC_np`] enum value.
	///
	/// [`CC_np`]: enum.CC_np.html
	///
	/// Default: `JNP`, `CMOVNP`, `SETNP`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_np")]
	pub fn cc_np(&self) -> CC_np {
		iced_to_cc_np(self.formatter.options().cc_np())
	}

	/// Mnemonic condition code selector (eg. `JNP` / `JPO`)
	///
	/// The value is a [`CC_np`] enum value.
	///
	/// [`CC_np`]: enum.CC_np.html
	///
	/// Default: `JNP`, `CMOVNP`, `SETNP`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_np")]
	pub fn set_cc_np(&mut self, value: CC_np) {
		self.formatter.options_mut().set_cc_np(cc_np_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JL` / `JNGE`)
	///
	/// The value is a [`CC_l`] enum value.
	///
	/// [`CC_l`]: enum.CC_l.html
	///
	/// Default: `JL`, `CMOVL`, `SETL`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_l")]
	pub fn cc_l(&self) -> CC_l {
		iced_to_cc_l(self.formatter.options().cc_l())
	}

	/// Mnemonic condition code selector (eg. `JL` / `JNGE`)
	///
	/// The value is a [`CC_l`] enum value.
	///
	/// [`CC_l`]: enum.CC_l.html
	///
	/// Default: `JL`, `CMOVL`, `SETL`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_l")]
	pub fn set_cc_l(&mut self, value: CC_l) {
		self.formatter.options_mut().set_cc_l(cc_l_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JGE` / `JNL`)
	///
	/// The value is a [`CC_ge`] enum value.
	///
	/// [`CC_ge`]: enum.CC_ge.html
	///
	/// Default: `JGE`, `CMOVGE`, `SETGE`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_ge")]
	pub fn cc_ge(&self) -> CC_ge {
		iced_to_cc_ge(self.formatter.options().cc_ge())
	}

	/// Mnemonic condition code selector (eg. `JGE` / `JNL`)
	///
	/// The value is a [`CC_ge`] enum value.
	///
	/// [`CC_ge`]: enum.CC_ge.html
	///
	/// Default: `JGE`, `CMOVGE`, `SETGE`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_ge")]
	pub fn set_cc_ge(&mut self, value: CC_ge) {
		self.formatter.options_mut().set_cc_ge(cc_ge_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JLE` / `JNG`)
	///
	/// The value is a [`CC_le`] enum value.
	///
	/// [`CC_le`]: enum.CC_le.html
	///
	/// Default: `JLE`, `CMOVLE`, `SETLE`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_le")]
	pub fn cc_le(&self) -> CC_le {
		iced_to_cc_le(self.formatter.options().cc_le())
	}

	/// Mnemonic condition code selector (eg. `JLE` / `JNG`)
	///
	/// The value is a [`CC_le`] enum value.
	///
	/// [`CC_le`]: enum.CC_le.html
	///
	/// Default: `JLE`, `CMOVLE`, `SETLE`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_le")]
	pub fn set_cc_le(&mut self, value: CC_le) {
		self.formatter.options_mut().set_cc_le(cc_le_to_iced(value));
	}

	/// Mnemonic condition code selector (eg. `JG` / `JNLE`)
	///
	/// The value is a [`CC_g`] enum value.
	///
	/// [`CC_g`]: enum.CC_g.html
	///
	/// Default: `JG`, `CMOVG`, `SETG`
	#[wasm_bindgen(getter)]
	#[wasm_bindgen(js_name = "cc_g")]
	pub fn cc_g(&self) -> CC_g {
		iced_to_cc_g(self.formatter.options().cc_g())
	}

	/// Mnemonic condition code selector (eg. `JG` / `JNLE`)
	///
	/// The value is a [`CC_g`] enum value.
	///
	/// [`CC_g`]: enum.CC_g.html
	///
	/// Default: `JG`, `CMOVG`, `SETG`
	///
	/// # Arguments
	///
	/// * `value`: New value
	#[wasm_bindgen(setter)]
	#[wasm_bindgen(js_name = "cc_g")]
	pub fn set_cc_g(&mut self, value: CC_g) {
		self.formatter.options_mut().set_cc_g(cc_g_to_iced(value));
	}
}
