use rft::{self, SampleMut};

use {Band, Precision};
use window::{Window, Channel};
use util::Ring;
use onset::Peak;
use onset::spectral_flux::SpectralFlux;
use onset::threshold::Threshold;

pub struct Onset<T: Default + Clone = ()>  {
	size:  usize,
	rate:  u32,

	state: Vec<State<T>>,
}

pub struct State<T: Default + Clone> {
	band:      Band<T>,
	spectral:  SpectralFlux,
	threshold: Threshold,
	fluxes:    Ring<Precision>,
}

pub type Result<T> = ::std::result::Result<Peak<T>, Peak<T>>;

impl<T: Default + Clone> Onset<T> {
	pub fn new<S: SampleMut>(window: &Window<S>) -> Self {
		Onset {
			size:  window.size(),
			rate:  window.rate(),

			state: Vec::new(),
		}
	}

	pub fn with_band(mut self, band: Band<T>, threshold: Option<(usize, Precision)>) -> Self {
		self.state.push(State {
			spectral: SpectralFlux::new(
				rft::spectrum::index_for(band.high(), self.size, self.rate) -
				rft::spectrum::index_for(band.low(), self.size, self.rate)),

			threshold: if let Some((size, sensitivity)) = threshold {
				Threshold::new(size, sensitivity)
			}
			else {
				Threshold::default()
			},

			fluxes: if let Some((size, _)) = threshold {
				Ring::new(size + 1)
			}
			else {
				Ring::new(25 + 1)
			},

			band: band,
		});

		self
	}

	// Initialize the no bands case.
	fn initialize(&mut self) {
		self.state.push(State {
			band: Band::new(0, self.rate / 2),

			spectral: SpectralFlux::new(
				rft::spectrum::index_for(self.rate / 2, self.size, self.rate)),

			threshold: Threshold::default(),

			fluxes: Ring::new(25 + 1),
		});
	}

	pub fn analyze(&mut self, channel: &Channel) -> Vec<Result<T>> {
		if self.state.is_empty() {
			self.initialize();
		}

		let mut result = Vec::new();
		let spectrum   = rft::spectrum::compute(&**channel);

		for &mut State { ref band, ref mut spectral, ref mut threshold, ref mut fluxes } in self.state.iter_mut() {
			// Get the start as index for the spectrum.
			let start = rft::spectrum::index_for(band.low(), self.size, self.rate);

			// Get the end as index for the spectrum.
			let end = rft::spectrum::index_for(band.high(), self.size, self.rate);

			// Compute the flux for the specified part of the spectrum.
			let flux = spectral.compute(&spectrum[start .. end]);

			// Cache the flux.
			fluxes.push(flux);

			// Update the threshold with the new flux.
			threshold.push(flux);

			// Check we have enough sample windows to calculate the threshold.
			if !threshold.is_enough() {
				continue;
			}

			let current             = *fluxes.front().unwrap();
			let (offset, threshold) = threshold.current();
			let seconds             = (1.0 / self.rate as f64) * (offset as f64 * self.size as f64);
			let peak                = Peak::new(band.clone(), seconds, threshold, current);

			// We have an outlier!
			if current > threshold {
				result.push(Ok(peak));
			}
			else {
				result.push(Err(peak));
			}
		}

		result
	}
}
