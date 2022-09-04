use clap::{Parser, Subcommand};
use std::error::Error;
use std::process;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// generates merkle root from file consisting of accounts and
    /// amounts in csv format at a given path
    /// the first column must be an address and second column is an amount
    /// in cosmos-sdk Coin string format.
    GenerateMerkleRoot {
     /// The path to the file to read
    #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    },
}

fn generate_merkle_root_cmd(path: std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::GenerateMerkleRoot { path }) => {
            if let Err(err) = generate_merkle_root_cmd(path.to_path_buf()) {
                println!("error generating merkle root: {}", err);
                process::exit(1);
            }
        }
        None => {}
    }
}
