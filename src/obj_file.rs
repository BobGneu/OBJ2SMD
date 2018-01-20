#[derive(Debug, Deserialize, Clone)]
pub struct Float3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, Deserialize, Clone)]
pub struct FaceComponents {
    pub vertex: usize,
    pub texture: usize,
    pub normal: usize
}

#[derive(Debug, Deserialize, Clone)]
pub struct Face {
    pub components: Vec<FaceComponents>
}

#[derive(Debug, Deserialize, Clone)]
pub struct GroupSpan {
    pub name: String,
    pub start: usize,
    pub end: usize
}

#[derive(Debug, Deserialize)]
pub struct ObjFile {
    pub faces: Vec<Face>,
    pub vertices: Vec<Float3>,
    pub normals: Vec<Float3>,
    pub texture_coordinates: Vec<Float3>,
    pub groups: Vec<GroupSpan>
}

impl Face {
    pub fn is_valid(&self, vertices: &Vec<Float3>, texture_coordinates: &Vec<Float3>, normals: &Vec<Float3>) -> bool {
        return self.is_triangulated() && 
            self.vertices_exist(&vertices) && 
            self.normals_exist(&normals) && 
            self.texture_coordinates_exist(&texture_coordinates);
    }

    fn is_triangulated(&self) -> bool {
        return self.components.len() == 3;
    }

    fn vertices_exist(&self, vertices: &Vec<Float3>) -> bool {
        return self.components[0].vertex <= vertices.len();
    }

    fn normals_exist(&self, normals: &Vec<Float3>) -> bool {
        return self.components[0].normal <= normals.len();
    }

    fn texture_coordinates_exist(&self, texture_coordinates: &Vec<Float3>) -> bool {
        return self.components[0].texture <= texture_coordinates.len();
    }
}

impl ObjFile {
    pub fn is_valid(&self) -> bool {
        return self.vertices_are_valid() && 
            self.normals_are_valid() && 
            self.texture_coordinates_are_valid() && 
            self.has_groups() && 
            self.faces_are_valid();
    }

    fn has_groups(&self) -> bool {
        return self.groups.len() > 0;
    }

    fn vertices_are_valid(&self) -> bool {
        return self.vertices.len() > 0;
    }

    fn normals_are_valid(&self) -> bool {
        return self.normals.len() > 0; 
    }

    fn texture_coordinates_are_valid(&self) -> bool {
        return self.texture_coordinates.len() >= 0;
    }

    fn faces_are_valid(&self) -> bool {
        for face in self.faces.iter() {
            if !face.is_valid(&self.vertices, &self.texture_coordinates, &self.normals) {
                // TODO: Better error message here.
                return false;
            }
        }

        return true;
    }
}
