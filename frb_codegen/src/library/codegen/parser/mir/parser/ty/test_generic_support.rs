use crate::codegen::config::internal_config_parser::compute_force_codec_mode_pack;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::mir::internal_config::{
    ParserMirInternalConfig, RustInputNamespacePack,
};
use crate::codegen::parser::{parse_inner, MirPack};
use crate::utils::logs::configure_opinionated_test_logging;
use crate::utils::namespace::Namespace;
use crate::utils::test_utils::{
    create_path_sanitizers, get_test_fixture_dir, json_golden_test,
};
use log::info;
use serial_test::serial;
use std::path::{Path, PathBuf};

#[test]
#[serial]
fn test_generic_support_comprehensive() -> anyhow::Result<()> {
    body("library/codegen/generator/api_dart/mod/generics", None)
}

#[allow(clippy::type_complexity)]
fn body(
    fixture_name: &str,
    rust_input_namespace_pack: Option<Box<dyn Fn(&Path) -> RustInputNamespacePack>>,
) -> anyhow::Result<()> {
    let (actual_ir, rust_crate_dir) = execute_parse(fixture_name, rust_input_namespace_pack)?;
    
    // Check that generic structs are now being processed (not ignored)
    let struct_pool = &actual_ir.struct_pool;
    let enum_pool = &actual_ir.enum_pool;
    
    // Verify Container<T> is processed
    let container_struct = struct_pool.get("crate::api/Container");
    assert!(container_struct.is_some(), "Container<T> should be in struct pool");
    
    // Verify Pair<T, U> is processed  
    let pair_struct = struct_pool.get("crate::api/Pair");
    assert!(pair_struct.is_some(), "Pair<T, U> should be in struct pool");
    
    // Verify Option<T> enum is processed
    let option_enum = enum_pool.get("crate::api/Option");
    assert!(option_enum.is_some(), "Option<T> should be in enum pool");
    
    // Verify Either<L, R> enum is processed
    let either_enum = enum_pool.get("crate::api/Either");
    assert!(either_enum.is_some(), "Either<L, R> should be in enum pool");
    
    // Check that none of these are marked as ignored
    if let Some(container) = container_struct {
        assert!(!container.ignore, "Container<T> should not be ignored");
    }
    
    if let Some(pair) = pair_struct {
        assert!(!pair.ignore, "Pair<T, U> should not be ignored");
    }
    
    if let Some(option) = option_enum {
        assert!(!option.ignore, "Option<T> should not be ignored");
    }
    
    if let Some(either) = either_enum {
        assert!(!either.ignore, "Either<L, R> should not be ignored");
    }
    
    println!("✅ Generic support test passed!");
    Ok(())
}

fn execute_parse(
    fixture_name: &str,
    rust_input_namespace_pack: Option<Box<dyn Fn(&Path) -> RustInputNamespacePack>>,
) -> anyhow::Result<(MirPack, PathBuf)> {
    configure_opinionated_test_logging();
    let test_fixture_dir = get_test_fixture_dir(fixture_name);
    info!("test_fixture_dir={test_fixture_dir:?}");

    let rust_crate_dir = test_fixture_dir.clone();

    let rust_input_namespace_pack = rust_input_namespace_pack
        .map(|f| f(&rust_crate_dir))
        .unwrap_or_else(|| RustInputNamespacePack {
            rust_input_namespace_prefixes: vec![Namespace::new_self_crate("crate".to_owned())],
            rust_output_path_namespace: Namespace::new_self_crate("frb_generated".to_owned()),
        });

    let config = ParserInternalConfig {
        hir: ParserHirInternalConfig {
            rust_crate_dir: rust_crate_dir.clone(),
            rust_input_namespace_pack,
            ..Default::default()
        },
        mir: ParserMirInternalConfig {
            force_codec_mode_pack: compute_force_codec_mode_pack(
                CodecMode::Nom,
                RustOpaqueCodecMode::Nom,
            ),
            ..Default::default()
        },
    };

    let pb = GeneratorProgressBarPack::new();
    let dumper_mir = Dumper::new_standard(&test_fixture_dir, &[], &create_path_sanitizers())?;

    let actual_ir = parse_inner(&config, &pb, &dumper_mir)?;

    Ok((actual_ir, rust_crate_dir))
}
