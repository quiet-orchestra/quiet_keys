use crate::{
    Angle::Turns, KeyboardLayout, Rect, firmware_collection::*, key_output_collection::*,
    modifiers::*, physical_collection::*,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KeyList {
    pub metadata: String,
    pub physical_metadata: String,
    pub firmware_metadata: String,
    pub key_output_metadata: String,
    pub keylist: Vec<Key>,
    pub modifier_list: ModifierList,
}

impl KeyList {
    pub fn into_keyboard(self) -> KeyboardLayout {
        let modifier_list = self.modifier_list;

        let (mut switch_maps, mut hid_keycode_maps, mut key_output_maps) =
            (Vec::new(), Vec::new(), Vec::new());
        for key in self.keylist {
            switch_maps.push(key.switch_map);
            hid_keycode_maps.extend(key.hid_keycode_maps);
            key_output_maps.extend(key.key_output_maps);
        }
        let (physical_layout, firmware_layout, key_output_layout) = (
            PhysicalLayout {
                metadata: self.physical_metadata,
                switch_maps,
            },
            FirmwareLayout {
                metadata: self.firmware_metadata,
                hid_keycode_maps,
            },
            KeyOutputLayout {
                metadata: self.key_output_metadata,
                key_output_maps,
                modifier_list,
            },
        );
        KeyboardLayout {
            metadata: self.metadata,
            physical_layout,
            firmware_layout,
            key_output_layout,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Key {
    switch_map: SwitchMap,
    hid_keycode_maps: Vec<HIDKeycodeMap>,
    key_output_maps: Vec<KeyOutputMap>,
}

impl Key {
    pub fn quick_key(
        key_output: KeyOutput,
        rect: Rect,
        switch_id: u16,
        keycode: HIDKeycode,
    ) -> Self {
        Key {
            switch_map: SwitchMap {
                metadata: "".to_string(),
                switch_id,
                rect,
                secondary_rect: None,
                angle: Turns(0.),
                homing_nub: false,
            },
            hid_keycode_maps: vec![HIDKeycodeMap {
                switch_id,
                layer: 0,
                keycode,
            }],
            key_output_maps: vec![KeyOutputMap {
                keycode,
                key_output,
            }],
        }
    }
}
