use clap::Args;
use compiler::ast;

const ABOUT: &str = "Builds a packages and dependencies";

const LONG_ABOUT: &str = r#"
compile packages and dependencies 😇

The build command compiles the specified package and its dependencies. If the
package is a library, the command will compile the library and its dependencies
into a shared object file. If the package is an executable, the command will
compile the executable and its dependencies into an executable file.
"#;

const NOTE: &str = "Run `fe help build` for more detailed information";

const LONG_NOTE: &str = r#"
See the `cargo build` command for more information on compiling packages and
dependencies.
"#;

#[derive(Args)]

#[command(visible_alias = "c")]
#[command(about = ABOUT, long_about = LONG_ABOUT)]
#[command(after_help = NOTE, after_long_help = LONG_NOTE)]
#[command(flatten_help = true)]
pub(crate) struct CompileCommand {
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(short, long)]
    name: String,
}

impl CompileCommand {
    pub(crate) fn build(&self) {
        ast::greet_user(&self.name);
    }
}
