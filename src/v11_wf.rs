use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_yaml::Value as YValue;

type Map<T> = std::collections::HashMap<String, T>;

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workflow {
    class: String,

    cwl_version: String,

    doc: Option<WorkflowDoc>,

    hints: Option<YValue>,

    id: Option<String>,

    inputs: WorkflowInputs,

    label: Option<String>,

    outputs: WorkflowOutputs,

    steps: WorkflowSteps,

    requirements: Option<YValue>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowDoc {
    Doc(String),
    Array(Vec<String>),
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowInputs {
    Array(Vec<WorkflowInputParameter>),
    Map(Map<WorkflowInputParameter>),
    Types(Map<String>)
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowInputParameter {
    #[serde(rename = "type")]
    param_type: YValue,

    label: Option<String>,

    secondary_files: WorkflowSecondaryFiles,

    streamable: Option<bool>,

    default: Option<YValue>,

    doc: Option<WorkflowDoc>,

    id: Option<String>,

    input_binding: Option<WorkflowInputBinding>,

    format: Option<WorkflowParameterFormat>,

    load_contents: Option<bool>,

    load_listing: Option<String>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowOutputs {
    Array(Vec<WorkflowOutputParameter>),
    Map(Map<WorkflowOutputParameter>),
    Types(Map<YValue>)
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowOutputParameter {
    #[serde(rename = "type")]
    param_type: YValue,

    label: Option<String>,

    secondary_files: WorkflowSecondaryFiles,

    streamable: Option<bool>,

    default: Option<YValue>,

    doc: Option<WorkflowDoc>,

    id: Option<String>,

    format: Option<WorkflowParameterFormat>,

    output_source: Option<WorkflowOutputSource>,

    link_merge: Option<String>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowSteps {
    Array(Vec<WorkflowStep>),
    Map(Map<WorkflowStep>),
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowStep {

}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowSecondaryFiles {
    SecondaryFile(SecondaryFileSchema),
    Array(Vec<SecondaryFileSchema>),
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecondaryFileSchema {
    pub pattern: SecondaryFileSchemaPattern,
    pub required: Option<SecondaryFileSchemaRequired>
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SecondaryFileSchemaPattern {
    Pattern(String),
    Expression(YValue)
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SecondaryFileSchemaRequired {
    Required(bool),
    Expression(YValue)
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowParameterFormat {
    Format(String),
    Array(Vec<String>),
    Expression(YValue),
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowInputBinding {
    pub load_contents: Option<bool>
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WorkflowOutputSource {
    OutputSource(String),
    Array(Vec<String>)
}
