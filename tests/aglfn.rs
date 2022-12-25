use glifnames::AGLFN;
use glifnames::{GlyphName as _, GlyphNameStrict as _};
use std::borrow::Cow;

#[test]
fn test_aglfn() {
    assert_eq!(AGLFN::glyph_name_impl('a' as u32), Some(Cow::from("a")));
    assert_eq!(AGLFN::glyph_name_impl('%' as u32), Some(Cow::from("percent")));
    assert_eq!(AGLFN::glyph_name_impl('↣' as u32), Some(Cow::from("uni21A3")));
    assert_eq!(AGLFN::glyph_name_impl('🕴' as u32), Some(Cow::from("u1F574")));
    assert_eq!(AGLFN::glyph_name_impl(0x110000), None);
    assert_eq!(AGLFN::glyph_name(0x110000), Cow::from(".invalid.0000000000110000"));
    assert_eq!(AGLFN::glyph_name_strict('コ'), None);
}
