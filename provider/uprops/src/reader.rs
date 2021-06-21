// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use eyre::Context;

/// Helper function to open a file and return failures as a crate error.
pub fn open_reader(path: &Path) -> eyre::Result<BufReader<File>> {
    #[cfg(feature = "log")]
    log::trace!("Reading: {:?}", path);
    File::open(&path)
        .map(BufReader::new)
        .with_context(|| format!("Could not open file: {:?}", path))
}
