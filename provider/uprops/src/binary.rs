// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::path::PathBuf;
use icu_provider::prelude::*;
use icu_uniset::provider::*;
use std::io;
use std::fs;
use eyre::Context;
use crate::upropdump_serde;
use crate::reader;
use icu_uniset::UnicodeSetBuilder;

pub struct BinaryPropertiesDataProvider {
    root_dir: PathBuf,
}

impl BinaryPropertiesDataProvider {
    fn get_toml_as_string(&self, name: &str) -> eyre::Result<String> {
        let mut path: PathBuf = self.root_dir.clone().join(name);
        path.set_extension("toml");
        fs::read_to_string(&path)
            .with_context(|| format!("Could not open file: {:?}", &path))
    }
}

impl<'d, 's> DataProvider<'d, 's, UnicodePropertyV1Marker> for BinaryPropertiesDataProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, UnicodePropertyV1Marker>, DataError> {
        let toml_str: String = self.get_toml_as_string(&req.resource_path.key.sub_category)
            .map_err(|e| DataError::new_resc_error(e.as_ref()))?;
        let toml_data: upropdump_serde::binary::Main = toml::from_str(&toml_str)
            .map_err(|e| DataError::new_resc_error(e))?;
        let mut builder = UnicodeSetBuilder::new();
        // TODO: put the toml_data into the builder and then
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(UnicodePropertyV1::from(r))),
        })
    }
}
