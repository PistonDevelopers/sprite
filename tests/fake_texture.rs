extern crate graphics;

use self::graphics::ImageSize;

pub struct FakeTexture;

impl FakeTexture {
    pub fn new() -> Self { FakeTexture }
}

impl ImageSize for FakeTexture {
    fn get_size(&self) -> (u32, u32) { (32, 32) }
    fn get_width(&self) -> u32 { 32 }
    fn get_height(&self) -> u32 { 32 }
}

