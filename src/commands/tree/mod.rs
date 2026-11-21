pub mod load;

use crate::commands::tree::load::LoadCommand;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(no_binary_name(true))]
pub struct TreeCommandsWrapper {
    #[clap(allow_hyphen_values = true)]
    pub commands: Vec<String>,
}

#[derive(Clone, Debug, Parser)]
#[clap(no_binary_name(true))]
pub enum TreeCommands {
    Load(LoadCommand),
}

#[derive(Clone, Debug, Parser)]
#[clap(no_binary_name(true))]
pub struct TreeCommand {
    #[command(subcommand)]
    pub command: TreeCommands,
}
