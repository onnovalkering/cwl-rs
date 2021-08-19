use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_yaml::Value as YValue;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "s:author")]
    pub author: Option<YValue>, // TODO
    #[serde(rename = "s:description")]
    pub description: Option<String>,
    #[serde(rename = "s:license")]
    pub license: Option<String>,
    #[serde(rename = "s:name")]
    pub name: Option<String>,
    #[serde(rename = "s:version")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SecondaryFiles {
    Schema(SecondaryFileSchema),
    Schemas(Vec<SecondaryFileSchema>),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryFileSchema {
    pattern: String,
    required: SecondaryFileSchemaRequired,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SecondaryFileSchemaRequired {
    Boolean(bool),
    Expression(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Any {
    Any(YValue),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CwlType {
    Null,
    Boolean(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Str(String),
    File(Box<File>),
    Directory(Directory),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub class: String,

    pub location: Option<String>,

    pub path: Option<String>,

    pub basename: Option<String>,

    pub dirname: Option<String>,

    pub nameroot: Option<String>,

    pub nameext: Option<String>,

    pub checksum: Option<String>,

    pub size: Option<i64>,

    pub secondary_files: Option<Vec<FileOrDirectory>>,

    pub format: Option<String>,

    pub contents: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    pub class: String,

    pub location: Option<String>,

    pub path: Option<String>,

    pub basename: Option<String>,

    pub listing: Option<Vec<FileOrDirectory>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum FileOrDirectory {
    File(File),
    Directory(Directory),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Documentation {
    SingleLine(String),
    MultiLine(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Format {
    Format(String),
    Formats(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Scatter {
    Parameter(String),
    Parameters(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Source {
    Source(String),
    Sources(Vec<String>),
}
