
use serde::{Deserialize, Serialize};
use crate::geometry::{Angle, Rect, angle_is_zero};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PhysicalLayout {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub metadata: String,
    pub switch_maps: Vec<SwitchMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SwitchMap {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub metadata: String, //Where you put things like switch type at least for now
    pub switch_id: u16,

    pub rect: Rect,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_rect: Option<Rect>,
    #[serde(skip_serializing_if = "angle_is_zero")]
    pub angle: Angle<f32>,
    #[serde(skip_serializing_if = "is_false")]
    pub homing_nub: bool,
    // Possibly more to add
}

fn is_false(boolean: &bool) -> bool {
    !boolean
}

impl Default for SwitchMap {
    fn default() -> Self {
        SwitchMap { 
            metadata: "".to_string(), 
            switch_id: 0, 
            rect: Rect::default(), 
            secondary_rect: None, 
            angle: Angle::Turns(0.), 
            homing_nub: false, 
        }
    }
}