use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn write_generated<F>(
    absolute_path: &Path,
    new_source: &str,
    postprocess: Option<F>,
) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    println!(
        "{} {}",
        "[WRITING FILE]".yellow(),
        absolute_path.display().to_string().cyan(),
    );

    let parent_dir = absolute_path
        .parent()
        .with_context(|| format!("Can't get parent of `{}`", &absolute_path.display()))?;

    if !parent_dir.exists() {
        println!(
            "{}",
            format!("[CREATE_DIRECTORY] {}", parent_dir.display()).yellow()
        );

        fs::create_dir_all(parent_dir)
            .with_context(|| format!("Failed to create parent dir `{}`", parent_dir.display()))?;
    }

    let mut old_source = None;
    if absolute_path.exists() {
        println!(
            "{}",
            format!("[FILE ALREADY EXISTS] {}", absolute_path.display()).yellow()
        );

        old_source = Some(fs::read_to_string(absolute_path)?);
    }

    fs::write(absolute_path, new_source)?;

    println!(
        "{} {}",
        "[WRITTEN]".yellow(),
        absolute_path.display().to_string().cyan()
    );

    if let Some(postprocess) = postprocess {
        postprocess().context("Failed on postprocess step")?;
    }

    let postprocessed_new_source = fs::read_to_string(absolute_path)?;

    if let Some(old_source) = old_source {
        if let Some(diff) = colored_diff(&old_source, &postprocessed_new_source) {
            println!("{}", "[DIFF BETWEEN OLD AND NEW SOURCE BELOW]".yellow());
            println!("{}", "-------------------------------------------".dimmed());
            println!("{}", diff);
            println!("{}", "-------------------------------------------".dimmed());
        } else {
            println!("<NO VISUAL DIFFERENCE>");
        }
    }

    Ok(())
}

pub fn colored_diff(left: &str, right: &str) -> Option<String> {
    use diff::{lines, Result};
    let mut result = String::new();

    if left == right {
        return None;
    }

    let lines = lines(left, right);
    result.push('\n');
    for line in lines {
        match line {
            Result::Left(l) => {
                result.push_str(&format!("{} {}\n", "-".red(), &l.red()));
            }
            Result::Right(r) => {
                result.push_str(&format!("{} {}\n", "+".green(), &r.green()));
            }
            Result::Both(l, _r) => {
                result.push_str(&format!("  {}\n", &l.dimmed()));
            }
        }
    }
    Some(result)
}
