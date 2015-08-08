use std::ops::Deref;
use num::Complex;

use Precision;

#[derive(Clone, Debug)]
pub struct Channels {
	mono:  Vec<Complex<Precision>>,
	left:  Vec<Complex<Precision>>,
	right: Vec<Complex<Precision>>,
}

impl Channels {
	#[inline(always)]
	pub fn new(mono: Vec<Complex<Precision>>,
	           left: Vec<Complex<Precision>>,
	           right: Vec<Complex<Precision>>)
		-> Self
	{
		Channels {
			mono:  mono,
			left:  left,
			right: right,
		}
	}

	#[inline(always)]
	pub fn mono(&self) -> Channel {
		Channel::Mono(&*self.mono)
	}

	#[inline(always)]
	pub fn left(&self) -> Channel {
		Channel::Left(&*self.left)
	}

	#[inline(always)]
	pub fn right(&self) -> Channel {
		Channel::Right(&*self.right)
	}
}

#[derive(Debug)]
pub enum Channel<'a> {
	Mono(&'a [Complex<Precision>]),
	Left(&'a [Complex<Precision>]),
	Right(&'a [Complex<Precision>]),
}

impl<'a> Deref for Channel<'a> {
	type Target = [Complex<Precision>];

	#[inline(always)]
	fn deref(&self) -> &[Complex<Precision>] {
		match self {
			&Channel::Mono(data)  => data,
			&Channel::Left(data)  => data,
			&Channel::Right(data) => data,
		}
	}
}
