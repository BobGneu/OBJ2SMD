#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
OBJ Converter

Usage:
  obj2smd.exe [--report] <source>...
  obj2smd.exe (-z | --zup) [--report] <source>...
  obj2smd.exe (-h | --help)
  obj2smd.exe --version

Options:
  -h --help    Show this screen.
  --version    Show version.
  -z --zup     Source is zUp 
  --report     Show the statistics report of the conversion
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_zup: bool,
    flag_report: bool,
    arg_source: Vec<String>
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
}
