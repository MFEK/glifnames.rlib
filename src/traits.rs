use std::borrow::Cow;
use super::deterministic::*;

/// All glyph lists implement this
pub trait GlyphName<'a> where Self: GlyphNameStrict<'a> {
    /// Get a glyph name from a [`char`]
    fn glyph_name(ch: u32) -> Cow<'a, str> {
        Self::glyph_name_impl(ch)
            .unwrap_or_else(|| Cow::from(invalid_glyph_name(ch)))
    }

    /// Look up a glyph name for the supplied glyph id, char code pair.
    fn glyph_name_impl(ch: u32) -> Option<Cow<'a, str>> {
        char::try_from(ch).ok().map(|ch| {
            Self::glyph_name_strict(ch)
                .unwrap_or_else(|| Cow::from(unicode_glyph_name(ch)))
        })
    }
}
impl<'a, T> GlyphName<'a> for T where T: GlyphNameStrict<'a> {}

/// Trait to implement on your own glyph lists, required by [`GlyphName`]
pub trait GlyphNameStrict<'a> {
    /// Look up char, return glyph name if available.
    fn glyph_name_strict(c: char) -> Option<Cow<'a, str>>;
}

/// Trait to implement returning a vector on your own glyph lists, required by [`GlyphName`]
pub trait GlyphNameVec<'a> {
    /// Look up [`AsRef<str>`], return glyph name if available.
    fn glyph_name_vec<'b, S: AsRef<[char]>>(s: S) -> Option<Cow<'a, str>>;
}
