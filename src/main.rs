#[macro_use]
extern crate serde_derive;
extern crate docopt;

use std::fs::File;
use std::io::prelude::*;

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
struct Args {
    flag_zup: bool,
    flag_report: bool,
    flag_image: Vec<String>,
    
    arg_source: String 
}

#[derive(Debug, Deserialize, Clone)]
struct Float3 {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug, Deserialize, Clone)]
struct FaceComponents {
    vertex: u64,
    texture: u64,
    normal: u64
}

#[derive(Debug, Deserialize, Clone)]
struct Face {
    components: Vec<FaceComponents>
}

#[derive(Debug, Deserialize, Clone)]
struct GroupSpan {
    name: String,
    start: u64,
    end: u64
}

#[derive(Debug, Deserialize)]
struct ObjFile {
    faces: Vec<Face>,
    vertices: Vec<Float3>,
    normals: Vec<Float3>,
    texture_coordinates: Vec<Float3>,
    groups: Vec<GroupSpan>
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let contents = load_file(args.arg_source);
    let obj = process_file(contents);

    println!("{:?}", obj);
}

fn load_file(file_source: String) -> String {
    let mut contents = String::new();

    println!("source: {}", file_source);

    let mut f = File::open(file_source).expect("file not found");
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    return contents;
}

fn process_file(file_contents: String) -> ObjFile {
    let mut obj = ObjFile {faces: [].to_vec(), groups:[].to_vec(), normals: [].to_vec(), texture_coordinates: [].to_vec(), vertices: [].to_vec()};

    for line in file_contents.lines() {
        let mut tokens = line.trim().split_whitespace();
        let key = tokens.next();

        if !key.is_none() {
            match key.unwrap() {
                "g" => {
                    let group_span = GroupSpan {
                        name: String::from(tokens.next().unwrap()),
                        start: 0,
                        end: 42
                    };

                    obj.groups.push(group_span);
                },
                "v" => {
                    let vector_point = Float3 {x:tokens.next().unwrap().parse::<f64>().unwrap(), y: tokens.next().unwrap().parse::<f64>().unwrap(), z: tokens.next().unwrap().parse::<f64>().unwrap()};
                    obj.vertices.push(vector_point);
                },
                "vn" => {
                    let vector_point = Float3 {x:tokens.next().unwrap().parse::<f64>().unwrap(), y: tokens.next().unwrap().parse::<f64>().unwrap(), z: tokens.next().unwrap().parse::<f64>().unwrap()};
                    obj.normals.push(vector_point);
                },
                "vt" => {
                    let vector_point = Float3 {x:tokens.next().unwrap().parse::<f64>().unwrap(), y: tokens.next().unwrap().parse::<f64>().unwrap(), z: tokens.next().unwrap().parse::<f64>().unwrap()};
                    obj.texture_coordinates.push(vector_point);
                },
                "f" => {
                    let mut face = Face {components: [].to_vec()};
                    let mut tmp: &str;
                    
                    loop {
                        let components = tokens.next();

                        if components == None {
                            break;
                        }

                        let mut component_tokens = components.unwrap().split('/');
                        let mut component = FaceComponents {
                            vertex: 0,
                            texture: 0,
                            normal: 0
                        };

                        let vertex_index = component_tokens.next();
                        let texture_index = component_tokens.next();
                        let normal_index = component_tokens.next();

                        if vertex_index.is_some() {
                            tmp = vertex_index.unwrap();

                            if !tmp.is_empty() {
                                component.vertex = normal_index.unwrap().parse::<u64>().unwrap();
                            }
                        }

                        if texture_index.is_some() {
                            tmp = texture_index.unwrap();

                            if !tmp.is_empty() {
                                component.texture = normal_index.unwrap().parse::<u64>().unwrap();
                            }
                        }

                        if normal_index.is_some() {
                            tmp = normal_index.unwrap();

                            if !tmp.is_empty() {
                                component.normal = normal_index.unwrap().parse::<u64>().unwrap();
                            }
                        }

                        face.components.push(component);
                    }

                    obj.faces.push(face);
                },
                _ => {
                    println!("??? {}", line);
                }
            }
        }
    }

    return obj;
}