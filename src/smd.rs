// https://developer.valvesoftware.com/wiki/Studiomdl_Data
use obj_file::ObjFile;

use obj_file::Face;

#[derive(Debug, Clone)]
pub struct SMD {
    pub groups: Vec<String>,
    pub entries: Vec<SMDEntry>
}

#[derive(Debug, Clone)]
pub struct SMDEntry {
    pub image_name: String,
    pub group: usize,
    pub points: Vec<Float8>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Float8 {
    pub u: f64,
    pub v: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_norm: f64,
    pub y_norm: f64,
    pub z_norm: f64
}

pub fn convert_obj(obj: &ObjFile) -> SMD {
    let mut instance = SMD {entries: [].to_vec(), groups: [].to_vec()};

    for face in obj.faces.iter() {
        let mut smdEntry = SMDEntry {
            image_name: "test.tga".to_owned(),
            group: 0,
            points: [].to_vec()
        };

        for component in face.components.iter() {
            let vertexComponent = &obj.vertices[component.vertex - 1];
            let normalComponent = &obj.texture_coordinates[component.texture - 1];
            let textureComponent = &obj.normals[component.normal - 1];

            smdEntry.points.push(Float8 {
                x: vertexComponent.x, y: vertexComponent.y, z: vertexComponent.z,
                x_norm: normalComponent.x, y_norm: normalComponent.y, z_norm: normalComponent.z,
                u: textureComponent.x, v: textureComponent.y
            });
        };

        instance.entries.push(smdEntry);
    };

    for group in obj.groups.iter() {
        instance.groups.push(group.name.to_string());

        for ndx in group.start..group.end {
            let mut smdEntry = &mut instance.entries[ndx];

            smdEntry.group = instance.groups.len() - 1;
        }
    };

    return instance;
}

impl SMD {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        
        result.push_str("version 1\nnodes");

        let mut ndx = 0;
        for group in self.groups.iter() {
            result.push_str(&format!("\n\t{}\t\"{}\"\t-1", ndx, group));
            ndx += 1;
        };

        result.push_str(&"\nend\nskeleton\ntime 0\n\t0 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000\nend\ntriangles\n");

        for entry in self.entries.iter() {
            result.push_str(&entry.to_string());
        };
        
        result.push_str(&"end\n");

        return result.to_owned();
    }

    pub fn save(&self, path: &str) {
        println!("this: {:?}", self);
    }

    pub fn is_valid(&self) -> bool {
        if self.entries.len() == 0 {
            return false
        }

        for element in self.entries.iter() {
            if !element.is_valid() {
                return false;
            }
        }

        return true;
    }
}

impl SMDEntry {
    pub fn is_valid(&self) -> bool { 
        return self.image_name != "" && self.points.len() == 3;
    }

    pub fn to_string(&self) -> String {
        return format!("{0}\n\t{1}\t{2}\n\t{1}\t{3}\n\t{1}\t{4}\n", self.image_name, self.group, self.points[0].to_string(), self.points[1].to_string(), self.points[1].to_string());
    }
}

impl Float8 {
    pub fn to_string(&self) -> String {
        // <float|PosX PosY PosZ> <normal|NormX NormY NormZ> <normal|U V> 
        return format!("{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}", self.x, self.y, self.z, self.x_norm, self.y_norm, self.z_norm, self.u, self.v);
    }
}

#[cfg(test)]
mod SMD_test_cases {
    use super::*;

    #[test]
    fn is_invalid_when_has_0_entries() {
        let smd = SMD {groups: [].to_vec(), entries: [].to_vec()};

        assert!(!smd.is_valid());
    }

    #[test]
    fn is_invalid_when_has_1_empty_smdentry() {
        let mut smd = SMD {groups: [].to_vec(), entries: [].to_vec()};

        smd.entries.push(SMDEntry {group: 0, image_name: "".to_owned(), points: [].to_vec()});

        assert!(!smd.is_valid());
    }

    #[test]
    fn is_valid_when_has_1_valid_entry() {
        let mut smd = SMD {groups: [].to_vec(), entries: [].to_vec()};
        let mut smdEntry = SMDEntry {group: 0, image_name: "test_image.tga".to_owned(), points: [].to_vec()};

        smdEntry.points.push(Float8 {
            u: 0.00,
            v: 0.00,
            x: 0.00,
            y: 0.00,
            z: 0.00,
            x_norm: 0.00,
            y_norm: 0.00,
            z_norm: 0.00
        });

        smdEntry.points.push(Float8 {
            u: 0.00,
            v: 0.00,
            x: 0.00,
            y: 0.00,
            z: 0.00,
            x_norm: 0.00,
            y_norm: 0.00,
            z_norm: 0.00
        });

        smdEntry.points.push(Float8 {
            u: 0.00,
            v: 0.00,
            x: 0.00,
            y: 0.00,
            z: 0.00,
            x_norm: 0.00,
            y_norm: 0.00,
            z_norm: 0.00
        });

        smd.entries.push(smdEntry);

        assert!(smd.is_valid());
    }
}

#[cfg(test)]
mod SMDEntry_test_cases {
    use super::*;

    #[test]
    fn should_result_in_relevant_string() {
        let mut smdEntry = SMDEntry {group: 0, image_name: "test_image.tga".to_owned(), points: [].to_vec()};

        smdEntry.points.push(Float8 {
            u: 0.00,
            v: 0.00,
            x: 0.00,
            y: 0.00,
            z: 0.00,
            x_norm: 0.00,
            y_norm: 0.00,
            z_norm: 0.00
        });

        smdEntry.points.push(Float8 {
            u: 0.00,
            v: 0.00,
            x: 0.00,
            y: 0.00,
            z: 0.00,
            x_norm: 0.00,
            y_norm: 0.00,
            z_norm: 0.00
        });

        smdEntry.points.push(Float8 {
            u: 0.00,
            v: 0.00,
            x: 0.00,
            y: 0.00,
            z: 0.00,
            x_norm: 0.00,
            y_norm: 0.00,
            z_norm: 0.00
        });

        assert_eq!("test_image.tga\n\t0\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\n\t0\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\n\t0\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\t0.0000\n", smdEntry.to_string());
    }
}