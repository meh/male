use std::collections::VecDeque;
use std::ops::Deref;

#[derive(Debug)]
pub struct Ring<T> {
	buffer: VecDeque<T>,
	size:   usize,
}

impl<T> Ring<T> {
	#[inline(always)]
	pub fn new(size: usize) -> Self {
		Ring {
			buffer: VecDeque::with_capacity(size),
			size:   size,
		}
	}

	#[inline(always)]
	pub fn push(&mut self, value: T) -> Option<T> {
		if self.buffer.len() >= self.size {
			let result = self.pop();
			self.buffer.push_back(value);
			result
		}
		else {
			self.buffer.push_back(value);
			None
		}
	}

	#[inline(always)]
	pub fn pop(&mut self) -> Option<T> {
		self.buffer.pop_front()
	}
}

impl<T> Deref for Ring<T> {
	type Target = VecDeque<T>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.buffer
	}
}
