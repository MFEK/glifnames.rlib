#![deny(missing_docs)]

//! # glifnames.rlib
//!
//! Map a character to a glyph name according to the
//! [Adobe Glyph List Specification](https://github.com/adobe-type-tools/agl-specification).
//!
//! ### Example
//! ```
//! use std::borrow::Cow;
//! use glyph_names::GlyphName as _;
//! use glyph_names::AGLFN;
//!
//! assert_eq!(AGLFN::glyph_name_impl('a' as u32), Some(Cow::from("a")));
//! assert_eq!(AGLFN::glyph_name_impl('%' as u32), Some(Cow::from("percent")));
//! assert_eq!(AGLFN::glyph_name_impl('☺' as u32), Some(Cow::from("smileface")));
//! assert_eq!(AGLFN::glyph_name_impl('↣' as u32), Some(Cow::from("uni21A3")));
//! assert_eq!(AGLFN::glyph_name_impl('🕴' as u32), Some(Cow::from("u1F574")));
//! assert_eq!(AGLFN::glyph_name_impl(0x110000), None);
//! assert_eq!(AGLFN::glyph_name(0x110000), Cow::from(".invalid.0000000000110000"));
//! ```

use std::borrow::Cow;
use std::convert::TryFrom;

/// Adobe Glyph List For New Fonts
mod aglfn;
#[doc(hidden)]
pub use aglfn::AdobeGlyphListForNewFonts;
pub use aglfn::AdobeGlyphListForNewFonts as AGLFN;

/// Adobe Glyph List (legacy)
mod legacy_agl;
#[doc(hidden)]
pub use legacy_agl::LegacyAdobeGlyphList;
pub use legacy_agl::LegacyAdobeGlyphList as LegacyAGL;

mod deterministic;
pub use deterministic::*;

/// All glyph lists implement this
pub trait GlyphName<'a> where Self: GlyphNameOpt<'a> {
    /// Get a glyph name from a [`char`]
    fn glyph_name(ch: u32) -> Cow<'a, str> {
        Self::glyph_name_impl(ch)
            .unwrap_or_else(|| Cow::from(invalid_glyph_name(ch)))
    }

    /// Look up a glyph name for the supplied glyph id, char code pair.
    fn glyph_name_impl(ch: u32) -> Option<Cow<'a, str>> {
        char::try_from(ch).ok().map(|ch| {
            Self::glyph_name_opt(ch)
                .unwrap_or_else(|| Cow::from(unicode_glyph_name(ch)))
        })
    }
}
impl<'a, T> GlyphName<'a> for T where T: GlyphNameOpt<'a> {}

/// Trait to implement on your own glyph lists, required by [`GlyphName`]
pub trait GlyphNameOpt<'a> {
    /// Look up char, return glyph name if available.
    fn glyph_name_opt(c: char) -> Option<Cow<'a, str>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_glyph_name() {
        assert_eq!(&unicode_glyph_name('a'), "uni0061");
        assert_eq!(&unicode_glyph_name('↣'), "uni21A3");
        assert_eq!(&unicode_glyph_name('🕴'), "u1F574");
    }
}
