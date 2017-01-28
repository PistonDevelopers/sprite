mod fake_texture;

extern crate sprite;
extern crate ai_behavior;
extern crate input;
extern crate graphics;

use std::rc::Rc;
use ai_behavior::{Action};

use sprite::*;
use fake_texture::FakeTexture;

#[test]
fn pruning_stopped_sprites() {
    let mut scene: Scene<FakeTexture> = Scene::new();
    let sprite = Sprite::from_texture(Rc::new(FakeTexture::new()));

    let id = scene.add_child(sprite);
    assert_eq!(0, scene.running_for_child(id).unwrap());

    // Add a 1s animation to the sprite
    scene.run(id, &Action(FadeOut(1.0)));
    assert_eq!(1, scene.running_for_child(id).unwrap());

    // Schedule sprite for removal
    scene.remove_child_when_done(id);
    assert!(scene.child(id).is_some()); // Hasn't been removed yet!

    scene.event(&dt_event(0.9)); // Advance to just before animation finishes

    assert_eq!(1, scene.running_for_child(id).unwrap());
    assert!(scene.child(id).is_some()); // Hasn't been removed yet!

    scene.event(&dt_event(0.2)); // Advance past the end of the animation

    assert!(scene.child(id).is_none()); // Now it is removed
    assert!(scene.running_for_child(id).is_none());
}

#[test]
fn remove_child_when_done_when_sprite_already_stopped() {
    let mut scene: Scene<FakeTexture> = Scene::new();
    let sprite = Sprite::from_texture(Rc::new(FakeTexture::new()));

    let id = scene.add_child(sprite);
    scene.remove_child_when_done(id);

    assert!(scene.child(id).is_none());
}

#[test]
fn remove_child_when_done_when_animations_paused() {
    let mut scene: Scene<FakeTexture> = Scene::new();
    let sprite = Sprite::from_texture(Rc::new(FakeTexture::new()));

    let id = scene.add_child(sprite);
    let animation = Action(FadeOut(1.0));
    scene.run(id, &animation);
    scene.pause(id, &animation);
    scene.remove_child_when_done(id);

    assert!(scene.child(id).is_some());
}

fn dt_event(dt: f64) -> input::Input {
    let event = input::Input::Update(input::UpdateArgs { dt: 0.0 });
    let event: input::Input = input::UpdateEvent::from_dt(dt, &event).unwrap();

    event
}
