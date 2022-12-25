use std::convert::TryFrom as _;

/// It is recommended to specify names by using the ‘uni’ prefix for characters in the Basic
/// Multilingual Plane (BMP), and the shorter ‘u’ prefix for characters in the 16 Supplemental
/// Planes.
/// <https://github.com/adobe-type-tools/agl-specification#6-assigning-glyph-names-in-new-fonts>
pub fn unicode_glyph_name(ch: char) -> String {
    let ch = ch as u32;
    if ch <= 0xFFFF {
        // Basic Multilingual Plane
        format!("uni{:04X}", ch)
    } else {
        format!("u{:04X}", ch)
    }
}

/// Glyph names for invalid Unicode codepoints of the format `.invalid.F000000000000000`
pub fn invalid_glyph_name(ch: u32) -> String {
    format!(".invalid.{:016X}", ch)
}

/// Guaranteed to return even when [`char::from`] fails.
pub fn glyph_name(ch: u32) -> String {
    char::try_from(ch).map(|ch| unicode_glyph_name(ch)).unwrap_or_else(|_| invalid_glyph_name(ch))
}
