pub use rft::{Sample, SampleMut};
pub use rft::window::Function as Filter;
pub use rft::window::Range;

use strided::MutStrided;
use rft;

use super::Channels;

#[derive(Clone, Debug)]
pub struct Window<S: SampleMut> {
	buffer: Vec<S>,
	size:   usize,
	rate:   u32,

	hop:    Option<usize>,
	filter: Option<rft::Window<S>>,
}

pub type Result = ::std::result::Result<Channels, ()>;

impl<S: SampleMut> Window<S> {
	#[inline(always)]
	pub fn new(size: usize, rate: u32) -> Self {
		Window {
			buffer: Vec::with_capacity(size * 4),
			size:   size,
			rate:   rate,

			hop:    None,
			filter: None,
		}
	}

	#[inline(always)]
	pub fn with_hop(mut self, hop: usize) -> Self {
		self.hop = Some(hop);
		self
	}

	#[inline(always)]
	pub fn with_filter<F: Filter, R: Range>(mut self, range: R) -> Self {
		self.filter = Some(rft::window::generate::<F, _, _>(range, self.size));
		self
	}

	#[inline(always)]
	pub fn size(&self) -> usize {
		self.size
	}

	#[inline(always)]
	pub fn rate(&self) -> u32 {
		self.rate
	}

	#[inline(always)]
	pub fn hop(&self) -> Option<usize> {
		self.hop
	}

	#[inline(always)]
	pub fn push(&mut self, samples: &[S]) {
		self.buffer.extend(samples.iter().cloned());
	}

	pub fn next(&mut self) -> Result {
		// Check we have enough samples in the buffer.
		if self.buffer.len() < self.size() * 2 {
			return Err(());
		}

		let (mono, left, right) = {
			// Get 2N samples since we expect packed stereo.
			let samples = &mut self.buffer[0 .. self.size * 2];
	
			// Our samples are stereo packed signed shorts, so split the two channels.
			let (mut left, mut right) = samples.as_stride_mut().substrides2_mut();
		
			// The two channels are averaged to get a mono channel, this might not be the
			// best way to do mono, but it works.
			let mut mono = Vec::<S>::with_capacity(self.size);
				
			// Average left and right channel to get the mono.
			for (left, right) in left.iter().zip(right.iter()) {
				let mut value = S::zero();
				value.set_normalized((left.normalize() + right.normalize()) / 2.0);

				mono.push(value);
			}
		
			// Apply the filter if it's enabled.
			if let Some(filter) = self.filter.as_ref() {
				filter.apply_on(&mut *mono);
	
				// We cannot apply the hamming in-place for left and right when we're
				// hopping.
				if self.hop.is_some() {
					(rft::forward(mono),
					 rft::forward(filter.apply::<f64, _, _>(left)),
					 rft::forward(filter.apply::<f64, _, _>(right)))
				}
				else {
					filter.apply_on(left.reborrow());
					filter.apply_on(right.reborrow());
	
					(rft::forward(mono),
					 rft::forward(left),
					 rft::forward(right))
				}
			}
			else {
				(rft::forward(mono),
				 rft::forward(left),
				 rft::forward(right))
			}
		};

		// Drain the hop size if present or the whole size, this implements
		// hopping.
		self.buffer.drain(0 .. self.hop.unwrap_or(self.size) * 2);

		Ok(Channels::new(mono, left, right))
	}
}
