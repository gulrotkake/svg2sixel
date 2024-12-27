extern crate svg2sixel;

use clap::Parser;
use clap_stdin::FileOrStdin;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    filename: FileOrStdin,
}

fn main() {
    let args = Args::parse();

    let mut kitty = false;
    if let Ok(term) = std::env::var("TERM") {
        kitty = term.contains("kitty") || term.contains("ghostty");
    }

    if kitty {
        match svg2sixel::svg2sixel(&args.filename.contents().unwrap()) {
            Ok(data) => println!("{data}"),
            Err(err) => eprintln!("{err}"),
        }
    } else {
        match svg2sixel::svg2kitty(&args.filename.contents().unwrap()) {
            Ok(data) => println!("{data}"),
            Err(err) => eprintln!("{err}"),
        }
    }
}
