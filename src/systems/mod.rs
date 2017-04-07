use amethyst::ecs::{System, Join, RunArg};
use components::*;


pub struct MovementSystem;
unsafe impl Sync for MovementSystem{}

impl System<()> for MovementSystem{
	fn run(&mut self, arg: RunArg, _: ()){
		use amethyst::ecs::resources::Time;
		use amethyst::ecs::components::LocalTransform;

		let (mut locals, mut movements, time) = arg.fetch(|w| {(
			w.write::<LocalTransform>(),
			w.write::<Movement>(),
			w.read_resource::<Time>()
		)});

		let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

		for(local, movement) in (&mut locals, &mut movements).iter(){
			local.translation[0] += movement.velocity[0] * delta_time;
			local.translation[1] += movement.velocity[1] * delta_time;
		}
	}
}
