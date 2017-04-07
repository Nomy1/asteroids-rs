extern crate amethyst;

mod states;
mod components;
mod systems;

use amethyst::{Application};
use amethyst::config::Element;
use amethyst::ecs::{World, Join, RunArg, System};
use amethyst::gfx_device::DisplayConfig;
use components::*;
use systems::*;


fn main() {
    let path = format!("{}/resources/config.yml", env!("CARGO_MANIFEST_DIR"));
    let cfg = DisplayConfig::from_file(path).expect("Could not find config!");
    let mut game = Application::build(::states::PlayState, cfg)
    	.register::<Player>()
    	.register::<Movement>()
    	.with::<MovementSystem>(MovementSystem, "MovementSystem", 1)
    	.done();
    game.run();
}
