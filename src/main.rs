#[macro_use]
extern crate serde_derive;
extern crate docopt;

mod cmd;
mod mesh_parser;

use cmd::parse_command_options;
use mesh_parser::load_file;
use mesh_parser::process_file;

fn main() {
    let args = parse_command_options();

    let contents = load_file(args.arg_source);
    let obj = process_file(contents);

    println!("{:?}", obj);
}