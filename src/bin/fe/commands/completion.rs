use std::io;

use clap::{value_parser, Args, Command};
use clap_complete::{generate, Generator, Shell};

#[derive(Args)]
#[command(alias = "comp")]
#[command(about = "Generate shell completions for the given shell")]
pub(crate) struct CompletionCommand {
    /// The shell to generate completions for
    #[arg(long, value_parser(value_parser!(Shell)))]
    generator: Shell,
}

impl CompletionCommand {
    pub(crate) fn complete(&self, command: &mut Command) {
        eprintln!("Generating completion file for {}...", self.generator);
        print_completions(self.generator, command);
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
