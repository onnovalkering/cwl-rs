mod common;

#[test]
fn real_world_clts() {
    let files = [
        "tests/examples/prefactor-ampl.cwl",
        "tests/examples/prefactor-calib-cal.cwl",
        "tests/examples/prefactor-create-image.cwl",
        "tests/examples/prefactor-createmap-cal.cwl",
        "tests/examples/prefactor-do-magic.cwl",
        "tests/examples/prefactor-examine-npys.cwl",
        "tests/examples/prefactor-fitclock.cwl",
        "tests/examples/prefactor-h5imp_cal.cwl",
        "tests/examples/prefactor-ndpp-prep-cal.cwl",
        "tests/examples/prefactor-phase.cwl",
        "tests/examples/prefactor-plot-cal-phases.cwl",
        "tests/examples/prefactor-plots.cwl",
        "tests/examples/prefactor-sky-cal.cwl",
    ];

    for file in &files {
        let doc = common::load(file);
        assert!(doc.is_ok());

        let clt = common::unpack_clt(doc.unwrap());
        assert!(clt.is_some());
    }
}
