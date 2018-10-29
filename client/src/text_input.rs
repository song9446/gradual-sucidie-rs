extern crate quicksilver;
use quicksilver::input::Keyboard;
use quicksilver::input::Key;
use quicksilver::input::ButtonState;

const KEY_LIST: &[Key] = &[Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, Key::Key6, Key::Key7, Key::Key8, Key::Key9, Key::Key0, Key::A, Key::B, Key::C, Key::D,
    Key::E, Key::F, Key::G, Key::H, Key::I, Key::J, Key::K, Key::L, Key::M, Key::N, Key::O, Key::P, Key::Q, Key::R, Key::S, Key::T, Key::U, Key::V, Key::W, Key::X, Key::Y, Key::Z,
    Key::Escape, Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6, Key::F7, Key::F8, Key::F9, Key::F10, Key::F11, Key::F12,
    Key::F13, Key::F14, Key::F15, Key::Snapshot, Key::Scroll, Key::Pause, Key::Insert, Key::Home, Key::Delete, Key::End, Key::PageDown, Key::PageUp, Key::Left, Key::Up, Key::Right,
    Key::Down, Key::Back, Key::Return, Key::Space, Key::Compose, Key::Caret, Key::Numlock, Key::Numpad0, Key::Numpad1, Key::Numpad2, Key::Numpad3, Key::Numpad4, Key::Numpad5,
    Key::Numpad6, Key::Numpad7, Key::Numpad8, Key::Numpad9, Key::AbntC1, Key::AbntC2, Key::Add, Key::Apostrophe, Key::Apps, Key::At, Key::Ax, Key::Backslash, Key::Calculator,
    Key::Capital, Key::Colon, Key::Comma, Key::Convert, Key::Decimal, Key::Divide, Key::Equals, Key::Grave, Key::Kana, Key::Kanji, Key::LAlt, Key::LBracket, Key::LControl,
    Key::LShift, Key::LWin, Key::Mail, Key::MediaSelect, Key::MediaStop, Key::Minus, Key::Multiply, Key::Mute, Key::MyComputer, Key::NavigateForward,
    Key::NavigateBackward, Key::NextTrack, Key::NoConvert, Key::NumpadComma, Key::NumpadEnter, Key::NumpadEquals, Key::OEM102, Key::Period, Key::PlayPause,
    Key::Power, Key::PrevTrack, Key::RAlt, Key::RBracket, Key::RControl, Key::RShift, Key::RWin, Key::Semicolon, Key::Slash, Key::Sleep, Key::Stop, Key::Subtract,
    Key::Sysrq, Key::Tab, Key::Underline, Key::Unlabeled, Key::VolumeDown, Key::VolumeUp, Key::Wake, Key::WebBack, Key::WebFavorites, Key::WebForward, Key::WebHome,
    Key::WebRefresh, Key::WebSearch, Key::WebStop, Key::Yen];

pub struct TextInput {
    capslock: bool,
}
impl TextInput {
    pub fn new() -> TextInput {
        TextInput{
            capslock: false,
        }
    }
    pub fn char(&mut self, keyboard: &Keyboard) -> Option<char> {
        if keyboard[Key::Capital] == ButtonState::Pressed {
            self.capslock = !self.capslock;
        }
        let shift_held = keyboard[Key::LShift].is_down();
        for i in (Key::Key1 as u8)..(Key::Z as u8 + 1) {
            if keyboard[KEY_LIST[i as usize]] == ButtonState::Pressed {
                return Some(
                    if i >= (Key::A as u8) {
                        let base = if (self.capslock && !shift_held) || (!self.capslock && shift_held) { 'A' as u8 } else { 'a' as u8 };
                        (base + (i - (Key::A as u8))) as char
                    }else if i == (Key::Key0 as u8) {
                        '0'
                    }else {
                        (('1' as u8) + (i - (Key::Key1 as u8))) as char
                    });
            }
        }
        None
    }
}
