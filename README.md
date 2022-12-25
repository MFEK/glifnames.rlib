glifnames.rlib v0.2.0 (2022-12-25)
==================================

<div align="center">
  <a href="https://docs.rs/glifnames">
    <img src="https://docs.rs/glifnames/badge.svg" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/glifnames">
    <img src="https://img.shields.io/crates/v/glifnames.svg" alt="Version">
  </a>
  <a href="https://github.com/yeslogic/glifnames/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/glifnames.svg" alt="License">
  </a>
</div>

<br>

Mapping of characters to glyph names according to the [Adobe Glyph List
Specification][agl-specification].

This is a fork
--------------

This is MFEK's fork of [glyph-names](https://github.com/yeslogic/glyph-names/).

Notable differences:
- Completely redone API.
- Support for the other files in the Adobe Glyph List format and in Adobe's AGL repository.

### Planned changes
- Support for user-provided files (`nom` parser).

Usage
-----

```rust
use std::borrow::Cow;
use glyph_names::glyph_name;

fn main() {
    assert_eq!(glyph_name('a' as u32), Some(Cow::from("a")));
    assert_eq!(glyph_name('%' as u32), Some(Cow::from("percent")));
    assert_eq!(glyph_name('☺' as u32), Some(Cow::from("smileface")));
    assert_eq!(glyph_name('↣' as u32), Some(Cow::from("uni21A3")));
    assert_eq!(glyph_name('🕴' as u32), Some(Cow::from("u1F574")));
    assert_eq!(glyph_name(0x110000), None);
}
```

Notes
-----

### Regenerating aglfn.rs

1. Ensure you have the [`agl-aglfn` submodule][agl-aglfn] checked out (`git submodule update --init`).
2. Run `make`.

License
-------

Like all MFEK modules, our forked version is relicensed as Apache-2.0 _only_.

The original version is 3-clause BSD. See `LICENSE.original` file.

    Copyright 2022 Fredrick R. Brennan <copypaste@kittens.ph>
    Copyright 2020 YesLogic Pty. Ltd.
    Copyright 2002-2019 Adobe (http://www.adobe.com/).

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

[ucd-generate]: https://github.com/BurntSushi/ucd-generate
[agl-specification]: https://github.com/adobe-type-tools/agl-specification
[agl-aglfn]: https://github.com/adobe-type-tools/agl-aglfn/
