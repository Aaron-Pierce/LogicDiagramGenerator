use core::num;
use image::{
    imageops, math, open, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage, Rgba,
    RgbaImage, SubImage,
};
use std::{collections::HashMap, vec};
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Sprite {
    AND,
    NAND,
    OR,
    NOR,
    XOR,
    XNOR,
    NOT,
    INPUT
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct SpriteInfo {
    pub width: u32,
    pub height: u32,
    pub inputs: Vec<u32>,
    pub outputs: Vec<u32>,
}

impl SpriteInfo {
    pub fn new(width: u32, height: u32, inputs: Vec<u32>, outputs: Vec<u32>) -> SpriteInfo {
        SpriteInfo {
            width,
            height,
            inputs,
            outputs,
        }
    }
}

pub struct SpriteCreator {
    spritesheet: DynamicImage,
    already_generated: HashMap<Sprite, DynamicImage>,
}

impl SpriteCreator {
    pub fn new() -> SpriteCreator {

        let mut sprite_map = HashMap::new();
        for sprite in vec![
            Sprite::AND,
            Sprite::NAND,
            Sprite::OR,
            Sprite::NOR,
            Sprite::XOR,
            Sprite::XNOR,
            Sprite::NOT,
            Sprite::INPUT,
        ] {
            let filename = match sprite {
                Sprite::AND => "assets/AND.png",
                Sprite::NAND => "assets/NAND.png",
                Sprite::OR => "assets/OR.png",
                Sprite::NOR => "assets/NOR.png",
                Sprite::XOR => "assets/XOR.png",
                Sprite::XNOR => "assets/XNOR.png",
                Sprite::NOT => "assets/NOT.png",
                Sprite::INPUT => "assets/INPUT.png"
            };

            let img = image::open(filename).unwrap();
            sprite_map.insert(sprite.clone(), img);
        };
        
        SpriteCreator {
            spritesheet: image::open("spritesheet.png").unwrap(),
            already_generated: sprite_map,
        }
    }

    pub fn get_sprite_info(&self, sprite_name: Sprite) -> SpriteInfo {
        match sprite_name {
            Sprite::AND => SpriteInfo {
                width: 90,
                height: 40,
                inputs: vec![9, 29],
                outputs: vec![19],
            },
            Sprite::NAND => SpriteInfo {
                width: 90,
                height: 40,
                inputs: vec![9, 29],
                outputs: vec![19],
            },
            Sprite::OR => SpriteInfo {
                width: 90,
                height: 40,
                inputs: vec![9, 29],
                outputs: vec![19],
            },
            Sprite::NOR => SpriteInfo {
                width: 90,
                height: 40,
                inputs: vec![9, 29],
                outputs: vec![19],
            },
            Sprite::XOR => SpriteInfo {
                width: 90,
                height: 40,
                inputs: vec![9, 29],
                outputs: vec![19],
            },
            Sprite::XNOR => SpriteInfo {
                width: 90,
                height: 40,
                inputs: vec![9, 29],
                outputs: vec![19],
            },
            Sprite::NOT => SpriteInfo {
                width: 90,
                height: 40,
                inputs: vec![21],
                outputs: vec![21],
            },
            Sprite::INPUT => SpriteInfo {
                width: 90,
                height: 40,
                inputs: vec![],
                outputs: vec![21],
            },
        }
    }

    pub fn get_sprite(&self, sprite_name: Sprite) -> &DynamicImage {
        return self.already_generated.get(&sprite_name).unwrap();
    }
}
