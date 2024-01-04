mod build;
mod json;
pub mod package;
mod release;
pub mod scaffold;

use anyhow::{Context, Result};
pub use build::build;
use miho::win_cmd;
pub use release::release;
use std::process::Stdio;
use std::{env, fs};

/// Synchronizes all README files of the monorepo.
pub fn readme() -> Result<()> {
  let filename = "README.md";
  let cwd = env::current_dir()?;
  let src_readme = cwd.join(filename);

  println!("Copying README files...");
  for pkg in package::all() {
    let dest_readme = package::dir(pkg)?.join(filename);
    fs::copy(&src_readme, &dest_readme)?;
    println!("Copied: {}", dest_readme.display());
  }

  println!("Done!");
  Ok(())
}

/// Format files using Prettier.
pub fn format_files<G: AsRef<str>>(glob: G) -> Result<()> {
  let glob = glob.as_ref();

  println!("Formatting files...");
  win_cmd!("pnpm")
    .args(["exec", "prettier", glob, "--write"])
    .stderr(Stdio::inherit())
    .stdout(Stdio::inherit())
    .output()
    .with_context(|| format!("Could not format files: {}", glob))?;

  Ok(())
}

/// Lint files, fixing as many issues as possible.
pub fn lint<G: AsRef<str>>(glob: G, extra_args: Option<Vec<&str>>) -> Result<()> {
  let glob = glob.as_ref();
  let mut cmd = win_cmd!("pnpm");
  cmd.args(["exec", "eslint", "--fix"]);

  if let Some(args) = extra_args {
    cmd.args(args);
  }

  println!("Linting files...");
  cmd
    .arg(glob)
    .stderr(Stdio::inherit())
    .stdout(Stdio::inherit())
    .output()
    .with_context(|| format!("Could not lint files: {}", glob))?;

  Ok(())
}