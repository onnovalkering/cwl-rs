use crate::v11_cm::{Any, CwlType, Documentation, Format, Schema, SecondaryFiles};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_yaml::Value as YValue;

type Map<T> = std::collections::HashMap<String, T>;

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub class: String,

    pub cwl_version: String,

    pub doc: Option<Documentation>,

    pub hints: Option<YValue>, // TODO

    pub id: Option<String>,

    pub inputs: WorkflowInputs,

    pub label: Option<String>,

    pub outputs: WorkflowOutputs,

    pub steps: WorkflowSteps,

    pub requirements: Option<YValue>, // TODO

    #[serde(flatten)]
    pub schema: Schema,

    #[serde(rename = "$namespaces")]
    pub namespaces: Option<YValue>, // TODO
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowInputs {
    ParameterArray(Vec<WorkflowInputParameter>),
    ParameterMap(Map<WorkflowInputParameter>),
    TypeMap(Map<WorkflowInputType>)
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowInputParameter {
    pub r#type: WorkflowInputParameterType,

    pub label: Option<String>,

    pub secondary_files: Option<SecondaryFiles>,

    pub streamable: Option<bool>,

    pub default: Option<Any>,

    pub doc: Option<Documentation>,

    pub id: Option<String>,

    pub format: Option<Format>,

    pub load_contents: Option<bool>,

    pub load_listing: Option<String>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowInputParameterType {
    Type(WorkflowInputType),
    TypeArray(Vec<WorkflowInputType>),
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowInputType {
    CwlType(CwlType),
    Schema(YValue), // TODO
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowOutputs {
    ParameterArray(Vec<WorkflowOutputParameter>),
    ParameterMap(Map<WorkflowOutputParameter>),
    TypeMap(Map<WorkflowOutputType>),
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowOutputParameter {
    pub r#type: WorkflowOutputParameterType,

    pub label: Option<String>,

    pub secondary_files: Option<SecondaryFiles>,

    pub streamable: Option<bool>,

    pub default: Option<YValue>,

    pub doc: Option<Documentation>,

    pub id: Option<String>,

    pub format: Option<Format>,

    pub output_source: Option<WorkflowOutputParameterOutputSource>,

    pub link_merge: Option<String>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowOutputParameterType {
    Type(WorkflowOutputType),
    TypeArray(Vec<WorkflowOutputType>),
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowOutputType {
    CwlType(CwlType),
    Schema(YValue), // TODO
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowOutputParameterOutputSource {
    OutputSource(String),
    OutputSourceArray(Vec<String>)
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowSteps {
    StepArray(Vec<WorkflowStep>),
    StepMap(Map<WorkflowStep>),
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowStep {
    // TODO
}
