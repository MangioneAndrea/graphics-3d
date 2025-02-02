use core::f32;

use glam::Vec3;

use crate::utils::ray::Ray;

use super::{Hit, Mesh};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Mesh for Sphere {
    fn hit(&self, ray: &Ray, ray_t_min: f32, ray_t_max: f32) -> Option<Hit> {
        let oc = self.center - ray.origin;

        let a = ray.direction.length_squared() + f32::MIN_POSITIVE;
        let h = ray.direction.dot(oc);

        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut distance = (h - sqrt_d) / a;

        if distance <= ray_t_min || ray_t_max <= distance {
            distance = (h + sqrt_d) / a;

            if distance <= ray_t_min || ray_t_max <= distance {
                return None;
            }
        }

        let point = ray.at(distance);

        let normal: Vec3 = (point - self.center) / self.radius;

        let front_face = ray.direction.dot(normal) < 0.;

        // Extremely expensive...
        let normal = if front_face { normal } else { -normal };

        Some(Hit {
            distance,
            point,
            normal,
            front_face,
        })
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn hit_naive2(&self, ray: &Ray) -> Option<f32> {
        let oc = self.center - ray.origin;

        let a = ray.direction.length_squared() + f32::MIN_POSITIVE;
        let h = ray.direction.dot(oc);

        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        return Some((h - discriminant.sqrt()) / a);
    }

    #[cfg(test)]
    fn hit_naive(&self, ray: &Ray) -> Option<f32> {
        let oc = self.center - ray.origin;

        let a = ray.direction.dot(ray.direction) + f32::MIN_POSITIVE;

        let b = -2. * ray.direction.dot(oc);

        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return None;
        }

        return Some((-b - discriminant.sqrt()) / (2. * a));
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::Sphere;
    use crate::utils::meshes::Mesh;
    use crate::utils::ray::Ray;
    use test::Bencher;
    const N: i32 = std::hint::black_box(1000);

    #[cfg(test)]
    fn test_examples() -> (Vec<Sphere>, Vec<Ray>) {
        use super::Sphere;
        use crate::utils::ray::Ray;
        use glam::Vec3;
        let spheres = vec![
            Sphere::new(Vec3::new(0., 0., 0.), 12.),
            Sphere::new(Vec3::new(15., 15., 0.), 12.),
            Sphere::new(Vec3::new(0., 0., 10.), 1.),
            Sphere::new(Vec3::new(0., 0., -1.), 0.5),
            Sphere::new(Vec3::new(0., -100.5, -1.), 100.),
        ];

        let rays = vec![
            Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.)),
            Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.)),
            Ray::new(Vec3::new(100., 100., 0.), Vec3::new(0., 0., 0.)),
        ];

        (spheres, rays)
    }

    #[test]
    fn naive_hit_same_as_hit() {
        let (spheres, rays) = test_examples();

        for s in &spheres {
            for r in &rays {
                let a = s.hit(r, f32::NEG_INFINITY, f32::INFINITY).map(|el| el.t);
                let b = s.hit_naive2(r);
                let c = s.hit_naive(r);
                assert_eq!(a, b);
                assert_eq!(a, c);
            }
        }
    }

    #[bench]
    fn hit(b: &mut Bencher) {
        let (spheres, rays) = test_examples();
        b.iter(|| {
            let mut v = vec![];
            for _ in 0..N {
                for s in &spheres {
                    for r in &rays {
                        v.push(s.hit(r, f32::NEG_INFINITY, f32::INFINITY).map(|e| e.t));
                    }
                }
            }
            v
        })
    }

    #[bench]
    fn hit_naive2_speed(b: &mut Bencher) {
        let (spheres, rays) = test_examples();
        b.iter(|| {
            let mut v = vec![];
            for _ in 0..N {
                for s in &spheres {
                    for r in &rays {
                        v.push(s.hit_naive2(r));
                    }
                }
            }
            v
        })
    }
    #[bench]
    fn hit_naive_speed(b: &mut Bencher) {
        let (spheres, rays) = test_examples();
        b.iter(|| {
            let mut v = vec![];
            for _ in 0..N {
                for s in &spheres {
                    for r in &rays {
                        v.push(s.hit_naive(r));
                    }
                }
            }
            v
        })
    }
}
