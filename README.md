# Mingot

**The Leptos UI library for applications that demand mathematical precision.**

[![Crates.io](https://img.shields.io/crates/v/mingot)](https://crates.io/crates/mingot)
[![Documentation](https://docs.rs/mingot/badge.svg)](https://docs.rs/mingot)
[![License](https://img.shields.io/crates/l/mingot)](LICENSE)

## Why Mingot?

Most web UI libraries are built for consumer applications where precision stops at JavaScript's `Number` type (safe integers up to 2^53 - 1). **Mingot is different.**

Built for scientific computing, financial applications, and mathematical software, Mingot provides first-class support for:

- **u64, u128** precision integers
- **Arbitrary-precision** decimals via [rust_decimal](https://docs.rs/rust_decimal) (128-bit, 28-29 significant digits)
- **High-precision decimals** with configurable decimal places
- **Zero precision loss** in user input and display

### The Problem with Standard UI Libraries

```javascript
// JavaScript Number precision limits
9007199254740992 + 1  // 9007199254740992 (WRONG!)
0.1 + 0.2             // 0.30000000000000004 (WRONG!)
```

HTML5 `<input type="number">` inherits these limitations, making standard UI libraries unsuitable for applications requiring mathematical rigor.

### The Mingot Solution

```rust
// Mingot NumberInput with u64 precision
<NumberInput
    precision=NumberInputPrecision::U64
    label="Transaction ID"
    on_valid_change=Callback::new(move |result| {
        // result: Result<String, ParseError>
        // Supports values up to 18,446,744,073,709,551,615
    })
/>

// Arbitrary precision with rust_decimal (requires high-precision feature)
<NumberInput
    precision=NumberInputPrecision::Arbitrary
    label="High-Precision Calculation"
    on_valid_change=Callback::new(move |result: Result<String, ParseError>| {
        // Up to 28-29 significant digits with exact decimal arithmetic
    })
/>
```

## Core Philosophy

**Precision First, Everything Else Second**

1. **No Compromises on Accuracy**: Every component that handles numeric data supports high-precision mathematics
2. **Type Safety**: Rust's type system prevents precision loss at compile time
3. **Validation at Input**: Real-time validation ensures invalid values never enter your system
4. **Arbitrary Precision**: Optional rust_decimal integration for 128-bit decimal arithmetic

## Features

### Ultra-Precision Components

- **NumberInput**: u64, u128, i64, i128, arbitrary-precision number input
- **More precision components coming**: DateInput with nanosecond precision, financial calculators, scientific notation support

### Standard UI Components

Mingot also provides all the components you need for building complete applications:

- **Layout**: Container, Stack, Group, Grid, AppShell
- **Forms**: Input, Textarea, Select, Checkbox, Radio, Switch
- **Navigation**: Navbar, Menu, Breadcrumbs, Tabs
- **Feedback**: Alert, Banner, Modal, Drawer, Notification
- **Data Display**: Table, Card, Badge, Avatar, Stats
- **Typography**: Text with full theming support

### Developer Experience

- **Type-Safe**: Built with Rust for compile-time safety
- **Reactive**: Leverages Leptos's fine-grained reactivity
- **Themeable**: Comprehensive theming system inspired by Mantine UI
- **Well Documented**: Extensive docs with real-world examples
- **Tested**: Comprehensive test suite with precision-focused tests

## Installation

Add Mingot to your `Cargo.toml`:

```toml
[dependencies]
mingot = "0.6.1"
leptos = "0.8"

# Optional: Enable arbitrary-precision support with rust_decimal
mingot = { version = "0.6.1", features = ["high-precision"] }
```

## Quick Start

### Basic Application

```rust
use leptos::prelude::*;
use mingot::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <MingotProvider>
            <Container>
                <Stack spacing="md">
                    <Text size=TextSize::Xl weight=TextWeight::Bold>
                        "Welcome to Mingot"
                    </Text>
                    <Button variant=ButtonVariant::Filled>
                        "Click me"
                    </Button>
                </Stack>
            </Container>
        </MingotProvider>
    }
}
```

### High-Precision Number Input

```rust
use leptos::prelude::*;
use mingot::prelude::*;

#[component]
fn PrecisionDemo() -> impl IntoView {
    let (value, set_value) = create_signal(None::<u64>);
    let (error, set_error) = create_signal(None::<String>);

    view! {
        <NumberInput
            precision=NumberInputPrecision::U64
            label="Enter a large integer"
            description="Supports values up to 18,446,744,073,709,551,615"
            on_valid_change=Callback::new(move |result: Result<String, ParseError>| {
                match result {
                    Ok(val) => {
                        if let Ok(num) = val.parse::<u64>() {
                            set_value.set(Some(num));
                            set_error.set(None);
                        }
                    }
                    Err(e) => {
                        set_error.set(Some(e.to_string()));
                    }
                }
            })
        />

        {move || value.get().map(|v| view! {
            <Text>"Parsed value: " {v.to_string()}</Text>
        })}

        {move || error.get().map(|e| view! {
            <Text color="red">{e}</Text>
        })}
    }
}
```

### Decimal Precision

```rust
<NumberInput
    precision=NumberInputPrecision::Decimal(14)
    label="Volatility"
    description="14 decimal places of precision"
    allow_decimal=true
    min="0"
    max="1"
/>
```

## Use Cases

Mingot is built for applications where precision matters:

### Financial Applications
- Trading platforms with high-frequency calculations
- Cryptocurrency wallets and exchanges
- Accounting software with exact decimal arithmetic
- Risk modeling and portfolio management

### Scientific Computing
- Physical simulations requiring numerical stability
- Statistical analysis with large datasets
- Computational chemistry and physics
- Climate modeling and environmental science

### Engineering & CAD
- Computer-aided design with precise measurements
- Structural analysis and finite element methods
- Manufacturing tolerances and specifications
- Aerospace and automotive engineering calculations

### Mathematical Software
- Computer algebra systems
- Theorem provers and verification tools
- Educational mathematics platforms
- Research and academic applications

## Component Documentation

### NumberInput (Precision Component)

The flagship component of Mingot, designed for high-precision numeric input.

**Precision Types**:
```rust
pub enum NumberInputPrecision {
    U64,           // Unsigned 64-bit (0 to 18,446,744,073,709,551,615)
    U128,          // Unsigned 128-bit (massive range)
    I64,           // Signed 64-bit
    I128,          // Signed 128-bit
    Decimal(u32),  // Fixed decimal places (e.g., Decimal(8) for financial)
    Arbitrary,     // Unlimited precision with Amari (requires feature)
}
```

**Error Handling**:
```rust
pub enum ParseError {
    InvalidFormat(String),
    Overflow(String),
    Underflow(String),
    TooManyDecimals(u32),
    NegativeNotAllowed,
    DecimalNotAllowed,
}
```

**Full API**:
```rust
<NumberInput
    // Precision configuration
    precision=NumberInputPrecision::U64
    min="0"
    max="1000000"

    // Value handling
    value=number_value           // RwSignal<String>
    on_change=on_raw_change      // Callback<String>
    on_valid_change=on_validated // Callback<Result<String, ParseError>>

    // Validation
    allow_negative=false
    allow_decimal=false
    allow_scientific=false

    // Display (coming in Phase 2/3)
    format=NumberInputFormat::Thousand  // 1,234,567
    decimal_separator='.'
    thousand_separator=','

    // Standard form props
    variant=InputVariant::Default
    size=InputSize::Md
    label="Field Label"
    description="Helper text"
    error="Error message"
    placeholder="0"
    disabled=false
    required=false
/>
```

### Standard Components

For complete documentation of all standard components (Button, Input, Select, etc.), see the [full component documentation](COMPONENTS.md).

## Theming

Mingot includes a comprehensive theming system:

```rust
use mingot::{MingotProvider, Theme, ColorSchemeMode};

let custom_theme = Theme {
    color_scheme: ColorSchemeMode::Dark,
    primary_color: "blue",
    // ... customize colors, spacing, typography, etc.
    ..Default::default()
};

view! {
    <MingotProvider theme=Some(custom_theme)>
        // Your app
    </MingotProvider>
}
```

## Roadmap

Mingot's development is organized around enhancing precision capabilities while maintaining a complete component library.

### Phase 1: Foundation ✅
- NumberInput with stdlib precision types (u64, u128, i64, i128, decimal)
- Input filtering and validation
- ParseError type system
- Comprehensive test coverage

### Phase 2: Arbitrary Precision ✅
- Optional `rust_decimal` dependency via feature flag
- `NumberInputPrecision::Arbitrary` mode (128-bit, 28-29 significant digits)
- Zero-cost abstraction when feature disabled

### Phase 3: Demo Site & Display Features ✅
- Interactive component documentation site
- Thousand separators, scientific/engineering notation
- Locale-aware formatting (US, EU, Swiss, Indian)
- Precision indicators and overflow warnings
- Gap analysis components (Slider, RangeSlider, SegmentedControl, FileInput, PinInput, Pagination)

### Phase 4: Scientific Input Components ✅ (Current - v0.6.0)
- **AngleInput**: Degrees, radians, gradians with visual preview
- **FractionInput**: Numerator/denominator with auto-simplification
- **UnitInput**: Physical units with conversion (length, mass, time, temperature, data)
- **ComplexNumberInput**: Rectangular and polar forms
- **UncertaintyInput**: Value ± error with multiple display formats
- NumberInput increment controls with modifier keys (Shift=10x, Ctrl=100x)
- Enhanced paste handling and undo/redo support

### Phase 5: Mathematical Expression & Data Entry
- EquationEditor with LaTeX/MathML output
- MatrixInput, VectorInput, TensorInput
- Parameter manipulation (Mathematica-style)

### Phase 6+: Visualization, Node Graphs, Themes, VFX
See [ROADMAP.md](ROADMAP.md) for detailed feature specifications.

## Architecture

### Precision-First Design Principles

1. **No Silent Precision Loss**: Components never silently coerce to lower precision
2. **Explicit Validation**: All precision conversions are explicit and validated
3. **Type-Safe Boundaries**: Rust's type system enforces precision constraints
4. **User-Visible Errors**: Precision errors surface immediately with clear messages

### Component Patterns

All Mingot components follow patterns documented in [COMPONENT_GUIDELINES.md](COMPONENT_GUIDELINES.md):

- Concrete `Callback<T>` types (not generics)
- Comprehensive HTML5 attribute support
- Consistent variant and size enums
- Themeable with StyleBuilder
- Tested with real-world integration

## Testing

Mingot includes extensive test coverage with special focus on precision:

```bash
# Run all tests
cargo test

# Run precision-specific tests
cargo test number_input

# Run with Amari integration (requires feature)
cargo test --features high-precision
```

Current test suite:
- 195 passing tests
- Comprehensive precision tests for NumberInput and scientific components
- Overflow/underflow detection
- Decimal place validation
- Input filtering verification
- Complex number, fraction, uncertainty, and unit conversion tests

## Contributing

Mingot is built for the Industrial Algebra ecosystem but welcomes contributions from anyone building precision-critical applications.

### Development Priorities

1. **Precision Components**: New components for high-precision numeric input
2. **Amari Integration**: Deeper integration with Amari's mathematical capabilities
3. **Domain-Specific Tools**: Financial, scientific, and engineering-focused components
4. **Performance**: Optimizing precision operations for real-time applications

### Getting Started

```bash
# Clone the repository
git clone https://github.com/Industrial-Algebra/Mingot.git
cd Mingot

# Run tests
cargo test

# Run examples
cargo run --example precision_demo

# Build documentation
cargo doc --open
```

## Real-World Examples

### Cryptocurrency Exchange

```rust
// Handling Satoshi values (Bitcoin's smallest unit)
<NumberInput
    precision=NumberInputPrecision::U64
    label="Amount (Satoshis)"
    description="1 BTC = 100,000,000 Satoshis"
    min="0"
    max="2100000000000000"  // Total Bitcoin supply in Satoshis
/>
```

### Scientific Simulation

```rust
// Physical constants requiring high precision
<NumberInput
    precision=NumberInputPrecision::Decimal(20)
    label="Planck Constant (J⋅s)"
    description="6.62607015 × 10⁻³⁴"
    allow_scientific=true
/>
```

### Financial Trading

```rust
// Stock price with 4 decimal places (standard)
<NumberInput
    precision=NumberInputPrecision::Decimal(4)
    label="Limit Price"
    description="USD per share"
    min="0"
/>
```

## Performance

Mingot's precision components are optimized for real-time applications:

- **Input latency**: < 16ms (60 FPS responsive)
- **Validation overhead**: Minimal (stdlib parsing is fast)
- **WASM binary size**: Optimized with LTO and opt-level='z'
- **Amari integration**: Zero-cost when feature disabled

## Browser Compatibility

- **Desktop**: Chrome, Firefox, Safari, Edge (latest 2 versions)
- **Mobile**: iOS Safari, Chrome Mobile
- **WASM**: All browsers with WebAssembly support

## License

Mingot is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

Choose the license that best suits your project.

## Acknowledgments

- **Mantine UI**: API design inspiration
- **Leptos**: Reactive foundation
- **Amari**: Arbitrary-precision mathematics
- **Industrial Algebra**: Primary development and use cases

## Links

- **Documentation**: https://docs.rs/mingot
- **Crate**: https://crates.io/crates/mingot
- **Repository**: https://github.com/Industrial-Algebra/Mingot
- **Amari**: https://github.com/justinelliottcobb/Amari
- **Leptos**: https://leptos.dev

---

**Built with precision. Built for science.**
