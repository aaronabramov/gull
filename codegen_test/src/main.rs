#![allow(clippy::inconsistent_struct_constructor)]

mod declarations;

#[cfg(test)]
mod generated;
#[cfg(test)]
mod tests;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    codegen().expect("Failed to codegen types");
}

fn format_file(absolute_path: &Path) -> Result<()> {
    let file_dir = absolute_path.parent().expect("must have parent dir");
    let mut command = Command::new("cargo");
    command.arg("fmt").current_dir(file_dir);

    let output = command
        .arg(absolute_path)
        .output()
        .context("Failed to format generated file")?;

    if !output.status.success() {
        let stdout = std::str::from_utf8(&output.stdout)?;
        let stderr = std::str::from_utf8(&output.stderr)?;

        anyhow::bail!(
            "Failed to run formatter on the file `{}`\nSTDOUT:\n{}\nSTDERR:\n{}\n",
            absolute_path.display(),
            stdout,
            stderr
        );
    }
    Ok(())
}

fn codegen() -> Result<()> {
    let d = declarations::make_declarations();

    let source = d.codegen_rust()?;

    let dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("must have cargo manifest dir if run with cargo");

    let mut codegen_path = PathBuf::from(dir);
    codegen_path.push("src/generated.rs");

    dbg!(&codegen_path.display().to_string());

    gull::write_generated(&codegen_path, &source, Some(|| format_file(&codegen_path)))?;

    Ok(())
}
