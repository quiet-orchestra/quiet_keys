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
    pub key_output_layout: KeyOutputLayout,
}

impl KeyboardLayout {
    pub fn from_keylist(keylist: KeyList) -> Self {

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
pub struct KeyOutputLayout {
    pub metadata: String,
    pub key_output_maps: Vec<KeyOutputMap>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct KeyList {
    pub metadata: String,
    pub physical_metadata: String,
    pub firmware_metadata: String,
    pub key_output_metadata: String,
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
    pub fn new() -> Self{
        KeyboardKeyBuilder::default()
    }
    pub fn quick_key_symbolic(&mut self, text: impl Into<String>, text_two: impl Into<String>, keycode_id: u16) -> Key {
        todo!()
    }
    pub fn quick_key_numpad_math(&mut self, text: impl Into<String>, keycode_id: u16) -> Key {
        todo!()
    }
    pub fn quick_key_alphabetic(&mut self, text: impl Into<String>, text_two: impl Into<String>, keycode_id: u16) -> Key {
        todo!()
    }
    pub fn quick_key_modifier(&mut self, label: impl Into<String>, modifier: Modifier, keycode_id: u16) -> Key {
        todo!()
    }    
    pub fn quick_key_action(&mut self, label: impl Into<String>, keycode_id: u16) -> Key {
        todo!()
    }   
    pub fn set_width(&mut self, width: f32){
        self.key_width = width;
    }
    pub fn set_height(&mut self, height: f32){
        self.key_height = height;
    }
    pub fn new_row(&mut self){
        self.position.x = 0.;
        self.position.y += self.key_height;
    }

}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Key {
    switch_map: SwitchMap,
    hid_keycode_maps: Vec<HIDKeycodeMap>,
    key_output_maps: Vec<KeyOutputMap>,
}

impl Key {
    fn quick_key(text_one: TextOutput, text_two: Option<TextOutput>, rect: Rect, rect_two: Option<Rect>, switch_id: u16, keycode: HIDKeycode, layer: u8) -> Self {

        let mut text_output = vec![text_one];
        if let Some(text_two) = text_two {
            text_output.push(text_two);
        }
        
        Key { 
            switch_map: SwitchMap { 
                metadata: "". to_string(), 
                switch_id, 
                rect, 
                secondary_rect: rect_two,
                angle: Turns(0.), 
                homing_nub: false,
            }, 
            hid_keycode_maps: vec![HIDKeycodeMap { 
                switch_id, 
                layer,
                keycode,
            }], 
            key_output_maps: vec![KeyOutputMap { 
                keycode, 
                key_output:  KeyOutput::TextOutputs(
                    text_output
                ),
            }], 
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
    pub layer: u8,
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
    fn with_position(self, position: LabelPosition) -> Self {
        LabelAttributes { position, ..self }
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

    let builder = KeyboardKeyBuilder::new();

    let KEY_ESC = builder.quick_key_action("ESC". to_string(), 41);
    let KEY_F1 = builder.quick_key_action("F1". to_string(), 58 );
    let KEY_F2 = builder.quick_key_action("F2". to_string(), 59 );
    let KEY_F3 = builder.quick_key_action("F3". to_string(), 60 );
    let KEY_F4 = builder.quick_key_action("F4". to_string(), 61 );
    let KEY_F5 = builder.quick_key_action("F5". to_string(), 62 );
    let KEY_F6 = builder.quick_key_action("F6". to_string(), 63 );
    let KEY_F7 = builder.quick_key_action("F7". to_string(), 64 );
    let KEY_F8 = builder.quick_key_action("F8". to_string(), 65 );
    let KEY_F9 = builder.quick_key_action("F9". to_string(), 66 );
    let KEY_F10 = builder.quick_key_action("F10". to_string(), 67 );
    let KEY_F11 = builder.quick_key_action("F11". to_string(), 68 );
    let KEY_F12 = builder.quick_key_action("F12". to_string(), 69 );
    let KEY_PRTSC = builder.quick_key_action("PrtSc". to_string(), 70 );
    let KEY_SCROLL_LOCK =           builder.quick_key_action("Scroll\nLock". to_string(), 71 );
    let KEY_PAUSE_BREAK =           builder.quick_key_action("Pause\n\nBreak". to_string(), 72 );

    builder.new_row();

    let KEY_TILDE_GRAVE =           builder.quick_key_symbolic("~", "`", 53);
    let KEY_BANG_ONE =              builder.quick_key_symbolic("!", "1", 30);
    let KEY_AT_TWO =                builder.quick_key_symbolic("@", "2", 31);
    let KEY_HASH_THREE =            builder.quick_key_symbolic("#", "3", 32);
    let KEY_DOLLAR_FOUR =           builder.quick_key_symbolic("$", "4", 33);
    let KEY_PERCENT_FIVE =          builder.quick_key_symbolic("%", "5", 34);
    let KEY_CARET_SIX =             builder.quick_key_symbolic("^", "6", 35);
    let KEY_AMPERSAND_SEVEN =       builder.quick_key_symbolic("&", "7", 36);
    let KEY_STAR_EIGHT =            builder.quick_key_symbolic("*", "8", 37);
    let KEY_LEFT_PERENTHESIS_NINE = builder.quick_key_symbolic("(", "9", 38);
    let KEY_RIGHT_PARENTHESIS_ZERO = builder.quick_key_symbolic(")","1", 39);
    let KEY_UNDERSCORE_DASH =       builder.quick_key_symbolic("_","-", 45);
    let KEY_PLUS_EQUALS =           builder.quick_key_symbolic("+","=", 46);
    let KEY_BACKSPACE = builder.quick_key_action("Backspace", 42);
    let KEY_INSERT = builder.quick_key_action("Insert", 73);
    let KEY_HOME = builder.quick_key_action("Home", 74);
    let KEY_PGUP = builder.quick_key_action("PgUp", 75);
    //
    let KEYPAD_NUM_LOCK = builder.quick_key_action("Num\nLock", 131);
    //
    let KEYPAD_FORWARD_SLASH = builder.quick_key_numpad_math("/", 84);
    let KEYPAD_STAR = builder.quick_key_numpad_math("*", 85);
    let KEYPAD_DASH = builder.quick_key_numpad_math("-", 86);

    builder.new_row();

    let KEY_TAB = builder.quick_key_action("Tab", 43);
    let KEY_Q = builder.quick_key_alphabetic("Q", "q", 20);
    let KEY_W = builder.quick_key_alphabetic("w", "w", 26);
    let KEY_E = builder.quick_key_alphabetic("E", "e", 8);
    let KEY_R = builder.quick_key_alphabetic("R", "r", 21);
    let KEY_T = builder.quick_key_alphabetic("T", "t", 23);
    let KEY_Y = builder.quick_key_alphabetic("Y", "y", 28);
    let KEY_U = builder.quick_key_alphabetic("U", "u", 24);
    let KEY_I = builder.quick_key_alphabetic("I", "i", 12);
    let KEY_O = builder.quick_key_alphabetic("O", "o", 18);
    let KEY_P = builder.quick_key_alphabetic("P", "p", 19);
    let KEY_LEFT_CURLY_BRACE_LEFT_BRACKET = builder.quick_key_symbolic("{", "[", keycode_id);
    let KEY_RIGHT_CURLY_BRACE_RIGHT_BRACKET = 
    let KEY_PIPE_BACKSLASH = 
    let KEY_DELETE = 
    let KEY_END = 
    let KEY_PGDN = 
    let KEYPAD_SEVEN_HOME = 
    let KEYPAD_EIGHT_UP_ARROW = 
    let KEYPAD_NINE_PGUP = 
    let KEYPAD_PLUS, = 

    builder.new_row();

    let KEY_CAPSLOCK = builder.quick_key_action("Home", 74);
    let KEY_A = builder.quick_key_alphabetic("a", "a", 4);
    let KEY_S = builder.quick_key_alphabetic("S", "s", 22);
    let KEY_D = builder.quick_key_alphabetic("D", "d", 7);
    let KEY_F = builder.quick_key_alphabetic("F", "f", 9);
    let KEY_G = builder.quick_key_alphabetic("G", "g", 10);
    let KEY_H = builder.quick_key_alphabetic("H", "h", 11);
    let KEY_J = builder.quick_key_alphabetic("J", "j", 13);
    let KEY_K = builder.quick_key_alphabetic("K", "k", 14);
    let KEY_L = builder.quick_key_alphabetic("L", "l", 15);
    let KEY_COLON_SEMICOLON = 
    let KEY_DOUBLE_QUOTE_SINGLE_QUOTE = 
    let KEY_ENTER = builder.quick_key_action("Enter". to_string(), 40);
    let KEYPAD_FOUR_LEFT_ARROW = 
    let KEYPAD_FIVE = 
    let KEYPAD_SIX_RIGHT_ARROW = 
    let KEY_SHIFT_LEFT = builder.quick_key_action("Shift". to_string(), 225);
    let KEY_Z = builder.quick_key_alphabetic("Z", "z", 29);
    let KEY_X = builder.quick_key_alphabetic("X", "x", 27);
    let KEY_C = builder.quick_key_alphabetic("C", "c", 6);
    let KEY_V = builder.quick_key_alphabetic("V", "v", 25);
    let KEY_B = builder.quick_key_alphabetic("B", "b", 5);
    let KEY_N = builder.quick_key_alphabetic("N", "n", 17);
    let KEY_M = builder.quick_key_alphabetic("M", "m", 16);
    let KEY_LEFT_ANGLE_BRACKET_COMMA = 
    let KEY_RIGHT_ANGLE_BRACKET_PERIOD = 
    let KEY_QUESTION_FORWARD_SLASH = 
    let KEY_SHIFT_RIGHT = 
    let KEY_UP_ARROW = 
    let KEYPAD_ONE_END = 
    let KEYPAD_TWO_DOWN_ARROW = 
    let KEYPAD_THREE_PGDN = 
    let KEYPAD_ENTER = builder.quick_key_action("Enter", 88);
    let KEY_CTRL_LEFT = 
    let KEY_SUPER_LEFT = 
    let KEY_ALT_LEFT = 
    let KEY_SPACE = 
    let KEY_ALT_RIGHT = 
    let KEY_SUPER_RIGHT = 
    let KEY_MENU = builder.quick_key_action("Menu", 118);
    let KEY_CTRL_RIGHT = 
    builder.position.x += 1.;
    let KEY_LEFT_ARROW = builder.quick_key_action("←", 80);
    let KEY_DOWN_ARROW = builder.quick_key_action("↓", 81);
    let KEY_RIGHT_ARROW = builder.quick_key_action("→", 79);
    builder.position.x += 1.;
    let KEYPAD_ZERO_INS = builder.quick_key_numpad_digit_action("0", "Ins", 98);
    let KEYPAD_PERIOD_DEL = builder.quick_key_numpad_digit_action(".", "Del", 99);

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
    metadata: "Standard ANSI 104 Keyboard". to_string(),
    physical_metadata: "Standard ANSI 104 Keyboard Physical Switch Layout Map". to_string(),
    firmware_metadata: "Standard ANSI 104 Keyboard Keycode Layout Map". to_string(),
    key_output_metadata: "Standard ANSI 104 Keyboard Character Layout Map". to_string(),
    keylist: DEFAULT_ANSI_104_KEYS,
};





