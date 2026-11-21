mod commands;
mod model;

use clap::Parser;

use commands::graph::load::SourceCommand as GraphSourceCommand;
use commands::graph::{GraphCommand, GraphCommands};
use commands::tree::load::SourceCommand as TreeSourceCommand;
use commands::tree::{TreeCommand, TreeCommands};
use commands::{Cli, TopLevelCommands};

use crate::model::tree::load::directory::build_tree;

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        TopLevelCommands::Graph(graph_commands_wrapper) => {
            let cmd_chain = GraphCommand::parse_from(graph_commands_wrapper.commands);
            match cmd_chain.command {
                GraphCommands::Load(load_command) => match load_command.command {
                    GraphSourceCommand::File(file) => {
                        println!("file::{}", file.path);
                    }
                },
            }
        }

        TopLevelCommands::Tree(tree_commands_wrapper) => {
            let cmd_chain = TreeCommand::parse_from(tree_commands_wrapper.commands);

            match cmd_chain.command {
                TreeCommands::Load(load_command) => match load_command.command {
                    TreeSourceCommand::Directory(directory) => {
                        println!("directory::{:?}", directory);
                        let _tree = build_tree(&directory)?;
                    }
                    _ => {}
                },
            }
        }
    };
    Ok(())
}
