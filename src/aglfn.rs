#![allow(dead_code)]
#![allow(unreachable_patterns)]
//! Adobe Glyph List for New Fonts (2019 edition)
//!
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

/// Adobe Glyph List for New Fonts (2019 edition)
///
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
pub struct AdobeGlyphListForNewFonts;

use std::borrow::Cow;
use crate::traits::{GlyphNameStrict, GlyphNameVec};

fn glyph_name_strict<'a>(c: char) -> Option<Cow<'a, str>> {
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
fn glyph_name_vec<'a, S: AsRef<[char]>>(s: S) -> Option<Cow<'a, str>> {
    match s.as_ref().into_iter().map(|c|*c as u32).collect::<Vec<u32>>().as_slice() {
            &[0x0020u32] => Some(Cow::from("space")),
            &[0x0021u32] => Some(Cow::from("exclam")),
            &[0x0022u32] => Some(Cow::from("quotedbl")),
            &[0x0023u32] => Some(Cow::from("numbersign")),
            &[0x0024u32] => Some(Cow::from("dollar")),
            &[0x0025u32] => Some(Cow::from("percent")),
            &[0x0026u32] => Some(Cow::from("ampersand")),
            &[0x0027u32] => Some(Cow::from("quotesingle")),
            &[0x0028u32] => Some(Cow::from("parenleft")),
            &[0x0029u32] => Some(Cow::from("parenright")),
            &[0x002Au32] => Some(Cow::from("asterisk")),
            &[0x002Bu32] => Some(Cow::from("plus")),
            &[0x002Cu32] => Some(Cow::from("comma")),
            &[0x002Du32] => Some(Cow::from("hyphen")),
            &[0x002Eu32] => Some(Cow::from("period")),
            &[0x002Fu32] => Some(Cow::from("slash")),
            &[0x0030u32] => Some(Cow::from("zero")),
            &[0x0031u32] => Some(Cow::from("one")),
            &[0x0032u32] => Some(Cow::from("two")),
            &[0x0033u32] => Some(Cow::from("three")),
            &[0x0034u32] => Some(Cow::from("four")),
            &[0x0035u32] => Some(Cow::from("five")),
            &[0x0036u32] => Some(Cow::from("six")),
            &[0x0037u32] => Some(Cow::from("seven")),
            &[0x0038u32] => Some(Cow::from("eight")),
            &[0x0039u32] => Some(Cow::from("nine")),
            &[0x003Au32] => Some(Cow::from("colon")),
            &[0x003Bu32] => Some(Cow::from("semicolon")),
            &[0x003Cu32] => Some(Cow::from("less")),
            &[0x003Du32] => Some(Cow::from("equal")),
            &[0x003Eu32] => Some(Cow::from("greater")),
            &[0x003Fu32] => Some(Cow::from("question")),
            &[0x0040u32] => Some(Cow::from("at")),
            &[0x0041u32] => Some(Cow::from("A")),
            &[0x0042u32] => Some(Cow::from("B")),
            &[0x0043u32] => Some(Cow::from("C")),
            &[0x0044u32] => Some(Cow::from("D")),
            &[0x0045u32] => Some(Cow::from("E")),
            &[0x0046u32] => Some(Cow::from("F")),
            &[0x0047u32] => Some(Cow::from("G")),
            &[0x0048u32] => Some(Cow::from("H")),
            &[0x0049u32] => Some(Cow::from("I")),
            &[0x004Au32] => Some(Cow::from("J")),
            &[0x004Bu32] => Some(Cow::from("K")),
            &[0x004Cu32] => Some(Cow::from("L")),
            &[0x004Du32] => Some(Cow::from("M")),
            &[0x004Eu32] => Some(Cow::from("N")),
            &[0x004Fu32] => Some(Cow::from("O")),
            &[0x0050u32] => Some(Cow::from("P")),
            &[0x0051u32] => Some(Cow::from("Q")),
            &[0x0052u32] => Some(Cow::from("R")),
            &[0x0053u32] => Some(Cow::from("S")),
            &[0x0054u32] => Some(Cow::from("T")),
            &[0x0055u32] => Some(Cow::from("U")),
            &[0x0056u32] => Some(Cow::from("V")),
            &[0x0057u32] => Some(Cow::from("W")),
            &[0x0058u32] => Some(Cow::from("X")),
            &[0x0059u32] => Some(Cow::from("Y")),
            &[0x005Au32] => Some(Cow::from("Z")),
            &[0x005Bu32] => Some(Cow::from("bracketleft")),
            &[0x005Cu32] => Some(Cow::from("backslash")),
            &[0x005Du32] => Some(Cow::from("bracketright")),
            &[0x005Eu32] => Some(Cow::from("asciicircum")),
            &[0x005Fu32] => Some(Cow::from("underscore")),
            &[0x0060u32] => Some(Cow::from("grave")),
            &[0x0061u32] => Some(Cow::from("a")),
            &[0x0062u32] => Some(Cow::from("b")),
            &[0x0063u32] => Some(Cow::from("c")),
            &[0x0064u32] => Some(Cow::from("d")),
            &[0x0065u32] => Some(Cow::from("e")),
            &[0x0066u32] => Some(Cow::from("f")),
            &[0x0067u32] => Some(Cow::from("g")),
            &[0x0068u32] => Some(Cow::from("h")),
            &[0x0069u32] => Some(Cow::from("i")),
            &[0x006Au32] => Some(Cow::from("j")),
            &[0x006Bu32] => Some(Cow::from("k")),
            &[0x006Cu32] => Some(Cow::from("l")),
            &[0x006Du32] => Some(Cow::from("m")),
            &[0x006Eu32] => Some(Cow::from("n")),
            &[0x006Fu32] => Some(Cow::from("o")),
            &[0x0070u32] => Some(Cow::from("p")),
            &[0x0071u32] => Some(Cow::from("q")),
            &[0x0072u32] => Some(Cow::from("r")),
            &[0x0073u32] => Some(Cow::from("s")),
            &[0x0074u32] => Some(Cow::from("t")),
            &[0x0075u32] => Some(Cow::from("u")),
            &[0x0076u32] => Some(Cow::from("v")),
            &[0x0077u32] => Some(Cow::from("w")),
            &[0x0078u32] => Some(Cow::from("x")),
            &[0x0079u32] => Some(Cow::from("y")),
            &[0x007Au32] => Some(Cow::from("z")),
            &[0x007Bu32] => Some(Cow::from("braceleft")),
            &[0x007Cu32] => Some(Cow::from("bar")),
            &[0x007Du32] => Some(Cow::from("braceright")),
            &[0x007Eu32] => Some(Cow::from("asciitilde")),
            &[0x00A1u32] => Some(Cow::from("exclamdown")),
            &[0x00A2u32] => Some(Cow::from("cent")),
            &[0x00A3u32] => Some(Cow::from("sterling")),
            &[0x00A4u32] => Some(Cow::from("currency")),
            &[0x00A5u32] => Some(Cow::from("yen")),
            &[0x00A6u32] => Some(Cow::from("brokenbar")),
            &[0x00A7u32] => Some(Cow::from("section")),
            &[0x00A8u32] => Some(Cow::from("dieresis")),
            &[0x00A9u32] => Some(Cow::from("copyright")),
            &[0x00AAu32] => Some(Cow::from("ordfeminine")),
            &[0x00ABu32] => Some(Cow::from("guillemotleft")),
            &[0x00ACu32] => Some(Cow::from("logicalnot")),
            &[0x00AEu32] => Some(Cow::from("registered")),
            &[0x00AFu32] => Some(Cow::from("macron")),
            &[0x00B0u32] => Some(Cow::from("degree")),
            &[0x00B1u32] => Some(Cow::from("plusminus")),
            &[0x00B4u32] => Some(Cow::from("acute")),
            &[0x00B5u32] => Some(Cow::from("mu")),
            &[0x00B6u32] => Some(Cow::from("paragraph")),
            &[0x00B7u32] => Some(Cow::from("periodcentered")),
            &[0x00B8u32] => Some(Cow::from("cedilla")),
            &[0x00BAu32] => Some(Cow::from("ordmasculine")),
            &[0x00BBu32] => Some(Cow::from("guillemotright")),
            &[0x00BCu32] => Some(Cow::from("onequarter")),
            &[0x00BDu32] => Some(Cow::from("onehalf")),
            &[0x00BEu32] => Some(Cow::from("threequarters")),
            &[0x00BFu32] => Some(Cow::from("questiondown")),
            &[0x00C0u32] => Some(Cow::from("Agrave")),
            &[0x00C1u32] => Some(Cow::from("Aacute")),
            &[0x00C2u32] => Some(Cow::from("Acircumflex")),
            &[0x00C3u32] => Some(Cow::from("Atilde")),
            &[0x00C4u32] => Some(Cow::from("Adieresis")),
            &[0x00C5u32] => Some(Cow::from("Aring")),
            &[0x00C6u32] => Some(Cow::from("AE")),
            &[0x00C7u32] => Some(Cow::from("Ccedilla")),
            &[0x00C8u32] => Some(Cow::from("Egrave")),
            &[0x00C9u32] => Some(Cow::from("Eacute")),
            &[0x00CAu32] => Some(Cow::from("Ecircumflex")),
            &[0x00CBu32] => Some(Cow::from("Edieresis")),
            &[0x00CCu32] => Some(Cow::from("Igrave")),
            &[0x00CDu32] => Some(Cow::from("Iacute")),
            &[0x00CEu32] => Some(Cow::from("Icircumflex")),
            &[0x00CFu32] => Some(Cow::from("Idieresis")),
            &[0x00D0u32] => Some(Cow::from("Eth")),
            &[0x00D1u32] => Some(Cow::from("Ntilde")),
            &[0x00D2u32] => Some(Cow::from("Ograve")),
            &[0x00D3u32] => Some(Cow::from("Oacute")),
            &[0x00D4u32] => Some(Cow::from("Ocircumflex")),
            &[0x00D5u32] => Some(Cow::from("Otilde")),
            &[0x00D6u32] => Some(Cow::from("Odieresis")),
            &[0x00D7u32] => Some(Cow::from("multiply")),
            &[0x00D8u32] => Some(Cow::from("Oslash")),
            &[0x00D9u32] => Some(Cow::from("Ugrave")),
            &[0x00DAu32] => Some(Cow::from("Uacute")),
            &[0x00DBu32] => Some(Cow::from("Ucircumflex")),
            &[0x00DCu32] => Some(Cow::from("Udieresis")),
            &[0x00DDu32] => Some(Cow::from("Yacute")),
            &[0x00DEu32] => Some(Cow::from("Thorn")),
            &[0x00DFu32] => Some(Cow::from("germandbls")),
            &[0x00E0u32] => Some(Cow::from("agrave")),
            &[0x00E1u32] => Some(Cow::from("aacute")),
            &[0x00E2u32] => Some(Cow::from("acircumflex")),
            &[0x00E3u32] => Some(Cow::from("atilde")),
            &[0x00E4u32] => Some(Cow::from("adieresis")),
            &[0x00E5u32] => Some(Cow::from("aring")),
            &[0x00E6u32] => Some(Cow::from("ae")),
            &[0x00E7u32] => Some(Cow::from("ccedilla")),
            &[0x00E8u32] => Some(Cow::from("egrave")),
            &[0x00E9u32] => Some(Cow::from("eacute")),
            &[0x00EAu32] => Some(Cow::from("ecircumflex")),
            &[0x00EBu32] => Some(Cow::from("edieresis")),
            &[0x00ECu32] => Some(Cow::from("igrave")),
            &[0x00EDu32] => Some(Cow::from("iacute")),
            &[0x00EEu32] => Some(Cow::from("icircumflex")),
            &[0x00EFu32] => Some(Cow::from("idieresis")),
            &[0x00F0u32] => Some(Cow::from("eth")),
            &[0x00F1u32] => Some(Cow::from("ntilde")),
            &[0x00F2u32] => Some(Cow::from("ograve")),
            &[0x00F3u32] => Some(Cow::from("oacute")),
            &[0x00F4u32] => Some(Cow::from("ocircumflex")),
            &[0x00F5u32] => Some(Cow::from("otilde")),
            &[0x00F6u32] => Some(Cow::from("odieresis")),
            &[0x00F7u32] => Some(Cow::from("divide")),
            &[0x00F8u32] => Some(Cow::from("oslash")),
            &[0x00F9u32] => Some(Cow::from("ugrave")),
            &[0x00FAu32] => Some(Cow::from("uacute")),
            &[0x00FBu32] => Some(Cow::from("ucircumflex")),
            &[0x00FCu32] => Some(Cow::from("udieresis")),
            &[0x00FDu32] => Some(Cow::from("yacute")),
            &[0x00FEu32] => Some(Cow::from("thorn")),
            &[0x00FFu32] => Some(Cow::from("ydieresis")),
            &[0x0100u32] => Some(Cow::from("Amacron")),
            &[0x0101u32] => Some(Cow::from("amacron")),
            &[0x0102u32] => Some(Cow::from("Abreve")),
            &[0x0103u32] => Some(Cow::from("abreve")),
            &[0x0104u32] => Some(Cow::from("Aogonek")),
            &[0x0105u32] => Some(Cow::from("aogonek")),
            &[0x0106u32] => Some(Cow::from("Cacute")),
            &[0x0107u32] => Some(Cow::from("cacute")),
            &[0x0108u32] => Some(Cow::from("Ccircumflex")),
            &[0x0109u32] => Some(Cow::from("ccircumflex")),
            &[0x010Au32] => Some(Cow::from("Cdotaccent")),
            &[0x010Bu32] => Some(Cow::from("cdotaccent")),
            &[0x010Cu32] => Some(Cow::from("Ccaron")),
            &[0x010Du32] => Some(Cow::from("ccaron")),
            &[0x010Eu32] => Some(Cow::from("Dcaron")),
            &[0x010Fu32] => Some(Cow::from("dcaron")),
            &[0x0110u32] => Some(Cow::from("Dcroat")),
            &[0x0111u32] => Some(Cow::from("dcroat")),
            &[0x0112u32] => Some(Cow::from("Emacron")),
            &[0x0113u32] => Some(Cow::from("emacron")),
            &[0x0114u32] => Some(Cow::from("Ebreve")),
            &[0x0115u32] => Some(Cow::from("ebreve")),
            &[0x0116u32] => Some(Cow::from("Edotaccent")),
            &[0x0117u32] => Some(Cow::from("edotaccent")),
            &[0x0118u32] => Some(Cow::from("Eogonek")),
            &[0x0119u32] => Some(Cow::from("eogonek")),
            &[0x011Au32] => Some(Cow::from("Ecaron")),
            &[0x011Bu32] => Some(Cow::from("ecaron")),
            &[0x011Cu32] => Some(Cow::from("Gcircumflex")),
            &[0x011Du32] => Some(Cow::from("gcircumflex")),
            &[0x011Eu32] => Some(Cow::from("Gbreve")),
            &[0x011Fu32] => Some(Cow::from("gbreve")),
            &[0x0120u32] => Some(Cow::from("Gdotaccent")),
            &[0x0121u32] => Some(Cow::from("gdotaccent")),
            &[0x0124u32] => Some(Cow::from("Hcircumflex")),
            &[0x0125u32] => Some(Cow::from("hcircumflex")),
            &[0x0126u32] => Some(Cow::from("Hbar")),
            &[0x0127u32] => Some(Cow::from("hbar")),
            &[0x0128u32] => Some(Cow::from("Itilde")),
            &[0x0129u32] => Some(Cow::from("itilde")),
            &[0x012Au32] => Some(Cow::from("Imacron")),
            &[0x012Bu32] => Some(Cow::from("imacron")),
            &[0x012Cu32] => Some(Cow::from("Ibreve")),
            &[0x012Du32] => Some(Cow::from("ibreve")),
            &[0x012Eu32] => Some(Cow::from("Iogonek")),
            &[0x012Fu32] => Some(Cow::from("iogonek")),
            &[0x0130u32] => Some(Cow::from("Idotaccent")),
            &[0x0131u32] => Some(Cow::from("dotlessi")),
            &[0x0132u32] => Some(Cow::from("IJ")),
            &[0x0133u32] => Some(Cow::from("ij")),
            &[0x0134u32] => Some(Cow::from("Jcircumflex")),
            &[0x0135u32] => Some(Cow::from("jcircumflex")),
            &[0x0138u32] => Some(Cow::from("kgreenlandic")),
            &[0x0139u32] => Some(Cow::from("Lacute")),
            &[0x013Au32] => Some(Cow::from("lacute")),
            &[0x013Du32] => Some(Cow::from("Lcaron")),
            &[0x013Eu32] => Some(Cow::from("lcaron")),
            &[0x013Fu32] => Some(Cow::from("Ldot")),
            &[0x0140u32] => Some(Cow::from("ldot")),
            &[0x0141u32] => Some(Cow::from("Lslash")),
            &[0x0142u32] => Some(Cow::from("lslash")),
            &[0x0143u32] => Some(Cow::from("Nacute")),
            &[0x0144u32] => Some(Cow::from("nacute")),
            &[0x0147u32] => Some(Cow::from("Ncaron")),
            &[0x0148u32] => Some(Cow::from("ncaron")),
            &[0x0149u32] => Some(Cow::from("napostrophe")),
            &[0x014Au32] => Some(Cow::from("Eng")),
            &[0x014Bu32] => Some(Cow::from("eng")),
            &[0x014Cu32] => Some(Cow::from("Omacron")),
            &[0x014Du32] => Some(Cow::from("omacron")),
            &[0x014Eu32] => Some(Cow::from("Obreve")),
            &[0x014Fu32] => Some(Cow::from("obreve")),
            &[0x0150u32] => Some(Cow::from("Ohungarumlaut")),
            &[0x0151u32] => Some(Cow::from("ohungarumlaut")),
            &[0x0152u32] => Some(Cow::from("OE")),
            &[0x0153u32] => Some(Cow::from("oe")),
            &[0x0154u32] => Some(Cow::from("Racute")),
            &[0x0155u32] => Some(Cow::from("racute")),
            &[0x0158u32] => Some(Cow::from("Rcaron")),
            &[0x0159u32] => Some(Cow::from("rcaron")),
            &[0x015Au32] => Some(Cow::from("Sacute")),
            &[0x015Bu32] => Some(Cow::from("sacute")),
            &[0x015Cu32] => Some(Cow::from("Scircumflex")),
            &[0x015Du32] => Some(Cow::from("scircumflex")),
            &[0x015Eu32] => Some(Cow::from("Scedilla")),
            &[0x015Fu32] => Some(Cow::from("scedilla")),
            &[0x0160u32] => Some(Cow::from("Scaron")),
            &[0x0161u32] => Some(Cow::from("scaron")),
            &[0x0164u32] => Some(Cow::from("Tcaron")),
            &[0x0165u32] => Some(Cow::from("tcaron")),
            &[0x0166u32] => Some(Cow::from("Tbar")),
            &[0x0167u32] => Some(Cow::from("tbar")),
            &[0x0168u32] => Some(Cow::from("Utilde")),
            &[0x0169u32] => Some(Cow::from("utilde")),
            &[0x016Au32] => Some(Cow::from("Umacron")),
            &[0x016Bu32] => Some(Cow::from("umacron")),
            &[0x016Cu32] => Some(Cow::from("Ubreve")),
            &[0x016Du32] => Some(Cow::from("ubreve")),
            &[0x016Eu32] => Some(Cow::from("Uring")),
            &[0x016Fu32] => Some(Cow::from("uring")),
            &[0x0170u32] => Some(Cow::from("Uhungarumlaut")),
            &[0x0171u32] => Some(Cow::from("uhungarumlaut")),
            &[0x0172u32] => Some(Cow::from("Uogonek")),
            &[0x0173u32] => Some(Cow::from("uogonek")),
            &[0x0174u32] => Some(Cow::from("Wcircumflex")),
            &[0x0175u32] => Some(Cow::from("wcircumflex")),
            &[0x0176u32] => Some(Cow::from("Ycircumflex")),
            &[0x0177u32] => Some(Cow::from("ycircumflex")),
            &[0x0178u32] => Some(Cow::from("Ydieresis")),
            &[0x0179u32] => Some(Cow::from("Zacute")),
            &[0x017Au32] => Some(Cow::from("zacute")),
            &[0x017Bu32] => Some(Cow::from("Zdotaccent")),
            &[0x017Cu32] => Some(Cow::from("zdotaccent")),
            &[0x017Du32] => Some(Cow::from("Zcaron")),
            &[0x017Eu32] => Some(Cow::from("zcaron")),
            &[0x017Fu32] => Some(Cow::from("longs")),
            &[0x0192u32] => Some(Cow::from("florin")),
            &[0x01A0u32] => Some(Cow::from("Ohorn")),
            &[0x01A1u32] => Some(Cow::from("ohorn")),
            &[0x01AFu32] => Some(Cow::from("Uhorn")),
            &[0x01B0u32] => Some(Cow::from("uhorn")),
            &[0x01E6u32] => Some(Cow::from("Gcaron")),
            &[0x01E7u32] => Some(Cow::from("gcaron")),
            &[0x01FAu32] => Some(Cow::from("Aringacute")),
            &[0x01FBu32] => Some(Cow::from("aringacute")),
            &[0x01FCu32] => Some(Cow::from("AEacute")),
            &[0x01FDu32] => Some(Cow::from("aeacute")),
            &[0x01FEu32] => Some(Cow::from("Oslashacute")),
            &[0x01FFu32] => Some(Cow::from("oslashacute")),
            &[0x02C6u32] => Some(Cow::from("circumflex")),
            &[0x02C7u32] => Some(Cow::from("caron")),
            &[0x02D8u32] => Some(Cow::from("breve")),
            &[0x02D9u32] => Some(Cow::from("dotaccent")),
            &[0x02DAu32] => Some(Cow::from("ring")),
            &[0x02DBu32] => Some(Cow::from("ogonek")),
            &[0x02DCu32] => Some(Cow::from("tilde")),
            &[0x02DDu32] => Some(Cow::from("hungarumlaut")),
            &[0x0300u32] => Some(Cow::from("gravecomb")),
            &[0x0301u32] => Some(Cow::from("acutecomb")),
            &[0x0303u32] => Some(Cow::from("tildecomb")),
            &[0x0309u32] => Some(Cow::from("hookabovecomb")),
            &[0x0323u32] => Some(Cow::from("dotbelowcomb")),
            &[0x0384u32] => Some(Cow::from("tonos")),
            &[0x0385u32] => Some(Cow::from("dieresistonos")),
            &[0x0386u32] => Some(Cow::from("Alphatonos")),
            &[0x0387u32] => Some(Cow::from("anoteleia")),
            &[0x0388u32] => Some(Cow::from("Epsilontonos")),
            &[0x0389u32] => Some(Cow::from("Etatonos")),
            &[0x038Au32] => Some(Cow::from("Iotatonos")),
            &[0x038Cu32] => Some(Cow::from("Omicrontonos")),
            &[0x038Eu32] => Some(Cow::from("Upsilontonos")),
            &[0x038Fu32] => Some(Cow::from("Omegatonos")),
            &[0x0390u32] => Some(Cow::from("iotadieresistonos")),
            &[0x0391u32] => Some(Cow::from("Alpha")),
            &[0x0392u32] => Some(Cow::from("Beta")),
            &[0x0393u32] => Some(Cow::from("Gamma")),
            &[0x0395u32] => Some(Cow::from("Epsilon")),
            &[0x0396u32] => Some(Cow::from("Zeta")),
            &[0x0397u32] => Some(Cow::from("Eta")),
            &[0x0398u32] => Some(Cow::from("Theta")),
            &[0x0399u32] => Some(Cow::from("Iota")),
            &[0x039Au32] => Some(Cow::from("Kappa")),
            &[0x039Bu32] => Some(Cow::from("Lambda")),
            &[0x039Cu32] => Some(Cow::from("Mu")),
            &[0x039Du32] => Some(Cow::from("Nu")),
            &[0x039Eu32] => Some(Cow::from("Xi")),
            &[0x039Fu32] => Some(Cow::from("Omicron")),
            &[0x03A0u32] => Some(Cow::from("Pi")),
            &[0x03A1u32] => Some(Cow::from("Rho")),
            &[0x03A3u32] => Some(Cow::from("Sigma")),
            &[0x03A4u32] => Some(Cow::from("Tau")),
            &[0x03A5u32] => Some(Cow::from("Upsilon")),
            &[0x03A6u32] => Some(Cow::from("Phi")),
            &[0x03A7u32] => Some(Cow::from("Chi")),
            &[0x03A8u32] => Some(Cow::from("Psi")),
            &[0x03AAu32] => Some(Cow::from("Iotadieresis")),
            &[0x03ABu32] => Some(Cow::from("Upsilondieresis")),
            &[0x03ACu32] => Some(Cow::from("alphatonos")),
            &[0x03ADu32] => Some(Cow::from("epsilontonos")),
            &[0x03AEu32] => Some(Cow::from("etatonos")),
            &[0x03AFu32] => Some(Cow::from("iotatonos")),
            &[0x03B0u32] => Some(Cow::from("upsilondieresistonos")),
            &[0x03B1u32] => Some(Cow::from("alpha")),
            &[0x03B2u32] => Some(Cow::from("beta")),
            &[0x03B3u32] => Some(Cow::from("gamma")),
            &[0x03B4u32] => Some(Cow::from("delta")),
            &[0x03B5u32] => Some(Cow::from("epsilon")),
            &[0x03B6u32] => Some(Cow::from("zeta")),
            &[0x03B7u32] => Some(Cow::from("eta")),
            &[0x03B8u32] => Some(Cow::from("theta")),
            &[0x03B9u32] => Some(Cow::from("iota")),
            &[0x03BAu32] => Some(Cow::from("kappa")),
            &[0x03BBu32] => Some(Cow::from("lambda")),
            &[0x03BDu32] => Some(Cow::from("nu")),
            &[0x03BEu32] => Some(Cow::from("xi")),
            &[0x03BFu32] => Some(Cow::from("omicron")),
            &[0x03C0u32] => Some(Cow::from("pi")),
            &[0x03C1u32] => Some(Cow::from("rho")),
            &[0x03C2u32] => Some(Cow::from("sigma1")),
            &[0x03C3u32] => Some(Cow::from("sigma")),
            &[0x03C4u32] => Some(Cow::from("tau")),
            &[0x03C5u32] => Some(Cow::from("upsilon")),
            &[0x03C6u32] => Some(Cow::from("phi")),
            &[0x03C7u32] => Some(Cow::from("chi")),
            &[0x03C8u32] => Some(Cow::from("psi")),
            &[0x03C9u32] => Some(Cow::from("omega")),
            &[0x03CAu32] => Some(Cow::from("iotadieresis")),
            &[0x03CBu32] => Some(Cow::from("upsilondieresis")),
            &[0x03CCu32] => Some(Cow::from("omicrontonos")),
            &[0x03CDu32] => Some(Cow::from("upsilontonos")),
            &[0x03CEu32] => Some(Cow::from("omegatonos")),
            &[0x03D1u32] => Some(Cow::from("theta1")),
            &[0x03D2u32] => Some(Cow::from("Upsilon1")),
            &[0x03D5u32] => Some(Cow::from("phi1")),
            &[0x03D6u32] => Some(Cow::from("omega1")),
            &[0x1E80u32] => Some(Cow::from("Wgrave")),
            &[0x1E81u32] => Some(Cow::from("wgrave")),
            &[0x1E82u32] => Some(Cow::from("Wacute")),
            &[0x1E83u32] => Some(Cow::from("wacute")),
            &[0x1E84u32] => Some(Cow::from("Wdieresis")),
            &[0x1E85u32] => Some(Cow::from("wdieresis")),
            &[0x1EF2u32] => Some(Cow::from("Ygrave")),
            &[0x1EF3u32] => Some(Cow::from("ygrave")),
            &[0x2012u32] => Some(Cow::from("figuredash")),
            &[0x2013u32] => Some(Cow::from("endash")),
            &[0x2014u32] => Some(Cow::from("emdash")),
            &[0x2017u32] => Some(Cow::from("underscoredbl")),
            &[0x2018u32] => Some(Cow::from("quoteleft")),
            &[0x2019u32] => Some(Cow::from("quoteright")),
            &[0x201Au32] => Some(Cow::from("quotesinglbase")),
            &[0x201Bu32] => Some(Cow::from("quotereversed")),
            &[0x201Cu32] => Some(Cow::from("quotedblleft")),
            &[0x201Du32] => Some(Cow::from("quotedblright")),
            &[0x201Eu32] => Some(Cow::from("quotedblbase")),
            &[0x2020u32] => Some(Cow::from("dagger")),
            &[0x2021u32] => Some(Cow::from("daggerdbl")),
            &[0x2022u32] => Some(Cow::from("bullet")),
            &[0x2024u32] => Some(Cow::from("onedotenleader")),
            &[0x2025u32] => Some(Cow::from("twodotenleader")),
            &[0x2026u32] => Some(Cow::from("ellipsis")),
            &[0x2030u32] => Some(Cow::from("perthousand")),
            &[0x2032u32] => Some(Cow::from("minute")),
            &[0x2033u32] => Some(Cow::from("second")),
            &[0x2039u32] => Some(Cow::from("guilsinglleft")),
            &[0x203Au32] => Some(Cow::from("guilsinglright")),
            &[0x203Cu32] => Some(Cow::from("exclamdbl")),
            &[0x2044u32] => Some(Cow::from("fraction")),
            &[0x20A1u32] => Some(Cow::from("colonmonetary")),
            &[0x20A3u32] => Some(Cow::from("franc")),
            &[0x20A4u32] => Some(Cow::from("lira")),
            &[0x20A7u32] => Some(Cow::from("peseta")),
            &[0x20ABu32] => Some(Cow::from("dong")),
            &[0x20ACu32] => Some(Cow::from("Euro")),
            &[0x2111u32] => Some(Cow::from("Ifraktur")),
            &[0x2118u32] => Some(Cow::from("weierstrass")),
            &[0x211Cu32] => Some(Cow::from("Rfraktur")),
            &[0x211Eu32] => Some(Cow::from("prescription")),
            &[0x2122u32] => Some(Cow::from("trademark")),
            &[0x2126u32] => Some(Cow::from("Omega")),
            &[0x212Eu32] => Some(Cow::from("estimated")),
            &[0x2135u32] => Some(Cow::from("aleph")),
            &[0x2153u32] => Some(Cow::from("onethird")),
            &[0x2154u32] => Some(Cow::from("twothirds")),
            &[0x215Bu32] => Some(Cow::from("oneeighth")),
            &[0x215Cu32] => Some(Cow::from("threeeighths")),
            &[0x215Du32] => Some(Cow::from("fiveeighths")),
            &[0x215Eu32] => Some(Cow::from("seveneighths")),
            &[0x2190u32] => Some(Cow::from("arrowleft")),
            &[0x2191u32] => Some(Cow::from("arrowup")),
            &[0x2192u32] => Some(Cow::from("arrowright")),
            &[0x2193u32] => Some(Cow::from("arrowdown")),
            &[0x2194u32] => Some(Cow::from("arrowboth")),
            &[0x2195u32] => Some(Cow::from("arrowupdn")),
            &[0x21A8u32] => Some(Cow::from("arrowupdnbse")),
            &[0x21B5u32] => Some(Cow::from("carriagereturn")),
            &[0x21D0u32] => Some(Cow::from("arrowdblleft")),
            &[0x21D1u32] => Some(Cow::from("arrowdblup")),
            &[0x21D2u32] => Some(Cow::from("arrowdblright")),
            &[0x21D3u32] => Some(Cow::from("arrowdbldown")),
            &[0x21D4u32] => Some(Cow::from("arrowdblboth")),
            &[0x2200u32] => Some(Cow::from("universal")),
            &[0x2202u32] => Some(Cow::from("partialdiff")),
            &[0x2203u32] => Some(Cow::from("existential")),
            &[0x2205u32] => Some(Cow::from("emptyset")),
            &[0x2206u32] => Some(Cow::from("Delta")),
            &[0x2207u32] => Some(Cow::from("gradient")),
            &[0x2208u32] => Some(Cow::from("element")),
            &[0x2209u32] => Some(Cow::from("notelement")),
            &[0x220Bu32] => Some(Cow::from("suchthat")),
            &[0x220Fu32] => Some(Cow::from("product")),
            &[0x2211u32] => Some(Cow::from("summation")),
            &[0x2212u32] => Some(Cow::from("minus")),
            &[0x2217u32] => Some(Cow::from("asteriskmath")),
            &[0x221Au32] => Some(Cow::from("radical")),
            &[0x221Du32] => Some(Cow::from("proportional")),
            &[0x221Eu32] => Some(Cow::from("infinity")),
            &[0x221Fu32] => Some(Cow::from("orthogonal")),
            &[0x2220u32] => Some(Cow::from("angle")),
            &[0x2227u32] => Some(Cow::from("logicaland")),
            &[0x2228u32] => Some(Cow::from("logicalor")),
            &[0x2229u32] => Some(Cow::from("intersection")),
            &[0x222Au32] => Some(Cow::from("union")),
            &[0x222Bu32] => Some(Cow::from("integral")),
            &[0x2234u32] => Some(Cow::from("therefore")),
            &[0x223Cu32] => Some(Cow::from("similar")),
            &[0x2245u32] => Some(Cow::from("congruent")),
            &[0x2248u32] => Some(Cow::from("approxequal")),
            &[0x2260u32] => Some(Cow::from("notequal")),
            &[0x2261u32] => Some(Cow::from("equivalence")),
            &[0x2264u32] => Some(Cow::from("lessequal")),
            &[0x2265u32] => Some(Cow::from("greaterequal")),
            &[0x2282u32] => Some(Cow::from("propersubset")),
            &[0x2283u32] => Some(Cow::from("propersuperset")),
            &[0x2284u32] => Some(Cow::from("notsubset")),
            &[0x2286u32] => Some(Cow::from("reflexsubset")),
            &[0x2287u32] => Some(Cow::from("reflexsuperset")),
            &[0x2295u32] => Some(Cow::from("circleplus")),
            &[0x2297u32] => Some(Cow::from("circlemultiply")),
            &[0x22A5u32] => Some(Cow::from("perpendicular")),
            &[0x22C5u32] => Some(Cow::from("dotmath")),
            &[0x2302u32] => Some(Cow::from("house")),
            &[0x2310u32] => Some(Cow::from("revlogicalnot")),
            &[0x2320u32] => Some(Cow::from("integraltp")),
            &[0x2321u32] => Some(Cow::from("integralbt")),
            &[0x2329u32] => Some(Cow::from("angleleft")),
            &[0x232Au32] => Some(Cow::from("angleright")),
            &[0x2500u32] => Some(Cow::from("SF100000")),
            &[0x2502u32] => Some(Cow::from("SF110000")),
            &[0x250Cu32] => Some(Cow::from("SF010000")),
            &[0x2510u32] => Some(Cow::from("SF030000")),
            &[0x2514u32] => Some(Cow::from("SF020000")),
            &[0x2518u32] => Some(Cow::from("SF040000")),
            &[0x251Cu32] => Some(Cow::from("SF080000")),
            &[0x2524u32] => Some(Cow::from("SF090000")),
            &[0x252Cu32] => Some(Cow::from("SF060000")),
            &[0x2534u32] => Some(Cow::from("SF070000")),
            &[0x253Cu32] => Some(Cow::from("SF050000")),
            &[0x2550u32] => Some(Cow::from("SF430000")),
            &[0x2551u32] => Some(Cow::from("SF240000")),
            &[0x2552u32] => Some(Cow::from("SF510000")),
            &[0x2553u32] => Some(Cow::from("SF520000")),
            &[0x2554u32] => Some(Cow::from("SF390000")),
            &[0x2555u32] => Some(Cow::from("SF220000")),
            &[0x2556u32] => Some(Cow::from("SF210000")),
            &[0x2557u32] => Some(Cow::from("SF250000")),
            &[0x2558u32] => Some(Cow::from("SF500000")),
            &[0x2559u32] => Some(Cow::from("SF490000")),
            &[0x255Au32] => Some(Cow::from("SF380000")),
            &[0x255Bu32] => Some(Cow::from("SF280000")),
            &[0x255Cu32] => Some(Cow::from("SF270000")),
            &[0x255Du32] => Some(Cow::from("SF260000")),
            &[0x255Eu32] => Some(Cow::from("SF360000")),
            &[0x255Fu32] => Some(Cow::from("SF370000")),
            &[0x2560u32] => Some(Cow::from("SF420000")),
            &[0x2561u32] => Some(Cow::from("SF190000")),
            &[0x2562u32] => Some(Cow::from("SF200000")),
            &[0x2563u32] => Some(Cow::from("SF230000")),
            &[0x2564u32] => Some(Cow::from("SF470000")),
            &[0x2565u32] => Some(Cow::from("SF480000")),
            &[0x2566u32] => Some(Cow::from("SF410000")),
            &[0x2567u32] => Some(Cow::from("SF450000")),
            &[0x2568u32] => Some(Cow::from("SF460000")),
            &[0x2569u32] => Some(Cow::from("SF400000")),
            &[0x256Au32] => Some(Cow::from("SF540000")),
            &[0x256Bu32] => Some(Cow::from("SF530000")),
            &[0x256Cu32] => Some(Cow::from("SF440000")),
            &[0x2580u32] => Some(Cow::from("upblock")),
            &[0x2584u32] => Some(Cow::from("dnblock")),
            &[0x2588u32] => Some(Cow::from("block")),
            &[0x258Cu32] => Some(Cow::from("lfblock")),
            &[0x2590u32] => Some(Cow::from("rtblock")),
            &[0x2591u32] => Some(Cow::from("ltshade")),
            &[0x2592u32] => Some(Cow::from("shade")),
            &[0x2593u32] => Some(Cow::from("dkshade")),
            &[0x25A0u32] => Some(Cow::from("filledbox")),
            &[0x25A1u32] => Some(Cow::from("H22073")),
            &[0x25AAu32] => Some(Cow::from("H18543")),
            &[0x25ABu32] => Some(Cow::from("H18551")),
            &[0x25ACu32] => Some(Cow::from("filledrect")),
            &[0x25B2u32] => Some(Cow::from("triagup")),
            &[0x25BAu32] => Some(Cow::from("triagrt")),
            &[0x25BCu32] => Some(Cow::from("triagdn")),
            &[0x25C4u32] => Some(Cow::from("triaglf")),
            &[0x25CAu32] => Some(Cow::from("lozenge")),
            &[0x25CBu32] => Some(Cow::from("circle")),
            &[0x25CFu32] => Some(Cow::from("H18533")),
            &[0x25D8u32] => Some(Cow::from("invbullet")),
            &[0x25D9u32] => Some(Cow::from("invcircle")),
            &[0x25E6u32] => Some(Cow::from("openbullet")),
            &[0x263Au32] => Some(Cow::from("smileface")),
            &[0x263Bu32] => Some(Cow::from("invsmileface")),
            &[0x263Cu32] => Some(Cow::from("sun")),
            &[0x2640u32] => Some(Cow::from("female")),
            &[0x2642u32] => Some(Cow::from("male")),
            &[0x2660u32] => Some(Cow::from("spade")),
            &[0x2663u32] => Some(Cow::from("club")),
            &[0x2665u32] => Some(Cow::from("heart")),
            &[0x2666u32] => Some(Cow::from("diamond")),
            &[0x266Au32] => Some(Cow::from("musicalnote")),
            &[0x266Bu32] => Some(Cow::from("musicalnotedbl")),
        _ => None,
    }
}

impl<'a> GlyphNameStrict<'a> for AdobeGlyphListForNewFonts {
    fn glyph_name_strict(c: char) -> Option<Cow<'a, str>> {
        self::glyph_name_strict(c)
    }
}

impl<'a> GlyphNameVec<'a> for AdobeGlyphListForNewFonts {
    fn glyph_name_vec<'b, S: AsRef<[char]>>(s: S) -> Option<Cow<'a, str>> {
        self::glyph_name_vec(s)
    }
}
