use egui::{CentralPanel, Color32, Frame, Shadow, Ui, scroll_area};

use crate::{KeyboardLayout, keyboard_widget::{self, KeyboardWidget}, premade_layouts::ansi104};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct QuietKeysApp {

    
    keyboard_layout: KeyboardLayout,
    

    #[serde(skip)] // This how you opt-out of serialization of a field
    current_switch: u16,
    search_entry: String,

}

impl Default for QuietKeysApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            search_entry: "".to_owned(),
            keyboard_layout: ansi104::build_keyboard(),
            current_switch: 2,
        }
    }
}

impl QuietKeysApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

const REGION_CORNER_RADIUS: u8 = 6;
const REGION_PADDING: i8 = 12;
const CONTENT_CORNER_RADIUS: u8 = 3;
const CONTENT_PADDING: i8 = 2;

const KEYBOARD_BACKGROUND_COLOR: Color32 = Color32::from_rgb(190, 140, 180);
const KEYBOARD_CONTENT_COLOR: Color32 = Color32::from_rgb(200, 160, 200);
const EDITOR_BACKGROUND_COLOR: Color32 = Color32::from_rgb(190, 180, 140);
const EDITOR_CONTENT_COLOR: Color32 = Color32::from_rgb(200, 200, 160);
const DROP_SHADOW: Shadow = Shadow {
        offset: [0, 4],
        blur: 8,
        spread: 0,
        color: egui::Color32::from_black_alpha(80),
    };

impl eframe::App for QuietKeysApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let available_size = ui.available_size();
        let ribbon_height_ratio = 5;
        let key_editor_height_ratio = 45;

        let ribbon_height = ribbon_height_ratio as f32 * available_size.y / 100.;
        let key_editor_height = key_editor_height_ratio as f32 * available_size.y / 100.;

        egui::Panel::top("top_panel")
            .default_size(ribbon_height)
            .show(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!

                    ui.menu_button("File", |ui| {
                        // if ui.button("Quit").clicked() {
                        //     ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        // }
                    });
                    ui.menu_button("Edit", |ui| {
                        // if ui.button("Quit").clicked() {
                        //     ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        // }
                    });

                
            
                egui::widgets::global_theme_preference_buttons(ui);
            });
            ui.take_available_space();
        });
        

        egui::Panel::bottom("settings panel")
        .exact_size(key_editor_height)
        .show(ui, |ui| {
            render_region(ui, EDITOR_BACKGROUND_COLOR, EDITOR_CONTENT_COLOR, |ui|{
                    ui.heading("Keyboard Settings");

                    ui.horizontal(|ui| {
                        ui.label("Write something: ");
                        ui.text_edit_singleline(&mut self.search_entry);
                    });

                    ui.add(egui::Slider::new(&mut self.current_switch, 0..=10).text("value"));
                    if ui.button("Increment").clicked() {
                        self.current_switch += 1;
                    }

                    ui.separator();


                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        powered_by_egui_and_eframe(ui);
                        egui::warn_if_debug_build(ui);
                    });
            });
                    
        });
     

        
        CentralPanel::default()
        .show(ui, |ui| {
           render_region(ui, KEYBOARD_BACKGROUND_COLOR, KEYBOARD_CONTENT_COLOR, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {

                        KeyboardWidget::render( ui, &self.keyboard_layout);
                        ui.take_available_space();
                    });
           });     
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn render_region(ui: &mut egui::Ui, background_color: Color32, content_color: Color32, content:  impl FnOnce(&mut Ui)){
    Frame::new()
    .fill(background_color)
    .corner_radius(REGION_CORNER_RADIUS)
    .inner_margin(REGION_PADDING)
    .show(ui, |ui|{
        Frame::new()
        .fill(content_color)
        .corner_radius(CONTENT_CORNER_RADIUS)
        .inner_margin(CONTENT_PADDING)
        .shadow(DROP_SHADOW)
        .show(ui, |ui| {
            content(ui)
        });
    });
}