mod framebuffer;
mod color;
use camera::Camera;
use color::Color;
mod materials;
mod objects;
mod ray_caster;
use minifb::{Key, Window, WindowOptions};
mod light;
use std::time::Duration;
use nalgebra_glm::{normalize, Vec3};
mod camera;
mod scene;
mod texture;
use rayon::prelude::*; 
use light::Light;

fn main() {
    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = 600;
    let framebuffer_height = 600;

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let frame_delay = Duration::from_millis(0);
    let objects = scene::get_scene();
    
    let mut window = Window::new(
        "Raycasting Project - Minecraft",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();
    let mut camera = Camera{
        eye: Vec3::new(0.0,0.0,-10.0), 
        center:  Vec3::new(0.0,0.0,8.0), 
        up: Vec3::new(0.0,-1.0,0.0),
        has_changed: true};


        let l1 = Light::new(
            Vec3::new(3.50, 2.0, 1.0),
            Color::from_hex(0xff8000),
            0.4
        );
        let l2 = Light::new(
            Vec3::new(-10.0, 5.0, 5.0),
            Color::new(255, 255, 255),
            0.6
        );
        let l3 = Light::new(
            Vec3::new(0.0, 8.0, -8.0),
            Color::new(255, 255, 255),
            0.6
        );
        let l4 = Light::new(
            Vec3::new(10.0, 5.0, 5.5),
            Color::from_hex(0xee00ff),
            0.4
        );

    let lights = vec![l1,l2, l3,l4];
    while window.is_open(){
        // closing game
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::W) {
           camera.orbit(0.0, -0.2)
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, 0.2)
        }
        if window.is_key_down(Key::A) {
            camera.orbit(-0.2, 0.0)
        }
        if window.is_key_down(Key::D) {
            camera.orbit(0.2, 0.0)
        }
        if window.is_key_down(Key::Up) {
            camera.zoom(1.0);
        }
        if window.is_key_down(Key::Down) {
            camera.zoom(-1.0);
        }

        if camera.check_if_change(){
            render(&mut framebuffer, &objects, &camera, &lights);
        }
        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}


pub fn render(framebuffer: &mut framebuffer::Framebuffer, objects: &[objects::Object], camera: &Camera, lights: &[Light]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    

    framebuffer.buffer.par_chunks_mut(framebuffer.width as usize)
        .enumerate()
        .for_each(|(y, row)| {
            let screen_y = (2.0 * y as f32) / height -1.0;
            for (x, pixel) in row.iter_mut().enumerate() {
                let screen_x = (2.0 * x as f32) / width - 1.0;
                let screen_x = screen_x * aspect_ratio;

                let ray_direction = normalize(
                    &camera.basis_change(&Vec3::new(screen_x, screen_y, -1.0))
                );

                let pixel_color: u32 = ray_caster::cast_ray(
                    &camera.eye, &ray_direction,
                    objects, lights,
                    0
                );

                *pixel = Color::from_hex(pixel_color);
            }
        });
}
