use docopt::Docopt;

const USAGE: &'static str = "
OBJ Converter

Usage:
  obj2smd.exe [-z | --zup] [--report] [--directory] <source> [--image=<path>]...
  obj2smd.exe (-h | --help)
  obj2smd.exe --version

Options:
  -h --help         Show this screen
  --version         Show version
  --image=<path>    Add a tga to the listing [default: path/to/image.tga]
  --directory       Treat the source as a directory
  -z --zup          Source mesh is zUp 
  --report          Show the statistics report of the conversion
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_zup: bool,
    pub flag_report: bool,
    pub flag_image: Vec<String>,
    
    pub arg_source: String 
}

pub fn parse_command_options() -> Args {
    return Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
}