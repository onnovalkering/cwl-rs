use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_yaml;
use serde_yaml::Value as YValue;

type FResult<T> = Result<T, failure::Error>;
type Map<T> = std::collections::HashMap<String, T>;

const SUPPORTED_VERSIONS: &[&'static str] = &["v1.0", "v1.1"];

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CwlDocument {
    CommandLineTool(CommandLineTool),
    Workflow(Workflow),
}

impl CwlDocument {
    pub fn from_str(s: &str) -> FResult<CwlDocument> {
        let yaml: YValue = serde_yaml::from_str(&s)?;

        // Check that we support the CWL specification version of the document.
        if let Some(YValue::String(version)) = yaml.get("cwlVersion") {
            let error_msg = format!("Unsupported CWL specification version: {}", version);
            ensure!(SUPPORTED_VERSIONS.contains(&version.as_str()), error_msg);
        } else {
            bail!("Failed to determine CWL specification version.");
        }
        
        // Deserialize into CommandLineTool or Workflow based on class property.
        if let Some(YValue::String(class)) = yaml.get("class") {
            match class.as_str() {
                "CommandLineTool" => {
                    let clt = serde_yaml::from_value::<CommandLineTool>(yaml)?;
                    return Ok(CwlDocument::CommandLineTool(clt));
                }
                "Workflow" => {
                    let wf = serde_yaml::from_value::<Workflow>(yaml)?;
                    return Ok(CwlDocument::Workflow(wf));
                },
                _ => bail!("Unsupported CWL document class: {}", class)
            }
        } else {
            bail!("Failed to determine CWL document class.");
        }
    }
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandLineTool {
    /// Command line bindings which are not directly associated with input parameters.
    arguments: Option<Vec<CommandLineToolArgument>>,

    /// Specifies the program to execute.
    base_command: Option<String>,

    /// Class of this object.
    class: String,
    
    /// CWL document version.
    cwl_version: String,
    
    /// Documentation for this object.
    doc: Option<String>,

    /// Declares hints applying to either the runtime environment or the workflow engine 
    /// that may be helpful in executing this process. It is not an error if an implementation
    /// cannot satisfy all hints, however the implementation may report a warning.
    hints: Option<Vec<String>>,
    
    /// The unique identifier for this object.
    id: Option<String>,

    /// Defines the input parameters of the process. The process is ready to run when all
    /// required input parameters are associated with concrete values. Input parameters 
    /// include a schema for each parameter which is used to validate the input object. 
    /// It may also be used to build a user interface for constructing the input object.
    inputs: CommandLineToolInput,

    /// A short, human-readable label of this object.
    label: Option<String>,

    /// Defines the parameters representing the output of the process. May be used to 
    /// generate and/or validate the output object.
    outputs: CommandLineToolOutput,

    /// Exit codes that indicate the process failed due to a permanent logic error, where
    /// executing the process with the same runtime environment and same inputs is expected
    /// to always fail.
    permanent_fail_codes: Option<Vec<u32>>,

    /// Declares requirements that apply to either the runtime environment or the workflow 
    /// engine that must be met in order to execute this process. If an implementation cannot
    /// satisfy all requirements, or a requirement is listed which is not recognized by the 
    /// implementation, it is a fatal error and the implementation must not attempt to run 
    /// the process, unless overridden at user option.
    requirements: Option<Vec<String>>, 

    /// A path to a file whose contents must be piped into the command's standard input stream.
    stdin: Option<String>, 

    /// Capture the command's standard error stream to a file written to the designated
    /// output directory. If stderr is a string, it specifies the file name to use. If stderr
    /// is an expression, the expression is evaluated and must return a string with the file
    /// name to use to capture stderr. If the return value is not a string, or the resulting
    /// path contains illegal characters (such as the path separator /) it is an error.
    stderr: Option<String>, 

    /// Capture the command's standard output stream to a file written to the designated 
    /// output directory. If stdout is a string, it specifies the file name to use. If sdout 
    /// is an expression, the expression is evaluated and must return a string with the file
    /// name to use to capture stdout. If the return value is not a string, or the resulting 
    /// path contains illegal characters (such as the path separator /) it is an error.
    stdout: Option<String>,

    /// Exit codes that indicate the process completed successfully.
    success_codes: Option<Vec<u32>>,

    /// Exit codes that indicate the process failed due to a possibly temporary condition, 
    /// where executing the process with the same runtime environment and inputs may produce
    /// different results.
    temporary_fail_codes: Option<Vec<u32>>,
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolArgument {
    Argument(String),
    Expression(Expression),
    CommandLineBinding(CommandLineBinding),
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandLineBinding {

}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolInput {
    Array(Vec<CommandInputParameter>),
    Map(Map<CommandInputParameter>),
    Types(Map<ParamterType>)
}

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandLineToolOutput {
    
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandInputParameter {
    // Required
    #[serde(rename = "type")]
    p_type: ParamterType,
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandOutputParameter {

}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expression {

}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ParamterType {
    
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workflow {
    cwl_version: String,
    class: String,
}
