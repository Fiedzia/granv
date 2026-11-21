pub mod graph;
pub mod tree;

use crate::commands::graph::GraphCommandsWrapper;
use crate::commands::tree::TreeCommandsWrapper;
use clap::{Parser, Subcommand};

#[derive(Clone, Debug, Subcommand)]
pub enum TopLevelCommands {
    Graph(GraphCommandsWrapper),
    Tree(TreeCommandsWrapper),
}

#[derive(Clone, Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: TopLevelCommands,
}
