use nalgebra_glm::Vec3;
use crate::color::Color;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::texture::Texture;

static SAND: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/sand.bmp")));
static SANDSTONE_TOP: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/sandstone/sandstone_top.bmp")));
static SANDSTONE_SIDE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/sandstone/sandstone_side.bmp")));
static SANDSTONE_BOTTOM: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/sandstone/sandstone_bottom.bmp")));
static FURNACE_FRONT: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/furnace/furnace_front.bmp")));
static FURNACE_SIDE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/furnace/furnace_side.bmp")));
static FURNACE_TOP: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/furnace/furnace_top.bmp")));
static CRAFT_TOP: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/craftingTable/craft_top.bmp")));
static CRAFT_FRONT: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/craftingTable/craft_front.bmp")));
static CRAFT_SIDE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/craftingTable/craft_side.bmp")));
static WATER: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/water.bmp")));

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32;4],
    pub refractive_index: f32,
    pub has_texture: bool,
    pub texture: usize
}
impl Material{
    pub fn new( material: &str)-> Self{
        let mut albedo = [0.8, 0.2, 0.0, 0.0];
        let hex_color:u32 = 0x000000;
        let specular:f32 = 1.0;
        let mut refractive_index = 1.0;
        let mut has_texture = false;
        let mut texture = 0;

        if material=="sand"{
            has_texture = true;
            texture = 0;
        } else if material=="sandstone_top" {
            has_texture = true;
            texture = 1;
        } else if material=="sandstone_side"{
            has_texture = true;
            texture = 2;
        } else if material=="sandstone_bottom"{
            has_texture = true;
            texture = 3;
        } else if material == "furnace_front"{
            albedo[0] = 0.9;
            albedo[1] = 0.1;
            has_texture = true;
            texture = 4;
        } else if material == "furnace_side"{
            albedo[0] = 0.9;
            albedo[1] = 0.1;
            has_texture = true;
            texture = 5;
        } else if material == "furnace_top"{
            albedo[0] = 0.9;
            albedo[1] = 0.1;
            has_texture = true;
            texture = 6;
        } else if material == "craft_top"{
            albedo[0] = 0.9;
            albedo[1] = 0.1;
            has_texture = true;
            texture = 7;
        } else if material == "craft_front"{
            albedo[0] = 0.9;
            albedo[1] = 0.1;
            has_texture = true;
            texture = 8; 
        } else if material == "craft_side"{
            albedo[0] = 0.9;
            albedo[1] = 0.1;
            has_texture = true;
            texture = 9;
        } else if material == "water"{
            albedo[0] = 0.8;
            albedo[1] = 0.5;
            albedo[2] = 0.1;
            albedo[3] = 0.1;
            refractive_index = 1.33;
            has_texture = true;
            texture = 10;
        }
        let diffuse = Color::from_hex(hex_color);
        Material{
            diffuse,
            specular,
            albedo,
            refractive_index,
            has_texture,
            texture
        }
    }

    pub fn get_diffuse (&self, u: f32, v: f32) -> Color{

        if self.has_texture{
            let mut texture = &SAND;
            if self.texture ==0{
                texture = &SAND
            } else if self.texture == 1{    
                texture = &SANDSTONE_TOP
            } else if self.texture == 2{
                texture = &SANDSTONE_SIDE
            } else if self.texture == 3{
                texture = &SANDSTONE_BOTTOM
            } else if self.texture == 4{
                texture = &FURNACE_FRONT
            } else if self.texture == 5{
                texture = &FURNACE_SIDE
            } else if self.texture == 6{
                texture = &FURNACE_TOP
            } else if self.texture == 7{
                texture = &CRAFT_TOP
            } else if self.texture == 8{
                texture = &CRAFT_FRONT
            } else if self.texture == 9{
                texture = &CRAFT_SIDE
            } else if self.texture == 10{
                texture = &WATER
            }

            let x =(u  * (texture.width as f32-1.0)) as usize;
            let y =(v  * (texture.height as f32-1.0)) as usize;
            texture.get_color(x, y)

        } else {
            self.diffuse
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
    pub u: f32,
    pub v: f32
}

impl Intersect {
        pub fn new(point: Vec3, normal: Vec3, distance: f32, is_intersecting: bool,material: Material, u: f32, v: f32) -> Self {
        Intersect {
            point, 
            normal,
            distance,
            is_intersecting,
            material,
            u, v
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: Material{
              diffuse: Color::new(0, 0, 0),
              specular: 0.0,
              albedo: [0.0, 0.0, 0.0, 0.0],
              refractive_index: 0.0,
              has_texture: false,
              texture: 0
            },
            u: 0.0,
            v: 0.0
        }
    }
}
