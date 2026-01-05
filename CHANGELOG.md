# Changelog

All notable changes to Mingot will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
