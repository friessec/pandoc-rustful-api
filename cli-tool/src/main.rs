mod client;
use clap::{Parser, Subcommand};
use std::io::Write as FmtWrite;
use env_logger::WriteStyle;

#[derive(clap::Args)]
struct LogArgs {
    #[clap(long, short = 'v', parse(from_occurrences))]
    verbose: i8,
}

fn log_level_filter(lvl: i8) -> log::LevelFilter {
    match lvl {
    0 => log::LevelFilter::Debug,
    1 | _ => log::LevelFilter::Trace,
    }
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    logging: LogArgs,
    #[clap(long, env = "SERVER_ADDRESS", default_value = "http://localhost")]
    address: String,
    #[clap(long, env = "SERVER_PORT", default_value = "8000", parse(try_from_str))]
    port: usize,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "List all jobs")]
    List {},
    #[clap(about = "Create a new job")]
    Create {},
    #[clap(about = "Job specific commands, requires a valid ID")]
    Job {
        #[clap(required = true, env = "JOB_ID")]
        id: uuid::Uuid,
        #[clap(subcommand)]
        task: Tasks,
    },
}

#[derive(Subcommand)]
enum Tasks {
    Status {},
    Delete {},
    Upload {
        #[clap(required = true)]
        file: String
    },
    Process {},
    Download {
        #[clap(required = true)]
        file: String
    },
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_bold(false);
            writeln!(buf, "{}", style.value(record.args()))
        })
        .filter(Some("crate::client"), log::LevelFilter::Info)
        .filter_level(log_level_filter(cli.logging.verbose))
        .write_style(WriteStyle::Always)
        .init();

    let client = crate::client::Client::new(&cli.address, cli.port);
    match &cli.command {
        Commands::List {} => {
            client.list().await?;
        }
        Commands::Create {} => {
            client.create().await?;
        }
        Commands::Job { id, task } => {
            match &task {
                Tasks::Status {} => {
                    client.status(id).await?;
                }
                Tasks::Delete {} => {
                    client.delete(id).await?;
                }
                Tasks::Upload { file } => {
                    client.upload(id, file).await?;
                }
                Tasks::Process {} => {
                    client.process(id).await?;
                }
                Tasks::Download { file} => {
                    client.download(id, file).await?;
                }
            }
        }
    }

    Ok(())
}