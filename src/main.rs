use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;

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
    #[structopt(short = "e", long, default_value = "markdown")]
    export_type: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?} \n\n\n", opt);

    let display = opt.file.display();

    let mut file = match File::open(&opt.file) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} read with success", display),
    };

    file_details(opt.file).expect("error");

    println!("file content: {:?}", file_content);
}

fn file_details(file: PathBuf) -> std::io::Result<()> {
    use std::fs;

    let metadata = fs::metadata(file)?;

    println!("{:?}", metadata.len());
    Ok(())
}
