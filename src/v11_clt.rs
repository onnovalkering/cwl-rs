use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_yaml::Value as YValue;

type Map<T> = std::collections::HashMap<String, T>;

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandLineTool {
    pub arguments: Option<Vec<CommandLineToolArgument>>,

    pub base_command: Option<CommandLineToolBaseCommand>,

    pub class: String,

    pub cwl_version: String,

    pub doc: Option<CommandLineToolDoc>,

    pub hints: Option<YValue>,

    pub id: Option<String>,

    pub inputs: CommandLineToolInput,

    pub label: Option<String>,

    pub outputs: CommandLineToolOutput,

    pub permanent_fail_codes: Option<Vec<u32>>,

    pub requirements: Option<YValue>,

    pub stderr: Option<CommandLineToolStderr>,

    pub stdin: Option<CommandLineToolStdin>,

    pub stdout: Option<CommandLineToolStdout>,

    pub success_codes: Option<Vec<u32>>,

    pub temporary_fail_codes: Option<Vec<u32>>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolArgument {
    Argument(String),
    Expression(YValue),
    CommandLineBinding(CommandLineBinding),
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandLineBinding {
    pub item_seperator: Option<String>,

    pub load_contents: Option<bool>,

    pub position: Option<CommandLineBindingPosition>,

    pub prefix: Option<String>,

    pub seperate: Option<bool>,

    pub shell_quote: Option<bool>,

    pub value_from: Option<CommandLineBindingValueFrom>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineBindingPosition {
    Position(u32),
    Expression(YValue)
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineBindingValueFrom {
    ValueFrom(String),
    Expression(YValue)
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolBaseCommand {
    BaseCommand(String),
    Array(Vec<String>)
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolDoc {
    Doc(String),
    Array(Vec<String>)
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolInput {
    Array(Vec<CommandInputParameter>),
    Map(Map<CommandInputParameter>),
    Types(Map<YValue>)
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandInputParameter {
    pub default: Option<YValue>,

    pub doc: Option<CommandLineToolDoc>,

    pub format: Option<CommandLineParameterFormat>,

    pub id: Option<String>,

    pub input_binding: Option<CommandLineBinding>,

    pub label: Option<String>,

    pub load_contents: Option<bool>,

    pub load_listing: Option<String>,

    #[serde(rename = "type")]
    pub param_type: YValue,

    pub secondary_files: Option<CommandLineSecondaryFiles>,

    pub streamable: Option<bool>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineSecondaryFiles {
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
pub enum CommandLineToolOutput {
    Array(Vec<CommandOutputParameter>),
    Map(Map<CommandOutputParameter>),
    Types(Map<YValue>)
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandOutputParameter {
    pub doc: Option<CommandLineToolDoc>,

    pub format: Option<CommandLineParameterFormat>,

    pub id: Option<String>,

    pub output_binding: Option<CommandLineBinding>,

    pub label: Option<String>,

    pub load_contents: Option<bool>,

    pub load_listing: Option<String>,

    #[serde(rename = "type")]
    pub param_type: YValue,

    pub secondary_files: Option<CommandLineSecondaryFiles>,

    pub streamable: Option<bool>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineParameterFormat {
    Format(String),
    Array(Vec<String>),
    Expression(YValue),
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolStderr {
    Stderr(String),
    Expression(YValue)
}

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolStdin {
    Stdin(String),
    Expression(YValue)
}

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolStdout {
    Stdout(String),
    Expression(YValue)
}
