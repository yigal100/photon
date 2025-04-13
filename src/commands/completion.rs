use std::io;

use clap::{value_parser, Args, Command};
use clap_complete::{Generator, Shell};

#[derive(Args)]
#[command(alias = "comp")]
#[command(about = "Generate shell completions for the given shell")]
pub(crate) struct CompletionCommand {
    /// The shell to generate completions for
    #[arg(short, long, value_parser(value_parser!(Shell)))]
    shell: Shell,
}

impl CompletionCommand {
    pub(crate) fn complete(&self, root_command: &mut Command) {
        eprintln!("Generating completion file for {}...", self.shell);
        {
            self.shell.generate(&root_command, &mut io::stdout());
        };
    }
}


