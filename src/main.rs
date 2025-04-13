mod commands {
    pub mod build;
    pub mod completion;
    pub mod manifesto;
}

use clap::builder::Styles;
use clap::builder::styling::AnsiColor;
use clap::{CommandFactory, Parser, Subcommand};

use commands::build::BuildCommand;
use commands::completion::CompletionCommand;
use commands::manifesto::ManifestoCommand;

const FE_STYLE: clap::builder::styling::Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default().bold())
    .usage(AnsiColor::Cyan.on_default())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(styles = FE_STYLE)]
#[command(allow_external_subcommands = true)]
// #[command(flatten_help = true)]
//#[command(infer_subcommands = true)]
// #[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build(BuildCommand),
    Manifesto(ManifestoCommand),
    Completion(CompletionCommand),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Manifesto(cmd) => {
            cmd.preach();
        }
        Commands::Completion(cmd) => {
            let mut command = Cli::command();
            cmd.complete(&mut command);
        }
        Commands::Build(cmd) => {
            cmd.build();
        }
    }
}
