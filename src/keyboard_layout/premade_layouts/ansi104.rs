use crate::{KeyboardLayout, builder::KeyboardKeyBuilder, keys::{Key, KeyList}, modifiers::ModifierIndex};

//Default keyboard
pub fn build_keyboard() -> KeyboardLayout {

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

    default_ansi_104_keylist.into_keyboard()
}