use crate::v11_cm::{Any, CwlType, Documentation, Format, Schema, SecondaryFiles};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_yaml::Value as YValue;

type Map<T> = std::collections::HashMap<String, T>;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandLineTool {
    pub arguments: Option<Vec<CommandLineToolArgument>>,

    pub base_command: Option<CommandLineToolBaseCommand>,

    pub class: String,

    pub cwl_version: String,

    pub doc: Option<Documentation>,

    pub hints: Option<YValue>, // TODO

    pub id: Option<String>,

    pub inputs: CommandLineToolInput,

    pub label: Option<String>,

    pub outputs: CommandLineToolOutput,

    pub permanent_fail_codes: Option<Vec<u32>>,

    pub requirements: Option<YValue>, // TODO

    pub stderr: Option<String>,

    pub stdin: Option<String>,

    pub stdout: Option<String>,

    pub success_codes: Option<Vec<u32>>,

    pub temporary_fail_codes: Option<Vec<u32>>,

    #[serde(flatten)]
    pub schema: Schema,

    #[serde(rename = "$namespaces")]
    pub namespaces: Option<YValue>, // TODO
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CommandLineToolArgument {
    Argument(String),
    Binding(CommandLineBinding),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandLineBinding {
    pub item_seperator: Option<String>,

    pub load_contents: Option<bool>,

    pub position: Option<CommandLineBindingPosition>,

    pub prefix: Option<String>,

    pub seperate: Option<bool>,

    pub shell_quote: Option<bool>,

    pub value_from: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CommandLineBindingPosition {
    Position(u32),
    Expression(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CommandLineToolBaseCommand {
    Command(String),
    CommandWithArguments(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CommandLineToolInput {
    ParameterArray(Vec<CommandInputParameter>),
    ParameterMap(Map<CommandInputParameter>),
    TypeMap(Map<CommandLineToolInputType>),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandInputParameter {
    pub default: Option<Any>,

    pub doc: Option<Documentation>,

    pub format: Option<Format>,

    pub id: Option<String>,

    pub input_binding: Option<CommandLineBinding>,

    pub label: Option<String>,

    pub load_contents: Option<bool>,

    pub load_listing: Option<String>,

    pub r#type: CommandInputParameterType,

    pub secondary_files: Option<SecondaryFiles>,

    pub streamable: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CommandInputParameterType {
    Type(CommandLineToolInputType),
    TypeArray(Vec<CommandLineToolInputType>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CommandLineToolInputType {
    CwlType(CwlType),
    Schema(YValue), // TODO
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CommandLineToolOutput {
    ParameterArray(Vec<CommandOutputParameter>),
    ParameterMap(Map<CommandOutputParameter>),
    TypeMap(Map<CommandLineToolOutputType>),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandOutputParameter {
    pub doc: Option<Documentation>,

    pub format: Option<Format>,

    pub id: Option<String>,

    pub output_binding: Option<CommandLineBinding>,

    pub label: Option<String>,

    pub load_contents: Option<bool>,

    pub load_listing: Option<String>,

    pub r#type: CommandOutputParameterType,

    pub secondary_files: Option<SecondaryFiles>,

    pub streamable: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CommandOutputParameterType {
    Type(CommandLineToolOutputType),
    TypeArray(Vec<CommandLineToolOutputType>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CommandLineToolOutputType {
    CwlType(CwlType),
    Schema(YValue), // TODO
}
