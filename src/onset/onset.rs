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
	previous:  (Precision, Precision),
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

			previous: (0.0, 0.0),

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

			previous: (0.0, 0.0),
		});
	}

	pub fn analyze(&mut self, channel: &Channel) -> Vec<Result<T>> {
		if self.state.is_empty() {
			self.initialize();
		}

		let mut result = Vec::new();
		let spectrum   = rft::spectrum::compute(&**channel);

		for &mut State { ref band, ref mut spectral, ref mut threshold, ref mut fluxes, ref mut previous } in self.state.iter_mut() {
			// Compute the flux for the specified part of the spectrum.
			let flux = spectral.compute(band.as_slice(&spectrum, self.rate));

			// Cache the flux.
			fluxes.push(flux);

			// Update the threshold with the new flux.
			threshold.push(flux);

			// Check we have enough sample windows to calculate the threshold.
			if !threshold.is_enough() {
				continue;
			}

			// Get the current flux, `fluxes` contains `threshold.size + 1`, so the
			// front of `fluxes` corresponds to the center flux in the `threshold`.
			let current  = *fluxes.front().unwrap();

			// Get the current threshold and its offset in sample size.
			let (offset, threshold) = threshold.current();

			// Get the offset in seconds of the previous peak.
			let seconds = (1.0 / self.rate as f64) * ((offset - 1) as f64 * self.size as f64);

			// We have an outlier!
			if current > threshold {
				let peak = Peak::new(band.clone(), seconds, threshold, previous.0);

				// Check if the peak is still going or we reached it.
				if previous.1 > current {
					result.push(Ok(peak));
				}
				else {
					result.push(Err(peak));
				}

				previous.0 = current;
				previous.1 = current;
			}
			else {
				result.push(Err(Peak::new(band.clone(), seconds, threshold, previous.0)));

				previous.0 = current;
				previous.1 = 0.0;
			}
		}

		result
	}
}
