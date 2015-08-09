#![feature(drain, default_type_parameter_fallback)]

extern crate rft;
pub use rft::Precision;

extern crate num;
extern crate strided;

mod util;

mod band;
pub use self::band::Band;

pub mod window;
pub use self::window::Window;

pub mod onset;
pub use self::onset::Onset;
