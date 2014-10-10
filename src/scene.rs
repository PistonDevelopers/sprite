
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

use action::{
    Action,
    ActionState,
};

/// A scene is used to manage sprite's life and run action with sprite
pub struct Scene<I: ImageSize> {
    children: Vec<Sprite<I>>,
    children_index: HashMap<Uuid, uint>,
    running: HashMap<Uuid, Vec<(Behavior<Action>, State<Action, ActionState>, bool)>>,
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

    /// Update action's state
    pub fn update(&mut self, e: &Event) {
        // regenerate the actions and their states
        let running = self.running.clone();
        self.running.clear();

        for (id, actions) in running.into_iter() {
            let mut new_actions = Vec::new();

            for (b, mut a, paused) in actions.into_iter() {
                if paused {
                    new_actions.push((b, a, paused));
                    continue;
                }

                let sprite = self.child_mut(id).unwrap();
                let (status, _) = a.event(e, |dt, action, s| {
                    let (state, status, remain) = {
                        let start_state;
                        let state = match *s {
                            None => { start_state = action.to_state(sprite); &start_state },
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
                        new_actions.push((b, a, paused));
                    },
                    _ => {},
                }
            }

            if new_actions.len() > 0 {
                self.running.insert(id, new_actions);
            }
        }
    }

    /// Render this scene
    pub fn draw<B: BackEnd<I>>(&self, c: &Context, b: &mut B) {
        for child in self.children.iter() {
            child.draw(c, b);
        }
    }

    /// Register action with sprite
    pub fn run(&mut self, sprite_id: Uuid, action: &Behavior<Action>) {
        let actions = self.running.find_or_insert_with(sprite_id, |_| Vec::new());
        let state = State::new(action.clone());
        actions.push((action.clone(), state, false));
    }

    fn find(&self, sprite_id: Uuid, action: &Behavior<Action>) -> Option<uint> {
        let mut index = None;
        match self.running.find(&sprite_id) {
            Some(actions) => {
                for i in range(0, actions.len()) {
                    let (ref b, _, _) = actions[i];
                    if b == action {
                        index = Some(i);
                        break;
                    }
                }
            },
            _ => {},
        }
        index
    }

    /// Pause a running action of the sprite
    pub fn pause(&mut self, sprite_id: Uuid, action: &Behavior<Action>) {
        let index = self.find(sprite_id, action);
        if index.is_some() {
            println!("found");
            let i = index.unwrap();
            let actions = self.running.get_mut(&sprite_id);
            let (b, s, _) = actions.remove(i).unwrap();
            actions.push((b, s, true));
        }
    }

    /// Resume a paused action of the sprite
    pub fn resume(&mut self, sprite_id: Uuid, action: &Behavior<Action>) {
        let index = self.find(sprite_id, action);
        if index.is_some() {
            println!("found");
            let i = index.unwrap();
            let actions = self.running.get_mut(&sprite_id);
            let (b, s, _) = actions.remove(i).unwrap();
            actions.push((b, s, false));
        }
    }

    /// Toggle an action of the sprite
    pub fn toggle(&mut self, sprite_id: Uuid, action: &Behavior<Action>) {
        let index = self.find(sprite_id, action);
        if index.is_some() {
            let i = index.unwrap();
            let actions = self.running.get_mut(&sprite_id);
            let (b, s, paused) = actions.remove(i).unwrap();
            actions.push((b, s, !paused));
        }
    }

    /// Stop a running action of the sprite
    pub fn stop(&mut self, sprite_id: Uuid, action: &Behavior<Action>) {
        let index = self.find(sprite_id, action);
        if index.is_some() {
            let i = index.unwrap();
            self.running.get_mut(&sprite_id).remove(i);
        }
    }

    /// Stop all running actions of the sprite
    pub fn stop_all(&mut self, sprite_id: Uuid) {
        self.running.remove(&sprite_id);
    }

    /// Get all the running actions in the scene
    pub fn running(&self) -> uint {
        let mut total = 0;
        for (_, actions) in self.running.iter() {
            total += actions.len();
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
    /// will stop all the actions run by this child
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

