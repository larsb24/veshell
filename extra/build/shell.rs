use std::path::Path;

use crate::{flutter_sdk::FLUTTER_REPO_DIR, FlutterEngineBuild};
use lazy_static::lazy_static;

lazy_static! {
    static ref dart_bin_path: String = format!("{FLUTTER_REPO_DIR}/bin/dart");
    static ref flutter_bin_path: String = format!("{FLUTTER_REPO_DIR}/bin/flutter");
}

const SHELL_DIRECTORY: &str = "src/shell";

pub fn build_shell(
    flutter_engine_build: FlutterEngineBuild,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=extra/build/shell.rs");
    println!("cargo:rerun-if-changed=src/shell/pubspec.yaml");
    println!("cargo:rerun-if-changed=src/shell/lib");

    let absolute_flutter_bin = Path::new(&*flutter_bin_path).canonicalize()?;
    let absolute_dart_bin = Path::new(&*dart_bin_path).canonicalize()?;
    let absolute_shell_directory = Path::new(&SHELL_DIRECTORY).canonicalize()?;
    // get pub dependencies
    let output = std::process::Command::new(absolute_flutter_bin.clone())
        .arg("pub")
        .arg("get")
        .current_dir(absolute_shell_directory.clone())
        .status()?;

    if !output.success() {
        panic!("Failed to get shell pub dependencies");
    }

    // run build_runner
    println!("Running build_runner...");
    let output = std::process::Command::new(absolute_dart_bin)
        .arg("run")
        .arg("build_runner")
        .arg("build")
        .arg("--delete-conflicting-outputs")
        .current_dir(absolute_shell_directory.clone())
        .status()?;
    if !output.success() {
        panic!("Failed to run build_runner");
    }

    // build shell
    println!("Building shell...");
    let output = std::process::Command::new(absolute_flutter_bin)
        .arg("build")
        .arg("linux")
        .arg(match flutter_engine_build {
            FlutterEngineBuild::Debug => "--debug",
            FlutterEngineBuild::Profile => "--profile",
            FlutterEngineBuild::Release => "--release",
        })
        .current_dir(absolute_shell_directory.clone())
        .status()?;

    if !output.success() {
        // print the command executed
        println!("Command executed: {:?}", output);
        panic!("Failed to build shell");
    }
    Ok(())
}
