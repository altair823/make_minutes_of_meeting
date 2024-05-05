//! Rich metadata that contains extra metadata which is not part of the standard metadata.
//!

use serde_derive::{Deserialize, Serialize};

/// Rich metadata struct that contains extra metadata which is not part of the standard metadata.
///
/// The `extra_metadata` field is a vector of strings that can hold any additional metadata fields.
#[derive(Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct RichMetadata {
    /// Extra metadata fields that are not part of the standard metadata.
    pub extra_metadata: Vec<String>,
}

