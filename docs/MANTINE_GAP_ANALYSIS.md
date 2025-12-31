# Mantine vs Mingot Gap Analysis

**Date**: December 2025
**Mingot Version**: 0.2.0
**Mantine Reference**: v8.x (120+ components, 50+ hooks)

This document compares Mingot's current component library against [Mantine UI](https://mantine.dev/), the React component library that inspired Mingot's design.

---

## Executive Summary

| Metric | Mantine | Mingot | Coverage |
|--------|---------|--------|----------|
| Core Components | 100+ | 40 | ~40% |
| Hooks | 50+ | 0 | 0% |
| Date Components | 14 | 0 | 0% |
| Chart Components | 13 | 0 | 0% |
| Form Library | Yes | Partial | - |

**Mingot Unique Features**:
- `NumberInput` with u64/u128/i64/i128 precision (Mantine limited to f64)
- `NumberInputPrecision::Arbitrary` via rust_decimal (Phase 2)
- Precision-first architecture for scientific/financial applications

---

## Component Comparison by Category

### Layout Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| AppShell | `appshell.rs` | ✅ Have | - | |
| AspectRatio | - | ❌ Missing | Low | CSS-only, easy to add |
| Center | - | ❌ Missing | Low | Simple flexbox wrapper |
| Container | `container.rs` | ✅ Have | - | |
| Flex | - | ❌ Missing | Medium | Generic flexbox component |
| Grid | `grid.rs` | ✅ Have | - | |
| Group | `group.rs` | ✅ Have | - | Horizontal layout |
| SimpleGrid | - | ❌ Missing | Low | Simplified grid |
| Space | - | ❌ Missing | Low | Spacing utility |
| Stack | `stack.rs` | ✅ Have | - | Vertical layout |

**Layout Coverage**: 5/10 (50%)

---

### Input Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| AngleSlider | - | ❌ Missing | Low | Specialized |
| Checkbox | `checkbox.rs` | ✅ Have | - | |
| Chip | - | ❌ Missing | Medium | Selection chip |
| ColorInput | - | ❌ Missing | Medium | Color picker input |
| ColorPicker | - | ❌ Missing | Medium | Color selection |
| Fieldset | - | ❌ Missing | Medium | Form grouping |
| FileInput | - | ❌ Missing | High | File upload |
| Input | `input.rs` | ✅ Have | - | Base input |
| JsonInput | - | ❌ Missing | Low | JSON editor |
| NativeSelect | - | ❌ Missing | Low | Native `<select>` |
| NumberInput | `number_input.rs` | ✅ Have | - | **Enhanced**: u64+ precision |
| PasswordInput | - | ❌ Missing | High | Show/hide password |
| PinInput | - | ❌ Missing | Medium | OTP/PIN entry |
| Radio | `radio.rs` | ✅ Have | - | |
| RangeSlider | - | ❌ Missing | Medium | Dual-handle slider |
| Rating | - | ❌ Missing | Low | Star rating |
| SegmentedControl | - | ❌ Missing | High | Toggle buttons |
| Slider | - | ❌ Missing | High | Single slider |
| Switch | `switch.rs` | ✅ Have | - | |
| Textarea | `textarea.rs` | ✅ Have | - | |
| TextInput | `input.rs` | ✅ Have | - | Same as Input |

**Input Coverage**: 8/21 (38%)

**High Priority Gaps**:
- `FileInput` - Essential for forms
- `PasswordInput` - Common auth pattern
- `SegmentedControl` - Popular toggle pattern
- `Slider` - Numeric input alternative

---

### Combobox Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| Autocomplete | - | ❌ Missing | High | Search suggestions |
| Combobox | - | ❌ Missing | High | Base for selects |
| MultiSelect | - | ❌ Missing | High | Multiple selection |
| Pill | - | ❌ Missing | Medium | Tag display |
| PillsInput | - | ❌ Missing | Medium | Tag input |
| Select | `select.rs` | ✅ Have | - | Single select |
| TagsInput | - | ❌ Missing | Medium | Tag entry |

**Combobox Coverage**: 1/7 (14%)

**Critical Gap**: Autocomplete and MultiSelect are commonly needed

---

### Button Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| ActionIcon | - | ❌ Missing | High | Icon-only button |
| Button | `button.rs` | ✅ Have | - | |
| CloseButton | - | ❌ Missing | Medium | X button |
| CopyButton | - | ❌ Missing | Low | Copy to clipboard |
| FileButton | - | ❌ Missing | Medium | File trigger |
| UnstyledButton | - | ❌ Missing | Low | Reset button |

**Button Coverage**: 1/6 (17%)

**High Priority Gap**: `ActionIcon` is used extensively in toolbars/actions

---

### Navigation Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| Anchor | - | ❌ Missing | Medium | Styled link |
| Breadcrumbs | `breadcrumbs.rs` | ✅ Have | - | |
| Burger | - | ❌ Missing | High | Mobile menu icon |
| NavLink | - | ❌ Missing | High | Navigation item |
| Pagination | - | ❌ Missing | High | Page navigation |
| Stepper | - | ❌ Missing | Medium | Multi-step wizard |
| TableOfContents | - | ❌ Missing | Low | TOC navigation |
| Tabs | `tabs.rs` | ✅ Have | - | |
| Tree | - | ❌ Missing | Low | Tree view |

**Navigation Coverage**: 2/9 (22%)

**High Priority Gaps**:
- `Burger` - Essential for responsive nav
- `NavLink` - Sidebar navigation
- `Pagination` - Data tables

---

### Feedback Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| Alert | `alert.rs` | ✅ Have | - | |
| Loader | - | ❌ Missing | High | Loading spinner |
| Notification | `notification.rs` | ✅ Have | - | |
| Progress | `progress.rs` | ✅ Have | - | |
| RingProgress | `ring_progress.rs` | ✅ Have | - | |
| SemiCircleProgress | - | ❌ Missing | Low | Half-circle progress |
| Skeleton | - | ❌ Missing | High | Loading placeholder |

**Feedback Coverage**: 4/7 (57%)

**High Priority Gaps**:
- `Loader` - Essential loading indicator
- `Skeleton` - Modern loading UX

---

### Overlay Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| Affix | - | ❌ Missing | Low | Fixed position |
| Dialog | - | ❌ Missing | Medium | Simple dialog |
| Drawer | `drawer.rs` | ✅ Have | - | |
| FloatingIndicator | - | ❌ Missing | Low | Floating highlight |
| HoverCard | - | ❌ Missing | Medium | Hover popup |
| LoadingOverlay | - | ❌ Missing | High | Full overlay loader |
| Menu | `menu.rs` | ✅ Have | - | |
| Modal | `modal.rs` | ✅ Have | - | |
| Overlay | - | ❌ Missing | Medium | Backdrop |
| Popover | `popover.rs` | ✅ Have | - | |
| Tooltip | `tooltip.rs` | ✅ Have | - | |

**Overlay Coverage**: 6/11 (55%)

**High Priority Gap**: `LoadingOverlay` for async operations

---

### Data Display Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| Accordion | `accordion.rs` | ✅ Have | - | |
| Avatar | `avatar.rs` | ✅ Have | - | |
| BackgroundImage | - | ❌ Missing | Low | Image container |
| Badge | `badge.rs` | ✅ Have | - | |
| Card | `card.rs` | ✅ Have | - | |
| ColorSwatch | - | ❌ Missing | Low | Color preview |
| Image | - | ❌ Missing | Medium | Image component |
| Indicator | - | ❌ Missing | Medium | Badge indicator |
| Kbd | - | ❌ Missing | Low | Keyboard key |
| NumberFormatter | - | ❌ Missing | High | Number display |
| Spoiler | - | ❌ Missing | Low | Show more/less |
| ThemeIcon | - | ❌ Missing | Medium | Icon in circle |
| Timeline | - | ❌ Missing | Medium | Timeline view |

**Data Display Coverage**: 5/13 (38%)

**High Priority Gap**: `NumberFormatter` for precision display (pairs with NumberInput)

---

### Typography Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| Blockquote | - | ❌ Missing | Low | Quote styling |
| Code | - | ❌ Missing | Medium | Code display |
| Highlight | - | ❌ Missing | Low | Text highlight |
| List | - | ❌ Missing | Medium | Styled lists |
| Mark | - | ❌ Missing | Low | Mark text |
| Table | `table.rs` | ✅ Have | - | |
| Text | `text.rs` | ✅ Have | - | |
| Title | - | ❌ Missing | Medium | Heading component |
| TypographyStylesProvider | - | ❌ Missing | Low | Rich text styles |

**Typography Coverage**: 2/9 (22%)

---

### Miscellaneous Components

| Mantine | Mingot | Status | Priority | Notes |
|---------|--------|--------|----------|-------|
| Box | - | ❌ Missing | Low | Base element |
| Collapse | - | ❌ Missing | Medium | Collapsible |
| Divider | `divider.rs` | ✅ Have | - | |
| FocusTrap | - | ❌ Missing | Medium | Focus management |
| Paper | `paper.rs` | ✅ Have | - | |
| Portal | - | ❌ Missing | Medium | Portal rendering |
| ScrollArea | - | ❌ Missing | Medium | Custom scrollbar |
| Transition | - | ❌ Missing | Medium | Animations |
| VisuallyHidden | - | ❌ Missing | Low | A11y utility |

**Misc Coverage**: 2/9 (22%)

---

### Mingot-Only Components

These components exist in Mingot but not in Mantine (or are significantly different):

| Component | Description |
|-----------|-------------|
| `banner.rs` | Full-width banner (Mantine has Alert) |
| `error_page.rs` | Error page templates |
| `footer.rs` | Page footer |
| `header.rs` | Page header |
| `hero.rs` | Hero section |
| `navbar.rs` | Navigation bar |
| `stats.rs` | Statistics display |

---

## Extension Packages (Not in @mantine/core)

### @mantine/dates - Date Components

| Component | Mingot | Priority | Notes |
|-----------|--------|----------|-------|
| Calendar | ❌ | Medium | Month view |
| DateInput | ❌ | High | Date text input |
| DatePicker | ❌ | High | Date selection |
| DatePickerInput | ❌ | High | Combined picker |
| DateTimePicker | ❌ | Medium | Date + time |
| MiniCalendar | ❌ | Low | Compact calendar |
| MonthPicker | ❌ | Low | Month selection |
| MonthPickerInput | ❌ | Low | Month input |
| TimeGrid | ❌ | Low | Time grid |
| TimeInput | ❌ | Medium | Time text input |
| TimePicker | ❌ | Medium | Time selection |
| YearPicker | ❌ | Low | Year selection |
| YearPickerInput | ❌ | Low | Year input |

**Date Coverage**: 0/13 (0%)

---

### @mantine/charts - Chart Components

| Component | Mingot | Priority | Notes |
|-----------|--------|----------|-------|
| AreaChart | ❌ | Medium | |
| BarChart | ❌ | High | |
| BubbleChart | ❌ | Low | |
| CompositeChart | ❌ | Low | |
| DonutChart | ❌ | Medium | |
| FunnelChart | ❌ | Low | |
| Heatmap | ❌ | Low | |
| LineChart | ❌ | High | |
| PieChart | ❌ | Medium | |
| RadarChart | ❌ | Low | |
| RadialBarChart | ❌ | Low | |
| ScatterChart | ❌ | Medium | |
| Sparkline | ❌ | Medium | |

**Chart Coverage**: 0/13 (0%)

---

### Other Mantine Extensions

| Package | Description | Mingot | Priority |
|---------|-------------|--------|----------|
| @mantine/form | Form state management | Partial (validation) | High |
| @mantine/hooks | 50+ utility hooks | ❌ | Medium |
| @mantine/notifications | Toast system | Partial | Medium |
| @mantine/spotlight | Command palette (Ctrl+K) | ❌ | Low |
| @mantine/code-highlight | Syntax highlighting | ❌ | Low |
| @mantine/tiptap | Rich text editor | ❌ | Low |
| @mantine/dropzone | File drag & drop | ❌ | Medium |
| @mantine/carousel | Image carousel | ❌ | Low |
| @mantine/nprogress | Page progress | ❌ | Low |
| @mantine/modals | Modal manager | ❌ | Medium |

---

## Priority Gap Summary

### Critical (Should have for basic apps)

1. **Loader** - Loading spinner
2. **Skeleton** - Loading placeholder
3. **PasswordInput** - Auth forms
4. **FileInput** - File uploads
5. **Autocomplete** - Search/suggestions
6. **MultiSelect** - Multi-selection
7. **ActionIcon** - Icon buttons
8. **Pagination** - Data navigation
9. **Burger** - Mobile nav
10. **LoadingOverlay** - Async feedback

### High (Common use cases)

11. **SegmentedControl** - Toggle groups
12. **Slider** - Numeric selection
13. **NavLink** - Sidebar nav
14. **NumberFormatter** - Display precision numbers
15. **DatePickerInput** - Date selection
16. **Form hooks** - Form state management

### Medium (Nice to have)

17. **Chip** - Selection chips
18. **ColorInput/ColorPicker** - Color selection
19. **PinInput** - OTP entry
20. **RangeSlider** - Range selection
21. **Stepper** - Wizards
22. **HoverCard** - Rich tooltips
23. **Timeline** - Event timeline
24. **Code** - Code display
25. **Collapse** - Collapsible sections
26. **ScrollArea** - Custom scrollbars

---

## Recommended Implementation Order

### Phase 3A: Essential UI (Next)

Focus on components needed for basic web applications:

1. `Loader` - Simple, high impact
2. `Skeleton` - Modern loading UX
3. `PasswordInput` - Extends Input
4. `ActionIcon` - Icon button variant
5. `Burger` - Mobile menu trigger
6. `LoadingOverlay` - Async operations

### Phase 3B: Forms Enhancement

7. `FileInput` - File handling
8. `Slider` / `RangeSlider` - Numeric input alternative
9. `SegmentedControl` - Button group toggle
10. `PinInput` - OTP/PIN entry

### Phase 3C: Advanced Selection

11. `Autocomplete` - Search suggestions
12. `MultiSelect` - Multiple selection
13. `Combobox` - Base combobox logic
14. `TagsInput` - Tag entry

### Phase 4: Data & Navigation

15. `Pagination` - Table navigation
16. `NavLink` - Sidebar items
17. `NumberFormatter` - Precision display
18. `Stepper` - Multi-step flows

### Phase 5: Dates (Optional Package)

19. `DateInput` - Date text entry
20. `DatePicker` - Date selection
21. `DatePickerInput` - Combined
22. `TimeInput` - Time entry

### Phase 6: Charts (Optional Package)

23. `LineChart` - Basic charting
24. `BarChart` - Bar charts
25. `DonutChart` - Pie/donut charts

---

## Mingot Differentiation Strategy

While achieving Mantine parity is valuable, Mingot should maintain its unique positioning:

### Core Differentiators

1. **Precision-First**: Every numeric component supports u64+ precision
2. **Type Safety**: Rust's type system prevents precision loss
3. **WASM Performance**: Native Rust performance in browser
4. **Scientific Focus**: Components designed for scientific/financial apps

### Unique Component Opportunities

| Component | Description | Mantine Equivalent |
|-----------|-------------|-------------------|
| `CurrencyInput` | Multi-currency with exact decimals | NumberInput (limited) |
| `ScientificInput` | Scientific notation entry | None |
| `MatrixInput` | Matrix data entry | None |
| `FormulaInput` | Mathematical expression entry | None |
| `PrecisionSlider` | High-precision slider | Slider (f64 only) |
| `PrecisionFormatter` | Arbitrary precision display | NumberFormatter (limited) |

---

## References

- [Mantine Documentation](https://mantine.dev/)
- [Mantine GitHub](https://github.com/mantinedev/mantine)
- [Mantine UI Templates](https://ui.mantine.dev/)

---

**Next Steps**: Review this analysis and prioritize which components to implement in Phase 3.
