#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod keyboard_layout;
pub use keyboard_layout::*;

mod keyboard_geometry;
pub use keyboard_geometry::{Angle, Rect, Point, angle_is_zero};

