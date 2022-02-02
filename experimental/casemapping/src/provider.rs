// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::internals::CaseMappingInternals;
use icu_provider::yoke::{self, *};

pub mod key {
    //! Resource keys for [`icu_casemapping`](crate)
    use icu_provider::{resource_key, ResourceKey};

    /// Key for case mapping data
    pub const CASE_MAPPING_V1: ResourceKey = resource_key!("props/casemap@1");
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
/// CaseMapping provides low-level access to the data necessary to
/// convert characters and strings to upper, lower, or title case.
pub struct CaseMappingV1<'data> {
    /// Case mapping data
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub casemap: CaseMappingInternals<'data>,
}
