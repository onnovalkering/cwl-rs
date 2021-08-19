use crate::v11_clt::CommandLineTool;
use crate::v11_cm::Schema;
use crate::v11_wf::Workflow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_yaml::Value;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

const SUPPORTED_VERSIONS: &[&str] = &["v1.0", "v1.1"];

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CwlDocument {
    CommandLineTool(CommandLineTool),
    Workflow(Workflow),
}

impl CwlDocument {
    pub fn from_path(path: &Path) -> Result<CwlDocument> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);

        CwlDocument::from_reader(buf_reader)
    }

    pub fn from_reader<R: Read>(r: R) -> Result<CwlDocument> {
        let v = serde_yaml::from_reader(r)?;
        CwlDocument::from_yaml(v)
    }

    pub fn from_yaml(v: Value) -> Result<CwlDocument> {
        // Check that we support the CWL specification version of the document.
        if let Some(Value::String(version)) = v.get("cwlVersion") {
            let error_msg = format!("Unsupported CWL specification version: {}", version);
            ensure!(SUPPORTED_VERSIONS.contains(&version.as_str()), error_msg);
        } else {
            bail!("Failed to determine CWL specification version.");
        }

        // Deserialize into CommandLineTool or Workflow based on class property.
        if let Some(Value::String(class)) = v.get("class") {
            match class.as_str() {
                "CommandLineTool" => {
                    let clt = serde_yaml::from_value::<CommandLineTool>(v)?;
                    Ok(CwlDocument::CommandLineTool(clt))
                }
                "Workflow" => {
                    let wf = serde_yaml::from_value::<Workflow>(v)?;
                    Ok(CwlDocument::Workflow(wf))
                }
                _ => bail!("Unsupported CWL document class: {}", class),
            }
        } else {
            bail!("Failed to determine CWL document class.");
        }
    }

    pub fn get_schema_props(&self) -> Schema {
        use CwlDocument::*;

        match self {
            CommandLineTool(clt) => clt.schema.clone(),
            Workflow(wf) => wf.schema.clone(),
        }
    }
}

impl FromStr for CwlDocument {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = serde_yaml::from_str(s)?;
        CwlDocument::from_yaml(v)
    }
}
