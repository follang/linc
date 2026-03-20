use std::path::PathBuf;

#[test]
fn torture_header_scans_through_public_header_config() {
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test/linus/c_interop_torture.h");
    let result = bic::HeaderConfig::new()
        .entry_header(&header)
        .include_dir("/usr/include")
        .include_dir("/usr/include/x86_64-linux-gnu")
        .no_origin_filter()
        .process()
        .unwrap();

    assert!(result.report.preprocessed_source.contains("torture_open"));
    assert!(result.report.preprocessed_source.contains("struct torture_config"));
}
