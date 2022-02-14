// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use icu_provider::yoke::{self, *};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use yoke::{Yokeable, ZeroCopyFrom};
use zerovec::ule::{custom::EncodeAsVarULE, AsULE, RawBytesULE, VarULE, ZeroVecError, ULE};
use zerovec::{VarZeroVec, ZeroSlice, ZeroVec};

use crate::error::Error;
use crate::internals::{ClosureSet, DotType, MappingKind};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct ExceptionHeader(u16);

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Exception<'data> {
    header: ExceptionHeader,
    #[cfg_attr(feature = "serde", serde(borrow))]
    slots: ZeroVec<'data, u16>,
}

#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct CaseMappingExceptions<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    slots: VarZeroVec<'data, ExceptionULE>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    strings: VarZeroVec<'data, str>,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ExceptionSlot {
    Lower = 0,
    Fold = 1,
    Upper = 2,
    Title = 3,
    Delta = 4,
    // Slot 5 is reserved
    Closure = 6,
    FullMappings = 7,
}

impl ExceptionSlot {
    fn contains_char(&self) -> bool {
        matches!(self, Self::Lower | Self::Fold | Self::Upper | Self::Title)
    }
}

impl From<MappingKind> for ExceptionSlot {
    fn from(full: MappingKind) -> Self {
        match full {
            MappingKind::Lower => Self::Lower,
            MappingKind::Fold => Self::Fold,
            MappingKind::Upper => Self::Upper,
            MappingKind::Title => Self::Title,
        }
    }
}

impl<'data> Exception<'data> {
    #[inline]
    fn get(&self, idx: usize) -> u16 {
        self.slots
            .as_zerovec()
            .get(idx)
            .expect("Checked in validate")
    }

    #[inline]
    fn slot_value(&self, slot: ExceptionSlot) -> Option<u32> {
        if self.header.has_slot(slot) {
            let logical_slot_idx = self.header.slot_num(slot);
            if self.header.has_double_slots() {
                let hi = self.get(logical_slot_idx * 2) as u32;
                let lo = self.get(logical_slot_idx * 2 + 1) as u32;
                Some(hi << 16 | lo)
            } else {
                Some(self.get(logical_slot_idx) as u32)
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn slot_char(&self, kind: MappingKind) -> Option<char> {
        self.slot_value(kind.into())
            .map(|raw| char::from_u32(raw).expect("Checked in validate"))
    }

    #[inline]
    pub fn slot_char_with_fallback(&self, kind: MappingKind) -> Option<char> {
        match kind {
            MappingKind::Fold => self
                .slot_char(MappingKind::Fold)
                .or_else(|| self.slot_char(MappingKind::Lower)),
            MappingKind::Title => self
                .slot_char(MappingKind::Title)
                .or_else(|| self.slot_char(MappingKind::Upper)),
            _ => self.slot_char(kind),
        }
    }

    #[inline]
    pub fn delta(&self) -> Option<i32> {
        self.slot_value(ExceptionSlot::Delta).map(|raw| {
            if self.header.delta_is_negative() {
                -(raw as i32)
            } else {
                raw as i32
            }
        })
    }

    pub fn no_simple_case_folding(&self) -> bool {
        self.header.no_simple_case_folding()
    }

    pub fn is_sensitive(&self) -> bool {
        self.header.is_sensitive()
    }

    // Returns whether there is a conditional case fold for this code point.
    // (This is used to implement Turkic mappings for dotted/dotless i.)
    pub fn has_conditional_fold(&self) -> bool {
        self.header.has_conditional_fold()
    }

    pub fn has_conditional_special(&self) -> bool {
        self.header.has_conditional_special()
    }

    // Returns the dot type.
    // (Note that this information is stored in the trie for code points without
    // exception data, but the exception index requires more bits than the delta.)
    pub fn dot_type(&self) -> DotType {
        self.header.dot_type()
    }

    fn full_mapping_idx(&self, slot: MappingKind) -> Option<usize> {
        self.slot_value(ExceptionSlot::FullMappings)
            .map(|mappings_idx| mappings_idx as usize + slot as usize)
    }

    fn closure_string_idx(&self) -> Option<usize> {
        self.slot_value(ExceptionSlot::Closure)
            .map(|idx| idx as usize)
    }

    fn encode_as_var_ule(&self) -> Box<ExceptionULE> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
#[repr(packed)]
struct ExceptionULE {
    header: <ExceptionHeader as AsULE>::ULE,
    slots: [<u16 as AsULE>::ULE],
}

impl ExceptionULE {
    #[inline]
    pub fn as_exception(&self) -> Exception {
        Exception {
            header: ExceptionHeader::from_unaligned(self.header),
            slots: ZeroVec::Borrowed(&self.slots),
        }
    }
}

// Safety (based on the safety checklist on the ULE trait):
//  1. ExceptionULE does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(packed)]` on a type that satisfies this invariant)
//  2. ExceptionULE is aligned to 1 byte
//     (achieved by `#[repr(packed)]` on a type that satisfies this invariant)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. The other VarULE methods use the default impl.
//  7. ExceptionULE byte equality is semantic equality.
unsafe impl VarULE for ExceptionULE {
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        let ptr = bytes.as_ptr();
        let len = (bytes.len() - 2) / 2;

        // It's hard to construct a custom DST. We fake a pointer/length construction.
        let fake_slice = core::ptr::slice_from_raw_parts(ptr as *const <u16 as AsULE>::ULE, len);
        let ret = &*(fake_slice as *const Self);
        debug_assert_eq!(core::mem::size_of_val(ret), core::mem::size_of_val(bytes));
        ret
    }

    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
        // Verify that the header exists.
        if bytes.len() < 2 {
            return Err(ZeroVecError::parse::<Self>());
        }
        let header = ExceptionHeader((bytes[1] as u16) << 8 | bytes[0] as u16);

        // Verify that the length encoded in the header matches the remaining bytes.
        let expected_slot_bytes = if header.has_double_slots() {
            2 * 2 * header.num_slots()
        } else {
            2 * header.num_slots()
        } as usize;
        if bytes[2..].len() != expected_slot_bytes {
            return Err(ZeroVecError::parse::<Self>());
        }

        Ok(())
    }
}

unsafe impl<'data> EncodeAsVarULE<ExceptionULE> for Exception<'data> {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[RawBytesULE::<2>::as_byte_slice(&[self.header.0.as_unaligned()]),
             self.slots.as_bytes()])
    }
}

impl ExceptionHeader {
    const SLOTS_MASK: u16 = 0xff;

    // Each slot is 2 u16 elements instead of 1
    const DOUBLE_SLOTS_FLAG: u16 = 0x100;

    const NO_SIMPLE_CASE_FOLDING_FLAG: u16 = 0x200;
    const DELTA_IS_NEGATIVE_FLAG: u16 = 0x400;
    const SENSITIVE_FLAG: u16 = 0x800;

    const DOT_SHIFT: u16 = 12;

    const CONDITIONAL_SPECIAL_FLAG: u16 = 0x4000;
    const CONDITIONAL_FOLD_FLAG: u16 = 0x8000;

    // The number of optional slots for this exception
    fn num_slots(&self) -> u16 {
        let slot_bits = self.0 & Self::SLOTS_MASK;
        slot_bits.count_ones() as u16
    }

    // Returns true if the given slot exists for this exception
    pub(crate) fn has_slot(&self, slot: ExceptionSlot) -> bool {
        let bit = 1u16 << (slot as u16);
        self.0 & bit != 0
    }

    // Returns the logical idx of this slot in the slots slice.
    // Note that this does not account for double-width slots.
    fn slot_num(&self, slot: ExceptionSlot) -> usize {
        debug_assert!(self.has_slot(slot));
        let slot_bit = 1u16 << (slot as u16);
        let previous_slot_mask = slot_bit - 1;
        let previous_slots = self.0 & previous_slot_mask;
        previous_slots.count_ones() as usize
    }

    // Returns whether this exception has double-width slots
    pub fn has_double_slots(&self) -> bool {
        self.0 & Self::DOUBLE_SLOTS_FLAG != 0
    }

    // Returns true if there is no simple case folding for this exception
    fn no_simple_case_folding(&self) -> bool {
        self.0 & Self::NO_SIMPLE_CASE_FOLDING_FLAG != 0
    }

    // Returns true if the delta for this exception is negative.
    fn delta_is_negative(&self) -> bool {
        debug_assert!(self.has_slot(ExceptionSlot::Delta));
        self.0 & Self::DELTA_IS_NEGATIVE_FLAG != 0
    }

    // Returns whether this code point is case-sensitive.
    // (Note that this information is stored in the trie for code points without
    // exception data, but the exception index requires more bits than the delta.)
    fn is_sensitive(&self) -> bool {
        self.0 & Self::SENSITIVE_FLAG != 0
    }

    // Returns the dot type.
    // (Note that this information is stored in the trie for code points without
    // exception data, but the exception index requires more bits than the delta.)
    fn dot_type(&self) -> DotType {
        let masked_bits = (self.0 >> Self::DOT_SHIFT) & DotType::DOT_MASK;
        DotType::from_masked_bits(masked_bits)
    }

    // Returns whether there is a language-specific case mapping.
    fn has_conditional_special(&self) -> bool {
        self.0 & Self::CONDITIONAL_SPECIAL_FLAG != 0
    }

    // Returns whether there is a conditional case fold.
    // (This is used for Turkic mappings for dotted/dotless i.)
    fn has_conditional_fold(&self) -> bool {
        self.0 & Self::CONDITIONAL_FOLD_FLAG != 0
    }
}

impl AsULE for ExceptionHeader {
    type ULE = RawBytesULE<2>;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        RawBytesULE(self.0.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        ExceptionHeader(u16::from_le_bytes(unaligned.0))
    }
}

#[cfg(feature = "provider_serde")]
impl Serialize for ExceptionULE {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_exception().serialize(serializer)
    }
}

#[cfg(feature = "provider_serde")]
impl<'de: 'data, 'data> Deserialize<'de> for Box<ExceptionULE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Exception::<'data>::deserialize(deserializer)?.encode_as_var_ule())
    }
}
