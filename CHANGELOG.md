## [1.0.4] - 2026-05-14

### Fixed
- Prevent overflow in table layout
- Prevent menu mouse events from propagating

## [1.0.3] - 2026-05-13

### Added
- Support for both Wayland and X11 windowing systems.
- Remove terminal in windows when running as a GUI application.
- Performance improvements, use `codegen_util = 1`.

## [1.0.2] - 2026-05-13

### Added
- License information.

## [1.0.1] - 2026-05-13

### Added
- Add themes to release

### Fixed
- fix package name
- make the menu horizontal

## [1.0.0] - 2026-05-13

### Added
- Initial release of the Rank application.
- Interactive GPUI-based graphical interface for process and resource management.
- Implementation of the Banker's Algorithm (`algo::check_safety`) for deadlock avoidance.
- Table view to visualize processes, showing `Allocation`, `Max`, `Need`, and `Available` resource matrices.
- Form controls to add, modify, and manage processes dynamically.
- Step-by-step algorithm simulation for educational purposes and debugging.
