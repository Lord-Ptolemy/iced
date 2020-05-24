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

use wasm_bindgen::prelude::*;

// GENERATOR-BEGIN: CC_b
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JB` / `JC` / `JNAE`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_b {
	/// `JB`, `CMOVB`, `SETB`
	b = 0,
	/// `JC`, `CMOVC`, `SETC`
	c = 1,
	/// `JNAE`, `CMOVNAE`, `SETNAE`
	nae = 2,
}
// GENERATOR-END: CC_b

// GENERATOR-BEGIN: CC_ae
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JAE` / `JNB` / `JNC`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_ae {
	/// `JAE`, `CMOVAE`, `SETAE`
	ae = 0,
	/// `JNB`, `CMOVNB`, `SETNB`
	nb = 1,
	/// `JNC`, `CMOVNC`, `SETNC`
	nc = 2,
}
// GENERATOR-END: CC_ae

// GENERATOR-BEGIN: CC_e
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JE` / `JZ`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_e {
	/// `JE`, `CMOVE`, `SETE`, `LOOPE`, `REPE`
	e = 0,
	/// `JZ`, `CMOVZ`, `SETZ`, `LOOPZ`, `REPZ`
	z = 1,
}
// GENERATOR-END: CC_e

// GENERATOR-BEGIN: CC_ne
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JNE` / `JNZ`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_ne {
	/// `JNE`, `CMOVNE`, `SETNE`, `LOOPNE`, `REPNE`
	ne = 0,
	/// `JNZ`, `CMOVNZ`, `SETNZ`, `LOOPNZ`, `REPNZ`
	nz = 1,
}
// GENERATOR-END: CC_ne

// GENERATOR-BEGIN: CC_be
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JBE` / `JNA`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_be {
	/// `JBE`, `CMOVBE`, `SETBE`
	be = 0,
	/// `JNA`, `CMOVNA`, `SETNA`
	na = 1,
}
// GENERATOR-END: CC_be

// GENERATOR-BEGIN: CC_a
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JA` / `JNBE`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_a {
	/// `JA`, `CMOVA`, `SETA`
	a = 0,
	/// `JNBE`, `CMOVNBE`, `SETNBE`
	nbe = 1,
}
// GENERATOR-END: CC_a

// GENERATOR-BEGIN: CC_p
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JP` / `JPE`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_p {
	/// `JP`, `CMOVP`, `SETP`
	p = 0,
	/// `JPE`, `CMOVPE`, `SETPE`
	pe = 1,
}
// GENERATOR-END: CC_p

// GENERATOR-BEGIN: CC_np
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JNP` / `JPO`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_np {
	/// `JNP`, `CMOVNP`, `SETNP`
	np = 0,
	/// `JPO`, `CMOVPO`, `SETPO`
	po = 1,
}
// GENERATOR-END: CC_np

// GENERATOR-BEGIN: CC_l
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JL` / `JNGE`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_l {
	/// `JL`, `CMOVL`, `SETL`
	l = 0,
	/// `JNGE`, `CMOVNGE`, `SETNGE`
	nge = 1,
}
// GENERATOR-END: CC_l

// GENERATOR-BEGIN: CC_ge
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JGE` / `JNL`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_ge {
	/// `JGE`, `CMOVGE`, `SETGE`
	ge = 0,
	/// `JNL`, `CMOVNL`, `SETNL`
	nl = 1,
}
// GENERATOR-END: CC_ge

// GENERATOR-BEGIN: CC_le
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JLE` / `JNG`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_le {
	/// `JLE`, `CMOVLE`, `SETLE`
	le = 0,
	/// `JNG`, `CMOVNG`, `SETNG`
	ng = 1,
}
// GENERATOR-END: CC_le

// GENERATOR-BEGIN: CC_g
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Mnemonic condition code selector (eg. `JG` / `JNLE`)
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CC_g {
	/// `JG`, `CMOVG`, `SETG`
	g = 0,
	/// `JNLE`, `CMOVNLE`, `SETNLE`
	nle = 1,
}
// GENERATOR-END: CC_g

#[allow(dead_code)]
pub(crate) fn cc_b_to_iced(value: CC_b) -> iced_x86_rust::CC_b {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_b(value: iced_x86_rust::CC_b) -> CC_b {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_ae_to_iced(value: CC_ae) -> iced_x86_rust::CC_ae {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_ae(value: iced_x86_rust::CC_ae) -> CC_ae {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_e_to_iced(value: CC_e) -> iced_x86_rust::CC_e {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_e(value: iced_x86_rust::CC_e) -> CC_e {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_ne_to_iced(value: CC_ne) -> iced_x86_rust::CC_ne {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_ne(value: iced_x86_rust::CC_ne) -> CC_ne {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_be_to_iced(value: CC_be) -> iced_x86_rust::CC_be {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_be(value: iced_x86_rust::CC_be) -> CC_be {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_a_to_iced(value: CC_a) -> iced_x86_rust::CC_a {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_a(value: iced_x86_rust::CC_a) -> CC_a {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_p_to_iced(value: CC_p) -> iced_x86_rust::CC_p {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_p(value: iced_x86_rust::CC_p) -> CC_p {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_np_to_iced(value: CC_np) -> iced_x86_rust::CC_np {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_np(value: iced_x86_rust::CC_np) -> CC_np {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_l_to_iced(value: CC_l) -> iced_x86_rust::CC_l {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_l(value: iced_x86_rust::CC_l) -> CC_l {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_ge_to_iced(value: CC_ge) -> iced_x86_rust::CC_ge {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_ge(value: iced_x86_rust::CC_ge) -> CC_ge {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_le_to_iced(value: CC_le) -> iced_x86_rust::CC_le {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_le(value: iced_x86_rust::CC_le) -> CC_le {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn cc_g_to_iced(value: CC_g) -> iced_x86_rust::CC_g {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}

#[allow(dead_code)]
pub(crate) fn iced_to_cc_g(value: iced_x86_rust::CC_g) -> CC_g {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}
