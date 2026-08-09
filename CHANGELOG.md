# Changelog

## [Unreleased]

### Added

- Added keyboard repeat mode with keyboard detection, configurable key and hold trigger, and repeat rate
- Added nanosecond delay configuration for mouse clicks and keyboard repeat
- Added an evdev input rate tester available through `--tester`

### Changed

- Renamed the project to Arreliny Clicker

## [0.4.1]

### Fixed

- Preserve the selected mouse identity for the virtual device so libinput keeps its DPI profile in grab mode

## [0.4.0]

### Added

- Added checkbox in settings — removes the 25 ms minimum cooldown limit for systems that can handle higher click rates

## [0.3.0]

### Changed

- `theclicker` is no longer required as a separate installed binary — the clicker backend is now embedded directly into `theclicker-gui`. Installing `theclicker-gui` is sufficient.

## [0.1.4]

### Added

- Global Start/Stop hotkey: bind any key combination (including Ctrl, Alt, Shift, Super modifiers) to toggle the clicker from anywhere

## [0.1.3]

### Added

- Console logging with configurable log level via `--log-level` flag:
  ```bash
  theclicker-gui --log-level info
  theclicker-gui --log-level debug
  theclicker-gui --log-level trace  # includes raw theclicker stdout output
  ```
  Available levels: `error`, `warn` (default), `info`, `debug`, `trace`

## [0.1.2]

### Added

- System tray: Start/Stop actions via context menu
- System tray: left click raises the application window
