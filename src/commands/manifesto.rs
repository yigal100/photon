use clap::Args;

#[derive(Args)]
#[command(hide = true)]
#[command(visible_alias = "say")]
#[command(about = "Greet a person")]
#[command(long_about = "Greet a person by their name a specified number of times")]
pub(crate) struct ManifestoCommand {
    // /// Name of the person to greet
    // #[arg(short, long)]
    // name: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

impl ManifestoCommand {
    pub(crate) fn preach(&self) {
        // for _ in 0..self.count {
        const VAGINA: &str = include_str!("../../VAGINA.md");
        println!("{}", VAGINA)
        // }
    }
}
