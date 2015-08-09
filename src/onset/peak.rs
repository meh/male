use {Band, Precision};

#[derive(Clone, Debug)]
pub struct Peak<T: Default + Clone> {
	band:      Band<T>,
	offset:    f64,
	threshold: Precision,
	flux:      Precision,
}

impl<T: Default + Clone> Peak<T> {
	pub fn new(band: Band<T>, offset: f64, threshold: Precision, flux: Precision) -> Self {
		Peak {
			band:      band,
			offset:    offset,
			threshold: threshold,
			flux:      flux,
		}
	}

	pub fn offset(&self) -> f64 {
		self.offset
	}

	pub fn band(&self) -> &Band<T> {
		&self.band
	}

	pub fn threshold(&self) -> Precision {
		self.threshold
	}

	pub fn flux(&self) -> Precision {
		self.flux
	}
}
