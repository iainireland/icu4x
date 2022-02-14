use serde::{Deserialize, Serialize};
use yoke::{Yokeable, ZeroCopyFrom};
use zerovec::{ZeroVec, ZeroSlice, VarZeroVec};
use zerovec::ule::{custom::EncodeAsVarULE, AsULE, RawBytesULE, VarULE, ZeroVecError, ULE};

#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
struct ExceptionHeader(u16);

impl ExceptionHeader {
    const SLOTS_MASK: u16 = 0xff;
    const DOUBLE_SLOTS_FLAG: u16 = 0x100;

    // The number of optional slots for this exception
    fn num_slots(&self) -> u16 {
        let slot_bits = self.0 & Self::SLOTS_MASK;
        slot_bits.count_ones() as u16
    }
    // Whether this exception has double-width slots
    pub fn has_double_slots(&self) -> bool {
        self.0 & Self::DOUBLE_SLOTS_FLAG != 0
    }
    // ...
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

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Exception<'data> {
    header: ExceptionHeader,
    #[serde(borrow)]
    slots: ZeroVec<'data, u16>,
}

impl<'data> Exception<'data> {
    fn encode_as_var_ule(&self) -> Box<ExceptionULE> {
        todo!();
    }
    // ...
}

unsafe impl<'data> EncodeAsVarULE<ExceptionULE> for Exception<'data> {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[RawBytesULE::<2>::as_byte_slice(&[self.header.0.as_unaligned()]),
             self.slots.as_bytes()])
    }
}

#[derive(Debug, PartialEq)]
#[repr(packed)]
struct ExceptionULE {
    header: <ExceptionHeader as AsULE>::ULE,
    slots: ZeroSlice<u16>,
}

impl ExceptionULE {
    #[inline]
    pub fn as_exception(&self) -> Exception {
        Exception {
            header: ExceptionHeader::from_unaligned(self.header),
            slots: self.slots.as_zerovec()
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

        // This reuses Zibi's approach from RelationULE
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

impl Serialize for ExceptionULE {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_exception().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Box<ExceptionULE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Exception::<'de>::deserialize(deserializer)?.encode_as_var_ule())
    }
}

#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[derive(Serialize)]
// Doesn't work yet:
// #[derive(Deserialize)]
struct CaseMappingExceptions<'data> {
    exceptions: VarZeroVec<'data, ExceptionULE>,
}
