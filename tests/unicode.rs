use glifnames::unicode_glyph_name;

#[test]
fn test_unicode_glyph_name() {
    assert_eq!(&unicode_glyph_name('a'), "uni0061");
    assert_eq!(&unicode_glyph_name('↣'), "uni21A3");
    assert_eq!(&unicode_glyph_name('🕴'), "u1F574");
}
