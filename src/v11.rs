use crate::v11_clt::CommandLineTool;
use crate::v11_wf::Workflow;
use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_yaml::Value as YValue;
use std::io::Read;

type FResult<T> = Result<T, failure::Error>;

const SUPPORTED_VERSIONS: &[&'static str] = &["v1.0", "v1.1"];

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CwlDocument {
    CommandLineTool(CommandLineTool),
    Workflow(Workflow),
}

impl CwlDocument {
    pub fn from_reader<R: Read>(r: R) -> FResult<CwlDocument> {
        let v = serde_yaml::from_reader(r)?;
        CwlDocument::from_value(v)
    }

    pub fn from_str(s: &str) -> FResult<CwlDocument> {
        let v = serde_yaml::from_str(&s)?;
        CwlDocument::from_value(v)
    }

    pub fn from_value(v: YValue) -> FResult<CwlDocument> {
        // Check that we support the CWL specification version of the document.
        if let Some(YValue::String(version)) = v.get("cwlVersion") {
            let error_msg = format!("Unsupported CWL specification version: {}", version);
            ensure!(SUPPORTED_VERSIONS.contains(&version.as_str()), error_msg);
        } else {
            bail!("Failed to determine CWL specification version.");
        }

        // Deserialize into CommandLineTool or Workflow based on class property.
        if let Some(YValue::String(class)) = v.get("class") {
            match class.as_str() {
                "CommandLineTool" => {
                    let clt = serde_yaml::from_value::<CommandLineTool>(v)?;
                    return Ok(CwlDocument::CommandLineTool(clt));
                }
                "Workflow" => {
                    let wf = serde_yaml::from_value::<Workflow>(v)?;
                    return Ok(CwlDocument::Workflow(wf));
                },
                _ => bail!("Unsupported CWL document class: {}", class)
            }
        } else {
            bail!("Failed to determine CWL document class.");
        }
    }
}
