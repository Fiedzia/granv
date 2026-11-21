use clap::Args;

#[derive(Clone, Debug, Args)]
pub struct File {
    #[arg(required = true)]
    pub path: String,
}
