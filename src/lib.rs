#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod keyboardlayout;
pub use keyboardlayout::*;

mod angles;
pub use angles::Angle;
