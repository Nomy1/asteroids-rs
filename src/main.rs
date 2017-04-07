extern crate amethyst;

use amethyst::{Application, Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::config::Element;
use amethyst::ecs::World;
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::Pipeline;


struct PlayState;

impl State for PlayState{
	fn on_start(&mut self, _: &mut World, _: &mut AssetManager, pipe: &mut Pipeline){
		use amethyst::renderer::pass::Clear;
		use amethyst::renderer::Layer;
		let clear_layer = Layer::new("main", vec![Clear::new([0.0, 0.0, 0.0, 1.0]),
			]);
		pipe.layers.push(clear_layer);
	}

	fn handle_events(&mut self, events: &[WindowEvent], _: &mut World, _: &mut AssetManager, _: &mut Pipeline) -> Trans{
		for e in events{
			match e.payload{
				Event::KeyboardInput(_,_,Some(VirtualKeyCode::Escape)) => return Trans::Quit,
				Event::Closed => return Trans::Quit,
				_ => (),
			}
		}

		Trans::None
	}
}


fn main() {
    let path = format!("{}/resources/config.yml", env!("CARGO_MANIFEST_DIR"));
    let cfg = DisplayConfig::from_file(path).expect("Could not find config!");
    let mut game = Application::build(PlayState, cfg).done();
    game.run();
}
