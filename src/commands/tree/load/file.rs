use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct File {
    #[arg(required = true)]
    pub path: String,
}
