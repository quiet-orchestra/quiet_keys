use egui::{Color32, Pos2, Stroke, Ui,};
use crate::{keyboard_layout::*, physical_collection::SwitchMap};

pub struct KeyboardWidget{

}

impl KeyboardWidget {
    pub fn render(ui: &mut Ui, keyboard_layout: &KeyboardLayout) {
        ui.label(format!("Keyboard Layout {}", keyboard_layout.metadata));
        let painter = ui.painter();
        let mut render_rect = ui.max_rect();
        render_rect.set_top(render_rect.top() + render_rect.height() - ui.available_height());
        let (width, height) = (render_rect.width(), render_rect.height());

        let center = render_rect.center();
        let rect = egui::Rect::from_center_size(center, ui.available_size());
        painter.circle(center, 50., Color32::from_black_alpha(20), Stroke::new(2., Color32::from_rgb(0, 50, 200)));
        painter.rect(rect, 0, Color32::TRANSPARENT, Stroke::new(2., Color32::BLACK), egui::StrokeKind::Middle);
        painter.rect(render_rect, 0, Color32::TRANSPARENT, Stroke::new(2., Color32::BLUE), egui::StrokeKind::Middle);
    }
}

