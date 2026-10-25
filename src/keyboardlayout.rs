//Keyboard Layout rs
//Created by Quiet Orchestra
//Licensed under MPL-2.0

use crate::Angle;

#[derive(Clone, Debug, Default)]
pub struct KeyboardLayout {
    pub metadata: String,
    pub physical_layout: PhysicalLayout,
    pub firmware_layout: FirmwareLayout,
    pub character_layout: CharacterLayout,
}

#[derive(Clone, Debug, Default)]
pub struct PhysicalLayout {
    pub metadata: String,
    pub keys: Vec<PhysicalKeyMap>,
}

#[derive(Clone, Debug, Default)]
pub struct FirmwareLayout {
    pub metadata: String,
    pub keys: Vec<KeycodeMap>,
}

#[derive(Clone, Debug, Default)]
pub struct CharacterLayout {
    pub metadata: String,
    pub keys: Vec<CharacterMap>,
}

#[derive(Clone, Debug, Default)]
pub struct PhysicalKeyMap {
    pub metadata: String, //Where you put things like switch type at least for now
    pub key_id: u16,
    pub x: f32,
    pub y: f32,
    pub x2: f32,
    pub y2: f32,
    pub width: f32,
    pub height: f32,
    pub width2: f32,
    pub height2: f32,
    pub angle: Angle<f32>,
    pub homing_nub: bool,
    // Possibly more to add
}

#[derive(Clone, Debug, Default)]
pub struct KeycodeMap {
    pub key_id: u16,
    pub keycode: HIDKeycode,
}

#[derive(Clone, Debug, Default)]
pub struct CharacterMap {
    pub modifier: Vec<HIDKeycode>,
    pub keycode: HIDKeycode,
    pub character: String,
    pub label: LabelAttributes,
}

#[derive(Clone, Debug, Default)]
pub struct LabelAttributes {
    pub visible: bool,
    pub font: Font,
    pub position: LabelPosition,
}

#[derive(Clone, Debug, Default)]
pub struct HIDKeycode {
    pub page: u16,
    pub id: u16,
}

#[derive(Clone, Debug)]
pub enum Font {
    Default,
    Font(String),
}

impl Default for Font {
    fn default() -> Self {
        Self::Default
    }
}


/*
   _________
  |\T̲L̲\T̲C̲\T̲R̲\
  \ \M̲L̲\M̲C̲\M̲R̲\
   \ \B̲L̲\L̲C̲\B̲R̲\
    \|F̲L̲|F̲C̲|F̲R̲|

*/
#[derive(Clone, Debug)]
pub enum LabelPosition {
    TopLeft = 0,
    TopCenter = 1,
    TopRight = 2,
    
    MiddleLeft = 3,
    MiddleCenter = 4,
    MiddleRight = 5,
    
    BottomLeft = 6,
    BottomCenter = 7,
    BottomRight = 8,
    
    FrontLeft = 9,
    FrontCenter = 10,
    FrontRight = 11,
}

impl Default for LabelPosition {
    fn default() -> Self {
        Self::TopLeft
    }
}