mod commands {
    pub mod porcelain {
        pub mod compile;
    }
    pub mod build;
    pub mod completion;
    pub mod manifesto;
}

use std::ffi::OsString;

use clap::builder::Styles;
use clap::builder::styling::AnsiColor;
use clap::{CommandFactory, Parser, Subcommand};

use commands::build::BuildCommand;
use commands::completion::CompletionCommand;
use commands::manifesto::ManifestoCommand;
use commands::porcelain::compile::CompileCommand;

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
    #[command(flatten)]
    color: colorchoice_clap::Color,
}


#[derive(Subcommand)]
enum Commands {
    Compile(CompileCommand),

    Build(BuildCommand),
    Manifesto(ManifestoCommand),
    Completion(CompletionCommand),
    
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compile(cmd) => {
            cmd.build();
        }
        Commands::Manifesto(cmd) => {
            cli.color.write_global();
            cmd.preach();
        }
        Commands::Completion(cmd) => {
            let mut command = Cli::command();
            cmd.complete(&mut command);
        }
        Commands::Build(cmd) => {
            cmd.build();
        }
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }
}
