use amethyst::ecs::{VecStorage, Component};


pub struct Velocity{
	pub linear: [f32; 2],
	pub angular: f32,
}

impl Velocity{
	pub fn new(linear: [f32; 2], angular: f32) -> Velocity{
		Velocity{
			linear: linear,
			angular: angular,
		}
	}
}

impl Default for Velocity{
	fn default() -> Velocity {
		Velocity{
			linear: [0.0, 0.0],
			angular: 0.0,
		}
	}
}

impl Component for Velocity{
	type Storage = VecStorage<Velocity>;
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

/*
pub struct PlayerInput{
	pub left : bool,
	pub right : bool,
	pub forward : bool,
}

impl Component for PlayerInput{
	type Storage = VecStorage<PlayerInput>;
}*/