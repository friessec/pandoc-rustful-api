use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(env = "SERVER_ADDRESS", default_value = "localhost")]
    server_address: String,
    #[clap(env = "SERVER_PORT", default_value = "8000", parse(try_from_str))]
    server_port: usize,
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
        id: String,
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
    Download {},
}

fn main() {
    let cli = Cli::parse();

    println!("{}:{}", cli.server_address, cli.server_port);

    match &cli.command {
        Commands::List {} => {
            println!("List")
        }
        Commands::Create {} => {
            println!("Create")
        }
        Commands::Job { id, task } => {
            println!("Job {}", id);
            match &task {
                Tasks::Status {} => {
                    println!("Status")
                }
                Tasks::Delete {} => {
                    println!("Delete")
                }
                Tasks::Upload { file } => {
                    println!("Upload")
                }
                Tasks::Process {} => {
                    println!("Process")
                }
                Tasks::Download {} => {
                    println!("Download")
                }
            }
        }
    }
}