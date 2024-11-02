use nalgebra_glm::{dot, Vec3};
use crate::materials::Material;
use crate::materials::Intersect;

pub enum Object {
    Sphere(Sphere),
    SquarePlane(SquarePlane),
}

impl RayIntersect for Object {
    fn ray_intersect(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Intersect {
        match self {
            Object::Sphere(sphere) => sphere.ray_intersect(ray_origin, ray_direction),
            Object::SquarePlane(plane) => plane.ray_intersect(ray_origin, ray_direction),
        }
    }
}

pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub material: Material
}
impl Sphere{
    pub fn new(
         center: Vec3,
         radius: f32,
         material: Material
    ) -> Self{
        Sphere {
            center,
            radius,
            material
        }
    }
}
impl RayIntersect for Sphere{
    fn ray_intersect(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Intersect {
        let oc = ray_origin - self.center;
        let a = dot(ray_direction, ray_direction);
        let b = 2.0* dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius*self.radius;

        let discriminant = b*b -4.0*a*c;
        if discriminant>0.0{
            let t = (-b -discriminant.sqrt())/(2.0*a);
            if t>0.0 {
                let point = ray_origin + (ray_direction * t);
                let normal = (point- self.center).normalize();
                return Intersect::new(point, normal,t, true,self.material, 0.0, 0.0)
            }

        }
        return Intersect::empty()
    }
}

pub struct SquarePlane {
    pub center: Vec3,
    pub normal: Vec3,
    pub size: f32,        
    pub material: Material,
    pub is_slab: bool,
}
impl SquarePlane {
    pub fn new(p_center: (f32, f32, f32), p_normal: (f32, f32, f32), size: f32, p_material: &str, is_slab: bool) -> Self {
        SquarePlane {
            center: Vec3::new(p_center.0,p_center.1,p_center.2),
            normal:  Vec3::new(p_normal.0,p_normal.1,p_normal.2).normalize(),
            size,
            material: Material::new(p_material),
            is_slab,
        }
    }
    pub fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        let tangent = if self.normal.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let bitangent = self.normal.cross(&tangent).normalize();
        let local_point = point - self.center;
        let mut u = local_point.dot(&tangent) / self.size + 0.5;
        let mut v = local_point.dot(&bitangent) / self.size + 0.5;

        if self.normal.z > 0.0 {
            v = 1.0 - v;
        }
        if self.normal.x > 0.0 {
            let tem = u;
            u = v;
            v = 1.0 -tem;
        } else if self.normal.x != 0.0{
            let tem = u;
            u = v;
            v = 1.0 - tem;
        }
        (u, v)
    }
}
impl RayIntersect for SquarePlane {
    fn ray_intersect(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Intersect {
        let denom = dot(&self.normal, ray_direction);
        
        if denom.abs() < 1e-6 {
            return Intersect::empty();
        }

        let t = dot(&(self.center - ray_origin), &self.normal) / denom;
        
        if t > 0.001 {
            let point:Vec3 = ray_origin + (ray_direction*t);
            let (u,v) = self.get_uv(&point);
            let local_point:Vec3 = point - self.center;
            let half_size = self.size / 2.0;

            static mut COUNTER: usize = 0;
            const LIMIT: usize = 10;

            if self.is_slab && local_point.y > 0.0 {
                unsafe {
                    if COUNTER < LIMIT {
                        // println!("Slab condition met. Discarding intersection at {:?}", local_point);
                        COUNTER += 1;
                    }
                }
                return Intersect::empty();
            }
            if local_point.x.abs() <= half_size && local_point.y.abs() <= half_size && local_point.z.abs() <= half_size {
                return Intersect::new(point, self.normal, t, true, self.material, u, v);
            }
        }
        
        Intersect::empty()
    }
}

pub trait RayIntersect {
    fn ray_intersect(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Intersect;
}