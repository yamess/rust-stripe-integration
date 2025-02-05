use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub config_path: String,
    #[clap(short, long)]
    pub secret_path: String,
}
