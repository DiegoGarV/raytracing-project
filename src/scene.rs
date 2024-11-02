use crate::objects::{Object, SquarePlane};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_scene() -> Vec<Object> {
    let mut objects = vec![];
    let size = 1.5;
    let scene_info = parse_scene_info("./src/builder.txt");
    let mut normal = (0.0,0.0,0.0);
    for plane in scene_info{
        let mat = plane.4;
        let is_slab = mat.contains("slab");
        let material_name = if is_slab { mat.replace("slab", "") } else {mat};

        if plane.3 == "front"{
            normal = (0.0,0.0,-1.0);
        } else if plane.3 == "left"{
            normal = (-1.0,0.0,0.0);
        }  else if plane.3 == "right"{
            normal = (1.0,0.0,0.0);
        } else if plane.3 == "up"{
            normal = (0.0,1.0,0.0);
        } else if plane.3 == "bottom"{
            normal = (0.0,-1.0,0.0);
        }

        // println!(
        //     "Creating SquarePlane - Center: {:?}, Normal: {:?}, Size: {}, Material: {}, Is Slab: {}",
        //     (plane.0 * size, plane.1 * size, plane.2 * size),
        //     normal,
        //     size,
        //     material_name,
        //     is_slab
        // );

        objects.push(
            Object::SquarePlane(SquarePlane::new(
                (plane.0*size, plane.1*size, plane.2*size),
                normal,
                size,
                &material_name,
                is_slab
            ))
        );
    }
    objects
}

fn parse_scene_info(file_path: &str) -> Vec<(f32, f32, f32, String, String)> {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();  // Panics if the file can't be opened
    let reader = BufReader::new(file);

    let mut tuples = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();  // Panics if there is an error reading the line
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() >= 5 {
            let id = parts[0].parse().unwrap();  // Panics if parsing fails
            let x = parts[1].parse().unwrap();
            let y = parts[2].parse().unwrap();
            let direction = parts[3].to_string();
            let material = parts[4].to_string();

            let material_as_slab = if parts.len() == 6 && parts[5] == "slab" {
                format!("{}slab",material)
            } else {
                material
            };

            tuples.push((id, x, y, direction, material_as_slab));
        }
    }
    tuples
}