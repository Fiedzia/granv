use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(no_binary_name(true))]
pub struct Directory {
    #[arg(short = 's', long, default_value = "type,name_lc")]
    pub sort: Option<String>,
    #[arg(required = true)]
    pub path: String,
}
