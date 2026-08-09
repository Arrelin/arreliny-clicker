# Arreliny Clicker

A graphical Linux autoclicker for X11 and Wayland.

## Requirements

- Linux (X11 or Wayland)
- Access to `/dev/input/` and `/dev/uinput` — typically requires the user to be in the `input` and `uinput` groups:
  ```bash
  sudo usermod -aG input,uinput $USER
  ```

## Installation

```bash
cargo install arreliny-clicker
```

## Features

- Select input device from a list or detect it automatically by clicking ("Find Mouse")
- Configure bindings for left, middle, and right click autoclicker
- Lock/Unlock binding to pause clicking without stopping
- Hold mode — hold the bind to click, release to stop
- Grab mode — captures the input device so bindings don't pass through to the system
- Configurable cooldown and press-release gap in nanoseconds
- System tray icon (SNI) showing current state: idle / locked / clicking
- Settings are persisted across restarts
- Global Start/Stop hotkey (keyboard binding to toggle the clicker from anywhere)
- Keyboard repeat mode with a selectable key, hold trigger, and repeat rate

## Usage

Launch the GUI:
```bash
arreliny-clicker
# or directly
~/.cargo/bin/arreliny-clicker
```

1. Select your input device from the dropdown or press **Find Mouse** and click with your mouse
2. Enable and configure bindings in the **Bindings** section
3. Adjust cooldown and other settings
4. Press **Start**

## Notes

- Very small delays depend on scheduler precision and application event handling
- Nanosecond delays can generate millions of events per second and may overload terminals, browsers, or the desktop session
- Grab mode may softlock input if your compositor does not recognize the virtual device
- The system tray icon requires a compositor or panel that supports the StatusNotifierItem (SNI) protocol (KDE Plasma, waybar, etc.)

## Input rate tester

Run a second instance while the clicker is active:

```bash
arreliny-clicker --tester --key 272 --seconds 10
```

`272` is the left mouse button. Keyboard keys use the numeric evdev code displayed in the GUI.
