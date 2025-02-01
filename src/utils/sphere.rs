use core::f32;

use glam::Vec3;

use super::ray::Ray;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn hit(&self, ray: &Ray, ray_t_min: f32, ray_t_max: f32) -> Option<Hit> {
        let oc = self.center - ray.origin;

        let a = ray.direction.length_squared() + 0.000000000000000000000001;
        let h = ray.direction.dot(oc);

        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (h - sqrt_d) / a;

        if root <= ray_t_min || ray_t_max <= root {
            root = (h + sqrt_d) / a;

            if root <= ray_t_min || ray_t_max <= root {
                return None;
            }
        }

        let p = ray.at(root);

        let normal: Vec3 = (p - self.center) / self.radius;

        let front_face = ray.direction.dot(normal) < 0.;
        //
        //let normal = if front_face { normal } else { -normal };

        Some(Hit {
            t: root,
            p,
            normal,
            front_face,
        })
    }

    pub fn hit_naive2(&self, ray: &Ray) -> Option<f32> {
        let oc = self.center - ray.origin;

        let a = ray.direction.length_squared() + 0.000000000000000000000001;
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

        let a = ray.direction.dot(ray.direction) + 0.000000000000000000000001;

        let b = -2. * ray.direction.dot(oc);

        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return None;
        }

        return Some((-b - discriminant.sqrt()) / (2. * a));
    }
}

mod test {
    extern crate test;

    #[cfg(test)]
    use super::Sphere;
    #[cfg(test)]
    use crate::utils::ray::Ray;
    #[cfg(test)]
    use test::Bencher;
    #[cfg(test)]
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
