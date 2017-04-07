use amethyst::ecs::{VecStorage, Component};


pub struct Movement{
	pub velocity: [f32; 2],
}

impl Component for Movement{
	type Storage = VecStorage<Movement>;
}


pub struct Player;

impl Player{
	pub fn new() -> Player{
		Player
	}
}

impl Component for Player{
	type Storage = VecStorage<Player>;
}