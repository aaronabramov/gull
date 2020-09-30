use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub const TEST_CARGO_TOML: &str = r#"
[package]
name = "gull_test_crate"
version = "0.1.0"
authors = ["Some Dude <dude@bro.io>"]
edition = "2018"

[dependencies]

[lib]
name = "test"
path = "lib.rs"

[workspace]
"#;

// Test utility/Abstraction over a temporary directory containing a test project.
// Given a name of the project it will create an empty directory under. (deleting anything that
// is in its place from potential previous runs)
//      /absolute/path/to/gull/gull/tmp
//
// And provide an API for creating/modifying files under this project directory.
#[derive(Debug)]
pub struct Project {
    root_dir: PathBuf,
}

impl Project {
    pub fn new(name: &str) -> Result<Self> {
        let gull_root_dir = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").expect("Can't get project root directory"),
        );

        let mut root_dir = gull_root_dir;
        root_dir.push("tmp");
        root_dir.push(name);
        // Make sure there's no ghost files in the tmp dir from previous runs
        fs::remove_dir_all(&root_dir).ok();

        Ok(Self { root_dir })
    }

    // Write provided content to a file under the given path.
    //  write_file("Cargo.toml", "[package]")?;
    //
    // Will write a file under `absolute/path/to/gull/gull/tmp/test_project_name/Cargo.toml`
    pub fn write_file(&self, path: &str, content: &str) -> Result<()> {
        fs::create_dir_all(&self.root_dir).context("can't create project root dir")?;
        let mut absolute_path = self.root_dir.clone();
        absolute_path.push(path);
        fs::write(&absolute_path, content)
            .with_context(|| format!("failed to write file `{}`", absolute_path.display()))?;
        Ok(())
    }

    // Write a deafult template for Cargo.toml file that should work for an average
    // empty rust projcet
    pub fn write_default_cargo_toml(&self) -> Result<()> {
        self.write_file("Cargo.toml", TEST_CARGO_TOML)?;
        Ok(())
    }

    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }
}
