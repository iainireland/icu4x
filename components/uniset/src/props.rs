// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! *** Note: DO NOT USE THESE APIs FOR NOW. ****
//!  Performance improvements and other fixes are still needed on this component.

#![allow(clippy::unreadable_literal, dead_code)]

use crate::enum_props::*;
use crate::provider::*;
use crate::{UnicodeSet, UnicodeSetError};
use core::convert::TryInto;
use icu_provider::prelude::*;

type UnisetResult = Result<UnicodeSet, UnicodeSetError>;

// helper fn
fn get_prop<'data, D>(ppucd_provider: &D, resc_key: ResourceKey) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    let data_req = DataRequest {
        resource_path: ResourcePath {
            key: resc_key,
            options: ResourceOptions { variant: None, langid: None },
        },
    };
    let resp: DataResponse<UnicodePropertyV1Marker> = ppucd_provider.load_payload(&data_req)?;

    let ppucd_property_payload: DataPayload<UnicodePropertyV1Marker> = resp.take_payload()?;
    let ppucd_property: UnicodePropertyV1 = ppucd_property_payload.get().clone();
    ppucd_property.try_into()
}

//
// Binary property getter fns
//

pub fn get_ascii_hex_digit_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::ASCII_HEX_DIGIT_V1)
}

pub fn get_alnum_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::ALNUM_V1)
}

pub fn get_alphabetic_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::ALPHABETIC_V1)
}

pub fn get_bidi_control_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::BIDI_CONTROL_V1)
}

pub fn get_bidi_mirrored_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::BIDI_MIRRORED_V1)
}

pub fn get_blank_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::BLANK_V1)
}

pub fn get_cased_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::CASED_V1)
}

pub fn get_case_ignorable_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::CASE_IGNORABLE_V1)
}

pub fn get_full_composition_exclusion_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::FULL_COMPOSITION_EXCLUSION_V1)
}

pub fn get_changes_when_casefolded_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::CHANGES_WHEN_CASEFOLDED_V1)
}

pub fn get_changes_when_casemapped_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::CHANGES_WHEN_CASEMAPPED_V1)
}

pub fn get_changes_when_nfkc_casefolded_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::CHANGES_WHEN_NFKC_CASEFOLDED_V1)
}

pub fn get_changes_when_lowercased_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::CHANGES_WHEN_LOWERCASED_V1)
}

pub fn get_changes_when_titlecased_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::CHANGES_WHEN_TITLECASED_V1)
}

pub fn get_changes_when_uppercased_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::CHANGES_WHEN_UPPERCASED_V1)
}

pub fn get_dash_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::DASH_V1)
}

pub fn get_deprecated_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::DEPRECATED_V1)
}

pub fn get_default_ignorable_code_point_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::DEFAULT_IGNORABLE_CODE_POINT_V1)
}

pub fn get_diacritic_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::DIACRITIC_V1)
}

pub fn get_emoji_modifier_base_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::EMOJI_MODIFIER_BASE_V1)
}

pub fn get_emoji_component_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::EMOJI_COMPONENT_V1)
}

pub fn get_emoji_modifier_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::EMOJI_MODIFIER_V1)
}

pub fn get_emoji_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::EMOJI_V1)
}

pub fn get_emoji_presentation_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::EMOJI_PRESENTATION_V1)
}

pub fn get_extender_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::EXTENDER_V1)
}

pub fn get_extended_pictographic_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::EXTENDED_PICTOGRAPHIC_V1)
}

pub fn get_graph_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::GRAPH_V1)
}

pub fn get_grapheme_base_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::GRAPHEME_BASE_V1)
}

pub fn get_grapheme_extend_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::GRAPHEME_EXTEND_V1)
}

pub fn get_grapheme_link_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::GRAPHEME_LINK_V1)
}

pub fn get_hex_digit_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::HEX_DIGIT_V1)
}

pub fn get_hyphen_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::HYPHEN_V1)
}

pub fn get_id_continue_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::ID_CONTINUE_V1)
}

pub fn get_ideographic_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::IDEOGRAPHIC_V1)
}

pub fn get_id_start_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::ID_START_V1)
}

pub fn get_ids_binary_operator_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::IDS_BINARY_OPERATOR_V1)
}

pub fn get_ids_trinary_operator_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::IDS_TRINARY_OPERATOR_V1)
}

pub fn get_join_control_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::JOIN_CONTROL_V1)
}

pub fn get_logical_order_exception_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::LOGICAL_ORDER_EXCEPTION_V1)
}

pub fn get_lowercase_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::LOWERCASE_V1)
}

pub fn get_math_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::MATH_V1)
}

pub fn get_noncharacter_code_point_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::NONCHARACTER_CODE_POINT_V1)
}

pub fn get_nfc_inert_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::NFC_INERT_V1)
}

pub fn get_nfd_inert_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::NFD_INERT_V1)
}

pub fn get_nfkc_inert_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::NFKC_INERT_V1)
}

pub fn get_nfkd_inert_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::NFKD_INERT_V1)
}

pub fn get_pattern_syntax_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::PATTERN_SYNTAX_V1)
}

pub fn get_pattern_white_space_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::PATTERN_WHITE_SPACE_V1)
}

pub fn get_prepended_concatenation_mark_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::PREPENDED_CONCATENATION_MARK_V1)
}

pub fn get_print_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::PRINT_V1)
}

pub fn get_quotation_mark_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::QUOTATION_MARK_V1)
}

pub fn get_radical_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::RADICAL_V1)
}

pub fn get_regional_indicator_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::REGIONAL_INDICATOR_V1)
}

pub fn get_soft_dotted_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::SOFT_DOTTED_V1)
}

pub fn get_segment_starter_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::SEGMENT_STARTER_V1)
}

pub fn get_case_sensitive_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::CASE_SENSITIVE_V1)
}

pub fn get_sentence_terminal_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::SENTENCE_TERMINAL_V1)
}

pub fn get_terminal_punctuation_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::TERMINAL_PUNCTUATION_V1)
}

pub fn get_unified_ideograph_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::UNIFIED_IDEOGRAPH_V1)
}

pub fn get_uppercase_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::UPPERCASE_V1)
}

pub fn get_variation_selector_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::VARIATION_SELECTOR_V1)
}

pub fn get_white_space_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::WHITE_SPACE_V1)
}

pub fn get_xdigit_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::XDIGIT_V1)
}

pub fn get_xid_continue_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::XID_CONTINUE_V1)
}

pub fn get_xid_start_property<'data, D>(provider: &D) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_prop(provider, key::XID_START_V1)
}

//
// Enumerated property getter fns
//

/// Return a [`UnicodeSet`] for a particular value of the General_Category Unicode enumerated property
/// General_Category specifies enumerated Unicode general category types.
/// See https://www.unicode.org/reports/tr44/ .
pub fn get_general_category_val_set<'data, D>(provider: &D, enum_val: GeneralCategory) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    let key = match enum_val {
        GeneralCategory::Other => key::GENERAL_CATEGORY_OTHER_V1,
        GeneralCategory::Control => key::GENERAL_CATEGORY_CONTROL_V1,
        GeneralCategory::Format => key::GENERAL_CATEGORY_FORMAT_V1,
        GeneralCategory::Unassigned => key::GENERAL_CATEGORY_UNASSIGNED_V1,
        GeneralCategory::PrivateUse => key::GENERAL_CATEGORY_PRIVATE_USE_V1,
        GeneralCategory::Surrogate => key::GENERAL_CATEGORY_SURROGATE_V1,
        GeneralCategory::Letter => key::GENERAL_CATEGORY_LETTER_V1,
        GeneralCategory::CasedLetter => key::GENERAL_CATEGORY_CASED_LETTER_V1,
        GeneralCategory::LowercaseLetter => key::GENERAL_CATEGORY_LOWERCASE_LETTER_V1,
        GeneralCategory::ModifierLetter => key::GENERAL_CATEGORY_MODIFIER_LETTER_V1,
        GeneralCategory::OtherLetter => key::GENERAL_CATEGORY_OTHER_LETTER_V1,
        GeneralCategory::TitlecaseLetter => key::GENERAL_CATEGORY_TITLECASE_LETTER_V1,
        GeneralCategory::UppercaseLetter => key::GENERAL_CATEGORY_UPPERCASE_LETTER_V1,
        GeneralCategory::Mark => key::GENERAL_CATEGORY_MARK_V1,
        GeneralCategory::SpacingMark => key::GENERAL_CATEGORY_SPACING_MARK_V1,
        GeneralCategory::EnclosingMark => key::GENERAL_CATEGORY_ENCLOSING_MARK_V1,
        GeneralCategory::NonspacingMark => key::GENERAL_CATEGORY_NONSPACING_MARK_V1,
        GeneralCategory::Number => key::GENERAL_CATEGORY_NUMBER_V1,
        GeneralCategory::Digit => key::GENERAL_CATEGORY_DIGIT_V1,
        GeneralCategory::LetterNumber => key::GENERAL_CATEGORY_LETTER_NUMBER_V1,
        GeneralCategory::OtherNumber => key::GENERAL_CATEGORY_OTHER_NUMBER_V1,
        GeneralCategory::Punctuation => key::GENERAL_CATEGORY_PUNCTUATION_V1,
        GeneralCategory::ConnectorPunctuation => key::GENERAL_CATEGORY_CONNECTOR_PUNCTUATION_V1,
        GeneralCategory::DashPunctuation => key::GENERAL_CATEGORY_DASH_PUNCTUATION_V1,
        GeneralCategory::ClosePunctuation => key::GENERAL_CATEGORY_CLOSE_PUNCTUATION_V1,
        GeneralCategory::FinalPunctuation => key::GENERAL_CATEGORY_FINAL_PUNCTUATION_V1,
        GeneralCategory::InitialPunctuation => key::GENERAL_CATEGORY_INITIAL_PUNCTUATION_V1,
        GeneralCategory::OtherPunctuation => key::GENERAL_CATEGORY_OTHER_PUNCTUATION_V1,
        GeneralCategory::OpenPunctuation => key::GENERAL_CATEGORY_OPEN_PUNCTUATION_V1,
        GeneralCategory::Symbol => key::GENERAL_CATEGORY_SYMBOL_V1,
        GeneralCategory::CurrencySymbol => key::GENERAL_CATEGORY_CURRENCY_SYMBOL_V1,
        GeneralCategory::ModifierSymbol => key::GENERAL_CATEGORY_MODIFIER_SYMBOL_V1,
        GeneralCategory::MathSymbol => key::GENERAL_CATEGORY_MATH_SYMBOL_V1,
        GeneralCategory::OtherSymbol => key::GENERAL_CATEGORY_OTHER_SYMBOL_V1,
        GeneralCategory::Separator => key::GENERAL_CATEGORY_SEPARATOR_V1,
        GeneralCategory::LineSeparator => key::GENERAL_CATEGORY_LINE_SEPARATOR_V1,
        GeneralCategory::ParagraphSeparator => key::GENERAL_CATEGORY_PARAGRAPH_SEPARATOR_V1,
        GeneralCategory::SpaceSeparator => key::GENERAL_CATEGORY_SPACE_SEPARATOR_V1,
    };
    get_prop(provider, key)
}
