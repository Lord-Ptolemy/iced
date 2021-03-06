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

#if FAST_FMT
using Iced.Intel;
using Xunit;

namespace Iced.UnitTests.Intel.FormatterTests.Fast {
	public sealed class FastStringOutputTests {
		[Theory]
		[InlineData(-1)]
		[InlineData(0)]
		[InlineData(1)]
		[InlineData(1000)]
		void AppendChars(int capacity) {
			var output = capacity < 0 ? new FastStringOutput() : new FastStringOutput(capacity);
			Assert.Empty(output.ToString());
			output.Append('a');
			Assert.Equal("a", output.ToString());
			output.Append('b');
			Assert.Equal("ab", output.ToString());
			for (int i = 0; i < 1000; i++)
				output.Append('c');
			Assert.Equal("ab" + new string('c', 1000), output.ToString());
		}

		[Theory]
		[InlineData("")]
		[InlineData("q")]
		[InlineData("qw")]
		[InlineData("qwerty")]
		void AppendString(string s) {
			var capacities = new int[] { -1, 0, 1, 1000 };
			foreach (var capacity in capacities) {
				var output = capacity < 0 ? new FastStringOutput() : new FastStringOutput(capacity);
				Assert.Empty(output.ToString());
				output.Append(s);
				Assert.Equal(s, output.ToString());
				output.Append("abc");
				Assert.Equal(s + "abc", output.ToString());
				for (int i = 0; i < 200; i++)
					output.Append("x");
				Assert.Equal(s + "abc" + new string('x', 200), output.ToString());
				for (int i = 0; i < 200; i++)
					output.Append("yy");
				Assert.Equal(s + "abc" + new string('x', 200) + new string('y', 400), output.ToString());
			}
		}

		[Fact]
		void AppendNullWorks() {
			var output = new FastStringOutput();
			Assert.Empty(output.ToString());
			output.Append(null);
			Assert.Empty(output.ToString());
			output.Append('a');
			Assert.Equal("a", output.ToString());
			output.Append(null);
			Assert.Equal("a", output.ToString());
		}

		[Fact]
		void ClearWorks() {
			var output = new FastStringOutput();
			Assert.Empty(output.ToString());
			output.Append('a');
			Assert.Equal("a", output.ToString());
			output.Clear();
			Assert.Empty(output.ToString());
			output.Append("abc");
			Assert.Equal("abc", output.ToString());
			output.Clear();
			Assert.Empty(output.ToString());
		}

		[Fact]
		void LengthWorks() {
			var output = new FastStringOutput();
			Assert.Equal(0, output.Length);
			output.Append('a');
			Assert.Equal(1, output.Length);
			output.Append('b');
			Assert.Equal(2, output.Length);
			output.Append("cde");
			Assert.Equal(5, output.Length);
			output.Clear();
			Assert.Equal(0, output.Length);
			output.Append("abc");
			Assert.Equal(3, output.Length);
			output.Clear();
			Assert.Equal(0, output.Length);
		}

#if HAS_SPAN
		[Fact]
		void AsSpanWorks() {
			var output = new FastStringOutput();
			Assert.Empty(output.AsSpan().ToString());
			output.Append('a');
			Assert.Equal("a", output.AsSpan().ToString());
			output.Append('b');
			Assert.Equal("ab", output.AsSpan().ToString());
			for (int i = 0; i < 1000; i++)
				output.Append('c');
			Assert.Equal("ab" + new string('c', 1000), output.AsSpan().ToString());
			output.Clear();
			Assert.Empty(output.AsSpan().ToString());
		}
#endif

		[Theory]
		[InlineData(0, "", "abcdefgh")]
		[InlineData(8, "", "abcdefgh")]
		[InlineData(0, "x", "xbcdefgh")]
		[InlineData(3, "x", "abcxefgh")]
		[InlineData(7, "x", "abcdefgx")]
		[InlineData(0, "xyz", "xyzdefgh")]
		[InlineData(3, "xyz", "abcxyzgh")]
		[InlineData(5, "xyz", "abcdexyz")]
		void CopyToWorks(int index, string data, string expected) {
			var charData = "abcdefgh".ToCharArray();
			var output = new FastStringOutput();
			output.Append(data);
			output.CopyTo(charData, index);
			var s = new string(charData);
			Assert.Equal(expected, s);
		}
	}
}
#endif
