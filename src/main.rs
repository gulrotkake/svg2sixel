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

    match svg2sixel::svg2sixel(&args.filename.contents().unwrap()) {
        Ok(data) => println!("{data}"),
        Err(err) => eprintln!("{err}"),
    }
}
