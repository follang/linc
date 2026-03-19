//! Real-world corpus tests for BIC.
//!
//! These tests parse real C library headers and validate the output.
//! They are ignored by default and require the corresponding system
//! headers/libraries to be installed.

use std::path::Path;

use bic::*;

fn find_system_header(name: &str) -> Option<std::path::PathBuf> {
    let paths = [
        format!("/usr/include/{}", name),
        format!("/usr/local/include/{}", name),
        format!("/usr/include/x86_64-linux-gnu/{}", name),
    ];
    for p in &paths {
        let path = Path::new(p);
        if path.exists() {
            return Some(path.to_path_buf());
        }
    }
    None
}

fn find_system_lib(name: &str) -> Option<std::path::PathBuf> {
    let paths = [
        format!("/usr/lib/{}", name),
        format!("/usr/lib/x86_64-linux-gnu/{}", name),
        format!("/usr/local/lib/{}", name),
        format!("/lib/x86_64-linux-gnu/{}", name),
    ];
    for p in &paths {
        let path = Path::new(p);
        if path.exists() {
            return Some(path.to_path_buf());
        }
    }
    None
}

// ============================================================================
// zlib corpus
// ============================================================================

#[test]
#[ignore] // Requires zlib-dev installed
fn zlib_parse_filtered() {
    let header = match find_system_header("zlib.h") {
        Some(p) => p,
        None => {
            eprintln!("SKIP: zlib.h not found");
            return;
        }
    };

    let result = bic::raw_headers::HeaderConfig::new()
        .header(&header)
        .process()
        .unwrap();

    let funcs: Vec<&str> = result
        .package
        .items
        .iter()
        .filter_map(|i| match i {
            BindingItem::Function(f) => Some(f.name.as_str()),
            _ => None,
        })
        .collect();

    // With origin filtering, should only get zlib functions (75-90)
    assert!(
        funcs.len() >= 30,
        "expected at least 30 zlib functions, got {}",
        funcs.len()
    );

    // Key zlib functions
    assert!(funcs.contains(&"deflate"), "missing deflate");
    assert!(funcs.contains(&"inflate"), "missing inflate");
    assert!(funcs.contains(&"compress"), "missing compress");
    assert!(funcs.contains(&"uncompress"), "missing uncompress");

    // Should NOT contain system functions
    assert!(!funcs.contains(&"printf"), "printf should be filtered out");
    assert!(!funcs.contains(&"memcpy"), "memcpy should be filtered out");

    // Check key types
    let type_names: Vec<&str> = result
        .package
        .items
        .iter()
        .filter_map(|i| match i {
            BindingItem::TypeAlias(t) => Some(t.name.as_str()),
            _ => None,
        })
        .collect();

    assert!(
        type_names.contains(&"Bytef") || type_names.contains(&"uLong"),
        "expected zlib typedefs"
    );
}

#[test]
#[ignore] // Requires zlib-dev and libz installed
fn zlib_validate_symbols() {
    let header = match find_system_header("zlib.h") {
        Some(p) => p,
        None => return,
    };
    let lib = match find_system_lib("libz.so") {
        Some(p) => p,
        None => return,
    };

    let result = bic::raw_headers::HeaderConfig::new()
        .header(&header)
        .process()
        .unwrap();

    let inventory = inspect_symbols(&lib).unwrap();
    let report = validate(&result.package, &inventory);

    // Most zlib functions should match
    let matched = report.matched().len();
    let total = report.matches.len();
    assert!(
        matched > 0,
        "expected some matched symbols, got 0 of {}",
        total
    );

    eprintln!(
        "zlib validation: {}/{} matched, {} missing",
        matched,
        total,
        report.missing().len()
    );
}

#[test]
#[ignore] // Requires zlib-dev
fn zlib_codegen() {
    let header = match find_system_header("zlib.h") {
        Some(p) => p,
        None => return,
    };

    let result = bic::raw_headers::HeaderConfig::new()
        .header(&header)
        .process()
        .unwrap();

    let rust_ffi = emit_rust_ffi(&result.package);
    assert!(rust_ffi.contains("extern \"C\""));
    assert!(rust_ffi.contains("pub fn deflate"));
    assert!(rust_ffi.contains("pub fn inflate"));

    // JSON roundtrip
    let json = bic::to_json(&result.package).unwrap();
    let pkg2 = bic::from_json(&json).unwrap();
    assert_eq!(result.package, pkg2);
}

// ============================================================================
// string.h corpus (musl-compatible)
// ============================================================================

#[test]
#[ignore] // Requires gcc/clang
fn string_h_parse() {
    let header = match find_system_header("string.h") {
        Some(p) => p,
        None => return,
    };

    let result = bic::raw_headers::HeaderConfig::new()
        .header(&header)
        .process()
        .unwrap();

    let funcs: Vec<&str> = result
        .package
        .items
        .iter()
        .filter_map(|i| match i {
            BindingItem::Function(f) => Some(f.name.as_str()),
            _ => None,
        })
        .collect();

    // Key string functions
    let expected = ["memcpy", "memset", "strlen", "strcmp", "memmove", "memcmp"];
    for name in &expected {
        assert!(funcs.contains(name), "missing {}", name);
    }
}

#[test]
#[ignore] // Requires gcc/clang
fn string_h_const_correctness() {
    let header = match find_system_header("string.h") {
        Some(p) => p,
        None => return,
    };

    let result = bic::raw_headers::HeaderConfig::new()
        .header(&header)
        .process()
        .unwrap();

    // Find strlen — should have const char * parameter
    let strlen = result
        .package
        .items
        .iter()
        .find_map(|i| match i {
            BindingItem::Function(f) if f.name == "strlen" => Some(f),
            _ => None,
        });

    if let Some(f) = strlen {
        assert_eq!(f.parameters.len(), 1);
        assert_eq!(
            f.parameters[0].ty,
            BindingType::const_ptr(BindingType::Char),
            "strlen parameter should be const char *"
        );
    }

    // Find memcpy — dest should be *mut void, src should be *const void
    let memcpy = result
        .package
        .items
        .iter()
        .find_map(|i| match i {
            BindingItem::Function(f) if f.name == "memcpy" => Some(f),
            _ => None,
        });

    if let Some(f) = memcpy {
        assert!(f.parameters.len() >= 3);
        assert_eq!(
            f.parameters[0].ty,
            BindingType::ptr(BindingType::Void),
            "memcpy dest should be void *"
        );
        assert_eq!(
            f.parameters[1].ty,
            BindingType::const_ptr(BindingType::Void),
            "memcpy src should be const void *"
        );
    }
}

#[test]
#[ignore] // Requires gcc/clang
fn string_h_codegen() {
    let header = match find_system_header("string.h") {
        Some(p) => p,
        None => return,
    };

    let result = bic::raw_headers::HeaderConfig::new()
        .header(&header)
        .process()
        .unwrap();

    let rust_ffi = emit_rust_ffi(&result.package);
    assert!(rust_ffi.contains("pub fn memcpy"));
    assert!(rust_ffi.contains("pub fn strlen"));
    assert!(rust_ffi.contains("*const"));
    assert!(rust_ffi.contains("*mut"));
}

// ============================================================================
// libpng corpus
// ============================================================================

#[test]
#[ignore] // Requires libpng-dev installed
fn libpng_parse() {
    let header = match find_system_header("png.h") {
        Some(p) => p,
        None => {
            eprintln!("SKIP: png.h not found");
            return;
        }
    };

    let result = bic::raw_headers::HeaderConfig::new()
        .header(&header)
        .process()
        .unwrap();

    let funcs: Vec<&str> = result
        .package
        .items
        .iter()
        .filter_map(|i| match i {
            BindingItem::Function(f) => Some(f.name.as_str()),
            _ => None,
        })
        .collect();

    // Key libpng functions
    assert!(
        funcs.iter().any(|f| f.starts_with("png_")),
        "expected png_ prefixed functions"
    );
}

// ============================================================================
// JSON snapshot stability
// ============================================================================

#[test]
#[ignore] // Requires gcc/clang
fn json_snapshot_determinism() {
    let header = match find_system_header("string.h") {
        Some(p) => p,
        None => return,
    };

    let result1 = bic::raw_headers::HeaderConfig::new()
        .header(&header)
        .process()
        .unwrap();
    let result2 = bic::raw_headers::HeaderConfig::new()
        .header(&header)
        .process()
        .unwrap();

    let json1 = bic::to_json(&result1.package).unwrap();
    let json2 = bic::to_json(&result2.package).unwrap();
    assert_eq!(json1, json2, "JSON output should be deterministic");
}
