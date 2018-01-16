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
    let mut group_span = GroupSpan {start: 0, end: 0, name: "".to_string()};

    for line in file_contents.lines() {
        let mut tokens = line.trim().split_whitespace();
        let key = tokens.next();

        if !key.is_none() {
            match key.unwrap() {
                "#" => {
                    // comments shouldnt be counted.
                },
                "g" => {
                    if group_span.name != "" {
                        group_span.end = obj.faces.len();
                        obj.groups.push(group_span);
                    }

                    group_span = GroupSpan {
                        name: String::from(tokens.next().unwrap()),
                        start: obj.faces.len(),
                        end: obj.faces.len()
                    };

                    println!("start> {}", obj.faces.len());
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
                                component.vertex = vertex_index.unwrap().parse::<usize>().unwrap();
                            }
                        }

                        if texture_index.is_some() {
                            tmp = texture_index.unwrap();

                            if !tmp.is_empty() {
                                component.texture = texture_index.unwrap().parse::<usize>().unwrap();
                            }
                        }

                        if normal_index.is_some() {
                            tmp = normal_index.unwrap();

                            if !tmp.is_empty() {
                                component.normal = normal_index.unwrap().parse::<usize>().unwrap();
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

    println!("end>   {}", obj.faces.len());

    if group_span.name != "" {
        group_span.end = obj.faces.len();
        obj.groups.push(group_span);
    }

    return obj;
}

#[cfg(test)]
mod obj_parser_validation_cube_1 {
    use super::*;

    const FILE_LOCATION: &str = "sample/cube.obj";

    #[test]
    fn should_have_8_vertices() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(8, tmp.vertices.len());
    }

    #[test]
    fn should_have_0_texture_coordinates() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(0, tmp.texture_coordinates.len());
    }

    #[test]
    fn should_have_1_groups() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        println!("{:?}", tmp.groups);

        assert_eq!(1, tmp.groups.len());
    }

    #[test]
    fn should_have_1_group_validating_span() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        let span = &tmp.groups[0];

        assert_eq!(0, span.start);
        assert_eq!(12, span.end);
    }

    #[test]
    fn should_have_6_normals() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(6, tmp.normals.len());
    }

    #[test]
    fn should_have_12_faces() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(12, tmp.faces.len());
    }

    #[test] 
    fn is_valid_mesh() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert!(tmp.is_valid());
    }
}

#[cfg(test)]
mod obj_parser_validation_cube_2 {
    use super::*;

    const FILE_LOCATION: &str = "sample/cube.2.obj";

    #[test]
    fn should_have_8_vertices() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(24, tmp.vertices.len());
    }

    #[test]
    fn should_have_24_texture_coordinates() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(24, tmp.texture_coordinates.len());
    }

    #[test]
    fn should_have_2_groups() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(2, tmp.groups.len());
    }

    #[test]
    fn should_have_2_group_validating_span_1() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        let span = &tmp.groups[0];

        assert_eq!(0, span.start);
        assert_eq!(0, span.end);
    }

    #[test]
    fn should_have_2_group_validating_span_2() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        let span = &tmp.groups[1];

        assert_eq!(0, span.start);
        assert_eq!(12, span.end);
    }

    #[test]
    fn should_have_6_normals() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(24, tmp.normals.len());
    }

    #[test]
    fn should_have_12_faces() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(12, tmp.faces.len());
    }

    #[test] 
    fn is_valid_mesh() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert!(tmp.is_valid());
    }
}

#[cfg(test)]
mod obj_parser_validation_cube_3 {
    use super::*;

    const FILE_LOCATION: &str = "sample/blender.cube.obj";

    #[test]
    fn should_have_8_vertices() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(8, tmp.vertices.len());
    }

    #[test]
    fn should_have_0_texture_coordinates() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(0, tmp.texture_coordinates.len());
    }

    #[test]
    fn should_have_0_groups() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(0, tmp.groups.len());
    }

    #[test]
    fn should_have_6_normals() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(6, tmp.normals.len());
    }

    #[test]
    fn should_have_6_faces() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(6, tmp.faces.len());
    }

    #[test] 
    fn is_not_valid_mesh() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert!(!tmp.is_valid());
    }
}

#[cfg(test)]
mod obj_parser_validation_cube_4 {
    use super::*;

    const FILE_LOCATION: &str = "sample/blender.cube.triangulated.obj";

    #[test]
    fn should_have_8_vertices() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(8, tmp.vertices.len());
    }

    #[test]
    fn should_have_0_texture_coordinates() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(0, tmp.texture_coordinates.len());
    }

    #[test]
    fn should_have_2_groups() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(2, tmp.groups.len());
    }

    #[test]
    fn should_have_2_group_validating_span_1() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        let span = &tmp.groups[0];

        assert_eq!(0, span.start);
        assert_eq!(0, span.end);
    }

    #[test]
    fn should_have_2_group_validating_span_2() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        let span = &tmp.groups[1];

        assert_eq!(0, span.start);
        assert_eq!(12, span.end);
    }

    #[test]
    fn should_have_6_normals() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(6, tmp.normals.len());
    }

    #[test]
    fn should_have_12_faces() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(12, tmp.faces.len());
    }

    #[test] 
    fn is_valid_mesh() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert!(tmp.is_valid());
    }
}

#[cfg(test)]
mod obj_parser_validation_sphere_1 {
    use super::*;

    const FILE_LOCATION: &str = "sample/blender.sphere.obj";

    #[test]
    fn should_have_482_vertices() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(482, tmp.vertices.len());
    }

    #[test]
    fn should_have_0_texture_coordinates() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(0, tmp.texture_coordinates.len());
    }

    #[test]
    fn should_have_1_groups() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(1, tmp.groups.len());
    }

    #[test]
    fn should_have_2_group_validating_span_1() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        let span = &tmp.groups[0];

        assert_eq!(0, span.start);
        assert_eq!(512, span.end);
    }

    #[test]
    fn should_have_512_normals() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(512, tmp.normals.len());
    }

    #[test]
    fn should_have_512_faces() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(512, tmp.faces.len());
    }

    #[test] 
    fn is_not_valid_mesh() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert!(!tmp.is_valid());
    }
}

#[cfg(test)]
mod obj_parser_validation_sphere_2 {
    use super::*;

    const FILE_LOCATION: &str = "sample/blender.sphere.triangulated.obj";

    #[test]
    fn should_have_482_vertices() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(482, tmp.vertices.len());
    }

    #[test]
    fn should_have_0_texture_coordinates() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(0, tmp.texture_coordinates.len());
    }

    #[test]
    fn should_have_1_groups() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(1, tmp.groups.len());
    }

    #[test]
    fn should_have_2_group_validating_span_1() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        let span = &tmp.groups[0];

        assert_eq!(0, span.start);
        assert_eq!(960, span.end);
    }

    #[test]
    fn should_have_528_normals() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(528, tmp.normals.len());
    }

    #[test]
    fn should_have_528_faces() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert_eq!(960, tmp.faces.len());
    }

    #[test] 
    fn is_valid_mesh() {
        let contents = load_file(FILE_LOCATION.to_owned());
        let tmp = process_file(contents);

        assert!(tmp.is_valid());
    }
}
