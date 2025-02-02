use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Camera {
    pub center: Vec3,
    pub delta_u: Vec3,
    pub delta_v: Vec3,
    pub upper_left: Vec3,
}

impl Camera {
    pub fn new(center: Vec3, width: u32, height: u32) -> Camera {
        let aspect_ratio = width as f32 / height as f32;

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * aspect_ratio;

        let u = Vec3::new(viewport_width, 0., 0.);
        let v = Vec3::new(0., -viewport_height, 0.);

        let delta_u = u / width as f32;
        let delta_v = v / height as f32;

        let upper_left = center - Vec3::new(0., 0., focal_length) - u / 2. - v / 2.;

        Self {
            center,
            delta_u,
            delta_v,
            upper_left,
        }
    }
}
