use amethyst::{
    assets::{DefaultLoader, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        let sprite_sheet_handle = load_sprite_sheet(data.resources);

        initialise_paddles(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

fn load_sprite_sheet(resources: &mut Resources) -> Handle<SpriteSheet> {
    let _texture_handle: Handle<Texture> = {
        let loader = resources.get::<DefaultLoader>().unwrap();
        loader.load("texture/pong_spritesheet.png")
    };
    let loader = resources.get::<DefaultLoader>().unwrap();
    loader.load("texture/pong_spritesheet.ron")
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world.push((Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT), transform));
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0); // paddle is the first sprite in the sprite_sheet

    // Create a left plank entity.
    world.push((
        sprite_render.clone(),
        Paddle::new(Side::Left),
        left_transform,
    ));

    // Create right plank entity.
    world.push((
        sprite_render.clone(),
        Paddle::new(Side::Right),
        right_transform,
    ));
}
