use serde::{Deserialize, Serialize};
use crate::keyboard_layout::is_default;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Label {
    #[serde(skip_serializing_if = "is_default")]
    pub custom_text: Option<String>,
    #[serde(skip_serializing_if = "is_true")]
    pub visible: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub font: Font,
    #[serde(skip_serializing_if = "is_default_font_size")]
    pub font_size: f32,
    pub position: LabelPosition,
}

fn is_true(boolean: &bool) -> bool {
    *boolean
}

impl Label {
    pub fn with_label(self, label: String) -> Self {
        Label { custom_text: Some(label), ..self }
    }
    pub fn with_position(self, position: LabelPosition) -> Self {
        Label { position, ..self }
    }
    pub fn with_visiblity(self, visible: bool) -> Self {
        Label { visible, ..self }
    }
}

impl Default for Label {
    fn default() -> Self {
        Label {
            custom_text: None,
            visible: true,
            font: Font::Default,
            font_size: 12.,
            position: LabelPosition::default(),
        }
    }
}

fn is_default_font_size(size: &f32) -> bool {
    *size == 12.0
}

/*
   _________
  |\T̲L̲\T̲C̲\T̲R̲\
  \ \M̲L̲\M̲C̲\M̲R̲\
   \ \B̲L̲\L̲C̲\B̲R̲\
    \|F̲L̲|F̲C̲|F̲R̲|

*/
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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
        Self::MiddleCenter
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Font {
    Default,
    Named(String),
}

impl Default for Font {
    fn default() -> Self {
        Self::Default
    }
}