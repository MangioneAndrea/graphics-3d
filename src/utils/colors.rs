use glam::Vec3;

pub type Color = u32;
pub type ColorVec = Vec3;

pub fn vec3àto_color(v: &ColorVec) -> Color {
    // vec3àto_color_uncorrected(v)
    vec3àto_color_uncorrected(&((v / 255.).powf(0.5) * 255.))
}

const fn vec3àto_color_uncorrected(v: &ColorVec) -> Color {
    v.z.clamp(0., 255.) as u32
        | ((v.y.clamp(0., 255.) as u32) << 8)
        | ((v.x.clamp(0., 255.) as u32) << 16)
}

pub fn vec3_to_scalar(v: &ColorVec) -> Vec3 {
    v / 255.
}

pub const BLACK: ColorVec = Vec3::new(0.01, 0.01, 0.01);
pub const WHITE: ColorVec = Vec3::new(255., 255., 255.);

pub const GREEN: ColorVec = Vec3::new(42., 157., 143.);
pub const BLUE: ColorVec = Vec3::new(38., 70., 83.);
pub const RED: ColorVec = Vec3::new(231., 111., 81.);
pub const YELLOW: ColorVec = Vec3::new(233., 196., 106.);

pub const SKY_BLUE: ColorVec = Vec3::new(127., 180., 255.);

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::vec3àto_color_uncorrected;

    #[test]
    fn sanity_check_conversion() {
        let num = 0xAA;
        let t = (num as f32).clamp(0., 255.) as u32;

        assert_eq!(num, t);
    }

    #[test]
    fn z_transformed_correctly() {
        let color = vec3àto_color_uncorrected(&Vec3::new(0 as f32, 0 as f32, 0x12 as f32));
        assert_eq!(color, 0x12)
    }

    #[test]
    fn zx_transformed_correctly() {
        let color = vec3àto_color_uncorrected(&Vec3::new(0xFF as f32, 0 as f32, 0x12 as f32));
        assert_eq!(color, 0xFF0012)
    }

    #[test]
    fn color_transformed_correctly() {
        let color = vec3àto_color_uncorrected(&Vec3::new(0xFF as f32, 0xAA as f32, 0x12 as f32));
        assert_eq!(color, 0xFFAA12)
    }
}
