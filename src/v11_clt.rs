use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_yaml::Value as YValue;

type Map<T> = std::collections::HashMap<String, T>;

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandLineTool {
    arguments: Option<Vec<CommandLineToolArgument>>,

    base_command: Option<CommandLineToolBaseCommand>,

    class: String,

    cwl_version: String,

    doc: Option<CommandLineToolDoc>,

    hints: Option<YValue>,

    id: Option<String>,

    inputs: CommandLineToolInput,

    label: Option<String>,

    outputs: CommandLineToolOutput,

    permanent_fail_codes: Option<Vec<u32>>,

    requirements: Option<YValue>,

    stderr: Option<CommandLineToolStderr>,

    stdin: Option<CommandLineToolStdin>,

    stdout: Option<CommandLineToolStdout>,

    success_codes: Option<Vec<u32>>,

    temporary_fail_codes: Option<Vec<u32>>,
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
    item_seperator: Option<String>,

    load_contents: Option<bool>,

    position: Option<CommandLineBindingPosition>,

    prefix: Option<String>,

    seperate: Option<bool>,

    shell_quote: Option<bool>,

    value_from: Option<CommandLineBindingValueFrom>,
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
    Array(String)
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolDoc {
    Doc(String),
    Array(String)
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
    default: Option<YValue>,

    doc: Option<CommandLineToolDoc>,

    format: Option<CommandLineParameterFormat>,

    id: Option<String>,

    input_binding: Option<CommandLineBinding>,

    label: Option<String>,

    load_contents: Option<bool>,

    load_listing: Option<String>,

    #[serde(rename = "type")]
    param_type: YValue,

    secondary_files: Option<CommandLineSecondaryFiles>,

    streamable: Option<bool>,
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
    pattern: SecondaryFileSchemaPattern,
    required: Option<SecondaryFileSchemaRequired>
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
    doc: Option<CommandLineToolDoc>,

    format: Option<CommandLineParameterFormat>,

    id: Option<String>,

    output_binding: Option<CommandLineBinding>,

    label: Option<String>,

    load_contents: Option<bool>,

    load_listing: Option<String>,

    #[serde(rename = "type")]
    param_type: YValue,

    secondary_files: Option<CommandLineSecondaryFiles>,

    streamable: Option<bool>,
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
