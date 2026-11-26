mod commands;
mod formats;
mod layouts;
mod model;
mod renderer;
mod templates;

use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use commands::graph::load::SourceCommand as GraphSourceCommand;
use commands::graph::{GraphCommand, GraphCommands};
use commands::tree::load::SourceCommand as TreeSourceCommand;
use commands::tree::{TreeCommand, TreeCommands};
use commands::{Cli, TopLevelCommands};

use crate::model::tree::load::directory::build_tree;
use formats::svg::dirtree::export_dirtree_svg;
use layouts::dirtree::svg::layout_dirtree_nodes_and_lines_svg;
use renderer::svg::render_svg_templates;
use templates::askama::tree::dirtree::{AskamaDirTreeTemplateSVGPlain, RenderSVG};

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

                        let m = MultiProgress::new();
                        let sty =
                            ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
                                .unwrap();

                        let pb = m.add(ProgressBar::new(0));
                        pb.set_style(sty.clone());
                        pb.set_message("Loading directory");

                        // load tree data
                        let tree = build_tree(&directory, Some(&pb))?;

                        //render node templates
                        let pb = m.add(ProgressBar::new(tree.tree.nodes().len() as u64));
                        let render_style = ProgressStyle::with_template(
                            "[{elapsed_precise}] {bar:40} {pos:>7}/{len:7} {msg}",
                        )
                        .unwrap();
                        pb.set_style(render_style);
                        pb.set_message("Rendering node templates");

                        let template_render_ref = AskamaDirTreeTemplateSVGPlain::render;
                        let rendered_node_templates =
                            render_svg_templates(&tree, template_render_ref, Some(&pb));

                        //layout node templates
                        let layout_style = ProgressStyle::with_template(
                            "[{elapsed_precise}] {bar:40} {pos:>7}/{len:7} {msg}",
                        )
                        .unwrap();
                        pb.set_style(layout_style);
                        pb.set_message("Generating layout");

                        //
                        let (node_layout, lines_layout) =
                            layout_dirtree_nodes_and_lines_svg(&tree, &rendered_node_templates)?;
                        //render lines
                        //generate final svg file
                        export_dirtree_svg(&rendered_node_templates, &node_layout)?;
                    }
                    _ => {}
                },
            }
        }
    };
    Ok(())
}
