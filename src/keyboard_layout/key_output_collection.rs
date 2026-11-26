use serde::{Deserialize, Serialize};
use crate::{
    firmware_collection::HIDKeycode, 
    labels::*, 
    modifiers::*, 
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct KeyOutputLayout {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub metadata: String,
    pub key_output_maps: Vec<KeyOutputMap>,
    pub modifier_list: ModifierList,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyOutputMap {
    pub keycode: HIDKeycode,
    pub key_output: KeyOutput,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KeyOutputType {
    TextOutputKey(Vec<TextOutput>),
    ModifierKey {
        modifier_index: ModifierIndex,
        labels: Vec<Label>,
    },
    ActionKey(Label),
}

impl KeyOutputType {
    pub fn to_key_output(self) -> KeyOutput {
        KeyOutput::Single(self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KeyOutput {
    Single(KeyOutputType),
    Locktype([KeyOutputType; 2]),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TextOutput {
    pub output_text: String,
    #[serde(skip_serializing_if = "is_base_modifier_mask")]
    pub modifier_mask: ModifierMask,
    pub label: Label,
}

impl TextOutput {
    pub(crate) fn simple_text_output(
        text: impl Into<String>,
        is_shifted: bool,
        label_visible: bool,
        label_position: LabelPosition,
    ) -> TextOutput {
        let output_text = text.into();
        let modifier_mask = if is_shifted {
            ModifierMask::shift()
        } else {
            ModifierMask::base()
        };

        let label = Label::default()
            .with_visiblity(label_visible)
            .with_position(label_position);

        TextOutput {
            output_text,
            modifier_mask,
            label,
        }
    }
}
