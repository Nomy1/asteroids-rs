use amethyst::ecs::{VecStorage, Component};


struct Movement{
	pub velocity: [f32; 2],
}

impl Component for Movement{
	type Storage = VecStorage<Movement>;
}