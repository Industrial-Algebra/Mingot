# Changelog

All notable changes to Mingot will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2026-03-08

### Added

#### Theme System Foundation (Phase 6 Complete)
- **Cow<'static, str> migration** - All string props migrated from `String` to `Cow<'static, str>` for zero-copy efficiency
- **8 new color palettes** - Rose, Amber, Emerald, Sky, Fuchsia, Lime, Slate, Stone
- **ThemeBuilder** - Fluent API for constructing custom themes with live preview
- **WCAG 2.1 AA validation** - Automatic contrast ratio checking for text/background combinations

#### CSS Variable Injection
- `--mingot-*` namespace covering colors, surfaces, spacing, radius, shadows, borders, layout, typography
- Auto color scheme with system preference detection (`prefers-color-scheme`)
- Shadow CSS variables (`--mingot-shadow-xs` through `--mingot-shadow-xl`) with individual ThemeBuilder setters
- BorderScale tokens (`--mingot-border-width`, `--mingot-border-style`)
- LayoutTokens (`--mingot-container-xs` through `--mingot-container-xl`)

#### Theme Presets & Customization
- **5 built-in theme presets** - Default, Dark, Industrial, Scientific, Financial
- **ThemeOverride component** - Scoped subtree theme customization
- **Design tokens export/import** via `theme-tokens` feature flag (JSON round-trip)

#### Demo Site
- Theming demo pages with live preset switching
- ThemeBuilder playground with color palette display
- Interactive theme customization examples

### Changed
- Demo site CSS vars migrated to `--mingot-*` namespace
- Version badge updated to v0.7.0

### Dependencies
- leptos_router 0.8.9 → 0.8.10
- rust_decimal 1.39.0 → 1.40.0
- wasm-bindgen-test 0.3.55 → 0.3.64
- actions/cache v4 → v5, actions/checkout v4 → v6

---

## [0.6.1] - 2026-02-14

### Added

#### Parameter Manipulation Components (Phase 5 Complete)
- **ParameterSlider** - High-precision slider with string-based values
  - Linear and logarithmic scale support
  - Configurable marks with labels
  - Keyboard navigation with modifier key multipliers (Shift=10x, Ctrl=100x)
  - Integrated value input for precise entry
  - All NumberInputPrecision types supported
- **ParameterGrid** - Multiple parameter sliders in organized layout
  - Preset save/load functionality
  - Parameter grouping with collapsible sections
  - Vertical, horizontal, and grid layout modes
  - Real-time value synchronization
- **ParameterTree** - PyQtGraph-style hierarchical parameter editor
  - Type-aware editors (string, number, bool, color, enum, action)
  - Expand/collapse groups with search filtering
  - Keyboard navigation and accessibility support
  - Value serialization via `tree_to_values()` helper

### Fixed

#### Demo Site
- Demo preview containers now full-width for slider components
- Dark mode backgrounds for sidebar nav-links
- Dark mode backgrounds for AngleInput, FractionInput, UnitInput, ComplexNumberInput, UncertaintyInput
- "Show code" button now uses Mingot Button component
- SymbolPalette demos now all show selection feedback
- Radio component label no longer jumps when selected
- ParameterTree demo shows visual feedback for actions
- Popover demo no longer double-toggles on click
- Import statement styling in component documentation pages

---

## [0.6.0] - 2026-01-20

### Added

#### Scientific Input Components (Phase 4 Complete)
- **AngleInput** - Degrees, radians, gradians with visual arc preview
- **FractionInput** - Numerator/denominator with auto-simplification
- **UnitInput** - Physical units with conversion (Length, Mass, Time, Temperature, Data)
- **ComplexNumberInput** - Rectangular and polar form support
- **UncertaintyInput** - Value ± error with multiple display formats
- **IntervalInput** - Min/max bounds with open/closed notation
- **CoordinateInput** - 2D/3D Cartesian, Polar, Spherical, Cylindrical
- **PointLocator** - Drag-and-drop point positioning on canvas

#### NumberInput Advanced Controls
- Increment/decrement buttons with precision-aware stepping
- Mouse wheel support (`allow_mouse_wheel` prop)
- Copy/paste with automatic format detection
- Drag-to-select precision indicators
- Context menu for format conversion
- Undo/redo for value changes

---

## [0.5.0] - 2026-01-15

### Added

#### Gap Analysis Components (Mantine Parity)
- **Slider** - Single value selection with customizable marks
- **RangeSlider** - Range selection with two draggable thumbs
- **SegmentedControl** - Radio-like segmented button group
- **FileInput** - File selection with drag-and-drop support
- **PinInput** - PIN/OTP code entry with auto-focus
- **Pagination** - Page navigation with customizable boundaries

#### NumberInput Display Features
- Thousand separators (1,234,567)
- Scientific notation formatting (1.23e8)
- Engineering notation (123.4e6) - exponents divisible by 3
- Locale-aware formatting (US, EU, Swiss, Indian)
- Precision indicators showing type and significant digit limits
- Overflow warning when approaching precision limits

---

## [0.4.0] - 2026-01-05

### Added

#### Demo Site
- Interactive component documentation site (Storybook-like)
- Live demos with syntax-highlighted code examples
- Props documentation tables for all components
- Dark/light theme toggle with smooth transitions
- Netlify deployment configuration
- Hot-reload development environment (`demo/` directory)

#### New Components
- **Loader** - Loading spinner with Oval, Dots, and Bars variants
- **Skeleton** - Loading placeholder with shimmer animation
- **SkeletonText** - Multi-line text skeleton helper
- **PasswordInput** - Password field with visibility toggle
- **ActionIcon** - Icon-only button with multiple variants
- **Burger** - Hamburger menu button with animated transform
- **LoadingOverlay** - Full overlay with centered loader

#### Theme System
- CSS variable injection for external styling integration
- Improved dark mode color contrast
- Smooth theme transition animations

### Changed
- Demo site now shows v0.4.0 version badge

## [0.3.0] - 2025-12-XX

### Added
- `NumberInputPrecision::Arbitrary` mode using rust_decimal
- `high-precision` feature flag for optional rust_decimal dependency
- 8 unit tests for arbitrary precision validation
- Support for 28-29 significant digits with exact decimal arithmetic

### Changed
- Updated documentation with rust_decimal examples
- Feature flag patterns for zero-cost abstraction when disabled

## [0.2.0] - 2025-11-XX

### Added
- **NumberInput** component with high-precision support
  - U64, U128, I64, I128 precision types
  - Decimal(u32) for fixed decimal places
  - Real-time validation with ParseError feedback
  - Character-level input filtering
- Validation framework for overflow/underflow detection
- 6 precision-specific unit tests
- HIGH_PRECISION_PROPOSAL.md documentation

## [0.1.0] - 2025-XX-XX

### Added
- Initial release with core Mantine-inspired components
- Theme system with light/dark mode support
- 40+ UI components including:
  - Core: Button, Text, Stack, Group, Container, Divider
  - Form: Input, Textarea, Checkbox, Radio, Select, Switch
  - Layout: AppShell, Card, Grid, Header, Paper, Navbar
  - Navigation: Breadcrumbs, Tabs
  - Overlay: Modal, Drawer, Popover, Tooltip
  - Feedback: Alert, Progress, Badge, Notification
  - Data Display: Accordion, Avatar, Table, Stats, RingProgress
