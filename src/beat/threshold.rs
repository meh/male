use Precision;
use util::Ring;

#[derive(Debug)]
pub struct Threshold {
	size:        usize,
	sensitivity: Precision,

	offset: usize,
	fluxes: Ring<Precision>,
}

impl Threshold {
	#[inline(always)]
	pub fn new(size: usize, sensitivity: Precision) -> Self {
		Threshold {
			size:        size,
			sensitivity: sensitivity,

			offset: 0,
			fluxes: Ring::new(size * 2 + 1),
		}
	}

	#[inline(always)]
	pub fn is_enough(&self) -> bool {
		self.fluxes.len() >= self.size * 2 + 1
	}

	pub fn push(&mut self, flux: Precision) {
		self.fluxes.push(flux);

		if self.is_enough() {
			self.offset += 1
		}
	}

	pub fn current(&mut self) -> (usize, Precision) {
		let average = self.fluxes.iter().fold(0.0, |acc, &n| acc + n)
			/ self.fluxes.len() as Precision;

		(self.size + self.offset, self.sensitivity * average)
	}
}

impl Default for Threshold {
	fn default() -> Self {
		Threshold::new(25, 1.5)
	}
}
