#![deny(missing_docs)]

//! # glifnames.rlib
//!
//! Map a character to a glyph name according to the [Adobe Glyph List
//! Specification](https://github.com/adobe-type-tools/agl-specification).
//!
//! ### Example
//! ```
//! use std::borrow::Cow;
//! use glifnames::GlyphName as _;
//! use glifnames::{AGLFN, LegacyAGL};
//!
//! assert_eq!(LegacyAGL::glyph_name_impl('コ' as u32), Some(Cow::from("kokatakana"))); // ko
//! assert_eq!(LegacyAGL::glyph_name_impl('ピ' as u32), Some(Cow::from("pikatakana"))); // pi
//! assert_eq!(LegacyAGL::glyph_name_impl('ペ' as u32), Some(Cow::from("pekatakana"))); // pe
//! assert_eq!(AGLFN::glyph_name_impl('★' as u32), Some(Cow::from("uni2605"))); // 星　    hoshi！！
//! assert_eq!(AGLFN::glyph_name_impl('☺' as u32), Some(Cow::from("smileface"))); // (´・ω・｀)
//! // Failure cases:
//! assert_eq!(AGLFN::glyph_name_impl(0xfffffff), None);
//! assert_eq!(AGLFN::glyph_name(0xfffffff), <&str as Into<Cow<'_, str>>>::into(".invalid.000000000FFFFFFF"));
//! ```

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

/// Zapf Dingbats
mod zapfdingbats;
pub use zapfdingbats::ZapfDingbats;

mod deterministic;
pub use deterministic::*;

/// Traits you can implement
pub mod traits;
pub use traits::{GlyphName, GlyphNameStrict};
