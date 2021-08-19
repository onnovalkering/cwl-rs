mod common;

#[test]
fn real_world_wfs() {
    let files = ["tests/examples/prefactor.cwl"];

    for file in &files {
        let doc = common::load(file);
        assert!(doc.is_ok());

        let wf = common::unpack_wf(doc.unwrap());
        assert!(wf.is_some());
    }
}
