
use std::collections::hashmap::HashMap;
use uuid::Uuid;

use graphics::*;

use event::{
    Event,
    Behavior,
    State,
    Running,
};

use Sprite;

use animation::{
    Animation,
    AnimationState,
};

/// A scene is used to manage sprite's life and run animation with sprite
pub struct Scene<I: ImageSize> {
    children: Vec<Sprite<I>>,
    children_index: HashMap<Uuid, uint>,
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
    pub fn event(&mut self, e: &Event) {
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

                let sprite = self.child_mut(id).unwrap();
                let (status, _) = a.event(e, |_, dt, animation, s| {
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
    pub fn draw<B: BackEnd<I>>(&self, c: &Context, b: &mut B) {
        for child in self.children.iter() {
            child.draw(c, b);
        }
    }

    /// Register animation with sprite
    pub fn run(&mut self, sprite_id: Uuid, animation: &Behavior<Animation>) {
        use std::collections::hashmap::{ Vacant, Occupied };
        let animations = match self.running.entry(sprite_id) {
            Vacant(entry) => entry.set(Vec::new()),
            Occupied(entry) => entry.into_mut()
        };
        let state = State::new(animation.clone());
        animations.push((animation.clone(), state, false));
    }

    fn find(&self, sprite_id: Uuid, animation: &Behavior<Animation>) -> Option<uint> {
        let mut index = None;
        match self.running.find(&sprite_id) {
            Some(animations) => {
                for i in range(0, animations.len()) {
                    let (ref b, _, _) = animations[i];
                    if b == animation {
                        index = Some(i);
                        break;
                    }
                }
            },
            _ => {},
        }
        index
    }

    /// Pause a running animation of the sprite
    pub fn pause(&mut self, sprite_id: Uuid, animation: &Behavior<Animation>) {
        let index = self.find(sprite_id, animation);
        if index.is_some() {
            println!("found");
            let i = index.unwrap();
            let animations = self.running.get_mut(&sprite_id);
            let (b, s, _) = animations.remove(i).unwrap();
            animations.push((b, s, true));
        }
    }

    /// Resume a paused animation of the sprite
    pub fn resume(&mut self, sprite_id: Uuid, animation: &Behavior<Animation>) {
        let index = self.find(sprite_id, animation);
        if index.is_some() {
            println!("found");
            let i = index.unwrap();
            let animations = self.running.get_mut(&sprite_id);
            let (b, s, _) = animations.remove(i).unwrap();
            animations.push((b, s, false));
        }
    }

    /// Toggle an animation of the sprite
    pub fn toggle(&mut self, sprite_id: Uuid, animation: &Behavior<Animation>) {
        let index = self.find(sprite_id, animation);
        if index.is_some() {
            let i = index.unwrap();
            let animations = self.running.get_mut(&sprite_id);
            let (b, s, paused) = animations.remove(i).unwrap();
            animations.push((b, s, !paused));
        }
    }

    /// Stop a running animation of the sprite
    pub fn stop(&mut self, sprite_id: Uuid, animation: &Behavior<Animation>) {
        let index = self.find(sprite_id, animation);
        if index.is_some() {
            let i = index.unwrap();
            self.running.get_mut(&sprite_id).remove(i);
        }
    }

    /// Stop all running animations of the sprite
    pub fn stop_all(&mut self, sprite_id: Uuid) {
        self.running.remove(&sprite_id);
    }

    /// Get all the running animations in the scene
    pub fn running(&self) -> uint {
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
        self.children_index.insert(id, self.children.len() - 1);
        id
    }

    fn stop_all_including_children(&mut self, sprite: &Sprite<I>) {
        self.stop_all(sprite.id());
        for child in sprite.children().iter() {
            self.stop_all_including_children(child);
        }
    }

    /// Remove the child by `id` from the scene's children or grandchild
    /// will stop all the animations run by this child
    pub fn remove_child(&mut self, id: Uuid) -> Option<Sprite<I>> {
        let removed = match self.children_index.pop(&id) {
            Some(i) => {
                let removed = self.children.remove(i).unwrap();
                // Removing a element of vector will alter the index,
                // update the mapping from uuid to index.
                for index in range(i, self.children.len()) {
                    let uuid = self.children[index].id();
                    self.children_index.insert(uuid, index);
                }
                Some(removed)
            },
            None => {
                for child in self.children.iter_mut() {
                    match child.remove_child(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        };

        if removed.is_some() {
            self.stop_all_including_children(removed.as_ref().unwrap());
        }

        removed
    }

    /// Find the child by `id` from the scene's children or grandchild
    pub fn child(&self, id: Uuid) -> Option<&Sprite<I>> {
        match self.children_index.find(&id) {
            Some(i) => { Some(&self.children[*i]) },
            None => {
                for child in self.children.iter() {
                    match child.child(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        }
    }

    /// Find the child by `id` from this sprite's children or grandchild, mutability
    pub fn child_mut(&mut self, id: Uuid) -> Option<&mut Sprite<I>> {
        match self.children_index.find(&id) {
            Some(i) => { Some(self.children.get_mut(*i)) },
            None => {
                for child in self.children.iter_mut() {
                    match child.child_mut(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        }
    }
}
