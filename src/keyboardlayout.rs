//Keyboard Layout rs
//Created by Quiet Orchestra
//Licensed under MPL-2.0

use serde::{Deserialize, Serialize};

use crate::{Angle::{self, Turns}};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KeyboardLayout {
    pub metadata: String,
    pub physical_layout: PhysicalLayout,
    pub firmware_layout: FirmwareLayout,
    pub key_output_layout: KeyOutputLayout,
}

impl KeyboardLayout {
    pub fn from_keylist(keylist: KeyList) -> Self {
        let modifier_list = keylist.modifier_list;

        let (mut switch_maps , mut  hid_keycode_maps, mut  key_output_maps ) = (Vec::new(), Vec::new(), Vec::new());
        for key in keylist.keylist {
            switch_maps.push(key.switch_map);
            hid_keycode_maps.extend(key.hid_keycode_maps);
            key_output_maps.extend(key.key_output_maps);
        };
        let (physical_layout , firmware_layout, key_output_layout ) = (
            PhysicalLayout {
                metadata: keylist.physical_metadata,
                switch_maps,
            },
            FirmwareLayout {
                metadata: keylist.firmware_metadata,
                hid_keycode_maps,
            },
            KeyOutputLayout {
                metadata: keylist.key_output_metadata,
                key_output_maps,
                modifier_list,
            },
        );

        KeyboardLayout { 
            metadata: keylist.metadata, 
            physical_layout, 
            firmware_layout, 
            key_output_layout, 
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PhysicalLayout {
    pub metadata: String,
    pub switch_maps: Vec<SwitchMap>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FirmwareLayout {
    pub metadata: String,
    pub hid_keycode_maps: Vec<HIDKeycodeMap>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KeyOutputLayout {
    pub metadata: String,
    pub key_output_maps: Vec<KeyOutputMap>,
    pub modifier_list: ModifierList,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KeyList {
    pub metadata: String,
    pub physical_metadata: String,
    pub firmware_metadata: String,
    pub key_output_metadata: String,
    pub keylist: Vec<Key>,
    pub modifier_list: ModifierList,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KeyboardKeyBuilder {
    switch_counter: u16,
    key_rect: Rect,
    one_label_position: LabelPosition,
    two_labels_position: [LabelPosition; 2],
    horizontal_offset: f32,
    modifier_list: ModifierList,
}

impl KeyboardKeyBuilder {
    pub fn new() -> Self{
        let mut keyboard_builder = KeyboardKeyBuilder::default();

        keyboard_builder.one_label_position = LabelPosition::MiddleLeft;
        keyboard_builder.two_labels_position = [LabelPosition::BottomLeft, LabelPosition::TopLeft];

        keyboard_builder.set_height(1.);
        keyboard_builder.set_width(1.);

        keyboard_builder
    }

    //Generic fn all quick keys must use
    fn quick_key(&mut self, key_output: KeyOutput, keycode_id: u16) -> Key {
        //Construct rect and HIDKeycode
        let rect = self.key_rect.clone();
        let keycode = HIDKeycode::make_keyboard_code(keycode_id);

        //Make key
        let key = Key::quick_key(
            key_output,
            rect,
            self.switch_counter,
            keycode,
        );

        //Move builder
        self.move_next();

        //Return Key
        return key;
    }

    fn key_with_dimensions<KeyFunction>(&mut self, key_function: KeyFunction, width: f32, height: f32) -> Key
    where 
        KeyFunction: FnOnce(&mut Self) -> Key,
    {
        //retain dimensions
        let prev_width = self.width();
        let prev_height = self.height();
        //adjust dimensions
        self.set_width(width);
        self.set_height(height);
        //construct key
        let key = key_function(self);
        //Revert dimensions
        self.set_width(prev_width);
        self.set_height(prev_height);

        return key;
    }

    //Used for making aphabetic and symbolic keys with two text outputs
    fn two_text_key(&mut self, text: impl Into<String>, text_two: impl Into<String>, keycode_id: u16, alphabetic: bool) -> Key {
        //Construct key items 
        let text_output_one = TextOutput::simple_text_output(text, false, !alphabetic, self.two_labels_position[0]);// If alphabetic don't show the lower label
        let text_output_two = TextOutput::simple_text_output(text_two, true, true, self.two_labels_position[1]);
        let key_output = KeyOutputType::TextOutputs(vec![text_output_one, text_output_two]).to_key_output();     

        self.quick_key(key_output,keycode_id)
    }

    pub fn quick_key_symbolic(&mut self, text: impl Into<String>, text_two: impl Into<String>, keycode_id: u16) -> Key {
        self.two_text_key(text, text_two, keycode_id, false)
    }

    pub fn quick_key_alphabetic(&mut self, text: impl Into<String>, text_two: impl Into<String>, keycode_id: u16) -> Key {
        self.two_text_key(text, text_two, keycode_id, true)
    }

    pub fn quick_key_modifier(&mut self, label: impl Into<String>, modifier_index: ModifierIndex, keycode_id: u16) -> Key {
        let label = label.into();
        //Construct label and then modifier
        let key_label = LabelAttribute::default().with_label(label.clone()).with_position(LabelPosition::MiddleLeft);
        let key_output = KeyOutputType::Modifier { modifier_index, labels: vec![key_label]}.to_key_output();

        //Register modifier
        self.modifier_list.register_modifier_with_index(label, modifier_index);

        self.quick_key(key_output, keycode_id)
 
    }

    pub fn quick_key_modifier_with_width(&mut self, label: impl Into<String>, modifier_index: ModifierIndex, keycode_id: u16, width: f32) -> Key {
        self.key_with_dimensions(|builder|builder.quick_key_modifier(label, modifier_index, keycode_id), width, self.height())
    }

    pub fn quick_key_action(&mut self, label: impl Into<String>, keycode_id: u16) -> Key {
        //Construct label and then modifier
        let key_label = LabelAttribute::default().with_label(label.into()).with_position(self.one_label_position);
        let key_output = KeyOutputType::Action(key_label).to_key_output();

        self.quick_key(key_output, keycode_id)

    }

    pub fn quick_key_action_with_width(&mut self, label: impl Into<String>, keycode_id: u16, width: f32) -> Key {
        self.key_with_dimensions(|builder|builder.quick_key_action(label, keycode_id), width, self.height())
    }
    pub fn quick_key_action_with_height(&mut self, label: impl Into<String>, keycode_id: u16, height: f32) -> Key {
        self.key_with_dimensions(|builder|builder.quick_key_action(label, keycode_id), self.width(), height)
    }
    pub fn quick_key_action_with_dimension(&mut self, label: impl Into<String>, keycode_id: u16, width: f32, height: f32) -> Key {
        self.key_with_dimensions(|builder|builder.quick_key_action(label, keycode_id), width, height)
    }

    pub fn quick_key_numpad_and_action(&mut self, text: impl Into<String>, action_label: impl Into<String>, keycode_id: u16) -> Key {
        let output_text = text.into();
        let numpad_label = LabelAttribute::default().with_label(output_text.clone()).with_position(self.two_labels_position[1]);
        let action_label = LabelAttribute::default().with_label(action_label.into()).with_position(self.two_labels_position[0]);

        let numpad_text_output = KeyOutputType::TextOutputs(vec![TextOutput{ output_text, modifier_mask: ModifierMask::base(), label: numpad_label}]);
        let numpad_action = KeyOutputType::Action(action_label);
        
        let key_output = KeyOutput::Locktype([numpad_text_output, numpad_action]);

        self.quick_key(key_output, keycode_id)
    }

    pub fn quick_key_single(&mut self, text: impl Into<String>, keycode_id: u16) -> Key {
        self.single_text_key(text, keycode_id)
    }

    fn single_text_key(&mut self, text: impl Into<String>, keycode_id: u16) -> Key {
        //Construct key items 
        let text_output_one = TextOutput::simple_text_output(text, false, true, self.two_labels_position[1]);
        let key_output = KeyOutputType::TextOutputs(vec![text_output_one]).to_key_output();     

        self.quick_key(key_output,keycode_id)
    }

    pub fn set_width(&mut self, width: f32){
        self.key_rect.width = width;
    }

    pub fn set_height(&mut self, height: f32){
        self.key_rect.height = height;
    }

    pub fn new_row(&mut self){
        self.set_x(self.horizontal_offset);
        self.push_y(self.height());
    }

    pub fn reset_with_horizontal_offset(&mut self, offset: f32){
        self.set_y(0.);
        self.set_x(offset);
        self.horizontal_offset = offset;
    }

    pub fn insert_space(&mut self, width: f32){
        self.push_x(width);
    }

    pub fn insert_vertical_space(&mut self, height: f32){
        self.push_y(height);
    }

    fn move_next(&mut self){
        self.push_x(self.width());
    }

    pub fn position(&self) -> Point {
        self.key_rect.get_top_left()
    }

    pub fn set_position(&mut self, point: Point){
        self.key_rect.x = point.x;
        self.key_rect.y = point.y;
    }

    fn push_x(&mut self, value: f32){
        self.key_rect.x += value;
    }

    fn push_y(&mut self, value: f32){
        self.key_rect.y += value;
    }

    fn width(&self) -> f32 {
        self.key_rect.width
    }

    fn height(&self) -> f32 {
        self.key_rect.height
    }

    fn set_x(&mut self, value: f32){
        self.key_rect.x = value;
    }

    fn set_y(&mut self, value: f32){
        self.key_rect.y = value;
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Key {
    switch_map: SwitchMap,
    hid_keycode_maps: Vec<HIDKeycodeMap>,
    key_output_maps: Vec<KeyOutputMap>,
}

impl Key {
    fn quick_key(key_output: KeyOutput , rect: Rect, switch_id: u16, keycode: HIDKeycode) -> Self {        
        Key { 
            switch_map: SwitchMap { 
                metadata: "". to_string(), 
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchMap {
    pub metadata: String, //Where you put things like switch type at least for now
    pub switch_id: u16,
    pub rect: Rect,
    pub secondary_rect: Option<Rect>,
    pub angle: Angle<f32>,
    pub homing_nub: bool,
    // Possibly more to add
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32, 
    pub y: f32, 
    pub width: f32, 
    pub height: f32, 
}

impl Rect {
    pub fn within_rect(&self, point: Point) -> bool {
        point.x > self.x && 
        point.x < self.x2() &&
        point.y > self.y &&
        point.y < self.y2()
    }

    fn x2(&self) -> f32{
        self.x + self.width
    }

    fn y2(&self) -> f32{
        self.y + self.height
    }

    pub fn get_top_left(&self) -> Point{
        Point { x: self.x, y: self.y }
    }

    pub fn get_top_right(&self) -> Point{
        Point { x: self.x2(), y: self.y }
    }

    pub fn get_bottom_left(&self) -> Point{
        Point { x: self.x, y: self.y2() }
    }

    pub fn get_bottom_right(&self) -> Point{
        Point { x: self.x2(), y: self.y2() }
    }

    pub fn get_center(&self) -> Point{
        Point { x: self.x + (self.width / 2.) , y: self.y + (self.height / 2.)}
    }

    pub fn scale_from_center(self, width_scale: f32, height_scale: f32) -> Self {
        let center = self.get_center();
        let Rect { x: _, y: _, width, height } = self;

        let width = width * width_scale;
        let height = height * height_scale;

        let x = center.x - (width / 2.);
        let y: f32 = center.y - (height / 2.);
        return Rect { x, y, width, height };
    }

}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct HIDKeycodeMap {
    pub switch_id: u16,
    pub keycode: HIDKeycode,
    pub layer: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyOutputMap {
    pub keycode: HIDKeycode,
    pub key_output: KeyOutput,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KeyOutputType {
    TextOutputs (Vec<TextOutput>),
    Modifier{
        modifier_index: ModifierIndex,
        labels: Vec<LabelAttribute>,
    },
    Action(LabelAttribute),
}

impl KeyOutputType {
    pub fn to_key_output(self) -> KeyOutput{
        KeyOutput::Single(self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KeyOutput{
    Single(KeyOutputType),
    Locktype([KeyOutputType;2]),
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TextOutput {
    pub output_text: String,
    pub modifier_mask: ModifierMask,
    pub label: LabelAttribute,
}

impl TextOutput{
    fn simple_text_output(text: impl Into<String>, is_shifted: bool, label_visible: bool, label_position: LabelPosition) -> TextOutput{
        let output_text = text.into();
        let modifier_mask= 
            if is_shifted {
                ModifierMask::shift()
            } 
            else {
                ModifierMask::base()
            };

        let label_text = output_text.clone();
        let label = LabelAttribute::default().with_label(label_text).with_visiblity(label_visible).with_position(label_position);

        TextOutput { 
            output_text, 
            modifier_mask, 
            label, 
        }
    }
}

//Modifier Structs
const MODIFIER_COUNT: usize = 8;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModifierMask(u8);

impl ModifierMask {
    pub const fn base() -> Self{
        return Self(0b0000_0000);
    }
    pub const fn shift() -> Self{
        return Self(0b0000_0001);
    }
    pub const fn alt_gr() -> Self{
        return Self(0b0000_0010);
    }
    pub const fn shift_alt_gr() -> Self{
        return Self(0b0000_0011);
    }

    pub fn get_indices_from_mask(&self) -> Vec<ModifierIndex>{
        let mut index_vec = Vec::new();
        for index in 0..MODIFIER_COUNT {
            if self.0 >> index & 1 == 1 {
                index_vec.push(ModifierIndex(index));
            }
        }
        return index_vec;
    }

}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]//Need to validate the usize as less that MODIFIER_COUNT during deserialization.
pub struct ModifierIndex(usize);
impl ModifierIndex {
    pub fn from_usize(index: usize) -> Option<Self> {
        if index < MODIFIER_COUNT {
            Some(ModifierIndex(index))
        }
        else{
            None
        }
    }
    pub fn to_mask(&self) -> ModifierMask{
        ModifierMask(1u8 << self.0)
    }
    pub const fn shift() -> Self{
        return Self(0);
    }
    pub const fn alt_gr() -> Self{
        return Self(1);
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModifierList{modifier_list: [Option<String>;MODIFIER_COUNT]}

impl ModifierList {

    pub fn get_index(&self, requested_modifier: &str) -> Option<ModifierIndex>{
        let modifier_index = None;
        for index in 0..MODIFIER_COUNT {
            if let Some(modifier) = &self.modifier_list[index] {
                if modifier.eq(requested_modifier) {
                    return Some(ModifierIndex(index));
                }
            }
        }
        return modifier_index;
    }

    pub fn get_mask(&self) -> ModifierMask{
        let mut mask = 0u8;
        for index in 0..MODIFIER_COUNT {
            if self.modifier_list[index].is_some() {
                mask |= 1 << index;
            }
        }
        return ModifierMask(mask);
    }

    fn get_modifier(&self, index: ModifierIndex) -> Option<String> {
        return self.modifier_list[index.0].clone();
    }

    pub fn get_modifiers_and_indices(&self) -> Vec<(String, ModifierIndex)> {
        let mask = self.get_mask();
        let indices_vec: Vec<ModifierIndex> = mask.get_indices_from_mask();
        indices_vec.iter().map(|index| {
            let modifier = self.get_modifier(*index).expect("Modifier should be guranteed from mask.");//Is verified to be there.
            (modifier,*index)
        }).collect()
    }

    pub fn register_modifier_with_index(&mut self, modifier: impl Into<String>, index: ModifierIndex){
        self.modifier_list[index.0] = Some(modifier.into());    
    }

    pub fn register_shift(&mut self){
        self.register_modifier_with_index("Shift", ModifierIndex::shift());
    }
    pub fn register_alt_gr(&mut self){
        self.register_modifier_with_index("AltGr", ModifierIndex::alt_gr());
    }

    pub fn register_modifier(&mut self, modifier: impl Into<String>) -> bool{
        let modifier = modifier.into();
        if self.get_index(&modifier).is_some(){true}
        else{
            if let Some(index) = self.get_first_empty_index() {
                self.register_modifier_with_index(modifier, index);
                true
            }
            else {false}
        }
    }

    fn get_first_empty_index(&self) -> Option<ModifierIndex> {
        for index in 0..MODIFIER_COUNT{
            if self.modifier_list[index].is_none() {
                return Some(ModifierIndex(index));
            }
        }
        return None;
    }
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LabelAttribute {
    pub label: String,
    pub visible: bool,
    pub font: Font,
    pub font_size: f32,
    pub position: LabelPosition,
}

impl LabelAttribute {
    fn with_label(self, label: String) -> Self {
        LabelAttribute { label, ..self }
    }
    fn with_position(self, position: LabelPosition) -> Self {
        LabelAttribute { position, ..self }
    }
    fn with_visiblity(self, visible: bool) -> Self {
        LabelAttribute { visible, ..self }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct HIDKeycode {
    pub page: u16,
    pub id: u16,
}

impl HIDKeycode {
    fn make_keyboard_code(id: u16) -> Self{
        HIDKeycode {
            page: 0x07,
            id
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Font {
    Default,
    Named(String),
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
        Self::TopLeft
    }
}

//Default keyboard

pub fn make_default_ansi_104_keyboard() -> KeyboardLayout {

    let mut builder = KeyboardKeyBuilder::new();

    let key_esc = builder.quick_key_action("ESC", 41);
    builder.insert_space(1.0);
    let key_f1 = builder.quick_key_action("F1", 58 );
    let key_f2 = builder.quick_key_action("F2", 59 );
    let key_f3 = builder.quick_key_action("F3", 60 );
    let key_f4 = builder.quick_key_action("F4", 61 );
    builder.insert_space(0.5);
    let key_f5 = builder.quick_key_action("F5", 62 );
    let key_f6 = builder.quick_key_action("F6", 63 );
    let key_f7 = builder.quick_key_action("F7", 64 );
    let key_f8 = builder.quick_key_action("F8", 65 );
    builder.insert_space(0.5);
    let key_f9 = builder.quick_key_action("F9", 66 );
    let key_f10 = builder.quick_key_action("F10", 67 );
    let key_f11 = builder.quick_key_action("F11", 68 );
    let key_f12 = builder.quick_key_action("F12", 69 );

    builder.insert_space(0.25);

    let key_prtsc = builder.quick_key_action("PrtSc", 70 );
    let key_scroll_lock = builder.quick_key_action("Scroll\nLock", 71 );
    let key_pause_break = builder.quick_key_action("Pause\n\nBreak", 72 );

    builder.new_row();

    let key_grave_tilde = builder.quick_key_symbolic("`", "~", 53);
    let key_one_bang = builder.quick_key_symbolic("1", "!", 30);
    let key_two_at = builder.quick_key_symbolic("2", "@", 31);
    let key_three_hash = builder.quick_key_symbolic("3", "#", 32);
    let key_four_dollar = builder.quick_key_symbolic("4", "$", 33);
    let key_five_percent = builder.quick_key_symbolic("5", "%", 34);
    let key_six_caret = builder.quick_key_symbolic("6", "^", 35);
    let key_seven_ampersand = builder.quick_key_symbolic("7", "&", 36);
    let key_eight_star = builder.quick_key_symbolic("8", "*", 37);
    let key_nine_left_perenthesis = builder.quick_key_symbolic("9", "(", 38);
    let key_zero_right_parenthesis = builder.quick_key_symbolic("0", ")", 39);
    let key_dash_underscore = builder.quick_key_symbolic("-", "_", 45);
    let key_equals_plus = builder.quick_key_symbolic("=", "+", 46);
    let key_backspace = builder.quick_key_action_with_width("Backspace", 42, 2.);

    builder.insert_space(0.25);

    let key_insert = builder.quick_key_action("Insert", 73);
    let key_home = builder.quick_key_action("Home", 74);
    let key_pgup = builder.quick_key_action("PgUp", 75);
   

    builder.new_row();

    let key_tab = builder.quick_key_action_with_width("Tab", 43, 1.5);
    let key_q = builder.quick_key_alphabetic( "q","Q", 20);
    let key_w = builder.quick_key_alphabetic( "w", "W",26);
    let key_e = builder.quick_key_alphabetic( "e", "E",8);
    let key_r = builder.quick_key_alphabetic( "r", "R",21);
    let key_t = builder.quick_key_alphabetic( "t", "T",23);
    let key_y = builder.quick_key_alphabetic( "y", "Y",28);
    let key_u = builder.quick_key_alphabetic( "u", "U",24);
    let key_i = builder.quick_key_alphabetic( "i", "I",12);
    let key_o = builder.quick_key_alphabetic( "o", "O",18);
    let key_p = builder.quick_key_alphabetic( "p", "P",19);
    let key_left_bracket_left_curly_brace = builder.quick_key_symbolic( "[","{", 0x2f);
    let key_right_bracket_right_curly_brace = builder.quick_key_symbolic( "]","}", 0x30);
    let key_backslash_pipe = builder.quick_key_symbolic( "\\","|", 0x31);
    let key_delete = builder.quick_key_action("Delete", 0x4C);
    let key_end = builder.quick_key_action("End", 0x4D);
    let key_pgdn = builder.quick_key_action("PgDn", 0x4E);

    builder.new_row();

    let key_capslock = builder.quick_key_action_with_width("CapsLock", 0x39, 1.75);
    let key_a = builder.quick_key_alphabetic("a", "A", 4);
    let key_s = builder.quick_key_alphabetic("s", "S", 22);
    let key_d = builder.quick_key_alphabetic("d", "D", 7);
    let key_f = builder.quick_key_alphabetic("f", "F", 9);
    let key_g = builder.quick_key_alphabetic("g", "G", 10);
    let key_h = builder.quick_key_alphabetic("h", "H", 11);
    let key_j = builder.quick_key_alphabetic("j", "J", 13);
    let key_k = builder.quick_key_alphabetic("k", "K", 14);
    let key_l = builder.quick_key_alphabetic("l", "L", 15);
    let key_semicolon_colon = builder.quick_key_symbolic( ";",":", 0x33);
    let key_single_quote_double_quote = builder.quick_key_symbolic( "\'","\"", 0x34);
    let key_enter = builder.quick_key_action_with_width("Enter", 40, 1.5);

    builder.new_row();

    let key_shift_left = builder.quick_key_modifier_with_width("Shift", ModifierIndex::shift(), 0xE1, 2.25);
    let key_z = builder.quick_key_alphabetic("z", "Z", 29);
    let key_x = builder.quick_key_alphabetic("x", "X", 27);
    let key_c = builder.quick_key_alphabetic("c", "C", 6);
    let key_v = builder.quick_key_alphabetic("v", "V", 25);
    let key_b = builder.quick_key_alphabetic("b", "B", 5);
    let key_n = builder.quick_key_alphabetic("n", "N", 17);
    let key_m = builder.quick_key_alphabetic("m", "M", 16);
    let key_comma_left_angle_bracket = builder.quick_key_symbolic( ",","<", 0x36);
    let key_period_right_angle_bracket = builder.quick_key_symbolic( ".",">", 0x37);
    let key_forward_slash_question = builder.quick_key_symbolic( "/","?", 0x38);
    let key_shift_right = builder.quick_key_modifier_with_width("Shift", ModifierIndex::shift(), 0xE5, 2.75);
    builder.insert_space(1.25);
    let key_up_arrow = builder.quick_key_action("↑", 0x52);

    builder.new_row();

    builder.set_width(1.25);
    let key_ctrl_left = builder.quick_key_action("Ctrl", 0xE0);
    let key_super_left = builder.quick_key_action("Super", 0xE3);
    let key_alt_left = builder.quick_key_action("Alt", 0xE2);
    builder.set_width(6.25);
    let key_space = builder.quick_key_action("", 0x2C);
    builder.set_width(1.25);
    let key_alt_right = builder.quick_key_action("Alt", 0xE6);
    let key_super_right = builder.quick_key_action("Super", 0xE7);
    let key_menu = builder.quick_key_action("Menu", 0x76);
    let key_ctrl_right = builder.quick_key_action("Ctrl", 0xE4);
    builder.set_width(1.);
    builder.insert_space(0.25);
    let key_left_arrow = builder.quick_key_action("←", 80);
    let key_down_arrow = builder.quick_key_action("↓", 81);
    let key_right_arrow = builder.quick_key_action("→", 79);


    //keypad
    builder.reset_with_horizontal_offset(18.5);
    let keypad_num_lock = builder.quick_key_action("Num\nLock", 0x53);
    let keypad_forward_slash = builder.quick_key_single("/", 84);
    let keypad_star = builder.quick_key_single("*", 85);
    let keypad_dash = builder.quick_key_single("-", 86);

    builder.new_row();

    let keypad_seven_home = builder.quick_key_numpad_and_action("7", "Home", 0x5F);
    let keypad_eight_up_arrow = builder.quick_key_numpad_and_action("8", "↑", 0x60);
    let keypad_nine_pgup = builder.quick_key_numpad_and_action("9", "PgUp", 0x61);
    builder.set_height(2.);
    let keypad_plus = builder.quick_key_single("+", 0x57);
    builder.set_height(1.);

    builder.new_row();

    let keypad_four_left_arrow = builder.quick_key_numpad_and_action("4", "←", 0x5C);
    let keypad_five = builder.quick_key_single("5", 0x5D);
    let keypad_six_right_arrow = builder.quick_key_numpad_and_action("6", "→", 0x5E);

    builder.new_row();

    let keypad_one_end = builder.quick_key_numpad_and_action("1", "End", 0x59);
    let keypad_two_down_arrow = builder.quick_key_numpad_and_action("2", "↓", 0x5A);
    let keypad_three_pgdn = builder.quick_key_numpad_and_action("3", "PgDn", 0x5B);
    let keypad_enter = builder.quick_key_action_with_height("Enter", 0x58, 2.);

    builder.new_row();
    
    let keypad_zero_ins = builder.quick_key_numpad_and_action("0", "Ins", 98);
    let keypad_period_del = builder.quick_key_numpad_and_action(".", "Del", 99);


    let default_ansi_104_keys: Vec<Key> = 
    vec![
        key_esc, key_f1, key_f2, key_f3, key_f4, key_f5, key_f6, key_f7, key_f8, key_f9, key_f10, key_f11, key_f12, key_prtsc, key_scroll_lock, key_pause_break,
        key_grave_tilde, key_one_bang, key_two_at, key_three_hash, key_four_dollar, key_five_percent, key_six_caret, key_seven_ampersand, key_eight_star, key_nine_left_perenthesis, key_zero_right_parenthesis, key_dash_underscore, key_equals_plus, key_backspace, key_insert, key_home, key_pgup, keypad_num_lock, keypad_forward_slash, keypad_star, keypad_dash,
        key_tab, key_q, key_w, key_e, key_r, key_t, key_y, key_u, key_i, key_o, key_p, key_left_bracket_left_curly_brace, key_right_bracket_right_curly_brace, key_backslash_pipe, key_delete, key_end, key_pgdn, keypad_seven_home, keypad_eight_up_arrow, keypad_nine_pgup, keypad_plus,
        key_capslock, key_a, key_s, key_d, key_f, key_g, key_h, key_j, key_k, key_l, key_semicolon_colon, key_single_quote_double_quote, key_enter, keypad_four_left_arrow, keypad_five, keypad_six_right_arrow,
        key_shift_left, key_z, key_x, key_c, key_v, key_b, key_n, key_m, key_comma_left_angle_bracket, key_period_right_angle_bracket, key_forward_slash_question, key_shift_right, key_up_arrow, keypad_one_end, keypad_two_down_arrow, keypad_three_pgdn, keypad_enter,
        key_ctrl_left, key_super_left, key_alt_left, key_space, key_alt_right, key_super_right, key_menu, key_ctrl_right, key_left_arrow, key_down_arrow, key_right_arrow, keypad_zero_ins, keypad_period_del
    ];

    let default_ansi_104_keylist: KeyList = KeyList{
        metadata: "Standard ANSI 104 Keyboard". to_string(),
        physical_metadata: "Standard ANSI 104 Keyboard Physical Switch Layout Map". to_string(),
        firmware_metadata: "Standard ANSI 104 Keyboard Keycode Layout Map". to_string(),
        key_output_metadata: "Standard ANSI 104 Keyboard Character Layout Map". to_string(),
        keylist: default_ansi_104_keys,
        modifier_list: builder.modifier_list,
    };

    KeyboardLayout::from_keylist(default_ansi_104_keylist)
}

#[test]
fn print_default_ansi_104_keyboard_json() {
    let keyboard = make_default_ansi_104_keyboard();
    let json = serde_json::to_string_pretty(&keyboard)
        .expect("failed to serialize keyboard");

    std::fs::write("keyboard.json", json)
        .expect("failed to write keyboard.json");
}
