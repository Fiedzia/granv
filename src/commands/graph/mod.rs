pub mod load;

use crate::commands::graph::load::LoadCommand;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct GraphCommandsWrapper {
    pub commands: Vec<String>,
}

#[derive(Clone, Debug, Parser)]
#[clap(no_binary_name(true))]
pub enum GraphCommands {
    Load(LoadCommand),
}

#[derive(Clone, Debug, Parser)]
pub struct GraphCommand {
    #[command(subcommand)]
    pub command: GraphCommands,
}
