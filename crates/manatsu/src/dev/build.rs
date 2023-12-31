use super::package::{self, is_standalone, PUBLIC_PACKAGES};
use anyhow::{bail, Result};
use colored::*;
use miho::pnpm;
use std::fs;
use std::time::Instant;

macro_rules! bold {
  ($message:expr) => {
    println!("{}", $message.bold());
  };
}

/// Build the packages.
pub fn build<I, S>(packages: I) -> Result<()>
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  let start = Instant::now();

  let filter_flag = "--filter";

  // The shared package must be built before anyone else.
  bold!("Building shared package...");
  pnpm!(["run", filter_flag, "shared", "build"])?;

  let mut args = vec!["run", "--parallel"];

  let iter = packages.into_iter();
  let packages = iter.map(|pkg| -> String {
    let pkg = pkg.as_ref();
    let pkg = pkg.trim().to_lowercase();
    pkg.replace("@manatsu/", "")
  });

  let packages: Vec<String> = packages.collect();
  if packages.is_empty() {
    bail!("Nothing to build");
  }

  for package in &packages {
    let package = package.as_str();

    if should_build(package) {
      args.push(filter_flag);
      args.push(package);
    }
  }

  if !args.contains(&filter_flag) {
    bail!("Selected package(s) cannot be built");
  }

  args.push("build");

  bold!("Building other packages...");
  pnpm!(args)?;

  bold!("Copying files...");
  copy_files(&packages)?;

  println!("Built in {:?}", start.elapsed());
  Ok(())
}

fn should_build(package: &str) -> bool {
  if PUBLIC_PACKAGES.contains(&package) {
    // The sass package doesn't need to be built.
    // And the shared package has already been built.
    package != "sass" && package != "shared"
  } else {
    // We shouldn't build private packages.
    false
  }
}

fn copy_files(packages: &Vec<String>) -> Result<()> {
  let dist = package::dist("manatsu")?;
  for pkg in packages {
    if !is_standalone(pkg) {
      let to = dist.join(format!("{pkg}.d.ts"));
      fs::copy(package::dts(pkg)?, to)?;
    }
  }

  Ok(())
}
