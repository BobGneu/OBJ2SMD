#[derive(Debug, Deserialize, Clone)]
pub struct Float3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, Deserialize, Clone)]
pub struct FaceComponents {
    pub vertex: u64,
    pub texture: u64,
    pub normal: u64
}

#[derive(Debug, Deserialize, Clone)]
pub struct Face {
    pub components: Vec<FaceComponents>
}

#[derive(Debug, Deserialize, Clone)]
pub struct GroupSpan {
    pub name: String,
    pub start: u64,
    pub end: u64
}

#[derive(Debug, Deserialize)]
pub struct ObjFile {
    pub faces: Vec<Face>,
    pub vertices: Vec<Float3>,
    pub normals: Vec<Float3>,
    pub texture_coordinates: Vec<Float3>,
    pub groups: Vec<GroupSpan>
}