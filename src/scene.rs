use std::collections::HashMap;
use uuid::Uuid;

use graphics::{ Graphics, ImageSize };
use graphics::math::Matrix2d;

use event::GenericEvent;
use ai_behavior::{
    Behavior,
    State,
    Running,
};

use sprite::Sprite;

use animation::{
    Animation,
    AnimationState,
};

/// A scene is used to manage sprite's life and run animation with sprite
pub struct Scene<I: ImageSize> {
    children: Vec<Sprite<I>>,
    children_index: HashMap<Uuid, usize>,
    running: HashMap<Uuid,
        Vec<(Behavior<Animation>, State<Animation, AnimationState>, bool)>>,
}

impl<I: ImageSize> Scene<I> {
    /// Create a new scene
    pub fn new() -> Scene<I> {
        Scene {
            children: Vec::new(),
            children_index: HashMap::new(),
            running: HashMap::new(),
        }
    }

    /// Update animation's state
    pub fn event<E>(&mut self, e: &E) where E: GenericEvent {
        // regenerate the animations and their states
        let running = self.running.clone();
        self.running.clear();

        for (id, animations) in running.into_iter() {
            let mut new_animations = Vec::new();

            for (b, mut a, paused) in animations.into_iter() {
                if paused {
                    new_animations.push((b, a, paused));
                    continue;
                }

                let sprite = self.child_mut(&id).unwrap();
                let (status, _) = a.event(e, &mut |_, dt, animation, s| {
                    let (state, status, remain) = {
                        let start_state;
                        let state = match *s {
                            None => { start_state = animation.to_state(sprite); &start_state },
                            Some(ref state) => state,
                        };
                        state.update(sprite, dt)
                    };
                    *s = state;
                    (status, remain)
                });

                match status {
                    // the behavior is still running, add it for next update
                    Running => {
                        new_animations.push((b, a, paused));
                    },
                    _ => {},
                }
            }

            if new_animations.len() > 0 {
                self.running.insert(id, new_animations);
            }
        }
    }

    /// Render this scene
    pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
        for child in self.children.iter() {
            child.draw(t, b);
        }
    }

    /// Register animation with sprite
    pub fn run<'b>(&'b mut self, sprite_id: &Uuid, animation: &'b Behavior<Animation>) {
        use std::collections::hash_map::Entry::{ Vacant, Occupied };
        let animations = match self.running.entry(sprite_id.clone()) {
            Vacant(entry) => entry.insert(Vec::new()),
            Occupied(entry) => entry.into_mut()
        };
        let state = State::new(animation.clone());
        animations.push((animation.clone(), state, false));
    }

    fn find(&self, sprite_id: &Uuid, animation: &Behavior<Animation>) -> Option<usize> {
        let mut index = None;
        if let Some(animations) = self.running.get(sprite_id) {
            for i in 0..animations.len() {
                let (ref b, _, _) = animations[i];
                if b == animation {
                    index = Some(i);
                    break;
                }
            }
        }
        index
    }

    /// Pause a running animation of the sprite
    pub fn pause(&mut self, sprite_id: &Uuid, animation: &Behavior<Animation>) {
        if let Some(index) = self.find(sprite_id, animation) {
            let animations = self.running.get_mut(&sprite_id).unwrap();
            let (b, s, _) = animations.remove(index);
            animations.push((b, s, true));
        }
    }

    /// Resume a paused animation of the sprite
    pub fn resume(&mut self, sprite_id: &Uuid, animation: &Behavior<Animation>) {
        if let Some(index) = self.find(sprite_id, animation) {
            let animations = self.running.get_mut(&sprite_id).unwrap();
            let (b, s, _) = animations.remove(index);
            animations.push((b, s, false));
        }
    }

    /// Toggle an animation of the sprite
    pub fn toggle(&mut self, sprite_id: &Uuid, animation: &Behavior<Animation>) {
        if let Some(index) = self.find(sprite_id, animation) {
            let animations = self.running.get_mut(&sprite_id).unwrap();
            let (b, s, paused) = animations.remove(index);
            animations.push((b, s, !paused));
        }
    }

    /// Stop a running animation of the sprite
    pub fn stop(&mut self, sprite_id: &Uuid, animation: &Behavior<Animation>) {
        if let Some(index) = self.find(sprite_id, animation) {
            self.running.get_mut(&sprite_id).unwrap().remove(index);
        }
    }

    /// Stop all running animations of the sprite
    pub fn stop_all(&mut self, sprite_id: &Uuid) {
        self.running.remove(&sprite_id);
    }

    /// Get all the running animations in the scene
    pub fn running(&self) -> usize {
        let mut total = 0;
        for (_, animations) in self.running.iter() {
            total += animations.len();
        }
        total
    }

    /// Add sprite to scene
    pub fn add_child(&mut self, sprite: Sprite<I>) -> Uuid {
        let id = sprite.id();
        self.children.push(sprite);
        self.children_index.insert(id.clone(), self.children.len() - 1);
        id
    }

    fn stop_all_including_children(&mut self, sprite: &Sprite<I>) {
        self.stop_all(&sprite.id());
        for child in sprite.children().iter() {
            self.stop_all_including_children(child);
        }
    }

    /// Remove the child by `id` from the scene's children or grandchild
    /// will stop all the animations run by this child
    pub fn remove_child(&mut self, id: &Uuid) -> Option<Sprite<I>> {
        let removed = if let Some(index) = self.children_index.remove(id) {
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
        };

        if removed.is_some() {
            self.stop_all_including_children(removed.as_ref().unwrap());
        }

        removed
    }

    /// Find the child by `id` from the scene's children or grandchild
    pub fn child(&self, id: &Uuid) -> Option<&Sprite<I>> {
        if let Some(index) = self.children_index.get(id) {
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
    pub fn child_mut(&mut self, id: &Uuid) -> Option<&mut Sprite<I>> {
        if let Some(index) = self.children_index.get(id) {
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
}
