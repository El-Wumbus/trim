use clap::Parser;
use std::fs::{self, File};
use std::io::{Read, Write};
const NAME: &'static str = env!("CARGO_PKG_NAME");

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Include a newline at the end of the output
    #[arg(short, long)]
    newline: bool,

    /// Write shell completions and exit
    #[arg(long)]
    completions: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.completions {
        use clap::CommandFactory;
        use clap_complete::Shell;
        let mut cmd = Args::command();
        let shell =
            Shell::from_env().expect("Couldn't determine shell from environment!");

        match shell {
            Shell::Fish => {
                let vendor_completions_dir = dirs::data_dir()
                    .expect("data dir")
                    .join("fish/vendor_completions.d");
                let vendor_completions_path =
                    vendor_completions_dir.join(format!("{NAME}.fish"));
                if !vendor_completions_dir.exists() {
                    fs::create_dir_all(&vendor_completions_dir)?;
                }
                let mut f = File::create(&vendor_completions_path)?;
                eprintln!("Writing completions to {vendor_completions_path:?}");
                clap_complete::generate(shell, &mut cmd, NAME, &mut f);
            }
            _ => {
                clap_complete::generate(shell, &mut cmd, NAME, &mut std::io::stdout());
            }
        };
    } else {
        let mut stdin = vec![];
        std::io::stdin().read_to_end(&mut stdin)?;
        let mut stdout = std::io::stdout();
        stdout.write_all(stdin.trim_ascii())?;
        if args.newline {
            stdout.write_all(b"\n")?;
        }
    }

    Ok(())
}
