// https://developer.valvesoftware.com/wiki/Studiomdl_Data
use obj_file::ObjFile;

#[derive(Debug, Clone)]
pub struct SMD {
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
    let instance = SMD {entries: [].to_vec()};

    return instance;
}

impl SMD {
    pub fn to_string(&self) -> String {
        return "".to_string();
    }

    pub fn save(&self, path: &str) {

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
        let smd = SMD {entries: [].to_vec()};

        assert!(!smd.is_valid());
    }

    #[test]
    fn is_invalid_when_has_1_empty_smdentry() {
        let mut smd = SMD {entries: [].to_vec()};

        smd.entries.push(SMDEntry {group: 0, image_name: "".to_owned(), points: [].to_vec()});

        assert!(!smd.is_valid());
    }

    #[test]
    fn is_valid_when_has_1_valid_entry() {
        let mut smd = SMD {entries: [].to_vec()};
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