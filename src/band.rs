use std::ops::{Deref, DerefMut};

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
