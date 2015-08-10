use std::ops::{Deref, DerefMut};
use rft;

use {Precision};

#[derive(Clone, Debug)]
pub struct Band<T: Clone + Default = ()> {
	low:  u32,
	high: u32,

	private: T,
}

impl<T: Clone + Default> Band<T> {
	#[inline(always)]
	pub fn new(low: u32, high: u32) -> Self {
		Band {
			low:  low,
			high: high,

			private: Default::default(),
		}
	}

	#[inline(always)]
	pub fn with<U: Clone + Default>(self, value: U) -> Band<U> {
		Band {
			low:  self.low,
			high: self.high,

			private: value,
		}
	}

	#[inline(always)]
	pub fn low(&self) -> u32 {
		self.low
	}

	#[inline(always)]
	pub fn high(&self) -> u32 {
		self.high
	}

	pub fn as_slice<'a, S: AsRef<[Precision]>>(&self, spectrum: &'a S, rate: u32) -> &'a [Precision] {
		let slice = spectrum.as_ref();
		let start = rft::spectrum::index_for(self.low, slice.len(), rate);
		let end   = rft::spectrum::index_for(self.high, slice.len(), rate);

		&slice[start .. end]
	}
}

impl<T: Clone + Default> PartialEq for Band<T> {
	fn eq(&self, other: &Band<T>) -> bool {
		self.low == other.low && self.high == other.high
	}
}

impl<T: Clone + Default> Deref for Band<T> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &T {
		&self.private
	}
}

impl<T: Clone + Default> DerefMut for Band<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T {
		&mut self.private
	}
}
