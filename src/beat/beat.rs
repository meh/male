use rft::{self, SampleMut};

use {Band, Precision};
use window::{Window, Channel};
use super::{SpectralFlux, Threshold, State};

#[derive(Debug)]
pub struct Beat<T: Clone + Default = ()>  {
	size:  usize,
	rate:  u32,

	band:      Vec<Band<T>>,
	spectral:  Vec<SpectralFlux>,
	threshold: Vec<Threshold>,
	state:     Vec<State>,
}

impl<T: Clone + Default> Beat<T> {
	pub fn new<S: SampleMut>(window: &Window<S>) -> Self {
		Beat {
			size:  window.size(),
			rate:  window.rate(),

			band:      Vec::new(),
			spectral:  Vec::new(),
			threshold: Vec::new(),
			state:     Vec::new(),
		}
	}

	pub fn with_band(mut self, band: Band<T>, threshold: Option<(usize, Precision)>) -> Self {
		self.spectral.push(SpectralFlux::new(
			rft::spectrum::index_for(band.high(), self.size, self.rate) -
			rft::spectrum::index_for(band.low(), self.size, self.rate)));

		if let Some((size, sensitivity)) = threshold {
			self.state.push(State::new(size));
			self.threshold.push(Threshold::new(size, sensitivity));
		}
		else {
			self.state.push(State::default());
			self.threshold.push(Threshold::default());
		}

		self.band.push(band);

		self
	}

	// Initialize the no bands case.
	fn initialize(&mut self) {
		let size = rft::spectrum::index_for(self.rate / 2, self.size, self.rate);

		self.band.push(Band::new(
			0, self.rate / 2));

		self.spectral.push(SpectralFlux::new(
			size));

		self.threshold.push(Threshold::default());

		self.state.push(State::default());
	}

	pub fn analyze(&mut self, channel: &Channel) -> Vec<(f64, Band<T>, Precision)> {
		if self.band.is_empty() {
			self.initialize();
		}

		let mut result = Vec::new();

		let spectrum = rft::spectrum::compute(&**channel);

		let band      = self.band.iter();
		let spectral  = self.spectral.iter_mut();
		let threshold = self.threshold.iter_mut();
		let state     = self.state.iter_mut();

		for (((band, spectral), threshold), state) in band.zip(spectral).zip(threshold).zip(state) {
			// Get the start as index for the spectrum.
			let start = rft::spectrum::index_for(band.low(), self.size, self.rate);

			// Get the end as index for the spectrum.
			let end = rft::spectrum::index_for(band.high(), self.size, self.rate);

			// Compute the flux for the specified part of the spectrum.
			let flux = spectral.compute(&spectrum[start .. end]);

			// Cache the flux.
			state.fluxes.push(flux);

			// Update the threshold with the new flux.
			threshold.push(flux);

			// Check we have enough sample windows to calculate the threshold.
			if !threshold.is_enough() {
				continue;
			}

			let current             = *state.fluxes.front().unwrap();
			let (offset, threshold) = threshold.current();

			// We have an outlier!
			if current > threshold {
				// Is it a beat?
				if state.previous > current {
					// The beat was actually in the previous sample.
					let time = (1.0 / self.rate as f64) * ((offset - 1) as f64 * self.size as f64);

					// Normalize the flux with the threshold.
					let flux = state.previous - threshold;

					// Add the peak.
					result.push((time, band.clone(), flux));
				}

				// Set the previous so we can get a new beat.
				state.previous = current - threshold;
			}
			else {
				// Reset the previous to 0 so we can get a new beat.
				state.previous = 0.0;
			}
		}

		result
	}
}
