use std::{convert, fmt, mem};

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
    #[cfg(target_os = "windows")]
    Backspace,
    Clear,
    Shift,
    CapsLock,
    Pause,
    Escape,
    Space,
    PageUp,
    LeftArrow,
    Zero,
    A,
    LeftWindows,
    RightWindows,
    Application,
    NumpadZero,
    NumpadMultiply,
    F1,
    NumLock,
    Semicolon,
    LeftShift,
    RightShift,
    BrowserBack,
    VolumeMute,
    Plus,
}

impl Key {
    pub fn to_json(&self) -> String {
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
        Ok(match c {
            ' ' => Self::Space,
            '0' | ')' => Self::Zero,
            'a' | 'A' => Self::A,
            ';' | ':' => Self::Semicolon,
            '+' | '=' => Self::Plus,
            _ => {
                return Err(Error::new(
                    ErrorKind::OSAgnostic,
                    ErrorCode::NO_EQUIVALENT_KEY,
                    format!("No equivalent key exist for character \"{}\".", c),
                ))
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
                    Key::A => KeyboardAndMouse::VK_A,
                    _ => todo!("// TODO: Implement u16::from(Key) for WindowsOS.")
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

const MAX_MODIFIER_LEN: usize = 5;

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
    let modifiers_len = match modifiers.len() {
        len @ 0 => {
            return Err(Error::new(
                ErrorKind::OSAgnostic,
                ErrorCode::INVALID_MODIFIRES_LIST,
                "Modifiers list cannot be empty.",
            ))
        }
        len @ MAX_MODIFIER_LEN..=usize::MAX => {
            return Err(Error::new(
                ErrorKind::OSAgnostic,
                ErrorCode::INVALID_MODIFIRES_LIST,
                format!(
                    "Modifiers list cannot contain more than {} modifiers.",
                    MAX_MODIFIER_LEN
                ),
            ))
        }
        len => len,
    };

    if (1..modifiers_len).any(|i| modifiers[i..].contains(&modifiers[i - 1])) {
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
                let mut msg = mem::MaybeUninit::uninit().assume_init();

                if WindowsAndMessaging::GetMessageW(&mut msg,
                    Foundation::HWND(0), 0, 0
                ).as_bool() && msg.wParam.0 == usize::from(id) {
                    return Ok(KeyState::Down);
                }
            }
        }
    }

    Ok(KeyState::default())
}
