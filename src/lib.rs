mod error;
mod ignore;

pub use error::Error;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

struct Ignorer {
  regexp: regex::bytes::RegexSet,
  prefix: String,
}

pub struct Ignorers(Vec<Ignorer>);

impl Ignorer {
  fn is_match(&self, fname: &str) -> bool {
    let fname = match fname.strip_prefix(&self.prefix) {
      Some(fname) => fname,
      None => return false,
    };
    self.regexp.is_match(fname.as_bytes())
  }

  fn new(mut fname: PathBuf) -> Result<Self, crate::Error> {
    let f = std::fs::File::open(&fname)?;
    let reader = io::BufReader::new(f);
    let regexp = ignore::parse_lines(reader.lines())?;
    fname.pop();
    let mut prefix = fname
      .into_os_string()
      .into_string()
      .unwrap_or_else(|err| err.to_string_lossy().to_string());
    prefix.push(std::path::MAIN_SEPARATOR);
    Ok(Ignorer { regexp, prefix })
  }
}

impl Ignorers {
  pub fn is_match(&self, fname: impl AsRef<Path>) -> Result<bool, crate::Error> {
    let fname = fname.as_ref().canonicalize()?;
    let mut s = fname.to_string_lossy().to_string();
    if fname.is_dir() {
      s.push(std::path::MAIN_SEPARATOR);
    }
    Ok(self.0.iter().any(|i| i.is_match(&s)))
  }

  pub fn new(p: impl AsRef<Path>) -> Result<Self, crate::Error> {
    let mut p = p.as_ref().canonicalize()?;
    if !p.is_dir() {
      p.pop();
    }
    let mut ignorers = Vec::new();
    for anc in p.ancestors() {
      let mut pbuf = anc.to_path_buf();
      pbuf.push(".tmpignore");
      if pbuf.is_file() {
        ignorers.push(Ignorer::new(pbuf)?)
      }
    }
    Ok(Ignorers(ignorers))
  }
}
