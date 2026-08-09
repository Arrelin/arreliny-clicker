#[derive(Clone, PartialEq, Debug)]
pub enum KeyTarget {
    LockUnlock,
    Left,
    Middle,
    Right,
    RepeatKey,
    RepeatTrigger,
    HotkeyStartStop,
}

impl KeyTarget {
    pub fn label(&self) -> &str {
        match self {
            KeyTarget::LockUnlock => "Lock/Unlock",
            KeyTarget::Left => "Left click",
            KeyTarget::Middle => "Middle click",
            KeyTarget::Right => "Right click",
            KeyTarget::RepeatKey => "Key to repeat",
            KeyTarget::RepeatTrigger => "Hold to repeat",
            KeyTarget::HotkeyStartStop => "Start/Stop hotkey",
        }
    }
}

#[derive(Default, PartialEq)]
pub enum Screen {
    #[default]
    Config,
    KeyCapture,
    FindMouse,
    FindKeyboard,
    Running,
}

pub enum Action {
    StartCapture(KeyTarget),
    FindMouse,
    FindKeyboard,
    Launch,
    Stop,
    Refresh,
}
