use anyhow::Result;
use clap::Parser;
use texttools_dedup::dedup_consecutive;

#[derive(Parser)]
#[command(name = "texttools-dedup", version, about = "Drop consecutive duplicate lines")]
struct Cli {
    /// Text with one line per entry (reads stdin when omitted)
    #[arg(long)]
    text: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input = match cli.text {
        Some(t) => t,
        None => {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    print!("{}", dedup_consecutive(&input));
    Ok(())
}
