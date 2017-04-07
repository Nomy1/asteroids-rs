use amethyst::{Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::{World};
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::renderer::{Pipeline, VertexPosNormal};


pub struct PlayState;

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