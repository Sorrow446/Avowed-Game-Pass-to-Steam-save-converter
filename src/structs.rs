use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Yandex Music Downloader")]
pub struct Args {
    #[clap(short, long, help="Input path to Game Pass save folder to convert.")]
    pub in_path: PathBuf,

    #[clap(short, long, help="Output path to write converted Steam save to excluding filename. Leave empty for binary/script dir.")]
    pub out_path: Option<PathBuf>,
}

pub struct Config {
    pub in_path: PathBuf,
    pub out_path: PathBuf,
}