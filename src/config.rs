#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq)]
pub struct HotkeyBind {
    pub key: u16,
    pub mods: u8,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq, Default)]
pub enum ClickerMode {
    #[default]
    Mouse,
    Keyboard,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Config {
    pub mode: ClickerMode,
    pub device_name: String,
    pub cooldown_ns: u64,
    pub cooldown_press_release_ns: u64,
    pub enable_lock_unlock: bool,
    pub lock_unlock_bind: Option<u16>,
    pub enable_left: bool,
    pub left_bind: Option<u16>,
    pub enable_middle: bool,
    pub middle_bind: Option<u16>,
    pub enable_right: bool,
    pub right_bind: Option<u16>,
    pub hold: bool,
    pub grab: bool,
    pub enable_hotkey: bool,
    pub hotkey_bind: Option<HotkeyBind>,
    pub repeat_key: Option<u16>,
    pub repeat_trigger: Option<u16>,
    pub repeat_delay_ns: u64,
}

impl Config {
    pub fn missing_binds(&self) -> bool {
        match self.mode {
            ClickerMode::Mouse => {
                (self.enable_lock_unlock && self.lock_unlock_bind.is_none())
                    || (self.enable_left && self.left_bind.is_none())
                    || (self.enable_middle && self.middle_bind.is_none())
                    || (self.enable_right && self.right_bind.is_none())
            }
            ClickerMode::Keyboard => self.repeat_key.is_none() || self.repeat_trigger.is_none(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: ClickerMode::Mouse,
            device_name: String::new(),
            cooldown_ns: 1_000_000,
            cooldown_press_release_ns: 0,
            enable_lock_unlock: false,
            lock_unlock_bind: None,
            enable_left: true,
            left_bind: None,
            enable_middle: false,
            middle_bind: None,
            enable_right: false,
            right_bind: None,
            hold: true,
            grab: true,
            enable_hotkey: false,
            hotkey_bind: None,
            repeat_key: None,
            repeat_trigger: None,
            repeat_delay_ns: 1_000_000,
        }
    }
}
