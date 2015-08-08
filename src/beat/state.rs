use Precision;
use util::Ring;

#[derive(Debug)]
pub struct State {
	pub fluxes:   Ring<Precision>,
	pub previous: Precision,
}

impl State {
	pub fn new(size: usize) -> Self {
		State {
			fluxes:   Ring::new(size + 1),
			previous: 0.0,
		}
	}
}

impl Default for State {
	fn default() -> Self {
		State::new(25)
	}
}
