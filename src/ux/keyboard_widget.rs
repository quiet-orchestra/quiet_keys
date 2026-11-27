use egui::Ui;
use crate::{keyboard_layout::*, physical_collection::SwitchMap};

pub struct KeyboardWidget{

}

impl KeyboardWidget {
    pub fn render(ui: &mut Ui, keyboard_layout: &KeyboardLayout) {
        ui.label(format!("Keyboard Layout {}", keyboard_layout.metadata));
    }
}
