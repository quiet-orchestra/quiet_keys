//Keyboard Layout rs
//Created by Quiet Orchestra
//Licensed under MPL-2.0

pub use serde::{Deserialize, Serialize};

use crate::firmware_collection::FirmwareLayout;
use crate::key_output_collection::KeyOutputLayout;
use crate::physical_collection::PhysicalLayout;

//Exports
pub mod premade_layouts;
pub mod builder;
pub mod labels;
pub mod modifiers;
pub mod keys;
pub mod physical_collection;
pub mod firmware_collection;
pub mod key_output_collection;
pub mod geometry;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct KeyboardLayout {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub metadata: String,
    pub physical_layout: PhysicalLayout,
    pub firmware_layout: FirmwareLayout,
    pub key_output_layout: KeyOutputLayout,
}

//Serializer skip functions
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &T::default()
}


