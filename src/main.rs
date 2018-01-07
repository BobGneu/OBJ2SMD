#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
OBJ Converter

Usage:
  obj2smd.exe [-z | --zup] [--report] <source> [--image=<path>]...
  obj2smd.exe (-h | --help)
  obj2smd.exe --version

Options:
  -h --help         Show this screen
  --version         Show version
  --image=<path>    Add a tga to the listing
  -z --zup          Source mesh is zUp 
  --report          Show the statistics report of the conversion
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_zup: bool,
    flag_report: bool,
    flag_image: Vec<String>,
    arg_source: String
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
}
