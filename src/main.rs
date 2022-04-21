mod cli;

use clap::Parser;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, tmpctl::Error>;

fn dfs<P, C>(path: P, cb: &mut C, ignorers: &tmpctl::Ignorers) -> Result<bool>
where
  P: AsRef<Path>,
  C: FnMut(PathBuf) -> Result<()>,
{
  let path = path.as_ref();
  let mut should_delete = !ignorers.is_match(path)?;
  for entry in path.read_dir()? {
    let entry = entry?;
    let p = entry.path();
    if entry.metadata()?.is_dir() {
      if dfs(&p, cb, ignorers)? {
        cb(p)?;
      } else {
        should_delete = false;
      }
    } else {
      if ignorers.is_match(&p)? {
        should_delete = false;
      } else {
        cb(p)?;
      }
    }
  }
  Ok(should_delete)
}

fn dry_run(p: PathBuf) -> Result<()> {
  println!("{}", p.display());
  Ok(())
}

fn remove(p: PathBuf) -> Result<()> {
  if p.is_dir() {
    std::fs::remove_dir(p).map_err(tmpctl::Error::from)
  } else {
    std::fs::remove_file(p).map_err(tmpctl::Error::from)
  }
}

fn main() -> Result<()> {
  let args = cli::Cli::parse();
  let ignorers = tmpctl::Ignorers::new(&args.path)?;
  let p = std::fs::canonicalize(args.path)?;
  if args.force {
    dfs(p, &mut remove, &ignorers)?;
  } else {
    dfs(p, &mut dry_run, &ignorers)?;
  }
  Ok(())
}
