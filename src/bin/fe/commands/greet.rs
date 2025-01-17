use clap::Args;

#[derive(Args)]
#[command(hide = true)]
#[command(visible_alias = "say")]
#[command(about = "Greet a person")]
#[command(long_about = "Greet a person by their name a specified number of times")]
pub(crate) struct GreetingCommand {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

impl GreetingCommand {
    pub(crate) fn greet(&self) {
        for _ in 0..self.count {
            iron::greet_user(&self.name);
        }
    }
}
