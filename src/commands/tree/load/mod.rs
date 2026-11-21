pub mod directory;
pub mod file;

use clap::Parser;
use directory::Directory;
use file::File;

#[derive(Clone, Debug, Parser)]
#[clap(no_binary_name(true))]
pub enum SourceCommand {
    File(File),
    Directory(Directory),
}

#[derive(Clone, Debug, Parser)]
pub struct LoadCommand {
    #[command(subcommand)]
    pub command: SourceCommand,
}
