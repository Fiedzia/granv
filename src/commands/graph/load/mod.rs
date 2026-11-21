pub mod file;

use clap::{Args, Subcommand};
use file::File;

#[derive(Clone, Debug, Subcommand)]
pub enum SourceCommand {
    File(File),
}

#[derive(Clone, Debug, Args)]
pub struct LoadCommand {
    #[command(subcommand)]
    pub command: SourceCommand,
}
