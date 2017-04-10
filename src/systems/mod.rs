use amethyst::ecs::{System, Join, RunArg};
use components::*;


pub struct VelocitySystem;
unsafe impl Sync for VelocitySystem{}

impl System<()> for VelocitySystem{
	fn run(&mut self, arg: RunArg, _: ()){
		use amethyst::ecs::resources::Time;
		use amethyst::ecs::components::LocalTransform;
		use cgmath::{Deg, Euler, Quaternion};

		let (mut locals, mut velocity, time) = arg.fetch(|w| {(
			w.write::<LocalTransform>(),
			w.write::<Velocity>(),
			w.read_resource::<Time>()
		)});

		let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

		for(local, v) in (&mut locals, &mut velocity).iter(){
			local.translation[0] += v.linear[0] * delta_time;
			local.translation[1] += v.linear[1] * delta_time;
 
 			let lr = local.rotation;
			let curr_rot = Quaternion::new(lr[0], lr[1], lr[2], lr[3]);
			let next_rot = Quaternion::from(Euler{ x: Deg(0.0), y: Deg(0.0), z: Deg(delta_time * v.angular)});
			let new_rot = curr_rot * next_rot;
			local.rotation[0] = new_rot[0];
			local.rotation[1] = new_rot[1];
			local.rotation[2] = new_rot[2];
			local.rotation[3] = new_rot[3];
		}
	}
}
/*
pub struct PlayerInputSystem;
unsafe impl Sync for PlayerInputSystem;

impl System<()> for PlayerInputSystem{
	fn run(&mut self, arg: RunArg, _: ()){
		use amethyst::ecs::resources::{Time, InputHandler};

		let (input, time
	}
}
*/