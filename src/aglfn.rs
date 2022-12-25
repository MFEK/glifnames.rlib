//! ```plain
//! -----------------------------------------------------------
//! Copyright 2002-2019 Adobe (http://www.adobe.com/).
//! 
//! Redistribution and use in source and binary forms, with or
//! without modification, are permitted provided that the
//! following conditions are met:
//! 
//! Redistributions of source code must retain the above
//! copyright notice, this list of conditions and the following
//! disclaimer.
//! 
//! Redistributions in binary form must reproduce the above
//! copyright notice, this list of conditions and the following
//! disclaimer in the documentation and/or other materials
//! provided with the distribution.
//! 
//! Neither the name of Adobe nor the names of its contributors
//! may be used to endorse or promote products derived from this
//! software without specific prior written permission.
//! 
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
//! CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//! INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
//! MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
//! CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//! SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
//! NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
//! LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
//! HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
//! CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
//! OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
//! SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//! ```
/// ```plain
/// -----------------------------------------------------------
/// Name:          Adobe Glyph List For New Fonts
/// Table version: 1.7
/// Date:          November 6, 2008
/// URL:           https://github.com/adobe-type-tools/agl-aglfn
/// 
/// Description:
/// 
/// AGLFN (Adobe Glyph List For New Fonts) provides a list of base glyph
/// names that are recommended for new fonts, which are compatible with
/// the AGL (Adobe Glyph List) Specification, and which should be used
/// as described in Section 6 of that document. AGLFN comprises the set
/// of glyph names from AGL that map via the AGL Specification rules to
/// the semantically correct UV (Unicode Value). For example, "Asmall"
/// is omitted because AGL maps this glyph name to the PUA (Private Use
/// Area) value U+F761, rather than to the UV that maps from the glyph
/// name "A." Also omitted is "ffi," because AGL maps this to the
/// Alphabetic Presentation Forms value U+FB03, rather than decomposing
/// it into the following sequence of three UVs: U+0066, U+0066, and
/// U+0069. The name "arrowvertex" has been omitted because this glyph
/// now has a real UV, and AGL is now incorrect in mapping it to the PUA
/// value U+F8E6. If you do not find an appropriate name for your glyph
/// in this list, then please refer to Section 6 of the AGL
/// Specification.
/// 
/// Format: three semicolon-delimited fields:
///   (1) Standard UV or CUS UV--four uppercase hexadecimal digits
///   (2) Glyph name--upper/lowercase letters and digits
///   (3) Character names: Unicode character names for standard UVs, and
///       descriptive names for CUS UVs--uppercase letters, hyphen, and
///       space
/// 
/// The records are sorted by glyph name in increasing ASCII order,
/// entries with the same glyph name are sorted in decreasing priority
/// order, the UVs and Unicode character names are provided for
/// convenience, lines starting with "#" are comments, and blank lines
/// should be ignored.
/// 
/// Revision History:
/// 
/// 1.7 [6 November 2008]
/// - Reverted to the original 1.4 and earlier mappings for Delta,
///   Omega, and mu.
/// - Removed mappings for "afii" names. These should now be assigned
///   "uni" names.
/// - Removed mappings for "commaaccent" names. These should now be
///   assigned "uni" names.
/// 
/// 1.6 [30 January 2006]
/// - Completed work intended in 1.5.
/// 
/// 1.5 [23 November 2005]
/// - Removed duplicated block at end of file.
/// - Changed mappings:
///   2206;Delta;INCREMENT changed to 0394;Delta;GREEK CAPITAL LETTER DELTA
///   2126;Omega;OHM SIGN changed to 03A9;Omega;GREEK CAPITAL LETTER OMEGA
///   03BC;mu;MICRO SIGN changed to 03BC;mu;GREEK SMALL LETTER MU
/// - Corrected statement above about why "ffi" is omitted.
/// 
/// 1.4 [24 September 2003]
/// - Changed version to 1.4, to avoid confusion with the AGL 1.3.
/// - Fixed spelling errors in the header.
/// - Fully removed "arrowvertex," as it is mapped only to a PUA Unicode
///   value in some fonts.
/// 
/// 1.1 [17 April 2003]
/// - Renamed [Tt]cedilla back to [Tt]commaaccent.
/// 
/// 1.0 [31 January 2003]
/// - Original version.
/// - Derived from the AGLv1.2 by:
///   removing the PUA area codes;
///   removing duplicate Unicode mappings; and
///   renaming "tcommaaccent" to "tcedilla" and "Tcommaaccent" to "Tcedilla"
/// 
/// ```

/// Adobe Glyph List for New Fonts (2019 edition)
///
pub struct AdobeGlyphListForNewFonts;

use std::borrow::Cow;
use super::GlyphNameOpt;

fn glyph_name_opt<'a>(c: char) -> Option<Cow<'a, str>> {
    match c as u32 {
        0x0020 => Some(Cow::from("space")),                // SPACE
        0x0021 => Some(Cow::from("exclam")),               // EXCLAMATION MARK
        0x0022 => Some(Cow::from("quotedbl")),             // QUOTATION MARK
        0x0023 => Some(Cow::from("numbersign")),           // NUMBER SIGN
        0x0024 => Some(Cow::from("dollar")),               // DOLLAR SIGN
        0x0025 => Some(Cow::from("percent")),              // PERCENT SIGN
        0x0026 => Some(Cow::from("ampersand")),            // AMPERSAND
        0x0027 => Some(Cow::from("quotesingle")),          // APOSTROPHE
        0x0028 => Some(Cow::from("parenleft")),            // LEFT PARENTHESIS
        0x0029 => Some(Cow::from("parenright")),           // RIGHT PARENTHESIS
        0x002A => Some(Cow::from("asterisk")),             // ASTERISK
        0x002B => Some(Cow::from("plus")),                 // PLUS SIGN
        0x002C => Some(Cow::from("comma")),                // COMMA
        0x002D => Some(Cow::from("hyphen")),               // HYPHEN-MINUS
        0x002E => Some(Cow::from("period")),               // FULL STOP
        0x002F => Some(Cow::from("slash")),                // SOLIDUS
        0x0030 => Some(Cow::from("zero")),                 // DIGIT ZERO
        0x0031 => Some(Cow::from("one")),                  // DIGIT ONE
        0x0032 => Some(Cow::from("two")),                  // DIGIT TWO
        0x0033 => Some(Cow::from("three")),                // DIGIT THREE
        0x0034 => Some(Cow::from("four")),                 // DIGIT FOUR
        0x0035 => Some(Cow::from("five")),                 // DIGIT FIVE
        0x0036 => Some(Cow::from("six")),                  // DIGIT SIX
        0x0037 => Some(Cow::from("seven")),                // DIGIT SEVEN
        0x0038 => Some(Cow::from("eight")),                // DIGIT EIGHT
        0x0039 => Some(Cow::from("nine")),                 // DIGIT NINE
        0x003A => Some(Cow::from("colon")),                // COLON
        0x003B => Some(Cow::from("semicolon")),            // SEMICOLON
        0x003C => Some(Cow::from("less")),                 // LESS-THAN SIGN
        0x003D => Some(Cow::from("equal")),                // EQUALS SIGN
        0x003E => Some(Cow::from("greater")),              // GREATER-THAN SIGN
        0x003F => Some(Cow::from("question")),             // QUESTION MARK
        0x0040 => Some(Cow::from("at")),                   // COMMERCIAL AT
        0x0041 => Some(Cow::from("A")),                    // LATIN CAPITAL LETTER A
        0x0042 => Some(Cow::from("B")),                    // LATIN CAPITAL LETTER B
        0x0043 => Some(Cow::from("C")),                    // LATIN CAPITAL LETTER C
        0x0044 => Some(Cow::from("D")),                    // LATIN CAPITAL LETTER D
        0x0045 => Some(Cow::from("E")),                    // LATIN CAPITAL LETTER E
        0x0046 => Some(Cow::from("F")),                    // LATIN CAPITAL LETTER F
        0x0047 => Some(Cow::from("G")),                    // LATIN CAPITAL LETTER G
        0x0048 => Some(Cow::from("H")),                    // LATIN CAPITAL LETTER H
        0x0049 => Some(Cow::from("I")),                    // LATIN CAPITAL LETTER I
        0x004A => Some(Cow::from("J")),                    // LATIN CAPITAL LETTER J
        0x004B => Some(Cow::from("K")),                    // LATIN CAPITAL LETTER K
        0x004C => Some(Cow::from("L")),                    // LATIN CAPITAL LETTER L
        0x004D => Some(Cow::from("M")),                    // LATIN CAPITAL LETTER M
        0x004E => Some(Cow::from("N")),                    // LATIN CAPITAL LETTER N
        0x004F => Some(Cow::from("O")),                    // LATIN CAPITAL LETTER O
        0x0050 => Some(Cow::from("P")),                    // LATIN CAPITAL LETTER P
        0x0051 => Some(Cow::from("Q")),                    // LATIN CAPITAL LETTER Q
        0x0052 => Some(Cow::from("R")),                    // LATIN CAPITAL LETTER R
        0x0053 => Some(Cow::from("S")),                    // LATIN CAPITAL LETTER S
        0x0054 => Some(Cow::from("T")),                    // LATIN CAPITAL LETTER T
        0x0055 => Some(Cow::from("U")),                    // LATIN CAPITAL LETTER U
        0x0056 => Some(Cow::from("V")),                    // LATIN CAPITAL LETTER V
        0x0057 => Some(Cow::from("W")),                    // LATIN CAPITAL LETTER W
        0x0058 => Some(Cow::from("X")),                    // LATIN CAPITAL LETTER X
        0x0059 => Some(Cow::from("Y")),                    // LATIN CAPITAL LETTER Y
        0x005A => Some(Cow::from("Z")),                    // LATIN CAPITAL LETTER Z
        0x005B => Some(Cow::from("bracketleft")),          // LEFT SQUARE BRACKET
        0x005C => Some(Cow::from("backslash")),            // REVERSE SOLIDUS
        0x005D => Some(Cow::from("bracketright")),         // RIGHT SQUARE BRACKET
        0x005E => Some(Cow::from("asciicircum")),          // CIRCUMFLEX ACCENT
        0x005F => Some(Cow::from("underscore")),           // LOW LINE
        0x0060 => Some(Cow::from("grave")),                // GRAVE ACCENT
        0x0061 => Some(Cow::from("a")),                    // LATIN SMALL LETTER A
        0x0062 => Some(Cow::from("b")),                    // LATIN SMALL LETTER B
        0x0063 => Some(Cow::from("c")),                    // LATIN SMALL LETTER C
        0x0064 => Some(Cow::from("d")),                    // LATIN SMALL LETTER D
        0x0065 => Some(Cow::from("e")),                    // LATIN SMALL LETTER E
        0x0066 => Some(Cow::from("f")),                    // LATIN SMALL LETTER F
        0x0067 => Some(Cow::from("g")),                    // LATIN SMALL LETTER G
        0x0068 => Some(Cow::from("h")),                    // LATIN SMALL LETTER H
        0x0069 => Some(Cow::from("i")),                    // LATIN SMALL LETTER I
        0x006A => Some(Cow::from("j")),                    // LATIN SMALL LETTER J
        0x006B => Some(Cow::from("k")),                    // LATIN SMALL LETTER K
        0x006C => Some(Cow::from("l")),                    // LATIN SMALL LETTER L
        0x006D => Some(Cow::from("m")),                    // LATIN SMALL LETTER M
        0x006E => Some(Cow::from("n")),                    // LATIN SMALL LETTER N
        0x006F => Some(Cow::from("o")),                    // LATIN SMALL LETTER O
        0x0070 => Some(Cow::from("p")),                    // LATIN SMALL LETTER P
        0x0071 => Some(Cow::from("q")),                    // LATIN SMALL LETTER Q
        0x0072 => Some(Cow::from("r")),                    // LATIN SMALL LETTER R
        0x0073 => Some(Cow::from("s")),                    // LATIN SMALL LETTER S
        0x0074 => Some(Cow::from("t")),                    // LATIN SMALL LETTER T
        0x0075 => Some(Cow::from("u")),                    // LATIN SMALL LETTER U
        0x0076 => Some(Cow::from("v")),                    // LATIN SMALL LETTER V
        0x0077 => Some(Cow::from("w")),                    // LATIN SMALL LETTER W
        0x0078 => Some(Cow::from("x")),                    // LATIN SMALL LETTER X
        0x0079 => Some(Cow::from("y")),                    // LATIN SMALL LETTER Y
        0x007A => Some(Cow::from("z")),                    // LATIN SMALL LETTER Z
        0x007B => Some(Cow::from("braceleft")),            // LEFT CURLY BRACKET
        0x007C => Some(Cow::from("bar")),                  // VERTICAL LINE
        0x007D => Some(Cow::from("braceright")),           // RIGHT CURLY BRACKET
        0x007E => Some(Cow::from("asciitilde")),           // TILDE
        0x00A1 => Some(Cow::from("exclamdown")),           // INVERTED EXCLAMATION MARK
        0x00A2 => Some(Cow::from("cent")),                 // CENT SIGN
        0x00A3 => Some(Cow::from("sterling")),             // POUND SIGN
        0x00A4 => Some(Cow::from("currency")),             // CURRENCY SIGN
        0x00A5 => Some(Cow::from("yen")),                  // YEN SIGN
        0x00A6 => Some(Cow::from("brokenbar")),            // BROKEN BAR
        0x00A7 => Some(Cow::from("section")),              // SECTION SIGN
        0x00A8 => Some(Cow::from("dieresis")),             // DIAERESIS
        0x00A9 => Some(Cow::from("copyright")),            // COPYRIGHT SIGN
        0x00AA => Some(Cow::from("ordfeminine")),          // FEMININE ORDINAL INDICATOR
        0x00AB => Some(Cow::from("guillemotleft")),        // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
        0x00AC => Some(Cow::from("logicalnot")),           // NOT SIGN
        0x00AE => Some(Cow::from("registered")),           // REGISTERED SIGN
        0x00AF => Some(Cow::from("macron")),               // MACRON
        0x00B0 => Some(Cow::from("degree")),               // DEGREE SIGN
        0x00B1 => Some(Cow::from("plusminus")),            // PLUS-MINUS SIGN
        0x00B4 => Some(Cow::from("acute")),                // ACUTE ACCENT
        0x00B5 => Some(Cow::from("mu")),                   // MICRO SIGN
        0x00B6 => Some(Cow::from("paragraph")),            // PILCROW SIGN
        0x00B7 => Some(Cow::from("periodcentered")),       // MIDDLE DOT
        0x00B8 => Some(Cow::from("cedilla")),              // CEDILLA
        0x00BA => Some(Cow::from("ordmasculine")),         // MASCULINE ORDINAL INDICATOR
        0x00BB => Some(Cow::from("guillemotright")),       // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
        0x00BC => Some(Cow::from("onequarter")),           // VULGAR FRACTION ONE QUARTER
        0x00BD => Some(Cow::from("onehalf")),              // VULGAR FRACTION ONE HALF
        0x00BE => Some(Cow::from("threequarters")),        // VULGAR FRACTION THREE QUARTERS
        0x00BF => Some(Cow::from("questiondown")),         // INVERTED QUESTION MARK
        0x00C0 => Some(Cow::from("Agrave")),               // LATIN CAPITAL LETTER A WITH GRAVE
        0x00C1 => Some(Cow::from("Aacute")),               // LATIN CAPITAL LETTER A WITH ACUTE
        0x00C2 => Some(Cow::from("Acircumflex")),          // LATIN CAPITAL LETTER A WITH CIRCUMFLEX
        0x00C3 => Some(Cow::from("Atilde")),               // LATIN CAPITAL LETTER A WITH TILDE
        0x00C4 => Some(Cow::from("Adieresis")),            // LATIN CAPITAL LETTER A WITH DIAERESIS
        0x00C5 => Some(Cow::from("Aring")),                // LATIN CAPITAL LETTER A WITH RING ABOVE
        0x00C6 => Some(Cow::from("AE")),                   // LATIN CAPITAL LETTER AE
        0x00C7 => Some(Cow::from("Ccedilla")),             // LATIN CAPITAL LETTER C WITH CEDILLA
        0x00C8 => Some(Cow::from("Egrave")),               // LATIN CAPITAL LETTER E WITH GRAVE
        0x00C9 => Some(Cow::from("Eacute")),               // LATIN CAPITAL LETTER E WITH ACUTE
        0x00CA => Some(Cow::from("Ecircumflex")),          // LATIN CAPITAL LETTER E WITH CIRCUMFLEX
        0x00CB => Some(Cow::from("Edieresis")),            // LATIN CAPITAL LETTER E WITH DIAERESIS
        0x00CC => Some(Cow::from("Igrave")),               // LATIN CAPITAL LETTER I WITH GRAVE
        0x00CD => Some(Cow::from("Iacute")),               // LATIN CAPITAL LETTER I WITH ACUTE
        0x00CE => Some(Cow::from("Icircumflex")),          // LATIN CAPITAL LETTER I WITH CIRCUMFLEX
        0x00CF => Some(Cow::from("Idieresis")),            // LATIN CAPITAL LETTER I WITH DIAERESIS
        0x00D0 => Some(Cow::from("Eth")),                  // LATIN CAPITAL LETTER ETH
        0x00D1 => Some(Cow::from("Ntilde")),               // LATIN CAPITAL LETTER N WITH TILDE
        0x00D2 => Some(Cow::from("Ograve")),               // LATIN CAPITAL LETTER O WITH GRAVE
        0x00D3 => Some(Cow::from("Oacute")),               // LATIN CAPITAL LETTER O WITH ACUTE
        0x00D4 => Some(Cow::from("Ocircumflex")),          // LATIN CAPITAL LETTER O WITH CIRCUMFLEX
        0x00D5 => Some(Cow::from("Otilde")),               // LATIN CAPITAL LETTER O WITH TILDE
        0x00D6 => Some(Cow::from("Odieresis")),            // LATIN CAPITAL LETTER O WITH DIAERESIS
        0x00D7 => Some(Cow::from("multiply")),             // MULTIPLICATION SIGN
        0x00D8 => Some(Cow::from("Oslash")),               // LATIN CAPITAL LETTER O WITH STROKE
        0x00D9 => Some(Cow::from("Ugrave")),               // LATIN CAPITAL LETTER U WITH GRAVE
        0x00DA => Some(Cow::from("Uacute")),               // LATIN CAPITAL LETTER U WITH ACUTE
        0x00DB => Some(Cow::from("Ucircumflex")),          // LATIN CAPITAL LETTER U WITH CIRCUMFLEX
        0x00DC => Some(Cow::from("Udieresis")),            // LATIN CAPITAL LETTER U WITH DIAERESIS
        0x00DD => Some(Cow::from("Yacute")),               // LATIN CAPITAL LETTER Y WITH ACUTE
        0x00DE => Some(Cow::from("Thorn")),                // LATIN CAPITAL LETTER THORN
        0x00DF => Some(Cow::from("germandbls")),           // LATIN SMALL LETTER SHARP S
        0x00E0 => Some(Cow::from("agrave")),               // LATIN SMALL LETTER A WITH GRAVE
        0x00E1 => Some(Cow::from("aacute")),               // LATIN SMALL LETTER A WITH ACUTE
        0x00E2 => Some(Cow::from("acircumflex")),          // LATIN SMALL LETTER A WITH CIRCUMFLEX
        0x00E3 => Some(Cow::from("atilde")),               // LATIN SMALL LETTER A WITH TILDE
        0x00E4 => Some(Cow::from("adieresis")),            // LATIN SMALL LETTER A WITH DIAERESIS
        0x00E5 => Some(Cow::from("aring")),                // LATIN SMALL LETTER A WITH RING ABOVE
        0x00E6 => Some(Cow::from("ae")),                   // LATIN SMALL LETTER AE
        0x00E7 => Some(Cow::from("ccedilla")),             // LATIN SMALL LETTER C WITH CEDILLA
        0x00E8 => Some(Cow::from("egrave")),               // LATIN SMALL LETTER E WITH GRAVE
        0x00E9 => Some(Cow::from("eacute")),               // LATIN SMALL LETTER E WITH ACUTE
        0x00EA => Some(Cow::from("ecircumflex")),          // LATIN SMALL LETTER E WITH CIRCUMFLEX
        0x00EB => Some(Cow::from("edieresis")),            // LATIN SMALL LETTER E WITH DIAERESIS
        0x00EC => Some(Cow::from("igrave")),               // LATIN SMALL LETTER I WITH GRAVE
        0x00ED => Some(Cow::from("iacute")),               // LATIN SMALL LETTER I WITH ACUTE
        0x00EE => Some(Cow::from("icircumflex")),          // LATIN SMALL LETTER I WITH CIRCUMFLEX
        0x00EF => Some(Cow::from("idieresis")),            // LATIN SMALL LETTER I WITH DIAERESIS
        0x00F0 => Some(Cow::from("eth")),                  // LATIN SMALL LETTER ETH
        0x00F1 => Some(Cow::from("ntilde")),               // LATIN SMALL LETTER N WITH TILDE
        0x00F2 => Some(Cow::from("ograve")),               // LATIN SMALL LETTER O WITH GRAVE
        0x00F3 => Some(Cow::from("oacute")),               // LATIN SMALL LETTER O WITH ACUTE
        0x00F4 => Some(Cow::from("ocircumflex")),          // LATIN SMALL LETTER O WITH CIRCUMFLEX
        0x00F5 => Some(Cow::from("otilde")),               // LATIN SMALL LETTER O WITH TILDE
        0x00F6 => Some(Cow::from("odieresis")),            // LATIN SMALL LETTER O WITH DIAERESIS
        0x00F7 => Some(Cow::from("divide")),               // DIVISION SIGN
        0x00F8 => Some(Cow::from("oslash")),               // LATIN SMALL LETTER O WITH STROKE
        0x00F9 => Some(Cow::from("ugrave")),               // LATIN SMALL LETTER U WITH GRAVE
        0x00FA => Some(Cow::from("uacute")),               // LATIN SMALL LETTER U WITH ACUTE
        0x00FB => Some(Cow::from("ucircumflex")),          // LATIN SMALL LETTER U WITH CIRCUMFLEX
        0x00FC => Some(Cow::from("udieresis")),            // LATIN SMALL LETTER U WITH DIAERESIS
        0x00FD => Some(Cow::from("yacute")),               // LATIN SMALL LETTER Y WITH ACUTE
        0x00FE => Some(Cow::from("thorn")),                // LATIN SMALL LETTER THORN
        0x00FF => Some(Cow::from("ydieresis")),            // LATIN SMALL LETTER Y WITH DIAERESIS
        0x0100 => Some(Cow::from("Amacron")),              // LATIN CAPITAL LETTER A WITH MACRON
        0x0101 => Some(Cow::from("amacron")),              // LATIN SMALL LETTER A WITH MACRON
        0x0102 => Some(Cow::from("Abreve")),               // LATIN CAPITAL LETTER A WITH BREVE
        0x0103 => Some(Cow::from("abreve")),               // LATIN SMALL LETTER A WITH BREVE
        0x0104 => Some(Cow::from("Aogonek")),              // LATIN CAPITAL LETTER A WITH OGONEK
        0x0105 => Some(Cow::from("aogonek")),              // LATIN SMALL LETTER A WITH OGONEK
        0x0106 => Some(Cow::from("Cacute")),               // LATIN CAPITAL LETTER C WITH ACUTE
        0x0107 => Some(Cow::from("cacute")),               // LATIN SMALL LETTER C WITH ACUTE
        0x0108 => Some(Cow::from("Ccircumflex")),          // LATIN CAPITAL LETTER C WITH CIRCUMFLEX
        0x0109 => Some(Cow::from("ccircumflex")),          // LATIN SMALL LETTER C WITH CIRCUMFLEX
        0x010A => Some(Cow::from("Cdotaccent")),           // LATIN CAPITAL LETTER C WITH DOT ABOVE
        0x010B => Some(Cow::from("cdotaccent")),           // LATIN SMALL LETTER C WITH DOT ABOVE
        0x010C => Some(Cow::from("Ccaron")),               // LATIN CAPITAL LETTER C WITH CARON
        0x010D => Some(Cow::from("ccaron")),               // LATIN SMALL LETTER C WITH CARON
        0x010E => Some(Cow::from("Dcaron")),               // LATIN CAPITAL LETTER D WITH CARON
        0x010F => Some(Cow::from("dcaron")),               // LATIN SMALL LETTER D WITH CARON
        0x0110 => Some(Cow::from("Dcroat")),               // LATIN CAPITAL LETTER D WITH STROKE
        0x0111 => Some(Cow::from("dcroat")),               // LATIN SMALL LETTER D WITH STROKE
        0x0112 => Some(Cow::from("Emacron")),              // LATIN CAPITAL LETTER E WITH MACRON
        0x0113 => Some(Cow::from("emacron")),              // LATIN SMALL LETTER E WITH MACRON
        0x0114 => Some(Cow::from("Ebreve")),               // LATIN CAPITAL LETTER E WITH BREVE
        0x0115 => Some(Cow::from("ebreve")),               // LATIN SMALL LETTER E WITH BREVE
        0x0116 => Some(Cow::from("Edotaccent")),           // LATIN CAPITAL LETTER E WITH DOT ABOVE
        0x0117 => Some(Cow::from("edotaccent")),           // LATIN SMALL LETTER E WITH DOT ABOVE
        0x0118 => Some(Cow::from("Eogonek")),              // LATIN CAPITAL LETTER E WITH OGONEK
        0x0119 => Some(Cow::from("eogonek")),              // LATIN SMALL LETTER E WITH OGONEK
        0x011A => Some(Cow::from("Ecaron")),               // LATIN CAPITAL LETTER E WITH CARON
        0x011B => Some(Cow::from("ecaron")),               // LATIN SMALL LETTER E WITH CARON
        0x011C => Some(Cow::from("Gcircumflex")),          // LATIN CAPITAL LETTER G WITH CIRCUMFLEX
        0x011D => Some(Cow::from("gcircumflex")),          // LATIN SMALL LETTER G WITH CIRCUMFLEX
        0x011E => Some(Cow::from("Gbreve")),               // LATIN CAPITAL LETTER G WITH BREVE
        0x011F => Some(Cow::from("gbreve")),               // LATIN SMALL LETTER G WITH BREVE
        0x0120 => Some(Cow::from("Gdotaccent")),           // LATIN CAPITAL LETTER G WITH DOT ABOVE
        0x0121 => Some(Cow::from("gdotaccent")),           // LATIN SMALL LETTER G WITH DOT ABOVE
        0x0124 => Some(Cow::from("Hcircumflex")),          // LATIN CAPITAL LETTER H WITH CIRCUMFLEX
        0x0125 => Some(Cow::from("hcircumflex")),          // LATIN SMALL LETTER H WITH CIRCUMFLEX
        0x0126 => Some(Cow::from("Hbar")),                 // LATIN CAPITAL LETTER H WITH STROKE
        0x0127 => Some(Cow::from("hbar")),                 // LATIN SMALL LETTER H WITH STROKE
        0x0128 => Some(Cow::from("Itilde")),               // LATIN CAPITAL LETTER I WITH TILDE
        0x0129 => Some(Cow::from("itilde")),               // LATIN SMALL LETTER I WITH TILDE
        0x012A => Some(Cow::from("Imacron")),              // LATIN CAPITAL LETTER I WITH MACRON
        0x012B => Some(Cow::from("imacron")),              // LATIN SMALL LETTER I WITH MACRON
        0x012C => Some(Cow::from("Ibreve")),               // LATIN CAPITAL LETTER I WITH BREVE
        0x012D => Some(Cow::from("ibreve")),               // LATIN SMALL LETTER I WITH BREVE
        0x012E => Some(Cow::from("Iogonek")),              // LATIN CAPITAL LETTER I WITH OGONEK
        0x012F => Some(Cow::from("iogonek")),              // LATIN SMALL LETTER I WITH OGONEK
        0x0130 => Some(Cow::from("Idotaccent")),           // LATIN CAPITAL LETTER I WITH DOT ABOVE
        0x0131 => Some(Cow::from("dotlessi")),             // LATIN SMALL LETTER DOTLESS I
        0x0132 => Some(Cow::from("IJ")),                   // LATIN CAPITAL LIGATURE IJ
        0x0133 => Some(Cow::from("ij")),                   // LATIN SMALL LIGATURE IJ
        0x0134 => Some(Cow::from("Jcircumflex")),          // LATIN CAPITAL LETTER J WITH CIRCUMFLEX
        0x0135 => Some(Cow::from("jcircumflex")),          // LATIN SMALL LETTER J WITH CIRCUMFLEX
        0x0138 => Some(Cow::from("kgreenlandic")),         // LATIN SMALL LETTER KRA
        0x0139 => Some(Cow::from("Lacute")),               // LATIN CAPITAL LETTER L WITH ACUTE
        0x013A => Some(Cow::from("lacute")),               // LATIN SMALL LETTER L WITH ACUTE
        0x013D => Some(Cow::from("Lcaron")),               // LATIN CAPITAL LETTER L WITH CARON
        0x013E => Some(Cow::from("lcaron")),               // LATIN SMALL LETTER L WITH CARON
        0x013F => Some(Cow::from("Ldot")),                 // LATIN CAPITAL LETTER L WITH MIDDLE DOT
        0x0140 => Some(Cow::from("ldot")),                 // LATIN SMALL LETTER L WITH MIDDLE DOT
        0x0141 => Some(Cow::from("Lslash")),               // LATIN CAPITAL LETTER L WITH STROKE
        0x0142 => Some(Cow::from("lslash")),               // LATIN SMALL LETTER L WITH STROKE
        0x0143 => Some(Cow::from("Nacute")),               // LATIN CAPITAL LETTER N WITH ACUTE
        0x0144 => Some(Cow::from("nacute")),               // LATIN SMALL LETTER N WITH ACUTE
        0x0147 => Some(Cow::from("Ncaron")),               // LATIN CAPITAL LETTER N WITH CARON
        0x0148 => Some(Cow::from("ncaron")),               // LATIN SMALL LETTER N WITH CARON
        0x0149 => Some(Cow::from("napostrophe")),          // LATIN SMALL LETTER N PRECEDED BY APOSTROPHE
        0x014A => Some(Cow::from("Eng")),                  // LATIN CAPITAL LETTER ENG
        0x014B => Some(Cow::from("eng")),                  // LATIN SMALL LETTER ENG
        0x014C => Some(Cow::from("Omacron")),              // LATIN CAPITAL LETTER O WITH MACRON
        0x014D => Some(Cow::from("omacron")),              // LATIN SMALL LETTER O WITH MACRON
        0x014E => Some(Cow::from("Obreve")),               // LATIN CAPITAL LETTER O WITH BREVE
        0x014F => Some(Cow::from("obreve")),               // LATIN SMALL LETTER O WITH BREVE
        0x0150 => Some(Cow::from("Ohungarumlaut")),        // LATIN CAPITAL LETTER O WITH DOUBLE ACUTE
        0x0151 => Some(Cow::from("ohungarumlaut")),        // LATIN SMALL LETTER O WITH DOUBLE ACUTE
        0x0152 => Some(Cow::from("OE")),                   // LATIN CAPITAL LIGATURE OE
        0x0153 => Some(Cow::from("oe")),                   // LATIN SMALL LIGATURE OE
        0x0154 => Some(Cow::from("Racute")),               // LATIN CAPITAL LETTER R WITH ACUTE
        0x0155 => Some(Cow::from("racute")),               // LATIN SMALL LETTER R WITH ACUTE
        0x0158 => Some(Cow::from("Rcaron")),               // LATIN CAPITAL LETTER R WITH CARON
        0x0159 => Some(Cow::from("rcaron")),               // LATIN SMALL LETTER R WITH CARON
        0x015A => Some(Cow::from("Sacute")),               // LATIN CAPITAL LETTER S WITH ACUTE
        0x015B => Some(Cow::from("sacute")),               // LATIN SMALL LETTER S WITH ACUTE
        0x015C => Some(Cow::from("Scircumflex")),          // LATIN CAPITAL LETTER S WITH CIRCUMFLEX
        0x015D => Some(Cow::from("scircumflex")),          // LATIN SMALL LETTER S WITH CIRCUMFLEX
        0x015E => Some(Cow::from("Scedilla")),             // LATIN CAPITAL LETTER S WITH CEDILLA
        0x015F => Some(Cow::from("scedilla")),             // LATIN SMALL LETTER S WITH CEDILLA
        0x0160 => Some(Cow::from("Scaron")),               // LATIN CAPITAL LETTER S WITH CARON
        0x0161 => Some(Cow::from("scaron")),               // LATIN SMALL LETTER S WITH CARON
        0x0164 => Some(Cow::from("Tcaron")),               // LATIN CAPITAL LETTER T WITH CARON
        0x0165 => Some(Cow::from("tcaron")),               // LATIN SMALL LETTER T WITH CARON
        0x0166 => Some(Cow::from("Tbar")),                 // LATIN CAPITAL LETTER T WITH STROKE
        0x0167 => Some(Cow::from("tbar")),                 // LATIN SMALL LETTER T WITH STROKE
        0x0168 => Some(Cow::from("Utilde")),               // LATIN CAPITAL LETTER U WITH TILDE
        0x0169 => Some(Cow::from("utilde")),               // LATIN SMALL LETTER U WITH TILDE
        0x016A => Some(Cow::from("Umacron")),              // LATIN CAPITAL LETTER U WITH MACRON
        0x016B => Some(Cow::from("umacron")),              // LATIN SMALL LETTER U WITH MACRON
        0x016C => Some(Cow::from("Ubreve")),               // LATIN CAPITAL LETTER U WITH BREVE
        0x016D => Some(Cow::from("ubreve")),               // LATIN SMALL LETTER U WITH BREVE
        0x016E => Some(Cow::from("Uring")),                // LATIN CAPITAL LETTER U WITH RING ABOVE
        0x016F => Some(Cow::from("uring")),                // LATIN SMALL LETTER U WITH RING ABOVE
        0x0170 => Some(Cow::from("Uhungarumlaut")),        // LATIN CAPITAL LETTER U WITH DOUBLE ACUTE
        0x0171 => Some(Cow::from("uhungarumlaut")),        // LATIN SMALL LETTER U WITH DOUBLE ACUTE
        0x0172 => Some(Cow::from("Uogonek")),              // LATIN CAPITAL LETTER U WITH OGONEK
        0x0173 => Some(Cow::from("uogonek")),              // LATIN SMALL LETTER U WITH OGONEK
        0x0174 => Some(Cow::from("Wcircumflex")),          // LATIN CAPITAL LETTER W WITH CIRCUMFLEX
        0x0175 => Some(Cow::from("wcircumflex")),          // LATIN SMALL LETTER W WITH CIRCUMFLEX
        0x0176 => Some(Cow::from("Ycircumflex")),          // LATIN CAPITAL LETTER Y WITH CIRCUMFLEX
        0x0177 => Some(Cow::from("ycircumflex")),          // LATIN SMALL LETTER Y WITH CIRCUMFLEX
        0x0178 => Some(Cow::from("Ydieresis")),            // LATIN CAPITAL LETTER Y WITH DIAERESIS
        0x0179 => Some(Cow::from("Zacute")),               // LATIN CAPITAL LETTER Z WITH ACUTE
        0x017A => Some(Cow::from("zacute")),               // LATIN SMALL LETTER Z WITH ACUTE
        0x017B => Some(Cow::from("Zdotaccent")),           // LATIN CAPITAL LETTER Z WITH DOT ABOVE
        0x017C => Some(Cow::from("zdotaccent")),           // LATIN SMALL LETTER Z WITH DOT ABOVE
        0x017D => Some(Cow::from("Zcaron")),               // LATIN CAPITAL LETTER Z WITH CARON
        0x017E => Some(Cow::from("zcaron")),               // LATIN SMALL LETTER Z WITH CARON
        0x017F => Some(Cow::from("longs")),                // LATIN SMALL LETTER LONG S
        0x0192 => Some(Cow::from("florin")),               // LATIN SMALL LETTER F WITH HOOK
        0x01A0 => Some(Cow::from("Ohorn")),                // LATIN CAPITAL LETTER O WITH HORN
        0x01A1 => Some(Cow::from("ohorn")),                // LATIN SMALL LETTER O WITH HORN
        0x01AF => Some(Cow::from("Uhorn")),                // LATIN CAPITAL LETTER U WITH HORN
        0x01B0 => Some(Cow::from("uhorn")),                // LATIN SMALL LETTER U WITH HORN
        0x01E6 => Some(Cow::from("Gcaron")),               // LATIN CAPITAL LETTER G WITH CARON
        0x01E7 => Some(Cow::from("gcaron")),               // LATIN SMALL LETTER G WITH CARON
        0x01FA => Some(Cow::from("Aringacute")),           // LATIN CAPITAL LETTER A WITH RING ABOVE AND ACUTE
        0x01FB => Some(Cow::from("aringacute")),           // LATIN SMALL LETTER A WITH RING ABOVE AND ACUTE
        0x01FC => Some(Cow::from("AEacute")),              // LATIN CAPITAL LETTER AE WITH ACUTE
        0x01FD => Some(Cow::from("aeacute")),              // LATIN SMALL LETTER AE WITH ACUTE
        0x01FE => Some(Cow::from("Oslashacute")),          // LATIN CAPITAL LETTER O WITH STROKE AND ACUTE
        0x01FF => Some(Cow::from("oslashacute")),          // LATIN SMALL LETTER O WITH STROKE AND ACUTE
        0x02C6 => Some(Cow::from("circumflex")),           // MODIFIER LETTER CIRCUMFLEX ACCENT
        0x02C7 => Some(Cow::from("caron")),                // CARON
        0x02D8 => Some(Cow::from("breve")),                // BREVE
        0x02D9 => Some(Cow::from("dotaccent")),            // DOT ABOVE
        0x02DA => Some(Cow::from("ring")),                 // RING ABOVE
        0x02DB => Some(Cow::from("ogonek")),               // OGONEK
        0x02DC => Some(Cow::from("tilde")),                // SMALL TILDE
        0x02DD => Some(Cow::from("hungarumlaut")),         // DOUBLE ACUTE ACCENT
        0x0300 => Some(Cow::from("gravecomb")),            // COMBINING GRAVE ACCENT
        0x0301 => Some(Cow::from("acutecomb")),            // COMBINING ACUTE ACCENT
        0x0303 => Some(Cow::from("tildecomb")),            // COMBINING TILDE
        0x0309 => Some(Cow::from("hookabovecomb")),        // COMBINING HOOK ABOVE
        0x0323 => Some(Cow::from("dotbelowcomb")),         // COMBINING DOT BELOW
        0x0384 => Some(Cow::from("tonos")),                // GREEK TONOS
        0x0385 => Some(Cow::from("dieresistonos")),        // GREEK DIALYTIKA TONOS
        0x0386 => Some(Cow::from("Alphatonos")),           // GREEK CAPITAL LETTER ALPHA WITH TONOS
        0x0387 => Some(Cow::from("anoteleia")),            // GREEK ANO TELEIA
        0x0388 => Some(Cow::from("Epsilontonos")),         // GREEK CAPITAL LETTER EPSILON WITH TONOS
        0x0389 => Some(Cow::from("Etatonos")),             // GREEK CAPITAL LETTER ETA WITH TONOS
        0x038A => Some(Cow::from("Iotatonos")),            // GREEK CAPITAL LETTER IOTA WITH TONOS
        0x038C => Some(Cow::from("Omicrontonos")),         // GREEK CAPITAL LETTER OMICRON WITH TONOS
        0x038E => Some(Cow::from("Upsilontonos")),         // GREEK CAPITAL LETTER UPSILON WITH TONOS
        0x038F => Some(Cow::from("Omegatonos")),           // GREEK CAPITAL LETTER OMEGA WITH TONOS
        0x0390 => Some(Cow::from("iotadieresistonos")),    // GREEK SMALL LETTER IOTA WITH DIALYTIKA AND TONOS
        0x0391 => Some(Cow::from("Alpha")),                // GREEK CAPITAL LETTER ALPHA
        0x0392 => Some(Cow::from("Beta")),                 // GREEK CAPITAL LETTER BETA
        0x0393 => Some(Cow::from("Gamma")),                // GREEK CAPITAL LETTER GAMMA
        0x0395 => Some(Cow::from("Epsilon")),              // GREEK CAPITAL LETTER EPSILON
        0x0396 => Some(Cow::from("Zeta")),                 // GREEK CAPITAL LETTER ZETA
        0x0397 => Some(Cow::from("Eta")),                  // GREEK CAPITAL LETTER ETA
        0x0398 => Some(Cow::from("Theta")),                // GREEK CAPITAL LETTER THETA
        0x0399 => Some(Cow::from("Iota")),                 // GREEK CAPITAL LETTER IOTA
        0x039A => Some(Cow::from("Kappa")),                // GREEK CAPITAL LETTER KAPPA
        0x039B => Some(Cow::from("Lambda")),               // GREEK CAPITAL LETTER LAMDA
        0x039C => Some(Cow::from("Mu")),                   // GREEK CAPITAL LETTER MU
        0x039D => Some(Cow::from("Nu")),                   // GREEK CAPITAL LETTER NU
        0x039E => Some(Cow::from("Xi")),                   // GREEK CAPITAL LETTER XI
        0x039F => Some(Cow::from("Omicron")),              // GREEK CAPITAL LETTER OMICRON
        0x03A0 => Some(Cow::from("Pi")),                   // GREEK CAPITAL LETTER PI
        0x03A1 => Some(Cow::from("Rho")),                  // GREEK CAPITAL LETTER RHO
        0x03A3 => Some(Cow::from("Sigma")),                // GREEK CAPITAL LETTER SIGMA
        0x03A4 => Some(Cow::from("Tau")),                  // GREEK CAPITAL LETTER TAU
        0x03A5 => Some(Cow::from("Upsilon")),              // GREEK CAPITAL LETTER UPSILON
        0x03A6 => Some(Cow::from("Phi")),                  // GREEK CAPITAL LETTER PHI
        0x03A7 => Some(Cow::from("Chi")),                  // GREEK CAPITAL LETTER CHI
        0x03A8 => Some(Cow::from("Psi")),                  // GREEK CAPITAL LETTER PSI
        0x03AA => Some(Cow::from("Iotadieresis")),         // GREEK CAPITAL LETTER IOTA WITH DIALYTIKA
        0x03AB => Some(Cow::from("Upsilondieresis")),      // GREEK CAPITAL LETTER UPSILON WITH DIALYTIKA
        0x03AC => Some(Cow::from("alphatonos")),           // GREEK SMALL LETTER ALPHA WITH TONOS
        0x03AD => Some(Cow::from("epsilontonos")),         // GREEK SMALL LETTER EPSILON WITH TONOS
        0x03AE => Some(Cow::from("etatonos")),             // GREEK SMALL LETTER ETA WITH TONOS
        0x03AF => Some(Cow::from("iotatonos")),            // GREEK SMALL LETTER IOTA WITH TONOS
        0x03B0 => Some(Cow::from("upsilondieresistonos")), // GREEK SMALL LETTER UPSILON WITH DIALYTIKA AND TONOS
        0x03B1 => Some(Cow::from("alpha")),                // GREEK SMALL LETTER ALPHA
        0x03B2 => Some(Cow::from("beta")),                 // GREEK SMALL LETTER BETA
        0x03B3 => Some(Cow::from("gamma")),                // GREEK SMALL LETTER GAMMA
        0x03B4 => Some(Cow::from("delta")),                // GREEK SMALL LETTER DELTA
        0x03B5 => Some(Cow::from("epsilon")),              // GREEK SMALL LETTER EPSILON
        0x03B6 => Some(Cow::from("zeta")),                 // GREEK SMALL LETTER ZETA
        0x03B7 => Some(Cow::from("eta")),                  // GREEK SMALL LETTER ETA
        0x03B8 => Some(Cow::from("theta")),                // GREEK SMALL LETTER THETA
        0x03B9 => Some(Cow::from("iota")),                 // GREEK SMALL LETTER IOTA
        0x03BA => Some(Cow::from("kappa")),                // GREEK SMALL LETTER KAPPA
        0x03BB => Some(Cow::from("lambda")),               // GREEK SMALL LETTER LAMDA
        0x03BD => Some(Cow::from("nu")),                   // GREEK SMALL LETTER NU
        0x03BE => Some(Cow::from("xi")),                   // GREEK SMALL LETTER XI
        0x03BF => Some(Cow::from("omicron")),              // GREEK SMALL LETTER OMICRON
        0x03C0 => Some(Cow::from("pi")),                   // GREEK SMALL LETTER PI
        0x03C1 => Some(Cow::from("rho")),                  // GREEK SMALL LETTER RHO
        0x03C2 => Some(Cow::from("sigma1")),               // GREEK SMALL LETTER FINAL SIGMA
        0x03C3 => Some(Cow::from("sigma")),                // GREEK SMALL LETTER SIGMA
        0x03C4 => Some(Cow::from("tau")),                  // GREEK SMALL LETTER TAU
        0x03C5 => Some(Cow::from("upsilon")),              // GREEK SMALL LETTER UPSILON
        0x03C6 => Some(Cow::from("phi")),                  // GREEK SMALL LETTER PHI
        0x03C7 => Some(Cow::from("chi")),                  // GREEK SMALL LETTER CHI
        0x03C8 => Some(Cow::from("psi")),                  // GREEK SMALL LETTER PSI
        0x03C9 => Some(Cow::from("omega")),                // GREEK SMALL LETTER OMEGA
        0x03CA => Some(Cow::from("iotadieresis")),         // GREEK SMALL LETTER IOTA WITH DIALYTIKA
        0x03CB => Some(Cow::from("upsilondieresis")),      // GREEK SMALL LETTER UPSILON WITH DIALYTIKA
        0x03CC => Some(Cow::from("omicrontonos")),         // GREEK SMALL LETTER OMICRON WITH TONOS
        0x03CD => Some(Cow::from("upsilontonos")),         // GREEK SMALL LETTER UPSILON WITH TONOS
        0x03CE => Some(Cow::from("omegatonos")),           // GREEK SMALL LETTER OMEGA WITH TONOS
        0x03D1 => Some(Cow::from("theta1")),               // GREEK THETA SYMBOL
        0x03D2 => Some(Cow::from("Upsilon1")),             // GREEK UPSILON WITH HOOK SYMBOL
        0x03D5 => Some(Cow::from("phi1")),                 // GREEK PHI SYMBOL
        0x03D6 => Some(Cow::from("omega1")),               // GREEK PI SYMBOL
        0x1E80 => Some(Cow::from("Wgrave")),               // LATIN CAPITAL LETTER W WITH GRAVE
        0x1E81 => Some(Cow::from("wgrave")),               // LATIN SMALL LETTER W WITH GRAVE
        0x1E82 => Some(Cow::from("Wacute")),               // LATIN CAPITAL LETTER W WITH ACUTE
        0x1E83 => Some(Cow::from("wacute")),               // LATIN SMALL LETTER W WITH ACUTE
        0x1E84 => Some(Cow::from("Wdieresis")),            // LATIN CAPITAL LETTER W WITH DIAERESIS
        0x1E85 => Some(Cow::from("wdieresis")),            // LATIN SMALL LETTER W WITH DIAERESIS
        0x1EF2 => Some(Cow::from("Ygrave")),               // LATIN CAPITAL LETTER Y WITH GRAVE
        0x1EF3 => Some(Cow::from("ygrave")),               // LATIN SMALL LETTER Y WITH GRAVE
        0x2012 => Some(Cow::from("figuredash")),           // FIGURE DASH
        0x2013 => Some(Cow::from("endash")),               // EN DASH
        0x2014 => Some(Cow::from("emdash")),               // EM DASH
        0x2017 => Some(Cow::from("underscoredbl")),        // DOUBLE LOW LINE
        0x2018 => Some(Cow::from("quoteleft")),            // LEFT SINGLE QUOTATION MARK
        0x2019 => Some(Cow::from("quoteright")),           // RIGHT SINGLE QUOTATION MARK
        0x201A => Some(Cow::from("quotesinglbase")),       // SINGLE LOW-9 QUOTATION MARK
        0x201B => Some(Cow::from("quotereversed")),        // SINGLE HIGH-REVERSED-9 QUOTATION MARK
        0x201C => Some(Cow::from("quotedblleft")),         // LEFT DOUBLE QUOTATION MARK
        0x201D => Some(Cow::from("quotedblright")),        // RIGHT DOUBLE QUOTATION MARK
        0x201E => Some(Cow::from("quotedblbase")),         // DOUBLE LOW-9 QUOTATION MARK
        0x2020 => Some(Cow::from("dagger")),               // DAGGER
        0x2021 => Some(Cow::from("daggerdbl")),            // DOUBLE DAGGER
        0x2022 => Some(Cow::from("bullet")),               // BULLET
        0x2024 => Some(Cow::from("onedotenleader")),       // ONE DOT LEADER
        0x2025 => Some(Cow::from("twodotenleader")),       // TWO DOT LEADER
        0x2026 => Some(Cow::from("ellipsis")),             // HORIZONTAL ELLIPSIS
        0x2030 => Some(Cow::from("perthousand")),          // PER MILLE SIGN
        0x2032 => Some(Cow::from("minute")),               // PRIME
        0x2033 => Some(Cow::from("second")),               // DOUBLE PRIME
        0x2039 => Some(Cow::from("guilsinglleft")),        // SINGLE LEFT-POINTING ANGLE QUOTATION MARK
        0x203A => Some(Cow::from("guilsinglright")),       // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
        0x203C => Some(Cow::from("exclamdbl")),            // DOUBLE EXCLAMATION MARK
        0x2044 => Some(Cow::from("fraction")),             // FRACTION SLASH
        0x20A1 => Some(Cow::from("colonmonetary")),        // COLON SIGN
        0x20A3 => Some(Cow::from("franc")),                // FRENCH FRANC SIGN
        0x20A4 => Some(Cow::from("lira")),                 // LIRA SIGN
        0x20A7 => Some(Cow::from("peseta")),               // PESETA SIGN
        0x20AB => Some(Cow::from("dong")),                 // DONG SIGN
        0x20AC => Some(Cow::from("Euro")),                 // EURO SIGN
        0x2111 => Some(Cow::from("Ifraktur")),             // BLACK-LETTER CAPITAL I
        0x2118 => Some(Cow::from("weierstrass")),          // SCRIPT CAPITAL P
        0x211C => Some(Cow::from("Rfraktur")),             // BLACK-LETTER CAPITAL R
        0x211E => Some(Cow::from("prescription")),         // PRESCRIPTION TAKE
        0x2122 => Some(Cow::from("trademark")),            // TRADE MARK SIGN
        0x2126 => Some(Cow::from("Omega")),                // OHM SIGN
        0x212E => Some(Cow::from("estimated")),            // ESTIMATED SYMBOL
        0x2135 => Some(Cow::from("aleph")),                // ALEF SYMBOL
        0x2153 => Some(Cow::from("onethird")),             // VULGAR FRACTION ONE THIRD
        0x2154 => Some(Cow::from("twothirds")),            // VULGAR FRACTION TWO THIRDS
        0x215B => Some(Cow::from("oneeighth")),            // VULGAR FRACTION ONE EIGHTH
        0x215C => Some(Cow::from("threeeighths")),         // VULGAR FRACTION THREE EIGHTHS
        0x215D => Some(Cow::from("fiveeighths")),          // VULGAR FRACTION FIVE EIGHTHS
        0x215E => Some(Cow::from("seveneighths")),         // VULGAR FRACTION SEVEN EIGHTHS
        0x2190 => Some(Cow::from("arrowleft")),            // LEFTWARDS ARROW
        0x2191 => Some(Cow::from("arrowup")),              // UPWARDS ARROW
        0x2192 => Some(Cow::from("arrowright")),           // RIGHTWARDS ARROW
        0x2193 => Some(Cow::from("arrowdown")),            // DOWNWARDS ARROW
        0x2194 => Some(Cow::from("arrowboth")),            // LEFT RIGHT ARROW
        0x2195 => Some(Cow::from("arrowupdn")),            // UP DOWN ARROW
        0x21A8 => Some(Cow::from("arrowupdnbse")),         // UP DOWN ARROW WITH BASE
        0x21B5 => Some(Cow::from("carriagereturn")),       // DOWNWARDS ARROW WITH CORNER LEFTWARDS
        0x21D0 => Some(Cow::from("arrowdblleft")),         // LEFTWARDS DOUBLE ARROW
        0x21D1 => Some(Cow::from("arrowdblup")),           // UPWARDS DOUBLE ARROW
        0x21D2 => Some(Cow::from("arrowdblright")),        // RIGHTWARDS DOUBLE ARROW
        0x21D3 => Some(Cow::from("arrowdbldown")),         // DOWNWARDS DOUBLE ARROW
        0x21D4 => Some(Cow::from("arrowdblboth")),         // LEFT RIGHT DOUBLE ARROW
        0x2200 => Some(Cow::from("universal")),            // FOR ALL
        0x2202 => Some(Cow::from("partialdiff")),          // PARTIAL DIFFERENTIAL
        0x2203 => Some(Cow::from("existential")),          // THERE EXISTS
        0x2205 => Some(Cow::from("emptyset")),             // EMPTY SET
        0x2206 => Some(Cow::from("Delta")),                // INCREMENT
        0x2207 => Some(Cow::from("gradient")),             // NABLA
        0x2208 => Some(Cow::from("element")),              // ELEMENT OF
        0x2209 => Some(Cow::from("notelement")),           // NOT AN ELEMENT OF
        0x220B => Some(Cow::from("suchthat")),             // CONTAINS AS MEMBER
        0x220F => Some(Cow::from("product")),              // N-ARY PRODUCT
        0x2211 => Some(Cow::from("summation")),            // N-ARY SUMMATION
        0x2212 => Some(Cow::from("minus")),                // MINUS SIGN
        0x2217 => Some(Cow::from("asteriskmath")),         // ASTERISK OPERATOR
        0x221A => Some(Cow::from("radical")),              // SQUARE ROOT
        0x221D => Some(Cow::from("proportional")),         // PROPORTIONAL TO
        0x221E => Some(Cow::from("infinity")),             // INFINITY
        0x221F => Some(Cow::from("orthogonal")),           // RIGHT ANGLE
        0x2220 => Some(Cow::from("angle")),                // ANGLE
        0x2227 => Some(Cow::from("logicaland")),           // LOGICAL AND
        0x2228 => Some(Cow::from("logicalor")),            // LOGICAL OR
        0x2229 => Some(Cow::from("intersection")),         // INTERSECTION
        0x222A => Some(Cow::from("union")),                // UNION
        0x222B => Some(Cow::from("integral")),             // INTEGRAL
        0x2234 => Some(Cow::from("therefore")),            // THEREFORE
        0x223C => Some(Cow::from("similar")),              // TILDE OPERATOR
        0x2245 => Some(Cow::from("congruent")),            // APPROXIMATELY EQUAL TO
        0x2248 => Some(Cow::from("approxequal")),          // ALMOST EQUAL TO
        0x2260 => Some(Cow::from("notequal")),             // NOT EQUAL TO
        0x2261 => Some(Cow::from("equivalence")),          // IDENTICAL TO
        0x2264 => Some(Cow::from("lessequal")),            // LESS-THAN OR EQUAL TO
        0x2265 => Some(Cow::from("greaterequal")),         // GREATER-THAN OR EQUAL TO
        0x2282 => Some(Cow::from("propersubset")),         // SUBSET OF
        0x2283 => Some(Cow::from("propersuperset")),       // SUPERSET OF
        0x2284 => Some(Cow::from("notsubset")),            // NOT A SUBSET OF
        0x2286 => Some(Cow::from("reflexsubset")),         // SUBSET OF OR EQUAL TO
        0x2287 => Some(Cow::from("reflexsuperset")),       // SUPERSET OF OR EQUAL TO
        0x2295 => Some(Cow::from("circleplus")),           // CIRCLED PLUS
        0x2297 => Some(Cow::from("circlemultiply")),       // CIRCLED TIMES
        0x22A5 => Some(Cow::from("perpendicular")),        // UP TACK
        0x22C5 => Some(Cow::from("dotmath")),              // DOT OPERATOR
        0x2302 => Some(Cow::from("house")),                // HOUSE
        0x2310 => Some(Cow::from("revlogicalnot")),        // REVERSED NOT SIGN
        0x2320 => Some(Cow::from("integraltp")),           // TOP HALF INTEGRAL
        0x2321 => Some(Cow::from("integralbt")),           // BOTTOM HALF INTEGRAL
        0x2329 => Some(Cow::from("angleleft")),            // LEFT-POINTING ANGLE BRACKET
        0x232A => Some(Cow::from("angleright")),           // RIGHT-POINTING ANGLE BRACKET
        0x2500 => Some(Cow::from("SF100000")),             // BOX DRAWINGS LIGHT HORIZONTAL
        0x2502 => Some(Cow::from("SF110000")),             // BOX DRAWINGS LIGHT VERTICAL
        0x250C => Some(Cow::from("SF010000")),             // BOX DRAWINGS LIGHT DOWN AND RIGHT
        0x2510 => Some(Cow::from("SF030000")),             // BOX DRAWINGS LIGHT DOWN AND LEFT
        0x2514 => Some(Cow::from("SF020000")),             // BOX DRAWINGS LIGHT UP AND RIGHT
        0x2518 => Some(Cow::from("SF040000")),             // BOX DRAWINGS LIGHT UP AND LEFT
        0x251C => Some(Cow::from("SF080000")),             // BOX DRAWINGS LIGHT VERTICAL AND RIGHT
        0x2524 => Some(Cow::from("SF090000")),             // BOX DRAWINGS LIGHT VERTICAL AND LEFT
        0x252C => Some(Cow::from("SF060000")),             // BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
        0x2534 => Some(Cow::from("SF070000")),             // BOX DRAWINGS LIGHT UP AND HORIZONTAL
        0x253C => Some(Cow::from("SF050000")),             // BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
        0x2550 => Some(Cow::from("SF430000")),             // BOX DRAWINGS DOUBLE HORIZONTAL
        0x2551 => Some(Cow::from("SF240000")),             // BOX DRAWINGS DOUBLE VERTICAL
        0x2552 => Some(Cow::from("SF510000")),             // BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE
        0x2553 => Some(Cow::from("SF520000")),             // BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE
        0x2554 => Some(Cow::from("SF390000")),             // BOX DRAWINGS DOUBLE DOWN AND RIGHT
        0x2555 => Some(Cow::from("SF220000")),             // BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE
        0x2556 => Some(Cow::from("SF210000")),             // BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE
        0x2557 => Some(Cow::from("SF250000")),             // BOX DRAWINGS DOUBLE DOWN AND LEFT
        0x2558 => Some(Cow::from("SF500000")),             // BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE
        0x2559 => Some(Cow::from("SF490000")),             // BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE
        0x255A => Some(Cow::from("SF380000")),             // BOX DRAWINGS DOUBLE UP AND RIGHT
        0x255B => Some(Cow::from("SF280000")),             // BOX DRAWINGS UP SINGLE AND LEFT DOUBLE
        0x255C => Some(Cow::from("SF270000")),             // BOX DRAWINGS UP DOUBLE AND LEFT SINGLE
        0x255D => Some(Cow::from("SF260000")),             // BOX DRAWINGS DOUBLE UP AND LEFT
        0x255E => Some(Cow::from("SF360000")),             // BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE
        0x255F => Some(Cow::from("SF370000")),             // BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE
        0x2560 => Some(Cow::from("SF420000")),             // BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
        0x2561 => Some(Cow::from("SF190000")),             // BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE
        0x2562 => Some(Cow::from("SF200000")),             // BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE
        0x2563 => Some(Cow::from("SF230000")),             // BOX DRAWINGS DOUBLE VERTICAL AND LEFT
        0x2564 => Some(Cow::from("SF470000")),             // BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE
        0x2565 => Some(Cow::from("SF480000")),             // BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE
        0x2566 => Some(Cow::from("SF410000")),             // BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
        0x2567 => Some(Cow::from("SF450000")),             // BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE
        0x2568 => Some(Cow::from("SF460000")),             // BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE
        0x2569 => Some(Cow::from("SF400000")),             // BOX DRAWINGS DOUBLE UP AND HORIZONTAL
        0x256A => Some(Cow::from("SF540000")),             // BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE
        0x256B => Some(Cow::from("SF530000")),             // BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE
        0x256C => Some(Cow::from("SF440000")),             // BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
        0x2580 => Some(Cow::from("upblock")),              // UPPER HALF BLOCK
        0x2584 => Some(Cow::from("dnblock")),              // LOWER HALF BLOCK
        0x2588 => Some(Cow::from("block")),                // FULL BLOCK
        0x258C => Some(Cow::from("lfblock")),              // LEFT HALF BLOCK
        0x2590 => Some(Cow::from("rtblock")),              // RIGHT HALF BLOCK
        0x2591 => Some(Cow::from("ltshade")),              // LIGHT SHADE
        0x2592 => Some(Cow::from("shade")),                // MEDIUM SHADE
        0x2593 => Some(Cow::from("dkshade")),              // DARK SHADE
        0x25A0 => Some(Cow::from("filledbox")),            // BLACK SQUARE
        0x25A1 => Some(Cow::from("H22073")),               // WHITE SQUARE
        0x25AA => Some(Cow::from("H18543")),               // BLACK SMALL SQUARE
        0x25AB => Some(Cow::from("H18551")),               // WHITE SMALL SQUARE
        0x25AC => Some(Cow::from("filledrect")),           // BLACK RECTANGLE
        0x25B2 => Some(Cow::from("triagup")),              // BLACK UP-POINTING TRIANGLE
        0x25BA => Some(Cow::from("triagrt")),              // BLACK RIGHT-POINTING POINTER
        0x25BC => Some(Cow::from("triagdn")),              // BLACK DOWN-POINTING TRIANGLE
        0x25C4 => Some(Cow::from("triaglf")),              // BLACK LEFT-POINTING POINTER
        0x25CA => Some(Cow::from("lozenge")),              // LOZENGE
        0x25CB => Some(Cow::from("circle")),               // WHITE CIRCLE
        0x25CF => Some(Cow::from("H18533")),               // BLACK CIRCLE
        0x25D8 => Some(Cow::from("invbullet")),            // INVERSE BULLET
        0x25D9 => Some(Cow::from("invcircle")),            // INVERSE WHITE CIRCLE
        0x25E6 => Some(Cow::from("openbullet")),           // WHITE BULLET
        0x263A => Some(Cow::from("smileface")),            // WHITE SMILING FACE
        0x263B => Some(Cow::from("invsmileface")),         // BLACK SMILING FACE
        0x263C => Some(Cow::from("sun")),                  // WHITE SUN WITH RAYS
        0x2640 => Some(Cow::from("female")),               // FEMALE SIGN
        0x2642 => Some(Cow::from("male")),                 // MALE SIGN
        0x2660 => Some(Cow::from("spade")),                // BLACK SPADE SUIT
        0x2663 => Some(Cow::from("club")),                 // BLACK CLUB SUIT
        0x2665 => Some(Cow::from("heart")),                // BLACK HEART SUIT
        0x2666 => Some(Cow::from("diamond")),              // BLACK DIAMOND SUIT
        0x266A => Some(Cow::from("musicalnote")),          // EIGHTH NOTE
        0x266B => Some(Cow::from("musicalnotedbl")),       // BEAMED EIGHTH NOTES
        _ => None,
    }
}

impl<'a> GlyphNameOpt<'a> for AdobeGlyphListForNewFonts {
    fn glyph_name_opt(c: char) -> Option<Cow<'a, str>> {
        self::glyph_name_opt(c)
    }
}
