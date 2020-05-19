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

#![allow(unused_results)]

use super::super::super::TupleType;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

lazy_static! {
	pub(super) static ref TO_TUPLE_TYPE_HASH: HashMap<&'static str, TupleType> = {
		// GENERATOR-BEGIN: TupleTypeHash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(34);
		h.insert("None", TupleType::None);
		h.insert("Full_128", TupleType::Full_128);
		h.insert("Full_256", TupleType::Full_256);
		h.insert("Full_512", TupleType::Full_512);
		h.insert("Half_128", TupleType::Half_128);
		h.insert("Half_256", TupleType::Half_256);
		h.insert("Half_512", TupleType::Half_512);
		h.insert("Full_Mem_128", TupleType::Full_Mem_128);
		h.insert("Full_Mem_256", TupleType::Full_Mem_256);
		h.insert("Full_Mem_512", TupleType::Full_Mem_512);
		h.insert("Tuple1_Scalar", TupleType::Tuple1_Scalar);
		h.insert("Tuple1_Scalar_1", TupleType::Tuple1_Scalar_1);
		h.insert("Tuple1_Scalar_2", TupleType::Tuple1_Scalar_2);
		h.insert("Tuple1_Scalar_4", TupleType::Tuple1_Scalar_4);
		h.insert("Tuple1_Scalar_8", TupleType::Tuple1_Scalar_8);
		h.insert("Tuple1_Fixed_4", TupleType::Tuple1_Fixed_4);
		h.insert("Tuple1_Fixed_8", TupleType::Tuple1_Fixed_8);
		h.insert("Tuple2", TupleType::Tuple2);
		h.insert("Tuple4", TupleType::Tuple4);
		h.insert("Tuple8", TupleType::Tuple8);
		h.insert("Tuple1_4X", TupleType::Tuple1_4X);
		h.insert("Half_Mem_128", TupleType::Half_Mem_128);
		h.insert("Half_Mem_256", TupleType::Half_Mem_256);
		h.insert("Half_Mem_512", TupleType::Half_Mem_512);
		h.insert("Quarter_Mem_128", TupleType::Quarter_Mem_128);
		h.insert("Quarter_Mem_256", TupleType::Quarter_Mem_256);
		h.insert("Quarter_Mem_512", TupleType::Quarter_Mem_512);
		h.insert("Eighth_Mem_128", TupleType::Eighth_Mem_128);
		h.insert("Eighth_Mem_256", TupleType::Eighth_Mem_256);
		h.insert("Eighth_Mem_512", TupleType::Eighth_Mem_512);
		h.insert("Mem128", TupleType::Mem128);
		h.insert("MOVDDUP_128", TupleType::MOVDDUP_128);
		h.insert("MOVDDUP_256", TupleType::MOVDDUP_256);
		h.insert("MOVDDUP_512", TupleType::MOVDDUP_512);
		// GENERATOR-END: TupleTypeHash
		h
	};
}
