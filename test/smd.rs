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
