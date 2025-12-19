# Mingot Project Context

**Last Updated**: November 19, 2025
**Version**: 0.1.0 (0.2.0 pending merge)
**Status**: Active Development - Phase 1 Complete, Phase 2 In Progress

---

## Project Identity

**Mingot** is the Leptos UI library for applications that demand mathematical precision.

### Unique Value Proposition

Mingot is positioned as **the only** web UI library with first-class support for:
- u64, u128, i64, i128 precision integers
- Arbitrary-precision arithmetic via Amari integration
- Zero precision loss in numeric input and display
- Type-safe validation preventing silent precision errors

### Target Audience

1. **Scientific Computing**: Researchers, physicists, chemists needing numerical precision
2. **Financial Applications**: Trading platforms, cryptocurrency, accounting systems
3. **Engineering & CAD**: Manufacturing, aerospace, structural analysis
4. **Mathematical Software**: Computer algebra systems, theorem provers, education

### Competitive Differentiation

**Standard UI Libraries**:
- Limited to JavaScript Number (max safe integer: 2^53 - 1)
- Floating-point precision errors (0.1 + 0.2 = 0.30000000000000004)
- No support for u64+ precision

**Mingot**:
- Text-based precision-preserving input
- Rust-side validation with detailed error types
- Optional Amari integration for unlimited precision
- Explicit, user-visible precision handling

---

## Technical Architecture

### Stack

- **Framework**: Leptos 0.8.12 (reactive web framework)
- **Language**: Rust (compile to WASM)
- **Target**: WebAssembly (browser)
- **Precision Engine**: Amari 0.9.10 (optional, Phase 2)

### Project Structure

```
Mingot/
├── src/
│   ├── lib.rs                      # Crate root, public API
│   ├── components/
│   │   ├── mod.rs                  # Component re-exports
│   │   ├── number_input.rs         # ⭐ Precision component (Phase 1)
│   │   ├── input.rs                # Standard text input
│   │   ├── textarea.rs             # Multi-line input
│   │   ├── button.rs               # Button component
│   │   ├── select.rs               # Dropdown selection
│   │   ├── checkbox.rs             # Checkbox input
│   │   ├── switch.rs               # Toggle switch
│   │   ├── radio.rs                # Radio buttons
│   │   ├── table.rs                # Data table
│   │   ├── modal.rs                # Modal dialogs
│   │   ├── drawer.rs               # Side drawer
│   │   ├── alert.rs                # Alert messages
│   │   ├── banner.rs               # Banner notifications
│   │   ├── menu.rs                 # Menu component
│   │   ├── navbar.rs               # Navigation bar
│   │   ├── container.rs            # Layout container
│   │   ├── stack.rs                # Vertical layout
│   │   ├── group.rs                # Horizontal layout
│   │   ├── grid.rs                 # Grid layout
│   │   ├── card.rs                 # Card container
│   │   ├── text.rs                 # Typography
│   │   ├── badge.rs                # Badge labels
│   │   ├── avatar.rs               # User avatars
│   │   ├── progress.rs             # Progress bars
│   │   ├── ring_progress.rs        # Circular progress
│   │   ├── stats.rs                # Statistics display
│   │   ├── accordion.rs            # Collapsible sections
│   │   ├── breadcrumbs.rs          # Breadcrumb navigation
│   │   ├── tabs.rs                 # Tab navigation
│   │   ├── tooltip.rs              # Tooltips
│   │   ├── popover.rs              # Popover overlays
│   │   ├── divider.rs              # Visual dividers
│   │   ├── paper.rs                # Paper surface
│   │   ├── appshell.rs             # App shell layout
│   │   ├── header.rs               # Header component
│   │   ├── footer.rs               # Footer component
│   │   ├── hero.rs                 # Hero section
│   │   ├── notification.rs         # Notifications
│   │   └── error_page.rs           # Error pages
│   ├── theme/
│   │   ├── mod.rs                  # Theme system
│   │   ├── color_scheme.rs         # Light/dark mode
│   │   └── colors.rs               # Color palette
│   ├── utils/
│   │   └── style_builder.rs        # CSS builder utility
│   └── validation/
│       └── validators.rs           # Form validation
├── docs/
│   ├── README.md                   # Main documentation (precision-focused)
│   ├── ROADMAP.md                  # 6-phase development plan
│   ├── COMPONENT_GUIDELINES.md     # Component development patterns
│   ├── HIGH_PRECISION_PROPOSAL.md  # NumberInput design document
│   └── API_IMPROVEMENTS.md         # Integration feedback (working doc)
├── Cargo.toml                      # Package manifest (precision keywords)
└── CONTEXT.md                      # This file
```

### Component Count

**Total Components**: 37
- **Precision Components**: 1 (NumberInput)
- **Form Components**: 6 (Input, Textarea, Select, Checkbox, Switch, Radio)
- **Layout Components**: 8 (Container, Stack, Group, Grid, AppShell, etc.)
- **Navigation Components**: 5 (Navbar, Menu, Breadcrumbs, Tabs, etc.)
- **Feedback Components**: 6 (Alert, Banner, Modal, Drawer, Notification, ErrorPage)
- **Data Display Components**: 7 (Table, Card, Badge, Avatar, Stats, etc.)
- **Overlay Components**: 2 (Tooltip, Popover)
- **Misc Components**: 2 (Divider, Paper, Text)

---

## Current Development Status

### Phase 1: Foundation ✅ COMPLETED

**Commits**: c490834, 21f58ed, f1a7407
**Branch**: `feature/high-precision-number-input`
**PR**: #1 (open)

**Delivered**:
- NumberInput component (551 lines)
  - Precision types: U64, U128, I64, I128, Decimal(u32)
  - Validation with ParseError enum
  - Input filtering based on precision configuration
  - Reuses InputVariant and InputSize from Input component
- Test suite: 6 new tests (59 total, all passing)
- Documentation:
  - HIGH_PRECISION_PROPOSAL.md (3-phase plan)
  - README.md rewrite (precision-first positioning)
  - ROADMAP.md (6-phase vision)
  - Updated Cargo.toml and src/lib.rs

**Key Files Modified**:
- `src/components/number_input.rs` (NEW)
- `src/components/mod.rs` (added number_input module)
- `README.md` (complete rewrite)
- `ROADMAP.md` (NEW)
- `Cargo.toml` (updated description/keywords)
- `src/lib.rs` (updated crate docs)

### Phase 2: Amari Integration 🚧 PLANNED (Next)

**Target Version**: 0.3.0
**Status**: Design complete, implementation pending

**Planned Deliverables**:
- Optional `amari` dependency (feature flag: `high-precision`)
- `NumberInputPrecision::Arbitrary` variant
- Callbacks returning `Result<amari::Number, ParseError>`
- Integration tests with Amari types
- Updated documentation with Amari examples

**Blockers**: None (Phase 1 must merge first)

---

## Key Technical Decisions

### 1. Callback API Pattern

**Decision**: Use concrete `Callback<T>` types instead of generic type parameters

**Rationale**:
- Generic `Option<F> where F: Fn(...)` causes type inference failures
- When optional callbacks are None, Rust cannot infer type F
- Concrete types require explicit `Callback::new()` wrapper but compile reliably

**Example**:
```rust
// ✅ Chosen approach
#[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>

// ❌ Rejected approach (type inference issues)
#[prop(optional)] on_click: Option<F> where F: Fn(ev::MouseEvent)
```

**Documented In**: COMPONENT_GUIDELINES.md

### 2. HTML5 Attribute Support

**Decision**: Comprehensively support HTML5 form attributes

**Rationale**:
- Missing `step` attribute caused production failures (decimal inputs rejected)
- Browser defaults (step="1") incompatible with precision requirements
- Accessibility features (autocomplete) require HTML5 attributes

**Added Attributes**:
- Input: step, min, max, pattern, autocomplete, maxlength, minlength
- Textarea: maxlength, minlength, autocomplete

**Documented In**: COMPONENT_GUIDELINES.md, API_IMPROVEMENTS.md

### 3. NumberInput Precision Approach

**Decision**: Text-based input with Rust-side validation

**Rationale**:
- HTML5 `<input type="number">` limited to JavaScript Number type
- Text input preserves exact user entry (no auto-coercion)
- Rust validation provides detailed, type-specific errors
- Enables unlimited precision via Amari integration

**Trade-offs**:
- ✅ No precision loss
- ✅ User sees exactly what they typed
- ✅ Detailed error messages
- ❌ More verbose than native number input
- ❌ Requires explicit validation callbacks

**Documented In**: HIGH_PRECISION_PROPOSAL.md

### 4. Feature Flag for Amari

**Decision**: Make Amari integration optional via feature flag

**Rationale**:
- Zero-cost abstraction: Users not needing Amari don't pay for it
- Keeps WASM binary small for standard use cases
- Allows gradual adoption of arbitrary precision

**Implementation**:
```toml
[features]
default = ["csr"]
high-precision = ["amari"]

[dependencies]
amari = { version = "0.9.10", optional = true }
```

**Status**: Planned for Phase 2

### 5. Precision-First Component Evolution

**Decision**: Systematically extend components with precision variants

**Rationale**:
- Every numeric component should have a precision-aware version
- Standard components remain for non-precision use cases
- Progressive enhancement: Input → NumberInput, etc.

**Future Components**:
- CurrencyInput (Phase 4)
- ScientificInput (Phase 4)
- DateTimeInput (Phase 4)
- MatrixInput (Phase 5)

**Documented In**: ROADMAP.md

---

## Integration Points

### Leptos 0.8.12

**Usage**:
- All components use `#[component]` macro
- Reactive signals: `Signal<T>`, `RwSignal<T>`
- Event handling: `Callback<T>` for prop callbacks
- Theming: Context API via `use_theme()`

**Key Patterns**:
```rust
use leptos::prelude::*;

#[component]
pub fn MyComponent(
    #[prop(optional)] value: Signal<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    view! { /* ... */ }
}
```

### Amari 0.9.10 (Planned Phase 2)

**Integration Points**:
- `amari::Number` for arbitrary precision
- Parsing: String → Number via `from_str()`
- Validation: Amari's precision capabilities
- Display: Number → String with formatting

**Conditional Compilation**:
```rust
#[cfg(feature = "high-precision")]
use amari::Number;

#[cfg(feature = "high-precision")]
pub enum NumberInputPrecision {
    // ...
    Arbitrary,  // Uses Amari::Number
}
```

### Industrial Algebra Ecosystem

**Position**: Mingot is the UI layer for Industrial Algebra applications

**Related Projects**:
- **Amari**: Mathematical computing engine (backend)
- **Mingot**: UI components (frontend)
- **Ultramarine-Red**: Real-world application (integration testing)

---

## Testing Strategy

### Current Test Coverage

**Total Tests**: 59
**NumberInput Tests**: 6
**Success Rate**: 100% passing

**Test Categories**:
1. **Component Tests**: Theme, color scheme, styling
2. **Validation Tests**: Number parsing, overflow detection
3. **Utility Tests**: StyleBuilder, validators
4. **Precision Tests**: NumberInput-specific

**Precision Test Cases**:
```rust
test_validate_u64_success          // Valid u64 parsing
test_validate_u64_overflow         // Overflow detection
test_validate_i64_success          // Signed integers
test_validate_decimal_precision    // Decimal place validation
test_add_thousand_separators       // Formatting functions
test_is_valid_char                 // Input filtering
```

### Pre-commit Checks

**Automated** (via git hooks):
1. `cargo fmt` - Code formatting
2. `cargo clippy` - Linting
3. `cargo test` - All tests must pass

**Status**: All checks passing ✅

### Integration Testing

**Strategy**: Test against real Industrial Algebra applications
- **Ultramarine-Red**: Primary integration testing platform
- **API_IMPROVEMENTS.md**: Tracks integration feedback
- **Iteration cycles**: Design → Implement → Integrate → Feedback

**Latest Findings**:
- HTML5 attribute gap (resolved in Phase 1)
- Callback API issues (resolved with concrete types)

---

## Known Issues & Limitations

### Current Limitations

1. **No Amari Integration Yet** (Phase 2 pending)
   - NumberInputPrecision::Arbitrary not implemented
   - Callbacks return String, not amari::Number

2. **Formatting Not Active** (Phase 3 planned)
   - Thousand separator formatting functions exist but unused
   - Scientific notation conversion not wired up
   - Auto-formatting on blur not implemented

3. **No Increment/Decrement Controls** (Phase 3 planned)
   - Manual keyboard entry only
   - No +/- buttons for stepping

4. **Limited ARIA Support**
   - Basic accessibility present
   - Full WCAG 2.1 AA compliance planned for 1.0

### Active Branch Status

**Branch**: `feature/high-precision-number-input`
**Commits Ahead of Main**: 5
**Uncommitted Changes**: API_IMPROVEMENTS.md (working doc, not committed)

**Ready for Merge**: Yes
**Blockers**: Branch protection needs to be configured on main

---

## Documentation Inventory

### Primary Documentation

1. **README.md** (495 lines)
   - Project overview and positioning
   - Quick start examples
   - Component documentation
   - Use cases and real-world examples
   - Installation and theming

2. **ROADMAP.md** (NEW - this session)
   - 6-phase development plan
   - Phase 1-6 objectives and deliverables
   - Long-term vision through 2027
   - Success metrics and milestones

3. **COMPONENT_GUIDELINES.md** (383 lines)
   - Callback props pattern
   - HTML5 attribute requirements
   - Component API design patterns
   - Testing strategy
   - Common pitfalls and solutions

4. **HIGH_PRECISION_PROPOSAL.md** (409 lines)
   - Problem statement (HTML5 limitations)
   - Proposed NumberInput API
   - 3-phase implementation plan
   - Technical considerations
   - Accessibility requirements

5. **API_IMPROVEMENTS.md** (working document)
   - Integration feedback from Ultramarine-Red
   - Discovered issues and iterations
   - Not committed (living document)

### Code Documentation

- **src/lib.rs**: Crate-level documentation with precision examples
- **Component modules**: Each component has doc comments
- **Public APIs**: Documented with examples

---

## Build & Release Process

### Development Workflow

1. **Feature Branch**: Create from main
2. **Implementation**: TDD with tests first
3. **Pre-commit Checks**: Automatic via git hooks
4. **PR Creation**: Use `gh pr create` with detailed description
5. **Review**: (Branch protection pending)
6. **Merge**: Squash or merge commit
7. **Version Bump**: Update Cargo.toml
8. **Tag**: `git tag v0.X.0`
9. **Publish**: `cargo publish` (when ready for crates.io)

### Version Strategy

**Current**: 0.1.0
**Next**: 0.2.0 (Phase 1 merge)
**Pre-1.0**: Minor bumps for phases (0.3.0, 0.4.0, etc.)
**1.0.0 Criteria**: All Phase 1-3 complete, production-tested, API stable

### Distribution Channels

- **GitHub**: Source code, issues, PRs
- **crates.io**: Package distribution (when published)
- **docs.rs**: Automated documentation

---

## Dependencies

### Production Dependencies

```toml
[dependencies]
leptos = { version = "0.8.12", features = ["csr"] }
leptos_meta = "0.8.5"
leptos_router = "0.8.9"
web-sys = { version = "0.3", features = ["HtmlElement", "Window", "Document", "CssStyleDeclaration"] }
wasm-bindgen = "0.2"
```

### Development Dependencies

```toml
[dev-dependencies]
wasm-bindgen-test = "0.3"
```

### Planned Dependencies (Phase 2)

```toml
[dependencies]
amari = { version = "0.9.10", optional = true }  # Feature-flagged
```

---

## Performance Characteristics

### NumberInput Performance

**Input Latency**: < 16ms (60 FPS target)
**Validation Overhead**: Minimal (stdlib parsing is fast)
**Memory**: String storage, no large allocations

### WASM Binary Size

**Current** (without Amari):
- Optimized with `opt-level = 'z'`
- LTO enabled in release profile
- Estimated: ~200-300KB (typical Leptos app)

**With Amari** (Phase 2 estimate):
- Additional ~50-100KB for Amari
- Zero-cost when feature disabled

---

## Community & Ecosystem

### Target Communities

1. **Rust Web Developers**: Leptos ecosystem
2. **Scientific Computing**: Researchers using Rust
3. **Financial Technology**: Trading/crypto developers
4. **Industrial Algebra Users**: Primary stakeholders

### Contribution Areas

**Welcomed Contributions**:
- Precision components (CurrencyInput, ScientificInput, etc.)
- Amari integration enhancements
- Accessibility improvements
- Performance optimizations
- Documentation and examples

**Contribution Process**:
- Open issue for discussion
- Fork and create feature branch
- Follow COMPONENT_GUIDELINES.md
- Submit PR with tests and docs

---

## Next Immediate Steps

### Before Merge

1. **Branch Protection**: Configure on main branch (blocker)
2. **Optional**: Create CHANGELOG.md
3. **Optional**: Integration test in Ultramarine-Red

### After Merge (Phase 2 Start)

1. Merge PR #1 to main
2. Tag v0.2.0
3. Create `feature/amari-integration` branch
4. Add Amari dependency with feature flag
5. Implement NumberInputPrecision::Arbitrary
6. Add Amari-specific callbacks and validation
7. Write integration tests with Amari types
8. Update documentation

---

## Environment & Tools

### Development Environment

- **OS**: Linux (Ubuntu/Debian-based)
- **Rust Version**: 1.75+ (Amari requirement)
- **IDE**: VS Code (typical)
- **Tools**: cargo, rustfmt, clippy

### CI/CD

**Status**: Git hooks for local checks
**Planned**: GitHub Actions for:
- Automated testing on PR
- Cross-browser testing (WASM)
- Documentation deployment
- Release automation

---

## Key Contacts & Resources

### Project Leadership

- **Organization**: Industrial Algebra
- **Repository**: https://github.com/Industrial-Algebra/Mingot
- **Primary Use Case**: Ultramarine-Red integration

### Related Resources

- **Leptos**: https://leptos.dev
- **Amari**: https://github.com/justinelliottcobb/Amari
- **Amari Docs**: https://docs.rs/amari/0.9.10

---

## Glossary

**Amari**: Advanced mathematical computing library with geometric algebra, tropical algebra, and automatic differentiation

**Arbitrary Precision**: Unlimited precision mathematics, only bounded by available memory

**Leptos**: Reactive web framework for Rust, compiles to WebAssembly

**ParseError**: Custom error type for precision validation failures

**Precision Loss**: Unintended reduction in numeric accuracy, often silent in web applications

**WASM**: WebAssembly, compilation target for running Rust in browsers

---

## Project Metrics (Current)

**Lines of Code**: ~15,000 (estimated)
**Components**: 37
**Tests**: 59 (100% passing)
**Documentation**: 5 major documents
**Open PRs**: 1
**Version**: 0.1.0 → 0.2.0 (pending)

---

**This document serves as a snapshot of Mingot's current state. Update after major milestones.**
