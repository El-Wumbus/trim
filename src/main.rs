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

    /// Additional characters to trim from the input.
    #[arg(short, long)]
    additional_chars: Vec<char>,

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
        let mut stdin = stdin.trim_ascii();
        if !args.additional_chars.is_empty() {
            let additional = args
                .additional_chars
                .into_iter()
                .filter_map(|x| u8::try_from(x).ok())
                .collect::<Vec<_>>();
            stdin = trim_end(trim_start(stdin, &additional), &additional);
        }
        let mut stdout = std::io::stdout();
        stdout.write_all(stdin.trim_ascii())?;
        if args.newline {
            stdout.write_all(b"\n")?;
        }
    }

    Ok(())
}

pub fn trim_start<'a>(mut bytes: &'a [u8], id: &[u8]) -> &'a [u8] {
    // Note: A pattern matching based approach (instead of indexing) allows
    // making the function const.

    while let [first, rest @ ..] = bytes {
        if id.contains(first) {
            bytes = rest;
        } else {
            break;
        }
    }
    bytes
}

pub fn trim_end<'a>(mut bytes: &'a [u8], id: &[u8]) -> &'a [u8] {
    // Note: A pattern matching based approach (instead of indexing) allows
    // making the function const.
    while let [rest @ .., last] = bytes {
        if id.contains(last) {
            bytes = rest;
        } else {
            break;
        }
    }
    bytes
}
