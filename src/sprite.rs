use std::rc::Rc;
use std::collections::HashMap;

use uuid::Uuid;

use graphics::{ self, Graphics, ImageSize };
use graphics::math::{ Scalar, Matrix2d, Vec2d, Vec3d };

/// A sprite is a texture with some properties.
pub struct Sprite<I: ImageSize> {
    id: Uuid,

    visible: bool,

    anchor: Vec2d,

    position: Vec2d,
    rotation: Scalar,
    scale: Vec2d,
    color: Vec3d,


    flip_x: bool,
    flip_y: bool,

    opacity: f32,

    children: Vec<Sprite<I>>,
    children_index: HashMap<Uuid, usize>,

    texture: Rc<I>,
}

impl<I: ImageSize> Sprite<I> {
    /// Crate sprite from a texture
    pub fn from_texture(texture: Rc<I>) -> Sprite<I> {
        Sprite {
            id: Uuid::new_v4(),

            visible: true,

            anchor: [0.5, 0.5],

            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
            color: [0.0,1.0,0.0],

            flip_x: false,
            flip_y: false,

            opacity: 1.0,

            texture: texture,

            children: Vec::new(),
            children_index: HashMap::new(),
        }
    }

    /// Get the sprite's id
    #[inline(always)]
    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    /// Whether or not the sprite is visible
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Set the sprite's visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Get the sprite's anchor point
    ///
    /// The value is normalized. Default value is [0.5, 0.5] (the center of texture)
    #[inline(always)]
    pub fn anchor(&self) -> (Scalar, Scalar) {
        (self.anchor[0], self.anchor[1])
    }

    /// Set the sprite's anchor point
    #[inline(always)]
    pub fn set_anchor(&mut self, x: Scalar, y: Scalar) {
        self.anchor = [x, y];
    }

    /// Get the sprite's position
    #[inline(always)]
    pub fn position(&self) -> (Scalar, Scalar) {
        (self.position[0], self.position[1])
    }

    /// Set the sprite's position
    #[inline(always)]
    pub fn set_position(&mut self, x: Scalar, y: Scalar) {
        self.position = [x, y];
    }

    /// Set the sprite's draw color (tint)
    #[inline(always)]
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.color = [r, g, b];
    }

    /// get the sprite's color.s
    #[inline(always)]
    pub fn color(&self) -> (f64, f64, f64) {
        (self.color[0], self.color[1], self.color[2])
    }

    /// Get the sprite's rotation (in degree)
    #[inline(always)]
    pub fn rotation(&self) -> Scalar {
        self.rotation
    }

    /// Set the sprite's rotation (in degree)
    #[inline(always)]
    pub fn set_rotation(&mut self, deg: Scalar) {
        self.rotation = deg;
    }

    /// Get the sprite's scale
    #[inline(always)]
    pub fn scale(&self) -> (Scalar, Scalar) {
        (self.scale[0], self.scale[1])
    }

    /// Set the sprite's scale
    #[inline(always)]
    pub fn set_scale(&mut self, sx: Scalar, sy: Scalar) {
        self.scale = [sx, sy];
    }

    /// Whether or not the sprite is flipped horizontally.
    ///
    /// It only flips the texture of the sprite,
    /// and not the texture of the sprite’s children.
    ///
    /// Also, flipping the texture doesn’t alter the `anchor`.
    ///
    /// If you want to flip the `anchor` too,
    /// and/or to flip the children too use: sprite.scale.x *= -1;
    #[inline(always)]
    pub fn flip_x(&self) -> bool {
        self.flip_x
    }

    /// Flip the sprite
    #[inline(always)]
    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }

    /// Whether or not the sprite is flipped vertically.
    ///
    /// It only flips the texture of the sprite,
    /// and not the texture of the sprite’s children.
    ///
    /// Also, flipping the texture doesn’t alter the `anchor`.
    ///
    /// If you want to flip the `anchor` too,
    /// and/or to flip the children too use: sprite.scale.y *= -1;
    #[inline(always)]
    pub fn flip_y(&self) -> bool {
        self.flip_y
    }

    /// Flip the sprite
    #[inline(always)]
    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
    }

    /// Get the sprite's opacity
    #[inline(always)]
    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Set the sprite's opacity
    #[inline(always)]
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
    }

    /// Get the sprite's texture
    #[inline(always)]
    pub fn texture(&self) -> &Rc<I> {
        &self.texture
    }

    /// Set the sprite's texture
    #[inline(always)]
    pub fn set_texture(&mut self, texture: Rc<I>) {
        self.texture = texture;
    }

    /// Add a sprite as the child of this sprite, return the added sprite's id.
    pub fn add_child(&mut self, sprite: Sprite<I>) -> Uuid {
        let id = sprite.id();
        self.children.push(sprite);
        self.children_index.insert(id.clone(), self.children.len() - 1);
        id
    }

    /// Remove the child by `id` from this sprite's children or grandchild
    pub fn remove_child(&mut self, id: Uuid) -> Option<Sprite<I>> {
        if let Some(index) = self.children_index.remove(&id) {
            let removed = self.children.remove(index);
            // Removing a element of vector will alter the index,
            // update the mapping from uuid to index.
            for i in index..self.children.len() {
                let uuid = self.children[i].id();
                self.children_index.insert(uuid, i);
            }
            Some(removed)
        } else {
            for child in self.children.iter_mut() {
                if let Some(c) = child.remove_child(id.clone()) {
                    return Some(c);
                }
            }
            None
        }
    }

    /// Find the child by `id` from this sprite's children or grandchild
    pub fn child(&self, id: Uuid) -> Option<&Sprite<I>> {
        if let Some(index) = self.children_index.get(&id) {
            Some(&self.children[*index])
        } else {
            for child in self.children.iter() {
                if let Some(c) = child.child(id.clone()) {
                    return Some(c);
                }
            }
            None
        }
    }

    /// Find the child by `id` from this sprite's children or grandchild, mutability
    pub fn child_mut(&mut self, id: Uuid) -> Option<&mut Sprite<I>> {
        if let Some(index) = self.children_index.get(&id) {
            Some(&mut self.children[*index])
        } else {
            for child in self.children.iter_mut() {
                if let Some(c) = child.child_mut(id.clone()) {
                    return Some(c);
                }
            }
            None
        }
    }

    /// Get the sprite's children
    #[inline(always)]
    pub fn children(&self) -> &Vec<Sprite<I>> {
        &self.children
    }

    /// Draw this sprite and its children
    pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
        use graphics::*;

        if !self.visible {
            return;
        }

        let (w, h) = self.texture.get_size();
        let w = w as f64;
        let h = h as f64;
        let anchor = [self.anchor[0] * w, self.anchor[1] * h];

        let transformed = t.trans(self.position[0], self.position[1])
                           .rot_deg(self.rotation)
                           .scale(self.scale[0], self.scale[1]);

        let mut model = transformed;

        if self.flip_x {
            model = model.trans(w - 2.0 * anchor[0], 0.0).flip_h();
        }

        if self.flip_y {
            model = model.trans(0.0, h - 2.0 * anchor[1]).flip_v();
        }

        let draw_state = default_draw_state();

        // for debug: bounding_box
        //model.rgb(1.0, 0.0, 0.0).draw(b);

        graphics::Image::new()
            .color([self.color[0] as f32, self.color[1] as f32, self.color[2] as f32, self.opacity])
            .rect([-anchor[0], -anchor[1], w, h])
            .draw(&*self.texture, draw_state, model, b);

        // for debug: anchor point
        //c.trans(self.position[0], self.position[1]).rect(-5.0, -5.0, 10.0, 10.0).rgb(0.0, 0.0, 1.0).draw(b);

        for child in self.children.iter() {
            child.draw(transformed, b);
        }
    }

    /// Get the sprite's bounding box
    pub fn bounding_box(&self) -> graphics::types::Rectangle {
        let (w, h) = self.texture.get_size();
        let w = w as f64 * self.scale[0];
        let h = h as f64 * self.scale[1];

        [
            self.position[0] - self.anchor[0] * w,
            self.position[1] - self.anchor[1] * h,
            w,
            h
        ]
    }
}
