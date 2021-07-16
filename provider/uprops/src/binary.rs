// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;
use std::path::PathBuf;
use icu_provider::prelude::*;
use icu_uniset::provider::*;
use std::fs;
use crate::upropdump_serde;
use icu_uniset::UnicodeSetBuilder;

pub struct BinaryPropertiesDataProvider {
    root_dir: PathBuf,
}

impl BinaryPropertiesDataProvider {
    pub fn new(root_dir: PathBuf) -> Self {
        BinaryPropertiesDataProvider { root_dir }
    }
    fn get_toml_as_string(&self, name: &str) -> Result<String, DataError> {
        let mut path: PathBuf = self.root_dir.clone().join(name);
        path.set_extension("toml");
        fs::read_to_string(&path)
            .map_err(DataError::new_resc_error)
    }
}

impl<'d, 's> DataProvider<'d, 's, UnicodePropertyV1Marker> for BinaryPropertiesDataProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, UnicodePropertyV1Marker>, DataError> {
        let toml_str: String = self.get_toml_as_string(&req.resource_path.key.sub_category)?;
        let toml_data: upropdump_serde::binary::Main = toml::from_str(&toml_str)
            .map_err(DataError::new_resc_error)?;
        let mut builder = UnicodeSetBuilder::new();
        for (start,end) in toml_data.unicode_set.data.ranges {
            let start = std::char::from_u32(start).ok_or(DataError::InvalidPayload)?;
            let end = std::char::from_u32(end + 1).ok_or(DataError::InvalidPayload)?;
            builder.add_range(&(start..end));
        }
        let uniset = builder.build();
        let name = Cow::from(toml_data.unicode_set.data.name);
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(
                UnicodePropertyV1::from_uniset(&uniset, name)))
        })
    }
}

#[test]
fn test_basic() {
    use std::convert::TryInto;
    use icu_uniset::UnicodeSet;

    let root_dir = icu_testdata::paths::data_root().join("uprops");
    let provider = BinaryPropertiesDataProvider::new(root_dir);

    let whitespace: UnicodeSet = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::WHITE_SPACE_V1,
                options: ResourceOptions::default(),
            },
        })
        .unwrap()
        .take_payload()
        .unwrap()
        .get()
        .clone()
        .try_into()
        .unwrap();
}
