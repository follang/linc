#[path = "../test/stress/libpcap.rs"]
mod libpcap;
#[path = "../test/stress/zlib.rs"]
mod zlib;

#[test]
fn zlib_vendored_example_is_code_driven_and_consumable() {
    let environment = zlib::zlib_vendored_environment().unwrap();
    let config = zlib::zlib_vendored_header_config().unwrap();
    let result = zlib::analyze_zlib_vendored().unwrap();

    assert!(environment.include_dir.ends_with("zlib/header/include"));
    assert!(environment.entry_header.ends_with("zlib/header/include/zlib.h"));
    assert_eq!(config.binding_surface().entry_headers.len(), 1);
    assert_eq!(config.preprocessing().include_dirs.len(), 1);
    assert!(result.package.find_function("deflate").is_some());
    assert!(result.package.find_function("inflate").is_some());
    assert!(result
        .package
        .layouts
        .iter()
        .any(|layout| layout.name == "z_stream" && layout.size > 0));
}

#[test]
fn libpcap_example_is_code_driven_and_consumable() {
    let Ok(environment) = libpcap::libpcap_environment() else {
        return;
    };

    let config = libpcap::libpcap_header_config().unwrap();
    let result = libpcap::analyze_libpcap().unwrap();

    assert!(environment.header.ends_with("pcap.h") || environment.header.ends_with("pcap/pcap.h"));
    assert!(!environment.support_headers.is_empty());
    assert!(config
        .linking()
        .link_libraries
        .iter()
        .any(|library| library.name == "pcap"));
    assert!(result.package.find_function("pcap_open_live").is_some());
    assert!(result.package.find_function("pcap_loop").is_some());
    assert!(result.package.find_record("pcap_pkthdr").is_some());
    assert!(result.package.find_type_alias("pcap_handler").is_some());
}
