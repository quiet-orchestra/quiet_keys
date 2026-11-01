//Keyboard Layout rs
//Created by Quiet Orchestra
//Licensed under MPL-2.0

use serde::Serialize;

use crate::{Angle::{self, Turns}};

#[derive(Clone, Debug, Default, Serialize)]
pub struct KeyboardLayout {
    pub metadata: String,
    pub physical_layout: PhysicalLayout,
    pub firmware_layout: FirmwareLayout,
    pub character_layout: CharacterLayout,
}

impl KeyboardLayout {
    pub fn from_keylist(keylist: KeyList) -> Self {

        let (mut switch_maps , mut  hid_keycode_maps, mut  key_output_maps ) = (Vec::new(), Vec::new(), Vec::new());
        for key in keylist.keylist {
            switch_maps.push(key.switch_map);
            hid_keycode_maps.push(key.hid_keycode_map);
            key_output_maps.push(key.key_output_map);
        };
        let (physical_layout , firmware_layout, character_layout ) = (
            PhysicalLayout {
                metadata: keylist.physical_metadata,
                switch_maps,
            },
            FirmwareLayout {
                metadata: keylist.firmware_metadata,
                hid_keycode_maps,
            },
            CharacterLayout {
                metadata: keylist.character_metadata,
                key_output_maps,
            },
        );

        KeyboardLayout { 
            metadata: keylist.metadata, 
            physical_layout, 
            firmware_layout, 
            character_layout, 
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PhysicalLayout {
    pub metadata: String,
    pub switch_maps: Vec<SwitchMap>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FirmwareLayout {
    pub metadata: String,
    pub hid_keycode_maps: Vec<HIDKeycodeMap>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CharacterLayout {
    pub metadata: String,
    pub key_output_maps: Vec<KeyOutputMap>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct KeyList {
    pub metadata: String,
    pub physical_metadata: String,
    pub firmware_metadata: String,
    pub character_metadata: String,
    pub keylist: Vec<Key>
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct KeyboardKeyBuilder {
    switch_counter: u16,
    position: Point,
    key_width: f32,
    key_height: f32,
}

impl KeyboardKeyBuilder {
    fn new() -> Self{
        KeyboardKeyBuilder::default()
    }
    fn quick_key_symbolic(&mut self, text: String, text_two: Option<String>, keycode_id: u16) {

    }
    fn quick_key_alphabetic(&mut self, text: String, text_two: Option<String>, keycode_id: u16) {

    }
    fn quick_key_modifier(&mut self, label: String, modifier: Modifier, keycode_id: u16) {

    }    
    fn quick_key_action(&mut self, label: String, keycode_id: u16) {

    }   
    fn set_width(&mut self, width: f32){
        self.key_width = width;
    }
    fn set_height(&mut self, height: f32){
        self.key_height = height;
    }
    fn new_row(&mut self){
        self.position.x = 0.;
        self.position.y += self.key_height;
    }

}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Key {
    switch_map: SwitchMap,
    hid_keycode_map: HIDKeycodeMap,
    key_output_map: KeyOutputMap,
}

impl Key {
    fn quick_key(text_one: String, text_two_and_visibility: Option<(String,bool)>, rect: Rect, rect_two: Option<Rect>, switch_id: u16, keycode: HIDKeycode) -> Self {
        let output_one = TextOutput{
            output_text: text_one.clone(),
            modifiers: ModifierComposition::default(),
            label: LabelAttributes::default().with_label(text_one),
        };

        let mut text_output = vec![output_one];

        if let Some((text_two, visibile)) = text_two_and_visibility {
            let label = if visibile {text_two.to_string()} else {"".to_owned()};
            let output_two = TextOutput{
                output_text: text_two.clone(),
                modifiers: ModifierComposition::default(),
                label: LabelAttributes::default().with_label(label),
            };
            text_output.push(output_two);
        }
        
        Key { 
            switch_map: SwitchMap { 
                metadata: "".to_owned(), 
                switch_id, 
                rect, 
                secondary_rect: rect_two,
                angle: Turns(0.), 
                homing_nub: false,
            }, 
            hid_keycode_map: HIDKeycodeMap { 
                switch_id, 
                keycode,
            }, 
            key_output_map: KeyOutputMap { 
                keycode, 
                key_output:  KeyOutput::TextOutputs(
                    text_output
                ),
            }, 
        }
    }
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Default, Serialize)]
pub struct Rect {
    pub x: f32, 
    pub y: f32, 
    pub w: f32, 
    pub h: f32, 
}

impl Rect {
    fn within_rect(&self, point: Point) -> bool {
        point.x > self.x && 
        point.x < self.x2() &&
        point.y > self.y &&
        point.y < self.y2()
    }

    fn x2(&self) -> f32{
        self.x + self.w
    }

    fn y2(&self) -> f32{
        self.y + self.h
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct HIDKeycodeMap {
    pub switch_id: u16,
    pub keycode: HIDKeycode,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct KeyOutputMap {
    pub keycode: HIDKeycode,
    pub key_output: KeyOutput,
}
#[derive(Clone, Debug, Serialize)]
pub enum KeyOutput {
    TextOutputs (Vec<TextOutput>),
    Modifier{
        modifier: Modifier,
        labels: Vec<LabelAttributes>,
    },
    Action(LabelAttributes),
}
impl Default for KeyOutput {
    fn default() -> Self {
        KeyOutput::TextOutputs(vec![TextOutput::default()])
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TextOutput {
    pub output_text: String,
    pub modifiers: ModifierComposition,
    pub label: LabelAttributes,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Modifier(u8);

#[derive(Clone, Debug, Default, Serialize)]
pub struct ModifierComposition (Vec<Modifier>);

#[derive(Clone, Debug, Default, Serialize)]
pub struct LabelAttributes {
    pub label: String,
    pub visible: bool,
    pub font: Font,
    pub font_size: f32,
    pub position: LabelPosition,
}

impl LabelAttributes {
    fn with_label(self, label: String) -> Self {
        LabelAttributes { label, ..self }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct HIDKeycode {
    pub page: u16,
    pub id: u16,
}

#[derive(Clone, Debug, Serialize)]
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
#[derive(Clone, Debug, Serialize)]
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

fn make_default_ansi_104_keyboard () -> KeyboardLayout {

    let KEY_ESC: Key::default;
    let KEY_F1
    let KEY_F2
    let KEY_F3
    let KEY_F4
    let KEY_F5
    let KEY_F6
    let KEY_F7
    let KEY_F8
    let KEY_F9
    let KEY_F10
    let KEY_F11
    let KEY_F12
    let KEY_PRTSC
    let KEY_SCROLL_LOCK
    let KEY_PAUSE_BREAK,
    let KEY_TILDE_BACKTICK
    let KEY_BANG_ONE
    let KEY_AT_TWO
    let KEY_HASH_THREE
    let KEY_DOLLAR_FOUR
    let KEY_PERCENT_FIVE
    let KEY_CARET_SIX
    let KEY_AMPERSAND_SEVEN
    let KEY_STAR_EIGHT
    let KEY_LEFT_PERENTHESIS_NINE
    let KEY_RIGHT_PARENTHESIS_ZERO
    let KEY_UNDERSCORE_DASH
    let KEY_PLUS_EQUALS
    let KEY_BACKSPACE
    let KEY_INSERT
    let KEY_HOME
    let KEY_PGUP
    let KEYPAD_NUM_LOCK
    let KEYPAD_FORWARD_SLASH
    let KEYPAD_STAR
    let KEYPAD_DASH,
    let KEY_TAB
    let KEY_Q
    let KEY_W
    let KEY_E
    let KEY_R
    let KEY_T
    let KEY_Y
    let KEY_U
    let KEY_I
    let KEY_O
    let KEY_P
    let KEY_LEFT_CURLY_BRACE_LEFT_BRACKET
    let KEY_RIGHT_CURLY_BRACE_RIGHT_BRACKET
    let KEY_PIPE_BACKSLASH
    let KEY_DELETE
    let KEY_END
    let KEY_PGDN
    let KEYPAD_SEVEN_HOME
    let KEYPAD_EIGHT_UP_ARROW
    let KEYPAD_NINE_PGUP
    let KEYPAD_PLUS,
    let KEY_CAPSLOCK
    let KEY_A
    let KEY_S
    let KEY_D
    let KEY_F
    let KEY_G
    let KEY_H
    let KEY_J
    let KEY_K
    let KEY_L
    let KEY_COLON_SEMICOLON
    let KEY_DOUBLE_QUOTE_SINGLE_QUOTE
    let KEY_ENTER
    let KEYPAD_FOUR_LEFT_ARROW
    let KEYPAD_FIVE
    let KEYPAD_SIX_RIGHT_ARROW,
    let KEY_SHIFT_LEFT
    let KEY_Z
    let KEY_X
    let KEY_C
    let KEY_V
    let KEY_B
    let KEY_N
    let KEY_M
    let KEY_LEFT_ANGLE_BRACKET_COMMA
    let KEY_RIGHT_ANGLE_BRACKET_PERIOD
    let KEY_QUESTION_FORWARD_SLASH
    let KEY_SHIFT_RIGHT
    let KEY_UP_ARROW
    let KEYPAD_ONE_END
    let KEYPAD_TWO_DOWN_ARROW
    let KEYPAD_THREE_PGDN
    let KEYPAD_ENTER,
    let KEY_CTRL_LEFT
    let KEY_SUPER_LEFT
    let KEY_ALT_LEFT
    let KEY_SPACE
    let KEY_ALT_RIGHT
    let KEY_SUPER_RIGHT
    let KEY_MENU
    let KEY_CTRL_RIGHT
    let KEY_LEFT_ARROW
    let KEY_DOWN_ARROW
    let KEY_RIGHT_ARROW
    let KEYPAD_ZERO_INS
    let KEYPAD_PERIOD_DEL

    let default_ansi_104_keys: Vec<Key> = 
    vec![
        KEY_ESC, KEY_F1, KEY_F2, KEY_F3, KEY_F4, KEY_F5, KEY_F6, KEY_F7, KEY_F8, KEY_F9, KEY_F10, KEY_F11, KEY_F12, KEY_PRTSC, KEY_SCROLL_LOCK, KEY_PAUSE_BREAK,
        KEY_TILDE_BACKTICK, KEY_BANG_ONE, KEY_AT_TWO, KEY_HASH_THREE, KEY_DOLLAR_FOUR, KEY_PERCENT_FIVE, KEY_CARET_SIX, KEY_AMPERSAND_SEVEN, KEY_STAR_EIGHT, KEY_LEFT_PERENTHESIS_NINE, KEY_RIGHT_PARENTHESIS_ZERO, KEY_UNDERSCORE_DASH, KEY_PLUS_EQUALS, KEY_BACKSPACE, KEY_INSERT, KEY_HOME, KEY_PGUP, KEYPAD_NUM_LOCK, KEYPAD_FORWARD_SLASH, KEYPAD_STAR, KEYPAD_DASH,
        KEY_TAB, KEY_Q, KEY_W, KEY_E, KEY_R, KEY_T, KEY_Y, KEY_U, KEY_I, KEY_O, KEY_P, KEY_LEFT_CURLY_BRACE_LEFT_BRACKET, KEY_RIGHT_CURLY_BRACE_RIGHT_BRACKET, KEY_PIPE_BACKSLASH, KEY_DELETE, KEY_END, KEY_PGDN, KEYPAD_SEVEN_HOME, KEYPAD_EIGHT_UP_ARROW, KEYPAD_NINE_PGUP, KEYPAD_PLUS,
        KEY_CAPSLOCK, KEY_A, KEY_S, KEY_D, KEY_F, KEY_G, KEY_H, KEY_J, KEY_K, KEY_L, KEY_COLON_SEMICOLON, KEY_DOUBLE_QUOTE_SINGLE_QUOTE, KEY_ENTER, KEYPAD_FOUR_LEFT_ARROW, KEYPAD_FIVE, KEYPAD_SIX_RIGHT_ARROW,
        KEY_SHIFT_LEFT, KEY_Z, KEY_X, KEY_C, KEY_V, KEY_B, KEY_N, KEY_M, KEY_LEFT_ANGLE_BRACKET_COMMA, KEY_RIGHT_ANGLE_BRACKET_PERIOD, KEY_QUESTION_FORWARD_SLASH, KEY_SHIFT_RIGHT, KEY_UP_ARROW, KEYPAD_ONE_END, KEYPAD_TWO_DOWN_ARROW, KEYPAD_THREE_PGDN, KEYPAD_ENTER,
        KEY_CTRL_LEFT, KEY_SUPER_LEFT, KEY_ALT_LEFT, KEY_SPACE, KEY_ALT_RIGHT, KEY_SUPER_RIGHT, KEY_MENU, KEY_CTRL_RIGHT, KEY_LEFT_ARROW, KEY_DOWN_ARROW, KEY_RIGHT_ARROW, KEYPAD_ZERO_INS, KEYPAD_PERIOD_DEL
    ];

    KeyboardLayout::from_keylist(DEFAULT_ANSI_104_KEYLIST)
}


const DEFAULT_ANSI_104_PHYSICAL_LAYOUT: PhysicalLayout = 
    DEFAULT_ANSI_104_KEYBOARD.physical_layout;

const DEFAULT_ANSI_104_KEYLIST: KeyList = KeyList{
    metadata: "Standard ANSI 104 Keyboard".to_owned(),
    physical_metadata: "Standard ANSI 104 Keyboard Physical Switch Layout Map".to_owned(),
    firmware_metadata: "Standard ANSI 104 Keyboard Keycode Layout Map".to_owned(),
    character_metadata: "Standard ANSI 104 Keyboard Character Layout Map".to_owned(),
    keylist: DEFAULT_ANSI_104_KEYS,
};





