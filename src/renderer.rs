use std::{collections::VecDeque, convert::TryInto};

use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use rusttype::{point, Font, Scale};

use crate::expression_parser::{Gate, GateType};
use crate::spritesheet::{Sprite, SpriteCreator, SpriteInfo};

pub struct Renderer {
    sprite_creator: SpriteCreator,
    image: RgbaImage,
    font: Font<'static>,
}

impl Renderer {
    pub fn new(tree: &Gate) -> Renderer {
        let to_return = Renderer {
            sprite_creator: SpriteCreator::new(),
            image: ImageBuffer::from_fn(
                tree.calculate_drawn_image_width(),
                tree.calculate_drawn_image_height(),
                |_, _| Rgba([255, 255, 255, 255]),
            ),
            font: Font::try_from_bytes(include_bytes!("../assets/fonts/cmunrm.ttf"))
                .expect("error constructing a Font from bytes"),
        };

        to_return
    }

    pub fn draw_tree(&mut self, tree: &Gate) {
        self.draw_tree_recursive(
            &tree,
            tree.adjusted_origin(self.image.height() / 2),
            tree.depth(),
            tree.depth(),
            None,
        );
        // self.draw_tree_breadth_first(tree, self.image.height()/2);
    }

    fn draw_tree_recursive(
        &mut self,
        tree: &Gate,
        y_origin: u32,
        depth: u32,
        max_depth: u32,
        connect_to_point: Option<[u32; 2]>,
    ) {
        let x_offset = 90 * (depth - 1);

        println!("Given last y_origin as {}", y_origin);

        let sprite_type = tree.sprite_type();
        self.draw(sprite_type, tree.get_name(), x_offset, y_origin);

        match connect_to_point {
            Some(point) => {
                self.wire(
                    x_offset + 90,
                    y_origin
                        + self
                            .sprite_creator
                            .get_sprite_info(sprite_type.clone())
                            .outputs[0],
                    point[0],
                    point[1],
                );
            }
            None => {}
        }

        for i in 0..tree.get_inputs().len() {
            self.draw_tree_recursive(
                tree.get_inputs().get(i).unwrap(),
                tree.get_inputs().get(i).unwrap().child_yoffset_function(
                    y_origin,
                    i as u32,
                    tree.get_inputs().len() as u32,
                ),
                depth - 1,
                max_depth,
                Some([
                    x_offset,
                    y_origin
                        + self
                            .sprite_creator
                            .get_sprite_info(sprite_type.clone())
                            .inputs[i],
                ]),
            );
        }
    }

    fn draw_tree_breadth_first(&mut self, tree: &Gate, y_origin: u32) {
        let mut queue: VecDeque<&Gate> = VecDeque::new();
        queue.push_back(tree);
        let columns = tree.column_sizes();
        println!("render columns: {:#?}", columns);
        let mut current_depth = tree.depth();
        let mut remaining_in_level = 1;
        let mut next_level = 0;
        const gate_padding: u32 = 40;
        while !queue.is_empty() {
            let root = queue.pop_front().unwrap();
            let x_offset = 100 * (current_depth - 1);

            println!(
                "In column {}, height should be {}",
                current_depth - 1,
                (40 + gate_padding) * columns.get(current_depth as usize - 1).unwrap()
            );

            remaining_in_level -= 1;
            self.draw(
                root.sprite_type(),
                root.get_name(),
                x_offset,
                y_origin + (40 + gate_padding) * remaining_in_level
                    - (40 + gate_padding) / 2 * columns.get(current_depth as usize - 1).unwrap(),
            );

            for child in root.get_inputs().iter() {
                queue.push_back(child);
                next_level += 1;
            }

            if remaining_in_level == 0 {
                current_depth -= 1;
                remaining_in_level = next_level;
                next_level = 0;
            }
        }
    }

    // fn draw_tree_depth_first(&mut self, tree: &Gate, y_origin: u32){
    //     let mut stack: Vec<&Gate> = Vec::new();
    //     stack.push(tree);
    //     let columns = tree.column_sizes();
    //     println!("render columns: {:#?}", columns);
    //     let mut current_depth = tree.depth();
    //     let mut remaining_in_level = 1;
    //     let mut next_level = 0;
    //     const gate_padding: u32 = 40;
    //     while !stack.is_empty() {
    //         let root = queue.pop_front().unwrap();
    //         let x_offset = 100 * (current_depth - 1);

    //         println!("In column {}, height should be {}", current_depth - 1, (40 + gate_padding) * columns.get(current_depth as usize - 1).unwrap());

    //         remaining_in_level -= 1;
    //         self.draw(root.sprite_type(), x_offset, y_origin + (40+gate_padding)*remaining_in_level - (40 + gate_padding)/2 * columns.get(current_depth as usize - 1).unwrap());

    //         for child in root.get_inputs().iter() {
    //             queue.push_back(child);
    //             next_level += 1;
    //         }

    //         if remaining_in_level == 0 {
    //             current_depth -= 1;
    //             remaining_in_level = next_level;
    //             next_level = 0;
    //         }
    //     }
    // }

    pub fn draw(&mut self, sprite: Sprite, name: String, x_offset: u32, y_offset: u32) {
        let sprite_image = self.sprite_creator.get_sprite(sprite.clone());

        for i in 0..sprite_image.width() {
            for j in 0..sprite_image.height() {
                self.image.put_pixel(
                    x_offset + i,
                    y_offset + j,
                    sprite_image.get_pixel(i, j).to_owned(),
                );
            }
        }

        if sprite != Sprite::INPUT {
            //only draw names of inputs
            return;
        }

        let size: f32 = 25.0; // arbitrary font size
        let pixel_size = size.ceil() as usize;

        // 2x scale in x direction to counter the aspect ratio of monospace characters.
        let scale = Scale {
            x: size,
            y: size,
        };

        
        let v_metrics = self.font.v_metrics(scale);
        let offset = point(0.0, 0.0);

        let glyphs: Vec<_> = self.font.layout(&name[..], scale, offset).collect();
        let img = &mut self.image;

        for g in glyphs.iter() {
            g.draw(|x, y, o| {
                let brightness: u8 = ((1.0 - o) * 255.0) as u8;
                // println!("Pixel height {}", );
                let origin = x_offset + 39;
                img.put_pixel(
                    (origin + x + g.position().x as u32),
                    ((y_offset + y - 12) as i32 + (g.pixel_bounding_box().unwrap().max.y) as i32 + (g.pixel_bounding_box().unwrap().min.y) as i32) as u32,
                    Rgba([brightness, brightness, brightness, 255]),
                );
            })
        }
        // DrawCallback::new(self, x_offset, y_offset, self.sprite_creator.get_sprite_info(sprite))
    }

    pub fn wire(&mut self, x_origin: u32, y_origin: u32, x_dest: u32, y_dest: u32) {
        println!(
            "A wire from {},{} to {},{}",
            x_origin, y_origin, x_dest, y_dest
        );

        let dy: i32 = y_dest as i32 - y_origin as i32;

        let output_offset = (dy < 0) as i32 * 2;

        for y in (0)..((dy).abs() + output_offset) {
            let pixel_x = x_origin;
            let pixel_y = (y_origin.min(y_dest) as i32 + y) as u32;
            self.image.put_pixel(pixel_x, pixel_y, Rgba([0, 0, 0, 255]));
            self.image
                .put_pixel(pixel_x + 1, pixel_y, Rgba([0, 0, 0, 255]));
        }

        for x in 0..(x_dest - x_origin) {
            let pixel_x = (x_origin + x) as u32;
            let pixel_y = (y_origin as i32 + dy) as u32;
            self.image.put_pixel(pixel_x, pixel_y, Rgba([0, 0, 0, 255]));
            self.image
                .put_pixel(pixel_x, pixel_y + 1, Rgba([0, 0, 0, 255]));
        }
    }

    pub fn export(&self) {
        self.image.save("output.png").unwrap();
    }
}
