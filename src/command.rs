use crate::{Environment, KernelParameterManager};
use structopt::StructOpt;

/// The top level of the CLI
#[derive(Debug, StructOpt)]
pub struct Command {
  #[structopt(
    long,
    short,
    env = "KPM_PREFIX",
    default_value = "sysctl-",
    help = "sets the prefix for environment variables."
  )]
  pub prefix: String,

  #[structopt(subcommand)]
  pub action: Action,
}

impl Command {
  pub fn execute(&self) -> i32 {
    let environment = Environment::new(&self.prefix);
    let manager = KernelParameterManager::from(environment);

    self.action.execute(&manager)
  }
}

#[derive(Debug, StructOpt)]
pub enum Action {
  #[structopt(alias = "a", about = "report non-compliant parameters")]
  Audit,
  #[structopt(
    alias = "e",
    about = "modify non-compliant parameters to match target values"
  )]
  Enforce,
  #[structopt(about = "dump internal state to STDOUT and exit")]
  Dump,
}

impl Action {
  pub fn execute(&self, manager: &KernelParameterManager) -> i32 {
    match self {
      Self::Audit => manager.audit(),
      Self::Enforce => manager.enforce(),
      Self::Dump => manager.dump(),
    }
  }
}
