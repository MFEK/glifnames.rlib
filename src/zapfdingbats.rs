#![allow(dead_code)]
#![allow(unreachable_patterns)]
//! ITC Zapf Dingbats Glyph List (2002)
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

/// ITC Zapf Dingbats Glyph List (2002)
///
/// ```plain
/// -----------------------------------------------------------
/// Name:          ITC Zapf Dingbats Glyph List
/// Table version: 2.0
/// Date:          September 20, 2002
/// URL:           https://github.com/adobe-type-tools/agl-aglfn
/// 
/// Format: two semicolon-delimited fields:
///   (1) glyph name--upper/lowercase letters and digits
///   (2) Unicode scalar value--four uppercase hexadecimal digits
/// 
/// ```
pub struct ZapfDingbats;

use std::borrow::Cow;
use crate::traits::{GlyphNameStrict, GlyphNameVec};

fn glyph_name_strict<'a>(c: char) -> Option<Cow<'a, str>> {
    match c as u32 {
            0x275E => Some(Cow::from("a100")),             // 
            0x2761 => Some(Cow::from("a101")),             // 
            0x2762 => Some(Cow::from("a102")),             // 
            0x2721 => Some(Cow::from("a10")),              // 
            0x2763 => Some(Cow::from("a103")),             // 
            0x2764 => Some(Cow::from("a104")),             // 
            0x2710 => Some(Cow::from("a105")),             // 
            0x2765 => Some(Cow::from("a106")),             // 
            0x2766 => Some(Cow::from("a107")),             // 
            0x2767 => Some(Cow::from("a108")),             // 
            0x2660 => Some(Cow::from("a109")),             // 
            0x2665 => Some(Cow::from("a110")),             // 
            0x2666 => Some(Cow::from("a111")),             // 
            0x2663 => Some(Cow::from("a112")),             // 
            0x261B => Some(Cow::from("a11")),              // 
            0x2709 => Some(Cow::from("a117")),             // 
            0x2708 => Some(Cow::from("a118")),             // 
            0x2707 => Some(Cow::from("a119")),             // 
            0x2460 => Some(Cow::from("a120")),             // 
            0x2461 => Some(Cow::from("a121")),             // 
            0x2462 => Some(Cow::from("a122")),             // 
            0x261E => Some(Cow::from("a12")),              // 
            0x2463 => Some(Cow::from("a123")),             // 
            0x2464 => Some(Cow::from("a124")),             // 
            0x2465 => Some(Cow::from("a125")),             // 
            0x2466 => Some(Cow::from("a126")),             // 
            0x2701 => Some(Cow::from("a1")),               // 
            0x2467 => Some(Cow::from("a127")),             // 
            0x2468 => Some(Cow::from("a128")),             // 
            0x2469 => Some(Cow::from("a129")),             // 
            0x2776 => Some(Cow::from("a130")),             // 
            0x2777 => Some(Cow::from("a131")),             // 
            0x2778 => Some(Cow::from("a132")),             // 
            0x270C => Some(Cow::from("a13")),              // 
            0x2779 => Some(Cow::from("a133")),             // 
            0x277A => Some(Cow::from("a134")),             // 
            0x277B => Some(Cow::from("a135")),             // 
            0x277C => Some(Cow::from("a136")),             // 
            0x277D => Some(Cow::from("a137")),             // 
            0x277E => Some(Cow::from("a138")),             // 
            0x277F => Some(Cow::from("a139")),             // 
            0x2780 => Some(Cow::from("a140")),             // 
            0x2781 => Some(Cow::from("a141")),             // 
            0x2782 => Some(Cow::from("a142")),             // 
            0x270D => Some(Cow::from("a14")),              // 
            0x2783 => Some(Cow::from("a143")),             // 
            0x2784 => Some(Cow::from("a144")),             // 
            0x2785 => Some(Cow::from("a145")),             // 
            0x2786 => Some(Cow::from("a146")),             // 
            0x2787 => Some(Cow::from("a147")),             // 
            0x2788 => Some(Cow::from("a148")),             // 
            0x2789 => Some(Cow::from("a149")),             // 
            0x278A => Some(Cow::from("a150")),             // 
            0x278B => Some(Cow::from("a151")),             // 
            0x278C => Some(Cow::from("a152")),             // 
            0x270E => Some(Cow::from("a15")),              // 
            0x278D => Some(Cow::from("a153")),             // 
            0x278E => Some(Cow::from("a154")),             // 
            0x278F => Some(Cow::from("a155")),             // 
            0x2790 => Some(Cow::from("a156")),             // 
            0x2791 => Some(Cow::from("a157")),             // 
            0x2792 => Some(Cow::from("a158")),             // 
            0x2793 => Some(Cow::from("a159")),             // 
            0x2794 => Some(Cow::from("a160")),             // 
            0x2192 => Some(Cow::from("a161")),             // 
            0x27A3 => Some(Cow::from("a162")),             // 
            0x270F => Some(Cow::from("a16")),              // 
            0x2194 => Some(Cow::from("a163")),             // 
            0x2195 => Some(Cow::from("a164")),             // 
            0x2799 => Some(Cow::from("a165")),             // 
            0x279B => Some(Cow::from("a166")),             // 
            0x279C => Some(Cow::from("a167")),             // 
            0x279D => Some(Cow::from("a168")),             // 
            0x279E => Some(Cow::from("a169")),             // 
            0x279F => Some(Cow::from("a170")),             // 
            0x27A0 => Some(Cow::from("a171")),             // 
            0x27A1 => Some(Cow::from("a172")),             // 
            0x2711 => Some(Cow::from("a17")),              // 
            0x27A2 => Some(Cow::from("a173")),             // 
            0x27A4 => Some(Cow::from("a174")),             // 
            0x27A5 => Some(Cow::from("a175")),             // 
            0x27A6 => Some(Cow::from("a176")),             // 
            0x27A7 => Some(Cow::from("a177")),             // 
            0x27A8 => Some(Cow::from("a178")),             // 
            0x27A9 => Some(Cow::from("a179")),             // 
            0x27AB => Some(Cow::from("a180")),             // 
            0x27AD => Some(Cow::from("a181")),             // 
            0x27AF => Some(Cow::from("a182")),             // 
            0x2712 => Some(Cow::from("a18")),              // 
            0x27B2 => Some(Cow::from("a183")),             // 
            0x27B3 => Some(Cow::from("a184")),             // 
            0x27B5 => Some(Cow::from("a185")),             // 
            0x27B8 => Some(Cow::from("a186")),             // 
            0x27BA => Some(Cow::from("a187")),             // 
            0x27BB => Some(Cow::from("a188")),             // 
            0x27BC => Some(Cow::from("a189")),             // 
            0x27BD => Some(Cow::from("a190")),             // 
            0x27BE => Some(Cow::from("a191")),             // 
            0x279A => Some(Cow::from("a192")),             // 
            0x2713 => Some(Cow::from("a19")),              // 
            0x27AA => Some(Cow::from("a193")),             // 
            0x27B6 => Some(Cow::from("a194")),             // 
            0x27B9 => Some(Cow::from("a195")),             // 
            0x2798 => Some(Cow::from("a196")),             // 
            0x27B4 => Some(Cow::from("a197")),             // 
            0x27B7 => Some(Cow::from("a198")),             // 
            0x27AC => Some(Cow::from("a199")),             // 
            0x27AE => Some(Cow::from("a200")),             // 
            0x27B1 => Some(Cow::from("a201")),             // 
            0x2703 => Some(Cow::from("a202")),             // 
            0x2714 => Some(Cow::from("a20")),              // 
            0x2750 => Some(Cow::from("a203")),             // 
            0x2752 => Some(Cow::from("a204")),             // 
            0x276E => Some(Cow::from("a205")),             // 
            0x2770 => Some(Cow::from("a206")),             // 
            0x2715 => Some(Cow::from("a21")),              // 
            0x2716 => Some(Cow::from("a22")),              // 
            0x2702 => Some(Cow::from("a2")),               // 
            0x2717 => Some(Cow::from("a23")),              // 
            0x2718 => Some(Cow::from("a24")),              // 
            0x2719 => Some(Cow::from("a25")),              // 
            0x271A => Some(Cow::from("a26")),              // 
            0x271B => Some(Cow::from("a27")),              // 
            0x271C => Some(Cow::from("a28")),              // 
            0x2722 => Some(Cow::from("a29")),              // 
            0x2723 => Some(Cow::from("a30")),              // 
            0x2724 => Some(Cow::from("a31")),              // 
            0x2725 => Some(Cow::from("a32")),              // 
            0x2704 => Some(Cow::from("a3")),               // 
            0x2726 => Some(Cow::from("a33")),              // 
            0x2727 => Some(Cow::from("a34")),              // 
            0x2605 => Some(Cow::from("a35")),              // 
            0x2729 => Some(Cow::from("a36")),              // 
            0x272A => Some(Cow::from("a37")),              // 
            0x272B => Some(Cow::from("a38")),              // 
            0x272C => Some(Cow::from("a39")),              // 
            0x272D => Some(Cow::from("a40")),              // 
            0x272E => Some(Cow::from("a41")),              // 
            0x272F => Some(Cow::from("a42")),              // 
            0x260E => Some(Cow::from("a4")),               // 
            0x2730 => Some(Cow::from("a43")),              // 
            0x2731 => Some(Cow::from("a44")),              // 
            0x2732 => Some(Cow::from("a45")),              // 
            0x2733 => Some(Cow::from("a46")),              // 
            0x2734 => Some(Cow::from("a47")),              // 
            0x2735 => Some(Cow::from("a48")),              // 
            0x2736 => Some(Cow::from("a49")),              // 
            0x2737 => Some(Cow::from("a50")),              // 
            0x2738 => Some(Cow::from("a51")),              // 
            0x2739 => Some(Cow::from("a52")),              // 
            0x2706 => Some(Cow::from("a5")),               // 
            0x273A => Some(Cow::from("a53")),              // 
            0x273B => Some(Cow::from("a54")),              // 
            0x273C => Some(Cow::from("a55")),              // 
            0x273D => Some(Cow::from("a56")),              // 
            0x273E => Some(Cow::from("a57")),              // 
            0x273F => Some(Cow::from("a58")),              // 
            0x2740 => Some(Cow::from("a59")),              // 
            0x2741 => Some(Cow::from("a60")),              // 
            0x2742 => Some(Cow::from("a61")),              // 
            0x2743 => Some(Cow::from("a62")),              // 
            0x271D => Some(Cow::from("a6")),               // 
            0x2744 => Some(Cow::from("a63")),              // 
            0x2745 => Some(Cow::from("a64")),              // 
            0x2746 => Some(Cow::from("a65")),              // 
            0x2747 => Some(Cow::from("a66")),              // 
            0x2748 => Some(Cow::from("a67")),              // 
            0x2749 => Some(Cow::from("a68")),              // 
            0x274A => Some(Cow::from("a69")),              // 
            0x274B => Some(Cow::from("a70")),              // 
            0x25CF => Some(Cow::from("a71")),              // 
            0x274D => Some(Cow::from("a72")),              // 
            0x271E => Some(Cow::from("a7")),               // 
            0x25A0 => Some(Cow::from("a73")),              // 
            0x274F => Some(Cow::from("a74")),              // 
            0x2751 => Some(Cow::from("a75")),              // 
            0x25B2 => Some(Cow::from("a76")),              // 
            0x25BC => Some(Cow::from("a77")),              // 
            0x25C6 => Some(Cow::from("a78")),              // 
            0x2756 => Some(Cow::from("a79")),              // 
            0x25D7 => Some(Cow::from("a81")),              // 
            0x2758 => Some(Cow::from("a82")),              // 
            0x271F => Some(Cow::from("a8")),               // 
            0x2759 => Some(Cow::from("a83")),              // 
            0x275A => Some(Cow::from("a84")),              // 
            0x276F => Some(Cow::from("a85")),              // 
            0x2771 => Some(Cow::from("a86")),              // 
            0x2772 => Some(Cow::from("a87")),              // 
            0x2773 => Some(Cow::from("a88")),              // 
            0x2768 => Some(Cow::from("a89")),              // 
            0x2769 => Some(Cow::from("a90")),              // 
            0x276C => Some(Cow::from("a91")),              // 
            0x276D => Some(Cow::from("a92")),              // 
            0x2720 => Some(Cow::from("a9")),               // 
            0x276A => Some(Cow::from("a93")),              // 
            0x276B => Some(Cow::from("a94")),              // 
            0x2774 => Some(Cow::from("a95")),              // 
            0x2775 => Some(Cow::from("a96")),              // 
            0x275B => Some(Cow::from("a97")),              // 
            0x275C => Some(Cow::from("a98")),              // 
            0x275D => Some(Cow::from("a99")),              // 
        _ => None,
    }
}
fn glyph_name_vec<'a, S: AsRef<[char]>>(s: S) -> Option<Cow<'a, str>> {
    match s.as_ref().into_iter().map(|c|*c as u32).collect::<Vec<u32>>().as_slice() {
            &[0x275Eu32] => Some(Cow::from("a100")),
            &[0x2761u32] => Some(Cow::from("a101")),
            &[0x2762u32] => Some(Cow::from("a102")),
            &[0x2721u32] => Some(Cow::from("a10")),
            &[0x2763u32] => Some(Cow::from("a103")),
            &[0x2764u32] => Some(Cow::from("a104")),
            &[0x2710u32] => Some(Cow::from("a105")),
            &[0x2765u32] => Some(Cow::from("a106")),
            &[0x2766u32] => Some(Cow::from("a107")),
            &[0x2767u32] => Some(Cow::from("a108")),
            &[0x2660u32] => Some(Cow::from("a109")),
            &[0x2665u32] => Some(Cow::from("a110")),
            &[0x2666u32] => Some(Cow::from("a111")),
            &[0x2663u32] => Some(Cow::from("a112")),
            &[0x261Bu32] => Some(Cow::from("a11")),
            &[0x2709u32] => Some(Cow::from("a117")),
            &[0x2708u32] => Some(Cow::from("a118")),
            &[0x2707u32] => Some(Cow::from("a119")),
            &[0x2460u32] => Some(Cow::from("a120")),
            &[0x2461u32] => Some(Cow::from("a121")),
            &[0x2462u32] => Some(Cow::from("a122")),
            &[0x261Eu32] => Some(Cow::from("a12")),
            &[0x2463u32] => Some(Cow::from("a123")),
            &[0x2464u32] => Some(Cow::from("a124")),
            &[0x2465u32] => Some(Cow::from("a125")),
            &[0x2466u32] => Some(Cow::from("a126")),
            &[0x2701u32] => Some(Cow::from("a1")),
            &[0x2467u32] => Some(Cow::from("a127")),
            &[0x2468u32] => Some(Cow::from("a128")),
            &[0x2469u32] => Some(Cow::from("a129")),
            &[0x2776u32] => Some(Cow::from("a130")),
            &[0x2777u32] => Some(Cow::from("a131")),
            &[0x2778u32] => Some(Cow::from("a132")),
            &[0x270Cu32] => Some(Cow::from("a13")),
            &[0x2779u32] => Some(Cow::from("a133")),
            &[0x277Au32] => Some(Cow::from("a134")),
            &[0x277Bu32] => Some(Cow::from("a135")),
            &[0x277Cu32] => Some(Cow::from("a136")),
            &[0x277Du32] => Some(Cow::from("a137")),
            &[0x277Eu32] => Some(Cow::from("a138")),
            &[0x277Fu32] => Some(Cow::from("a139")),
            &[0x2780u32] => Some(Cow::from("a140")),
            &[0x2781u32] => Some(Cow::from("a141")),
            &[0x2782u32] => Some(Cow::from("a142")),
            &[0x270Du32] => Some(Cow::from("a14")),
            &[0x2783u32] => Some(Cow::from("a143")),
            &[0x2784u32] => Some(Cow::from("a144")),
            &[0x2785u32] => Some(Cow::from("a145")),
            &[0x2786u32] => Some(Cow::from("a146")),
            &[0x2787u32] => Some(Cow::from("a147")),
            &[0x2788u32] => Some(Cow::from("a148")),
            &[0x2789u32] => Some(Cow::from("a149")),
            &[0x278Au32] => Some(Cow::from("a150")),
            &[0x278Bu32] => Some(Cow::from("a151")),
            &[0x278Cu32] => Some(Cow::from("a152")),
            &[0x270Eu32] => Some(Cow::from("a15")),
            &[0x278Du32] => Some(Cow::from("a153")),
            &[0x278Eu32] => Some(Cow::from("a154")),
            &[0x278Fu32] => Some(Cow::from("a155")),
            &[0x2790u32] => Some(Cow::from("a156")),
            &[0x2791u32] => Some(Cow::from("a157")),
            &[0x2792u32] => Some(Cow::from("a158")),
            &[0x2793u32] => Some(Cow::from("a159")),
            &[0x2794u32] => Some(Cow::from("a160")),
            &[0x2192u32] => Some(Cow::from("a161")),
            &[0x27A3u32] => Some(Cow::from("a162")),
            &[0x270Fu32] => Some(Cow::from("a16")),
            &[0x2194u32] => Some(Cow::from("a163")),
            &[0x2195u32] => Some(Cow::from("a164")),
            &[0x2799u32] => Some(Cow::from("a165")),
            &[0x279Bu32] => Some(Cow::from("a166")),
            &[0x279Cu32] => Some(Cow::from("a167")),
            &[0x279Du32] => Some(Cow::from("a168")),
            &[0x279Eu32] => Some(Cow::from("a169")),
            &[0x279Fu32] => Some(Cow::from("a170")),
            &[0x27A0u32] => Some(Cow::from("a171")),
            &[0x27A1u32] => Some(Cow::from("a172")),
            &[0x2711u32] => Some(Cow::from("a17")),
            &[0x27A2u32] => Some(Cow::from("a173")),
            &[0x27A4u32] => Some(Cow::from("a174")),
            &[0x27A5u32] => Some(Cow::from("a175")),
            &[0x27A6u32] => Some(Cow::from("a176")),
            &[0x27A7u32] => Some(Cow::from("a177")),
            &[0x27A8u32] => Some(Cow::from("a178")),
            &[0x27A9u32] => Some(Cow::from("a179")),
            &[0x27ABu32] => Some(Cow::from("a180")),
            &[0x27ADu32] => Some(Cow::from("a181")),
            &[0x27AFu32] => Some(Cow::from("a182")),
            &[0x2712u32] => Some(Cow::from("a18")),
            &[0x27B2u32] => Some(Cow::from("a183")),
            &[0x27B3u32] => Some(Cow::from("a184")),
            &[0x27B5u32] => Some(Cow::from("a185")),
            &[0x27B8u32] => Some(Cow::from("a186")),
            &[0x27BAu32] => Some(Cow::from("a187")),
            &[0x27BBu32] => Some(Cow::from("a188")),
            &[0x27BCu32] => Some(Cow::from("a189")),
            &[0x27BDu32] => Some(Cow::from("a190")),
            &[0x27BEu32] => Some(Cow::from("a191")),
            &[0x279Au32] => Some(Cow::from("a192")),
            &[0x2713u32] => Some(Cow::from("a19")),
            &[0x27AAu32] => Some(Cow::from("a193")),
            &[0x27B6u32] => Some(Cow::from("a194")),
            &[0x27B9u32] => Some(Cow::from("a195")),
            &[0x2798u32] => Some(Cow::from("a196")),
            &[0x27B4u32] => Some(Cow::from("a197")),
            &[0x27B7u32] => Some(Cow::from("a198")),
            &[0x27ACu32] => Some(Cow::from("a199")),
            &[0x27AEu32] => Some(Cow::from("a200")),
            &[0x27B1u32] => Some(Cow::from("a201")),
            &[0x2703u32] => Some(Cow::from("a202")),
            &[0x2714u32] => Some(Cow::from("a20")),
            &[0x2750u32] => Some(Cow::from("a203")),
            &[0x2752u32] => Some(Cow::from("a204")),
            &[0x276Eu32] => Some(Cow::from("a205")),
            &[0x2770u32] => Some(Cow::from("a206")),
            &[0x2715u32] => Some(Cow::from("a21")),
            &[0x2716u32] => Some(Cow::from("a22")),
            &[0x2702u32] => Some(Cow::from("a2")),
            &[0x2717u32] => Some(Cow::from("a23")),
            &[0x2718u32] => Some(Cow::from("a24")),
            &[0x2719u32] => Some(Cow::from("a25")),
            &[0x271Au32] => Some(Cow::from("a26")),
            &[0x271Bu32] => Some(Cow::from("a27")),
            &[0x271Cu32] => Some(Cow::from("a28")),
            &[0x2722u32] => Some(Cow::from("a29")),
            &[0x2723u32] => Some(Cow::from("a30")),
            &[0x2724u32] => Some(Cow::from("a31")),
            &[0x2725u32] => Some(Cow::from("a32")),
            &[0x2704u32] => Some(Cow::from("a3")),
            &[0x2726u32] => Some(Cow::from("a33")),
            &[0x2727u32] => Some(Cow::from("a34")),
            &[0x2605u32] => Some(Cow::from("a35")),
            &[0x2729u32] => Some(Cow::from("a36")),
            &[0x272Au32] => Some(Cow::from("a37")),
            &[0x272Bu32] => Some(Cow::from("a38")),
            &[0x272Cu32] => Some(Cow::from("a39")),
            &[0x272Du32] => Some(Cow::from("a40")),
            &[0x272Eu32] => Some(Cow::from("a41")),
            &[0x272Fu32] => Some(Cow::from("a42")),
            &[0x260Eu32] => Some(Cow::from("a4")),
            &[0x2730u32] => Some(Cow::from("a43")),
            &[0x2731u32] => Some(Cow::from("a44")),
            &[0x2732u32] => Some(Cow::from("a45")),
            &[0x2733u32] => Some(Cow::from("a46")),
            &[0x2734u32] => Some(Cow::from("a47")),
            &[0x2735u32] => Some(Cow::from("a48")),
            &[0x2736u32] => Some(Cow::from("a49")),
            &[0x2737u32] => Some(Cow::from("a50")),
            &[0x2738u32] => Some(Cow::from("a51")),
            &[0x2739u32] => Some(Cow::from("a52")),
            &[0x2706u32] => Some(Cow::from("a5")),
            &[0x273Au32] => Some(Cow::from("a53")),
            &[0x273Bu32] => Some(Cow::from("a54")),
            &[0x273Cu32] => Some(Cow::from("a55")),
            &[0x273Du32] => Some(Cow::from("a56")),
            &[0x273Eu32] => Some(Cow::from("a57")),
            &[0x273Fu32] => Some(Cow::from("a58")),
            &[0x2740u32] => Some(Cow::from("a59")),
            &[0x2741u32] => Some(Cow::from("a60")),
            &[0x2742u32] => Some(Cow::from("a61")),
            &[0x2743u32] => Some(Cow::from("a62")),
            &[0x271Du32] => Some(Cow::from("a6")),
            &[0x2744u32] => Some(Cow::from("a63")),
            &[0x2745u32] => Some(Cow::from("a64")),
            &[0x2746u32] => Some(Cow::from("a65")),
            &[0x2747u32] => Some(Cow::from("a66")),
            &[0x2748u32] => Some(Cow::from("a67")),
            &[0x2749u32] => Some(Cow::from("a68")),
            &[0x274Au32] => Some(Cow::from("a69")),
            &[0x274Bu32] => Some(Cow::from("a70")),
            &[0x25CFu32] => Some(Cow::from("a71")),
            &[0x274Du32] => Some(Cow::from("a72")),
            &[0x271Eu32] => Some(Cow::from("a7")),
            &[0x25A0u32] => Some(Cow::from("a73")),
            &[0x274Fu32] => Some(Cow::from("a74")),
            &[0x2751u32] => Some(Cow::from("a75")),
            &[0x25B2u32] => Some(Cow::from("a76")),
            &[0x25BCu32] => Some(Cow::from("a77")),
            &[0x25C6u32] => Some(Cow::from("a78")),
            &[0x2756u32] => Some(Cow::from("a79")),
            &[0x25D7u32] => Some(Cow::from("a81")),
            &[0x2758u32] => Some(Cow::from("a82")),
            &[0x271Fu32] => Some(Cow::from("a8")),
            &[0x2759u32] => Some(Cow::from("a83")),
            &[0x275Au32] => Some(Cow::from("a84")),
            &[0x276Fu32] => Some(Cow::from("a85")),
            &[0x2771u32] => Some(Cow::from("a86")),
            &[0x2772u32] => Some(Cow::from("a87")),
            &[0x2773u32] => Some(Cow::from("a88")),
            &[0x2768u32] => Some(Cow::from("a89")),
            &[0x2769u32] => Some(Cow::from("a90")),
            &[0x276Cu32] => Some(Cow::from("a91")),
            &[0x276Du32] => Some(Cow::from("a92")),
            &[0x2720u32] => Some(Cow::from("a9")),
            &[0x276Au32] => Some(Cow::from("a93")),
            &[0x276Bu32] => Some(Cow::from("a94")),
            &[0x2774u32] => Some(Cow::from("a95")),
            &[0x2775u32] => Some(Cow::from("a96")),
            &[0x275Bu32] => Some(Cow::from("a97")),
            &[0x275Cu32] => Some(Cow::from("a98")),
            &[0x275Du32] => Some(Cow::from("a99")),
        _ => None,
    }
}

impl<'a> GlyphNameStrict<'a> for ZapfDingbats {
    fn glyph_name_strict(c: char) -> Option<Cow<'a, str>> {
        self::glyph_name_strict(c)
    }
}

impl<'a> GlyphNameVec<'a> for ZapfDingbats {
    fn glyph_name_vec<'b, S: AsRef<[char]>>(s: S) -> Option<Cow<'a, str>> {
        self::glyph_name_vec(s)
    }
}
