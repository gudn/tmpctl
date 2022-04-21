use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(about, author, version, long_about = None)]
pub(crate) struct Cli {
  #[clap(long, help = "really remove matched files")]
  pub(crate) force: bool,
  #[clap(parse(from_os_str), default_value = ".")]
  pub(crate) path: PathBuf,
}
