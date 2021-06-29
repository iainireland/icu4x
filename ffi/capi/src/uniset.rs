// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_uniset::UnicodeSet;
use icu_uniset::props::*;
use icu_provider_uprops::*;
use std::ptr;

/// Opaque type for use behind a pointer, is [`UnicodeSet`]
///
/// Can be obtained via [`icu4x_uniset_props_alphabetic()`] and co,
/// and destroyed via [`icu4x_uniset_destroy()`]
pub type ICU4XUniset = UnicodeSet;

fn get_binary_provider() -> BinaryPropertiesDataProvider {
    let root_dir = icu_testdata::paths::data_root().join("uprops");
    BinaryPropertiesDataProvider::new(root_dir)
}

#[no_mangle]
/// FFI version of [`uniset::props::get_alphabetic_property`], see its docs for more details
///
/// # Safety
/// - `provider` should be constructed via one of the functions in [`crate::provider`](crate::provider)
pub unsafe extern "C" fn icu4x_uniset_props_alphabetic() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_alphabetic_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_ascii_hex_digit_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_ascii_hex_digit() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_ascii_hex_digit_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_bidi_control_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_bidi_control() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_bidi_control_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_bidi_mirrored_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_bidi_mirrored() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_bidi_mirrored_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_case_ignorable_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_case_ignorable() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_case_ignorable_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_cased_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_cased() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_cased_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_changes_when_casefolded_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_changes_when_casefolded() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_changes_when_casefolded_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_changes_when_lowercased_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_changes_when_lowercased() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_changes_when_lowercased_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_changes_when_nfkc_casefolded_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_changes_when_nfkc_casefolded() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_changes_when_nfkc_casefolded_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_changes_when_titlecased_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_changes_when_titlecased() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_changes_when_titlecased_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_changes_when_uppercased_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_changes_when_uppercased() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_changes_when_uppercased_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_dash_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_dash() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_dash_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_default_ignorable_code_point_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_default_ignorable_code_point() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_default_ignorable_code_point_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_deprecated_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_deprecated() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_deprecated_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_diacritic_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_diacritic() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_diacritic_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_emoji_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_emoji() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_emoji_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_emoji_component_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_emoji_component() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_emoji_component_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_emoji_modifier_base_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_emoji_modifier_base() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_emoji_modifier_base_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_emoji_modifier_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_emoji_modifier() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_emoji_modifier_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_emoji_presentation_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_emoji_presentation() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_emoji_presentation_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_extended_pictographic_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_extended_pictographic() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_extended_pictographic_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_extender_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_extender() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_extender_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_grapheme_base_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_grapheme_base() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_grapheme_base_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_grapheme_extend_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_grapheme_extend() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_grapheme_extend_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_hex_digit_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_hex_digit() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_hex_digit_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_id_continue_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_id_continue() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_id_continue_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_id_start_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_id_start() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_id_start_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_ideographic_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_ideographic() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_ideographic_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_ids_binary_operator_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_ids_binary_operator() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_ids_binary_operator_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_ids_trinary_operator_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_ids_trinary_operator() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_ids_trinary_operator_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_join_control_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_join_control() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_join_control_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_logical_order_exception_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_logical_order_exception() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_logical_order_exception_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_lowercase_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_lowercase() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_lowercase_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_math_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_math() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_math_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_noncharacter_code_point_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_noncharacter_code_point() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_noncharacter_code_point_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_pattern_syntax_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_pattern_syntax() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_pattern_syntax_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_pattern_white_space_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_pattern_white_space() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_pattern_white_space_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_quotation_mark_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_quotation_mark() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_quotation_mark_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_radical_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_radical() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_radical_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_regional_indicator_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_regional_indicator() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_regional_indicator_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_soft_dotted_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_soft_dotted() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_soft_dotted_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_sentence_terminal_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_sentence_terminal() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_sentence_terminal_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_terminal_punctuation_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_terminal_punctuation() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_terminal_punctuation_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_unified_ideograph_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_unified_ideograph() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_unified_ideograph_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_uppercase_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_uppercase() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_uppercase_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_variation_selector_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_variation_selector() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_variation_selector_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_white_space_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_white_space() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_white_space_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_xid_continue_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_xid_continue() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_xid_continue_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`uniset::props::get_xid_start_property`], see its docs for more details
pub unsafe extern "C" fn icu4x_uniset_props_xid_start() -> *mut ICU4XUniset {
    let provider = get_binary_provider();
    if let Ok(uniset) = get_xid_start_property(&provider) {
        let boxed = Box::new(uniset);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// Destructor for [`ICU4XUniset`].
///
/// # Safety
///
/// `uniset` must be a pointer to a UnicodeSet allocated by `icu4x_uniset_props_*`.
pub unsafe extern "C" fn icu4x_uniset_destroy(uniset: *mut ICU4XUniset) {
    let _ = Box::from_raw(uniset);
}
