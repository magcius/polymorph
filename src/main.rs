use std::path::PathBuf;

use clap::{arg, Parser, Subcommand};
use polymorph::{cdn::CDNFetcher, error::Error};

const PATCH_SERVER: &str = "http://us.patch.battle.net:1119";
const PRODUCT: &str = "wow_classic";
const REGION: &str = "us";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    cache_path: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Serve {
        #[arg(short, long, default_value_t = 8081)]
        port: u16,

        #[arg(short, long, default_value_t = false)]
        no_fetch: bool,
    },
    GetId {
        file_id: u32,

        #[arg(short, long, value_name = "FILE")]
        out_path: PathBuf,
    },
    GetName {
        name: String,

        #[arg(short, long, value_name = "FILE")]
        out_path: PathBuf,
    },
    Init,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let cli = Cli::parse();
    let fetcher = CDNFetcher::init(cli.cache_path, PATCH_SERVER, PRODUCT, REGION).await?;
    match cli.command {
        Commands::Serve { .. } => todo!(),
        Commands::GetId { file_id, out_path } => {
            let data = fetcher.fetch_file_id(file_id).await?;
            tokio::fs::write(out_path, &data).await?;
            Ok(())
        },
        Commands::GetName { .. } => todo!(),
        Commands::Init => {
            let num_archives = fetcher.archive_index.len();
            let mut i = 0;
            for archive in &fetcher.archive_index {
                println!("[{}/{}] fetching archive {}", i, num_archives, archive.key);
                fetcher.fetch_archive(archive).await?;
                i += 1;
            }
            Ok(())
        },
    }
}
