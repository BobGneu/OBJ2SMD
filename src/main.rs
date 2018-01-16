#[macro_use]
extern crate serde_derive;
extern crate docopt;

mod cmd;
mod obj_file;
mod conversion;
mod mesh_parser;
mod smd;

use cmd::parse_command_options;

use mesh_parser::load_file;
use mesh_parser::process_file;
use smd::convert_obj;

fn main() {
    let args = parse_command_options();

    let contents = load_file(args.arg_source);
    let obj = process_file(contents);

    if !obj.is_valid() {
        println!("The mesh you provided was invalid.");
        return;
    } 

    println!("Mesh loaded. Beginning conversion.");

    let smdFile = convert_obj(&obj);
    println!("{:?}", smdFile);
}
