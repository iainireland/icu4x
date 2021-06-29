// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_UNISET_H
#define ICU4X_UNISET_H

#include "provider.h"

#ifdef __cplusplus
extern "C" {
#endif

//! *** Note: DO NOT USE THESE APIs FOR NOW. ****
//!  Performance improvements and other fixes are still needed on this component.

// opaque
typedef struct ICU4XUniset ICU4XUniset;

//
// Binary property getter fns
//  These functions currently recreate the data provider and read from
//  the file system for every call.
// TODO: improve performance
// TODO: expose enum-based query API?
//

ICU4XUniset* icu4x_uniset_props_alphabetic();

ICU4XUniset* icu4x_uniset_props_ascii_hex_digit();

ICU4XUniset* icu4x_uniset_props_bidi_control();

ICU4XUniset* icu4x_uniset_props_bidi_mirrored();

ICU4XUniset* icu4x_uniset_props_case_ignorable();

ICU4XUniset* icu4x_uniset_props_cased();

ICU4XUniset* icu4x_uniset_props_changes_when_casefolded();

ICU4XUniset* icu4x_uniset_props_changes_when_lowercased();

ICU4XUniset* icu4x_uniset_props_changes_when_nfkc_casefolded();

ICU4XUniset* icu4x_uniset_props_changes_when_titlecased();

ICU4XUniset* icu4x_uniset_props_changes_when_uppercased();

ICU4XUniset* icu4x_uniset_props_dash();

ICU4XUniset* icu4x_uniset_props_default_ignorable_code_point();

ICU4XUniset* icu4x_uniset_props_deprecated();

ICU4XUniset* icu4x_uniset_props_diacritic();

ICU4XUniset* icu4x_uniset_props_emoji();

ICU4XUniset* icu4x_uniset_props_emoji_component();

ICU4XUniset* icu4x_uniset_props_emoji_modifier_base();

ICU4XUniset* icu4x_uniset_props_emoji_modifier();

ICU4XUniset* icu4x_uniset_props_emoji_presentation();

ICU4XUniset* icu4x_uniset_props_extended_pictographic();

ICU4XUniset* icu4x_uniset_props_extender();

ICU4XUniset* icu4x_uniset_props_grapheme_base();

ICU4XUniset* icu4x_uniset_props_grapheme_extend();

ICU4XUniset* icu4x_uniset_props_hex_digit();

ICU4XUniset* icu4x_uniset_props_id_continue();

ICU4XUniset* icu4x_uniset_props_id_start();

ICU4XUniset* icu4x_uniset_props_ideographic();

ICU4XUniset* icu4x_uniset_props_ids_binary_operator();

ICU4XUniset* icu4x_uniset_props_ids_trinary_operator();

ICU4XUniset* icu4x_uniset_props_join_control();

ICU4XUniset* icu4x_uniset_props_logical_order_exception();

ICU4XUniset* icu4x_uniset_props_lowercase();

ICU4XUniset* icu4x_uniset_props_math();

ICU4XUniset* icu4x_uniset_props_noncharacter_code_point();

ICU4XUniset* icu4x_uniset_props_pattern_syntax();

ICU4XUniset* icu4x_uniset_props_pattern_white_space();

ICU4XUniset* icu4x_uniset_props_quotation_mark();

ICU4XUniset* icu4x_uniset_props_radical();

ICU4XUniset* icu4x_uniset_props_regional_indicator();

ICU4XUniset* icu4x_uniset_props_soft_dotted();

ICU4XUniset* icu4x_uniset_props_sentence_terminal();

ICU4XUniset* icu4x_uniset_props_terminal_punctuation();

ICU4XUniset* icu4x_uniset_props_unified_ideograph();

ICU4XUniset* icu4x_uniset_props_uppercase();

ICU4XUniset* icu4x_uniset_props_variation_selector();

ICU4XUniset* icu4x_uniset_props_white_space();

ICU4XUniset* icu4x_uniset_props_xid_continue();

ICU4XUniset* icu4x_uniset_props_xid_start();

void icu4x_uniset_destroy(ICU4XUniset* uniset);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_UNISET_H
