use super::super::math::ray::Ray3;
use super::super::math::vector::Vec3;
use super::hit::Info as HitInfo;

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

pub trait Material {
    /// returns: (attenuation, scattered)
    fn scatter(&self, r_in: &Ray3, rec: &HitInfo) -> Option<(Vec3, Ray3)>;
}


pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray3, rec: &HitInfo) -> Option<(Vec3, Ray3)> {
        let target = &rec.p + &(&rec.n + &Vec3::random_in_unit_sphere());
        Some((self.albedo, Ray3::new(rec.p, &target - &rec.p)))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray3, rec: &HitInfo) -> Option<(Vec3, Ray3)> {
        let reflected = r_in.d.normalized().reflect(&rec.n);
        let scattered = Ray3::new(rec.p, &reflected + &(&Vec3::random_in_unit_sphere() * self.fuzz));
        let attenuation = self.albedo;
        if scattered.d.dot(&rec.n) > 0.0 {
            return Some((attenuation, scattered));
        }
        None
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray3, rec: &HitInfo) -> Option<(Vec3, Ray3)> {
        let mut outward_normal = Vec3::new();
        let reflected = r_in.d.reflect(&rec.n);
        let mut ni_over_nt = 0.0;
        let attenuation = Vec3 { x: 1.0, y: 1.0, z: 1.0 }; 
        let mut refracted = Vec3::new();
        let mut reflect_prob = 0.0;
        let mut cosine = 0.0;
        if r_in.d.dot(&rec.n) > 0.0 {
            outward_normal = -&rec.n;
            ni_over_nt = self.ref_idx;
            cosine = r_in.d.dot(&rec.n) / r_in.d.length();
            cosine = (1.0 - self.ref_idx * self.ref_idx * ( 1.0 - cosine * cosine)).sqrt();
        } else {
            outward_normal = rec.n;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.d.dot(&rec.n) / r_in.d.length();
        }
        let reflected = r_in.d.refract(&outward_normal, ni_over_nt);
        if reflected.is_some() {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else  {
            reflect_prob = 1.0;
        }
        //////// todo
        if (drand48() < reflect_prob) 
            scattered = ray(rec.p, reflected);
        else 
            scattered = ray(rec.p, refracted);
        return true;
    }
}
