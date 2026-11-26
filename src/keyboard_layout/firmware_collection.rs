use num_traits::Zero;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FirmwareLayout {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub metadata: String,
    pub hid_keycode_maps: Vec<HIDKeycodeMap>,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HIDKeycodeMap {
    pub switch_id: u16,
    pub keycode: HIDKeycode,
    #[serde(skip_serializing_if = "is_zero")]
    pub layer: u8,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct HIDKeycode {
    #[serde(
        default = "default_keyboard_page",
        skip_serializing_if = "is_keyboard_page"
    )]
    pub page: u16,
    pub id: u16,
}

fn default_keyboard_page() -> u16 {
    0x07
}

impl Default for HIDKeycode {
    fn default() -> Self {
        HIDKeycode { page: 0x07, id: 0x00/*Is an error*/ }
    }
}

fn is_keyboard_page(value: &u16) -> bool {
    *value == 0x07
}

impl HIDKeycode {
    pub fn make_keyboard_hid_code(id: u16) -> Self{
        HIDKeycode {
            page: 0x07,
            id
        }
    }
}

fn is_zero<T: Zero>(value: &T) -> bool {
    value.is_zero()
}