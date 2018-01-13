use std::fs::File;
use std::io::prelude::*;

use obj_file::*;
use conversion::*;

/// Load a file into string format
pub fn load_file(file_source: String) -> String {
    let mut contents = String::new();

    let mut f = File::open(file_source).expect("file not found");
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    return contents;
}

pub fn process_file(file_contents: String) -> ObjFile {
    let mut obj = ObjFile {faces: [].to_vec(), groups:[].to_vec(), normals: [].to_vec(), texture_coordinates: [].to_vec(), vertices: [].to_vec()};

    for line in file_contents.lines() {
        let mut tokens = line.trim().split_whitespace();
        let key = tokens.next();

        if !key.is_none() {
            match key.unwrap() {
                "#" => {
                    // comments shouldnt be counted.
                },
                "g" => {
                    let group_span = GroupSpan {
                        name: String::from(tokens.next().unwrap()),
                        start: 0,
                        end: 42
                    };

                    obj.groups.push(group_span);
                },
                "v" => {
                    let vector_point = token_to_float3(tokens);
                    obj.vertices.push(vector_point);
                },
                "vn" => {
                    let vector_point = token_to_float3(tokens);
                    obj.normals.push(vector_point);
                },
                "vt" => {
                    let vector_point = token_to_float3(tokens);
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
                                component.vertex = vertex_index.unwrap().parse::<u64>().unwrap();
                            }
                        }

                        if texture_index.is_some() {
                            tmp = texture_index.unwrap();

                            if !tmp.is_empty() {
                                component.texture = texture_index.unwrap().parse::<u64>().unwrap();
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

#[cfg(test)]
mod file_io {
    use super::*;

    #[test]
    fn should_be_able_to_load_the_contents_of_the_target_file() {
        let tmp = load_file("sample/cube.obj".to_owned());
        
        assert_eq!(tmp.len(), 553); 
    }
}