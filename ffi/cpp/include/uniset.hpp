// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_UNISET_HPP
#define ICU4X_UNISET_HPP

#include <memory>

#include "../../capi/include/uniset.h"

namespace icu4x {

enum class UnisetProperty {
  Alphabetic,
  AsciiHexDigit,
  BidiControl,
  BidiMirrored,
  CaseIgnorable,
  Cased,
  ChangesWhenCasefolded,
  ChangesWhenLowercased,
  ChangesWhenNfkcCasefolded,
  ChangesWhenTitlecased,
  ChangesWhenUppercased,
  Dash,
  DefaultIgnorableCodePoint,
  Deprecated,
  Diacritic,
  Emoji,
  EmojiComponent,
  EmojiModifierBase,
  EmojiModifier,
  EmojiPresentation,
  ExtendedPictographic,
  Extender,
  GraphemeBase,
  GraphemeExtend,
  HexDigit,
  IdContinue,
  IdStart,
  Ideographic,
  IdsBinaryOperator,
  IdsTrinaryOperator,
  JoinControl,
  LogicalOrderException,
  Lowercase,
  Math,
  NoncharacterCodePoint,
  PatternSyntax,
  PatternWhiteSpace,
  QuotationMark,
  Radical,
  RegionalIndicator,
  SoftDotted,
  SentenceTerminal,
  TerminalPunctuation,
  UnifiedIdeograph,
  Uppercase,
  VariationSelector,
  WhiteSpace,
  XidContinue,
  XidStart
};

struct ICU4XUnisetDeleter {
  void operator()(ICU4XUniset* u) const noexcept { icu4x_uniset_destroy(u); }
};

class Uniset {
 private:
  Uniset(ICU4XUniset* uniset) : inner(uniset) {}

  static ICU4XUniset* FromProperty(UnisetProperty prop) {
    switch (prop) {
      case UnisetProperty::Alphabetic:
        return icu4x_uniset_props_alphabetic();
      case UnisetProperty::AsciiHexDigit:
        return icu4x_uniset_props_ascii_hex_digit();
      case UnisetProperty::BidiControl:
        return icu4x_uniset_props_bidi_control();
      case UnisetProperty::BidiMirrored:
        return icu4x_uniset_props_bidi_mirrored();
      case UnisetProperty::CaseIgnorable:
        return icu4x_uniset_props_case_ignorable();
      case UnisetProperty::Cased:
        return icu4x_uniset_props_cased();
      case UnisetProperty::ChangesWhenCasefolded:
        return icu4x_uniset_props_changes_when_casefolded();
      case UnisetProperty::ChangesWhenLowercased:
        return icu4x_uniset_props_changes_when_lowercased();
      case UnisetProperty::ChangesWhenNfkcCasefolded:
        return icu4x_uniset_props_changes_when_nfkc_casefolded();
      case UnisetProperty::ChangesWhenTitlecased:
        return icu4x_uniset_props_changes_when_titlecased();
      case UnisetProperty::ChangesWhenUppercased:
        return icu4x_uniset_props_changes_when_uppercased();
      case UnisetProperty::Dash:
        return icu4x_uniset_props_dash();
      case UnisetProperty::DefaultIgnorableCodePoint:
        return icu4x_uniset_props_default_ignorable_code_point();
      case UnisetProperty::Deprecated:
        return icu4x_uniset_props_deprecated();
      case UnisetProperty::Diacritic:
        return icu4x_uniset_props_diacritic();
      case UnisetProperty::Emoji:
        return icu4x_uniset_props_emoji();
      case UnisetProperty::EmojiComponent:
        return icu4x_uniset_props_emoji_component();
      case UnisetProperty::EmojiModifierBase:
        return icu4x_uniset_props_emoji_modifier_base();
      case UnisetProperty::EmojiModifier:
        return icu4x_uniset_props_emoji_modifier();
      case UnisetProperty::EmojiPresentation:
        return icu4x_uniset_props_emoji_presentation();
      case UnisetProperty::ExtendedPictographic:
        return icu4x_uniset_props_extended_pictographic();
      case UnisetProperty::Extender:
        return icu4x_uniset_props_extender();
      case UnisetProperty::GraphemeBase:
        return icu4x_uniset_props_grapheme_base();
      case UnisetProperty::GraphemeExtend:
        return icu4x_uniset_props_grapheme_extend();
      case UnisetProperty::HexDigit:
        return icu4x_uniset_props_hex_digit();
      case UnisetProperty::IdContinue:
        return icu4x_uniset_props_id_continue();
      case UnisetProperty::IdStart:
        return icu4x_uniset_props_id_start();
      case UnisetProperty::Ideographic:
        return icu4x_uniset_props_ideographic();
      case UnisetProperty::IdsBinaryOperator:
        return icu4x_uniset_props_ids_binary_operator();
      case UnisetProperty::IdsTrinaryOperator:
        return icu4x_uniset_props_ids_trinary_operator();
      case UnisetProperty::JoinControl:
        return icu4x_uniset_props_join_control();
      case UnisetProperty::LogicalOrderException:
        return icu4x_uniset_props_logical_order_exception();
      case UnisetProperty::Lowercase:
        return icu4x_uniset_props_lowercase();
      case UnisetProperty::Math:
        return icu4x_uniset_props_math();
      case UnisetProperty::NoncharacterCodePoint:
        return icu4x_uniset_props_noncharacter_code_point();
      case UnisetProperty::PatternSyntax:
        return icu4x_uniset_props_pattern_syntax();
      case UnisetProperty::PatternWhiteSpace:
        return icu4x_uniset_props_pattern_white_space();
      case UnisetProperty::QuotationMark:
        return icu4x_uniset_props_quotation_mark();
      case UnisetProperty::Radical:
        return icu4x_uniset_props_radical();
      case UnisetProperty::RegionalIndicator:
        return icu4x_uniset_props_regional_indicator();
      case UnisetProperty::SoftDotted:
        return icu4x_uniset_props_soft_dotted();
      case UnisetProperty::SentenceTerminal:
        return icu4x_uniset_props_sentence_terminal();
      case UnisetProperty::TerminalPunctuation:
        return icu4x_uniset_props_terminal_punctuation();
      case UnisetProperty::UnifiedIdeograph:
        return icu4x_uniset_props_unified_ideograph();
      case UnisetProperty::Uppercase:
        return icu4x_uniset_props_uppercase();
      case UnisetProperty::VariationSelector:
        return icu4x_uniset_props_variation_selector();
      case UnisetProperty::WhiteSpace:
        return icu4x_uniset_props_white_space();
      case UnisetProperty::XidContinue:
        return icu4x_uniset_props_xid_continue();
      case UnisetProperty::XidStart:
        return icu4x_uniset_props_xid_start();
    }
    return nullptr;
  }

 public:
  Uniset(UnisetProperty prop) : inner(FromProperty(prop)) {}

  inline const ICU4XUniset* AsFFI() const { return this->inner.get(); }

 private:
  std::unique_ptr<ICU4XUniset, ICU4XUnisetDeleter> inner;
};
}  // namespace icu4x

#endif  // ICU4X_LOCALE_HPP
