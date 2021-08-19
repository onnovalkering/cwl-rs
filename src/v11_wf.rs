use crate::v11_cm::{Any, CwlType, Documentation, Format, Scatter, Schema, SecondaryFiles, Source};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_yaml::Value as YValue;

type Map<T> = std::collections::HashMap<String, T>;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowInputs {
    ParameterArray(Vec<WorkflowInputParameter>),
    ParameterMap(Map<WorkflowInputParameter>),
    TypeMap(Map<WorkflowInputType>),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowInputParameterType {
    Type(WorkflowInputType),
    TypeArray(Vec<WorkflowInputType>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowInputType {
    CwlType(CwlType),
    Schema(YValue), // TODO
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowOutputs {
    ParameterArray(Vec<WorkflowOutputParameter>),
    ParameterMap(Map<WorkflowOutputParameter>),
    TypeMap(Map<WorkflowOutputType>),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowOutputParameterType {
    Type(WorkflowOutputType),
    TypeArray(Vec<WorkflowOutputType>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowOutputType {
    CwlType(CwlType),
    Schema(YValue), // TODO
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowOutputParameterOutputSource {
    OutputSource(String),
    OutputSourceArray(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowSteps {
    StepArray(Vec<WorkflowStep>),
    StepMap(Map<WorkflowStep>),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStep {
    pub r#in: WorkflowStepIn,

    pub out: Vec<String>,

    pub run: String,

    pub id: Option<String>,

    pub label: Option<String>,

    pub doc: Option<Documentation>,

    pub requirements: Option<YValue>, // TODO

    pub hints: Option<YValue>, // TODO

    pub scatter: Option<Scatter>,

    pub scatter_method: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowStepIn {
    InputArray(Vec<WorkflowStepInput>),
    InputMap(Map<WorkflowStepInput>),
    InputSourceMap(Map<Source>),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepInput {
    pub id: Option<String>,

    pub source: Option<Source>,

    pub link_merge: Option<String>,

    pub load_contents: Option<bool>,

    pub load_listing: Option<String>,

    pub label: Option<String>,

    pub default: Option<Any>,

    pub value_from: Option<String>,
}
