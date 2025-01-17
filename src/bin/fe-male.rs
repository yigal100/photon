use std::fs;
use std::io;

fn main() -> io::Result<()> {
    const VAGINA: &str = include_str!("../../VAGINA.md");
    println!("{VAGINA}");
    Ok(())
}
