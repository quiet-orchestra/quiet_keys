use crate::{Point, Rect, firmware_collection::HIDKeycode, key_output_collection::*, keys::*, labels::*, modifiers::*};



#[derive(Clone, Debug, Default)]
pub struct KeyboardKeyBuilder {
    switch_counter: u16,
    key_rect: Rect,
    one_label_position: LabelPosition,
    two_labels_position: [LabelPosition; 2],
    horizontal_offset: f32,
    pub(crate) modifier_list: ModifierList,
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
        let keycode = HIDKeycode::make_keyboard_hid_code(keycode_id);

        //Make key
        let key = Key::quick_key(
            key_output,
            rect,
            self.switch_counter,
            keycode,
        );

        //Move builder
        self.move_next();
        self.switch_counter += 1;

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
        let key_output = KeyOutputType::TextOutputKey(vec![text_output_one, text_output_two]).to_key_output();     

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
        let key_label = Label::default().with_label(label.clone()).with_position(LabelPosition::MiddleLeft);
        let key_output = KeyOutputType::ModifierKey { modifier_index, labels: vec![key_label]}.to_key_output();

        //Register modifier
        self.modifier_list.register_modifier_with_index(label, modifier_index);

        self.quick_key(key_output, keycode_id)
 
    }

    pub fn quick_key_modifier_with_width(&mut self, label: impl Into<String>, modifier_index: ModifierIndex, keycode_id: u16, width: f32) -> Key {
        self.key_with_dimensions(|builder|builder.quick_key_modifier(label, modifier_index, keycode_id), width, self.height())
    }

    pub fn quick_key_action(&mut self, label: impl Into<String>, keycode_id: u16) -> Key {
        //Construct label and then modifier
        let key_label = Label::default().with_label(label.into()).with_position(self.one_label_position);
        let key_output = KeyOutputType::ActionKey(key_label).to_key_output();

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
        let numpad_label = Label::default().with_position(self.two_labels_position[1]);
        let action_label = Label::default().with_label(action_label.into()).with_position(self.two_labels_position[0]);

        let numpad_text_output = KeyOutputType::TextOutputKey(vec![TextOutput{ output_text, modifier_mask: ModifierMask::base(), label: numpad_label}]);
        let numpad_action = KeyOutputType::ActionKey(action_label);
        
        let key_output = KeyOutput::Locktype([numpad_text_output, numpad_action]);

        self.quick_key(key_output, keycode_id)
    }

    pub fn quick_key_single(&mut self, text: impl Into<String>, keycode_id: u16) -> Key {
        self.single_text_key(text, keycode_id)
    }

    fn single_text_key(&mut self, text: impl Into<String>, keycode_id: u16) -> Key {
        //Construct key items 
        let text_output_one = TextOutput::simple_text_output(text, false, true, self.two_labels_position[1]);
        let key_output = KeyOutputType::TextOutputKey(vec![text_output_one]).to_key_output();     

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