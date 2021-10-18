use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "clippings",
    about = "All the commands available on clippings manager."
)]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    // Input file
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,

    // Exported parsed file format
    #[structopt(short = "e", long)]
    export_type: String,
}

fn main() {
    println!("Hello, clippings!");

    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
