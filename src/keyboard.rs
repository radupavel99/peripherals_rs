use std::{convert, fmt};

use cfg_if::cfg_if;
use defaults::Defaults;
use parse_display_derive::Display;
use serde::Serialize;
use serde_json::json;

use super::peripherals::{Error, ErrorCode, ErrorKind};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        use windows::Win32::{
            Foundation,
            UI:: {
                Input::KeyboardAndMouse,
                WindowsAndMessaging,
            },
        };
    }
}

cfg_if! {
    if #[cfg(target_os = "macos")] {
        extern "C" {
            fn macos_get_key_state(key: u16) -> bool;
        }
    }
}

cfg_if! {
    if #[cfg(target_os = "macos")] {
        const MAX_MODIFIER_NUM: usize = 5;
    } else {
        const MAX_MODIFIER_NUM: usize = 6;
    }
}

// <==============================> KEY <=============================>

#[derive(Serialize, Display, Defaults)]
#[def = "Up"]
pub enum KeyState {
    Up,
    Down,
}

// TODO: Add keys.
#[derive(Copy, Clone, Debug, Display)]
pub enum Key {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Grave,
    Minus,
    Equals,
    RightBracket,
    LeftBracket,
    BackwardsSlash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,

    Space,
}

/*
    macos
    F1 =0x7A,
    F2 = 0x78,
    F3 = 0x63,
    F4 = 0x76,
    F5 = 0x60,
    F6 = 0x61,
    F7 = 0x62,
    F8 = 0x64,
    F9 = 0x65,
    F10 = 0x6D,
    F11 = 0x67,
    F12 = 0x6F,
    F13 = 0x69,
    F14 = 0x6B,
    F15 = 0x71,
    F16 = 0x6A,
    F17 = 0x40,
    F18 = 0x4F,
    F19 = 0x50,
    F20 = 0x5A,

    /*
    Zero = 0x1D,
    One = 0x12,
    Two = 0x13,
    Three = 0x14,
    Four = 0x15,
    Five = 0x17,
    Six = 0x16,
    Seven = 0x1A,
    Eight = 0x1C,
    Nine = 0x19,

    A = 0x00,
    B = 0x0B,
    C = 0x08,
    D = 0x02,
    E = 0x0E,
    F = 0x03,
    G = 0x05,
    H = 0x04,
    I = 0x22,
    J = 0x26,
    K = 0x28,
    L = 0x25,
    M = 0x2E,
    N = 0x2D,
    O = 0x1F,
    P = 0x23,
    Q = 0x0C,
    R = 0x0F,
    S = 0x01,
    T = 0x11,
    U = 0x20,
    V = 0x09,
    W = 0x0D,
    X = 0x07,
    Y = 0x10,
    Z = 0x06,

    Grave = 0x32,
    Minus = 0x1B,
    Equals = 0x18,
    RightBracket = 0x1E,
    LeftBracket = 0x21,
    BackwardsSlash = 0x2A,
    Semicolon = 0x29,
    Quote = 0x27,
    Comma = 0x2B,
    Period = 0x2F,
    Slash = 0x2C,

    Space = 0x31,
    */

    Escape = 0x35,
    CapsLock = 0x39,
    Tab = 0x30,
    Delete = 0x33,
    Return = 0x24,

    Shift = 0x38,
    RightShift = 0x3C,
    Function = 0x3F,
    Control = 0x3B,
    RightControl = 0x3E,
    Option = 0x3A,
    RightOption = 0x3D,
    Command = 0x37,
    RightCommand = 0x36,

    RightArrow = 0x7C,
    UpArrow = 0x7E,
    LeftArrow = 0x7B,
    DownArrow = 0x7D,

    VolumeUp = 0x48,
    VolumeDown = 0x49,
    Mute = 0x4A,

    Help = 0x72,
    Home = 0x73,
    PageUp = 0x74,
    ForwardDelete = 0x75,
    End = 0x77,
    PageDown = 0x79,

    KeypadZero = 0x52,
    KeypadOne = 0x53,
    KeypadTwo = 0x54,
    KeypadThree = 0x55,
    KeypadFour = 0x56,
    KeypadFive = 0x57,
    KeypadSix = 0x58,
    KeypadSeven = 0x59,
    KeypadEight = 0x5B,
    KeypadNine = 0x5C,

    KeypadDecimal = 0x41,
    KeypadMultiply = 0x43,
    KeypadPlus = 0x45,
    KeypadClear = 0x47,
    KeypadDivide = 0x4B,
    KeypadEnter = 0x4C,
    KeypadMinus = 0x4E,
    KeypadEquals = 0x51,

*/

/*
win order

Backspace,
    Tab,
    Clear,
    Enter,
    Shift,
    Ctrl,
    Alt,
    CapsLock,
    Pause,
    Escape,
    Space,
    PageUp,
    PageDown,
    EndKey,
    HomeKey,
    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,
    SelectKey,
    PrintKey,
    ExecuteKey,
    PrintScreenKey,
    InsKey,
    DelKey,
    HelpKey,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftWindows,
    RightWindows,
    Application,
    SleepKey,
    NumpadZero,
    NumpadOne,
    NumpadTwo,
    NumpadThree,
    NumpadFour,
    NumpadFive,
    NumpadSix,
    NumpadSeven,
    NumpadEight,
    NumpadNine,
    NumpadMultiply,
    NumpadAdd,
    NumpadSeparator,
    NumpadSubstract,
    NumpadDecimal,
    NumpadDivide,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    NumLock,
    ScrollLock,
    Semicolon,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    BrowserStop,
    BrowserSearch,
    BrowserFavorites,
    BrowserStartAndHome,
    VolumeMute,
    VolumeDown,
    VolumeUp,
    NextTrack,
    PreviousTrack,
    StopMedia,
    Play_or_Pause_Media,
    StartMail,
    SelectMedia,
    StartApplicationOne,
    StartApplicationTwo,
    ColumnSemiColumn, // :;
    Plus,
    Comma,
    Dash,
    Dot,
    SlashQuestionMark, // /?
    WaveKey, // ~
    OpenBrackets, // [{
    AboveEnterKey, // \|
    ClosedBrackets, // ]}
    SingleOrDoubleQuotes,
    AttnKey,
    CrSel,
    ExSel,
    Erase,
    Play,
    Zoom,
    PA_One,
    Clear

*/

impl Key {
    pub fn json(&self) -> String {
        json!({
            "name": self.to_string(),
            "state": (*self).state()
        })
        .to_string()
    }

    pub fn state(&self) -> KeyState {
        unsafe {
            cfg_if! {
                if #[cfg(target_os = "macos")] {
                    if macos_get_key_state(u16::from(*self)) {
                        return KeyState::Down;
                    }
                } else if #[cfg(target_os = "linux")] {
                    todo!("// TODO: Implement Key.state() for LinuxOS.")
                } else {
                    if KeyboardAndMouse::GetKeyState(i32::from(*self)) < 0 {
                        return KeyState::Down;
                    }
                }
            }
        }

        KeyState::default()
    }
}

impl convert::TryFrom<char> for Key {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if [
            '~', '!', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|', ':', '\"',
            '<', '>', '?',
        ]
        .contains(&c)
        {
            return Err(Error::new(
                ErrorKind::OSAgnostic,
                ErrorCode::NO_EQUIVALENT_KEY,
                format!(
                    "No equivalent key exists for the character \"{}\". It seems like you are using a character generated with the help of a modifier key such as Shift or CapsLook. These characters are generally not supported with the exception of upper-case letters.",
                    c
                ),
            ));
        }

        Ok(match c {
            '0' => Key::Zero,
            '1' => Key::One,
            '2' => Key::Two,
            '3' => Key::Three,
            '4' => Key::Four,
            '5' => Key::Five,
            '6' => Key::Six,
            '7' => Key::Seven,
            '8' => Key::Eight,
            '9' => Key::Nine,

            'a' | 'A' => Key::A,
            'b' | 'B' => Key::B,
            'c' | 'C' => Key::C,
            'd' | 'D' => Key::D,
            'e' | 'E' => Key::E,
            'f' | 'F' => Key::F,
            'g' | 'G' => Key::G,
            'h' | 'H' => Key::H,
            'i' | 'I' => Key::I,
            'j' | 'J' => Key::J,
            'k' | 'K' => Key::K,
            'l' | 'L' => Key::L,
            'm' | 'M' => Key::M,
            'n' | 'N' => Key::N,
            'o' | 'O' => Key::O,
            'p' | 'P' => Key::P,
            'q' | 'Q' => Key::Q,
            'r' | 'R' => Key::R,
            's' | 'S' => Key::S,
            't' | 'T' => Key::T,
            'u' | 'U' => Key::U,
            'v' | 'V' => Key::V,
            'w' | 'W' => Key::W,
            'x' | 'X' => Key::X,
            'y' | 'Y' => Key::Y,
            'z' | 'Z' => Key::Z,

            '`' => Key::Grave,
            '-' => Key::Minus,
            '=' => Key::Equals,
            '[' => Key::LeftBracket,
            ']' => Key::RightBracket,
            '\\' => Key::BackwardsSlash,
            ';' => Key::Semicolon,
            '\'' => Key::Quote,
            ',' => Key::Comma,
            '.' => Key::Period,
            '/' => Key::Slash,

            ' ' => Key::Space,

            _ => {
                return Err(Error::new(
                    ErrorKind::OSAgnostic,
                    ErrorCode::NO_EQUIVALENT_KEY,
                    format!("No equivalent key exists for the character \"{}\".", c),
                ));
            }
        })
    }
}

impl convert::From<Key> for u16 {
    fn from(key: Key) -> Self {
        cfg_if! {
            if #[cfg(target_os = "macos")] {
                todo!("// TODO: Implement u16::from(Key) for MacOS.")
            } else if #[cfg(target_os = "linux")] {
                todo!("// TODO: Implement u16::from(Key) for MacOS.")
            } else {
                match key {
                    Key::Zero => KeyboardAndMouse::VK_0,
                    Key::One => KeyboardAndMouse::VK_1,
                    Key::Two => KeyboardAndMouse::VK_2,
                    Key::Three => KeyboardAndMouse::VK_3,
                    Key::Four => KeyboardAndMouse::VK_4,
                    Key::Five => KeyboardAndMouse::VK_5,
                    Key::Six => KeyboardAndMouse::VK_6,
                    Key::Seven => KeyboardAndMouse::VK_7,
                    Key::Eight => KeyboardAndMouse::VK_8,
                    Key::Nine => KeyboardAndMouse::VK_9,

                    Key::A => KeyboardAndMouse::VK_A,
                    Key::B => KeyboardAndMouse::VK_B,
                    Key::C => KeyboardAndMouse::VK_C,
                    Key::D => KeyboardAndMouse::VK_D,
                    Key::E => KeyboardAndMouse::VK_E,
                    Key::F => KeyboardAndMouse::VK_F,
                    Key::G => KeyboardAndMouse::VK_G,
                    Key::H => KeyboardAndMouse::VK_H,
                    Key::I => KeyboardAndMouse::VK_I,
                    Key::J => KeyboardAndMouse::VK_J,
                    Key::K => KeyboardAndMouse::VK_K,
                    Key::L => KeyboardAndMouse::VK_L,
                    Key::M => KeyboardAndMouse::VK_M,
                    Key::N => KeyboardAndMouse::VK_N,
                    Key::O => KeyboardAndMouse::VK_O,
                    Key::P => KeyboardAndMouse::VK_P,
                    Key::Q => KeyboardAndMouse::VK_Q,
                    Key::R => KeyboardAndMouse::VK_R,
                    Key::S => KeyboardAndMouse::VK_S,
                    Key::T => KeyboardAndMouse::VK_T,
                    Key::U => KeyboardAndMouse::VK_U,
                    Key::V => KeyboardAndMouse::VK_V,
                    Key::W => KeyboardAndMouse::VK_W,
                    Key::X => KeyboardAndMouse::VK_X,
                    Key::Y => KeyboardAndMouse::VK_Y,
                    Key::Z => KeyboardAndMouse::VK_Z,

                    // Key::Grave => KeyboardAndMouse::VK_,
                    Key::Minus => KeyboardAndMouse::VK_OEM_MINUS,
                    // Key::Equals => KeyboardAndMouse::VK_,
                    // Key::LeftBracket => KeyboardAndMouse::VK_,
                    // Key::RightBracket => KeyboardAndMouse::VK_,
                    // Key::BackwardsSlash => KeyboardAndMouse::VK_,
                    // Key::Semicolon => KeyboardAndMouse::VK_,
                    // Key::Quote => KeyboardAndMouse::VK_,
                    Key::Comma => KeyboardAndMouse::VK_OEM_COMMA,
                    Key::Period => KeyboardAndMouse::VK_OEM_PERIOD,
                    // Key::Slash => KeyboardAndMouse::VK_,
   

                    Key::Space => KeyboardAndMouse::VK_SPACE,

                    _ => todo!("// TODO: Finish u16::from(Key) for WindowsOS."),
                }.0
            }
        }
    }
}

impl convert::From<Key> for u32 {
    fn from(key: Key) -> Self {
        u32::from(u16::from(key))
    }
}

impl convert::From<Key> for i32 {
    fn from(key: Key) -> Self {
        i32::from(u16::from(key))
    }
}
// <==================================================================>

// <===========================> MODIFIER <===========================>

#[derive(Copy, Clone, Debug, Display)]
pub enum Modifier {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    Alt,
    #[cfg(target_os = "macos")]
    Option,
    Control,
    #[cfg(target_os = "macos")]
    Command,
    #[cfg(target_os = "windows")]
    NoRepeat,
    Shift,
    #[cfg(target_os = "windows")]
    Windows,
    #[cfg(target_os = "macos")]
    Fn,
}

impl convert::From<Modifier> for u32 {
    fn from(modifier: Modifier) -> Self {
        cfg_if! {
            if #[cfg(target_os = "macos")] {
                todo!("// TODO: Implement u32::from(Modifier) for MacOS.")
            } else if #[cfg(target_os = "linux")] {
                todo!("// TODO: Implement u32::from(Modifier) for MacOS.")
            } else {
                match modifier {
                    Modifier::Alt => KeyboardAndMouse::MOD_ALT,
                    Modifier::Control => KeyboardAndMouse::MOD_CONTROL,
                    Modifier::NoRepeat => KeyboardAndMouse::MOD_NOREPEAT,
                    Modifier::Shift => KeyboardAndMouse::MOD_SHIFT,
                    Modifier::Windows => KeyboardAndMouse::MOD_WIN,
                }.0
            }
        }
    }
}

impl PartialEq for Modifier {
    fn eq(&self, other: &Self) -> bool {
        u32::from(*self) == u32::from(*other)
    }
}

// <==================================================================>

pub fn register_shortcut(id: u16, modifiers: &[Modifier], key: Key) -> Result<(), Error> {
    let modifiers_num = match modifiers.len() {
        len @ 0 => {
            return Err(Error::new(
                ErrorKind::OSAgnostic,
                ErrorCode::INVALID_MODIFIRES_LIST,
                "Modifiers list cannot be empty.",
            ))
        }
        len @ MAX_MODIFIER_NUM..=usize::MAX => {
            return Err(Error::new(
                ErrorKind::OSAgnostic,
                ErrorCode::INVALID_MODIFIRES_LIST,
                format!(
                    "Modifiers list cannot contain more than {} modifiers.",
                    MAX_MODIFIER_NUM
                ),
            ))
        }
        num => num,
    };

    if (1..modifiers_num).any(|i| modifiers[i..].contains(&modifiers[i - 1])) {
        return Err(Error::new(
            ErrorKind::OSAgnostic,
            ErrorCode::INVALID_MODIFIRES_LIST,
            "Modifiers list cannot contain duplicates.",
        ));
    }

    unsafe {
        cfg_if! {
                if #[cfg(target_os = "macos")] {
                    todo!("Implement register_shortcut() for MacOS.")
                } else if #[cfg(target_os = "linux")] {
                    todo!("Implement register_shortcut() for LinuxOS")
                } else {
                    let modifiers = modifiers.iter().fold(0, |mut acc, modifire| {
                        acc |= u32::from(*modifire);
                        acc
                    });

                    if !KeyboardAndMouse::RegisterHotKey(
                        Foundation::HWND(0),
                        i32::from(id),
                        KeyboardAndMouse::HOT_KEY_MODIFIERS(modifiers),
                        key.into()
                    ).as_bool() {
                        let error = Foundation::GetLastError().to_hresult();
                        let code = ErrorCode::from(error.0);
                        let description = error.message().to_string();

                        return Err(Error::new(ErrorKind::OSSpecific, code, description));
                    }
                 }
        }
    }

    Ok(())
}

pub fn unregister_shortcut(id: u16) -> Result<(), Error> {
    unsafe {
        cfg_if! {
            if #[cfg(target_os = "macos")] {
                todo!("Implement unregister_shortcut() for MacOS.")
            } else if #[cfg(target_os = "linux")] {
                todo!("Implement unregister_shortcut() for LinuxOS")
            } else {
                if KeyboardAndMouse::UnregisterHotKey(Foundation::HWND(0), i32::from(id)).as_bool() {
                    let error = Foundation::GetLastError().to_hresult();
                    let code = ErrorCode::from(error.0);
                    let description = error.message().to_string();

                    return Err(Error::new(ErrorKind::OSSpecific, code, description));
                }
            }
        }
    }

    Ok(())
}

pub fn shortcut_state(id: u16) -> Result<KeyState, Error> {
    unsafe {
        cfg_if! {
            if #[cfg(target_os = "macos")] {
                todo!("Implement shortcut_state() for MacOS.")
            } else if #[cfg(target_os = "linux")] {
                todo!("Implement shortcut_state() for LinuxOS")
            } else {
                let mut message = WindowsAndMessaging::MSG::default();

                if WindowsAndMessaging::GetMessageW(&mut message,
                    Foundation::HWND(0), 0, 0
                ).as_bool() && message.wParam.0 == usize::from(id) {
                    return Ok(KeyState::Down);
                }
            }
        }
    }

    Ok(KeyState::default())
}
