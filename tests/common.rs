use anyhow::Result;
use cwl::v11::CwlDocument;
use cwl::v11_clt::CommandLineTool;
use cwl::v11_wf::Workflow;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
pub fn load(path: &str) -> Result<CwlDocument> {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);

    CwlDocument::from_reader(buf_reader)
}

#[allow(dead_code)]
pub fn unpack_clt(doc: CwlDocument) -> Option<CommandLineTool> {
    if let CwlDocument::CommandLineTool(clt) = doc {
        Some(clt)
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn unpack_wf(doc: CwlDocument) -> Option<Workflow> {
    if let CwlDocument::Workflow(wf) = doc {
        Some(wf)
    } else {
        None
    }
}
