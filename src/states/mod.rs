use amethyst::{Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::{World};
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::renderer::{Pipeline, VertexPosNormal};
use components::*;

pub struct PlayState;

impl State for PlayState{
	fn on_start(&mut self, world: &mut World, assets: &mut AssetManager, pipe: &mut Pipeline){
		use amethyst::renderer::pass::{Clear, DrawFlat};
		use amethyst::renderer::Layer;
		use amethyst::ecs::resources::{Camera, InputHandler, Projection, ScreenDimensions};


		{// create clear render pass and flat (no shading) render pass
			let render_layer = Layer::new("main", 
			vec![Clear::new([0.0, 0.0, 0.0, 1.0]),
				 DrawFlat::new("main_camera", "drawflat_camera")
			]);

			pipe.layers.push(render_layer);
		}

		{// create Camera resource
			let dim = world.read_resource::<ScreenDimensions>();
			let mut camera = world.write_resource::<Camera>();
			let aspect_ratio = dim.aspect_ratio;
			let eye = [0., 0., 0.1];
			let target = [0., 0., 0.];
			let up = [0., 1., 0.];
			let scale = 100.0;

			let proj = Projection::Orthographic{
					left: -1.0 * scale * aspect_ratio,
					right: 1.0 * scale* aspect_ratio,
					bottom: -1.0 * scale,
					top: 1.0 * scale,
					near: 0.0,
					far: 1.0,
				};

			camera.proj = proj;
			camera.eye = eye;
			camera.target = target;
			camera.up = up;
		}

		// create simple square asset
		assets.register_asset::<Mesh>();
		assets.register_asset::<Texture>();
		assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
		assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", gen_square(3.0, 3.0));
		let square = assets.create_renderable("square", "white", "white", "white", 1.0).unwrap();
	
		// create Player and place in the World (game)
		let mut player = Player::new();
		world.create_now()
			.with(square.clone())
			.with(player)
			.with(LocalTransform::default())
			.with(Transform::default())
			.with(Velocity::new([0.0, 0.0], 50.0))
			.build();
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

fn gen_square(w: f32, h: f32) -> Vec<VertexPosNormal>{
	vec![VertexPosNormal {
              pos: [-w / 2., -h / 2., 0.],
              normal: [0., 0., 1.],
              tex_coord: [0., 0.],
          },
          VertexPosNormal {
              pos: [w / 2., -h / 2., 0.],
              normal: [0., 0., 1.],
              tex_coord: [1., 0.],
          },
          VertexPosNormal {
              pos: [w / 2., h / 2., 0.],
              normal: [0., 0., 1.],
              tex_coord: [1., 1.],
          },
          VertexPosNormal {
              pos: [w / 2., h / 2., 0.],
              normal: [0., 0., 1.],
              tex_coord: [1., 1.],
          },
          VertexPosNormal {
              pos: [-w / 2., h / 2., 0.],
              normal: [0., 0., 1.],
              tex_coord: [1., 1.],
          },
          VertexPosNormal {
              pos: [-w / 2., -h / 2., 0.],
              normal: [0., 0., 1.],
              tex_coord: [1., 1.],
          }]
}