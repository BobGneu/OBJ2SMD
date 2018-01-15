#[macro_use]
extern crate serde_derive;
extern crate docopt;

mod cmd;
mod obj_file;
mod conversion;
mod mesh_parser;

use cmd::parse_command_options;

use mesh_parser::load_file;
use mesh_parser::process_file;

fn main() {
    let args = parse_command_options();

    let contents = load_file(args.arg_source);
    let obj = process_file(contents);

    if (!obj.is_valid()) {
        println!("The mesh you provided was invalid.");
    } else {
        println!("Mesh loaded. Beginning conversion.");
        println!("{:?}", obj);
    }
}