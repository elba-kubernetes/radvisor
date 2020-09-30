use std::collections::BTreeMap;

use serde::Serialize;

/// Contains all metadata used for perf table parsing
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TableMetadata {
    pub delimiter: &'static str,
    pub columns:   BTreeMap<String, Column>,
}

/// Contains the definitions for a single column
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged, rename_all = "PascalCase")]
pub enum Column {
    Scalar { r#type: ColumnType },
    Vector { r#type: ColumnType, count: usize },
}

/// Enum representing known variants of a column
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnType {
    /// Generic integer type
    Int,
    /// Nanosecond timestamp
    Epoch19,
}
