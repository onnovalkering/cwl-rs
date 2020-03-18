use cwl::v11::CwlDocument;

fn main() {
    let doc = CwlDocument::from_str("cwlVersion: v1.1\nclass: Workflow").unwrap();

    println!("{:?}", doc);
}