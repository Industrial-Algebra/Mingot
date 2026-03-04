# Mingot Roadmap

**Vision**: Make Mingot the definitive UI library for precision-critical applications, with first-class support for arbitrary-precision mathematics.

## Guiding Principles

1. **Precision is Non-Negotiable**: Every component that touches numeric data must support u64+ precision
2. **Arbitrary Precision First**: rust_decimal for 128-bit decimals, with future Amari integration for specialized math
3. **Zero Precision Loss**: No silent coercions, no rounding without explicit user control
4. **Scientific Rigor**: Components designed with input from scientists, engineers, and mathematicians
5. **Production-Ready**: Performance and reliability suitable for high-stakes applications

---

## Phase 1: Foundation ✅ COMPLETED

**Status**: Merged to main (v0.2.0)
**Timeline**: November 2025

### Deliverables

- [x] NumberInput component with stdlib precision types
  - [x] U64, U128, I64, I128 support
  - [x] Decimal(u32) for fixed decimal places
  - [x] Real-time validation with ParseError types
  - [x] Character-level input filtering
- [x] Validation framework
  - [x] Overflow/underflow detection
  - [x] Decimal place validation
  - [x] Negative/decimal/scientific notation control
- [x] Test suite
  - [x] 6 precision-specific tests
  - [x] Overflow/underflow edge cases
  - [x] Input filtering verification
- [x] Documentation
  - [x] HIGH_PRECISION_PROPOSAL.md
  - [x] Component API documentation
  - [x] Real-world usage examples

### Key Achievements

- Established precision-first architecture
- Proved viability of text-based high-precision input
- Created reusable validation patterns
- Zero dependencies beyond Leptos

---

## Phase 2: Arbitrary Precision ✅ COMPLETED

**Status**: Complete
**Completed**: December 2025
**Version**: 0.3.0

### Key Decision: rust_decimal vs Amari

During implementation, we discovered that Amari's type system focuses on specialized mathematical structures:
- `DualNumber` - Automatic differentiation (a + bε where ε² = 0)
- `TropicalNumber` - Max-plus semiring algebra
- `Scalar/Multivector` - Geometric algebra (Clifford algebras)

None of these provide arbitrary-precision decimal arithmetic. The assumed `amari::Number` type does not exist.

**Solution**: Integrated `rust_decimal` (v1.39) instead, which provides:
- 128-bit fixed-point decimal representation
- Up to 28-29 significant digits
- Exact decimal arithmetic (no floating-point errors)
- `FromStr` parsing for validation
- WASM compatibility

### Deliverables

#### Core Integration
- [x] Add `rust_decimal` as optional dependency with feature flag
- [x] Implement `NumberInputPrecision::Arbitrary` mode
- [x] Validation using rust_decimal's parsing capabilities
- [x] Zero-cost abstraction when feature disabled

#### Testing
- [x] 8 unit tests with `#[cfg(feature = "high-precision")]`
- [x] Large number validation (28-29 significant digits)
- [x] Negative number handling
- [x] Separator stripping (commas, underscores)

#### Documentation
- [x] Updated HIGH_PRECISION_PROPOSAL.md with decision rationale
- [x] Updated README.md with rust_decimal examples
- [x] Feature flag usage patterns

### Technical Specifications

**Feature Flag**:
```toml
[features]
default = ["csr"]
high-precision = ["rust_decimal"]

[dependencies]
rust_decimal = { version = "1.39", optional = true, default-features = false }
```

**API Example**:
```rust
#[cfg(feature = "high-precision")]
<NumberInput
    precision=NumberInputPrecision::Arbitrary
    label="High-Precision Value"
    on_valid_change=Callback::new(move |result: Result<String, ParseError>| {
        // Up to 28-29 significant digits with exact decimal arithmetic
    })
/>
```

### Future Amari Integration

Amari types may be added in future phases for specialized use cases:
- `DualNumber` inputs for automatic differentiation workflows
- `Scalar` inputs for geometric algebra applications
- These would be additional precision variants, not replacements for rust_decimal

---

## Phase 3A: Demo Site & UI Components ✅ COMPLETED

**Status**: Complete
**Completed**: January 2026
**Version**: 0.4.0

### Objectives

Build an interactive component documentation site and close gaps with Mantine UI library.

### Deliverables

#### Demo Site
- [x] Storybook-like component documentation site
- [x] Interactive demos with live code examples
- [x] Props documentation tables
- [x] Syntax-highlighted code blocks (highlight.js)
- [x] Dark/light theme support with CSS variable injection
- [x] Netlify deployment configuration
- [x] Hot-reload development environment

#### New UI Components (Mantine Gap Analysis)
- [x] **Loader** - Loading spinner with Oval, Dots, Bars variants
- [x] **Skeleton** - Loading placeholder with shimmer animation
- [x] **SkeletonText** - Multi-line text skeleton helper
- [x] **PasswordInput** - Password field with visibility toggle
- [x] **ActionIcon** - Icon-only button with multiple variants
- [x] **Burger** - Hamburger menu button with animated transform
- [x] **LoadingOverlay** - Full overlay with centered loader

#### Theme System Improvements
- [x] CSS variable injection for demo site theming
- [x] Proper dark mode color contrast
- [x] Smooth theme transitions

### Technical Highlights

- Demo site uses Mingot as path dependency for dogfooding
- 46 components now documented with interactive examples
- Zero external JS dependencies (except highlight.js CDN)
- Full CSR (client-side rendering) for static deployment

---

## Phase 3B: Gap Analysis & Precision Display ✅ COMPLETED

**Status**: Complete
**Completed**: January 2026
**Version**: 0.5.0

### Objectives

Close component gaps with Mantine UI library and add precision display features to NumberInput.

### Deliverables

#### Gap Analysis Components (Mantine Parity)
- [x] **Slider** - Single value selection with customizable marks
- [x] **RangeSlider** - Range selection with two draggable thumbs
- [x] **SegmentedControl** - Radio-like segmented button group
- [x] **FileInput** - File selection with drag-and-drop support
- [x] **PinInput** - PIN/OTP code entry with auto-focus
- [x] **Pagination** - Page navigation with customizable boundaries

#### NumberInput Display Features
- [x] Thousand separators (1,234,567)
- [x] Scientific notation formatting (1.23e8)
- [x] Engineering notation (123.4e6) - exponents divisible by 3
- [x] Locale-aware formatting (US, EU, Swiss, Indian)
- [x] Precision indicators (shows type and significant digit limits)
- [x] Overflow warning (approaching precision limits)
- [x] Auto-scientific notation threshold for large values
- [x] ARIA labels for precision indicators

### API Example

```rust
<NumberInput
    precision=NumberInputPrecision::Decimal(14)

    // Formatting
    format=NumberInputFormat::Engineering
    locale=NumberInputLocale::EU

    // Visual indicators
    show_precision_indicator=true
    show_overflow_warning=true
    auto_scientific_threshold=Some(1_000_000_000)
/>
```

---

## Phase 4: Scientific Input Components ✅ COMPLETED

**Status**: Complete
**Completed**: January 2026
**Version**: 0.6.0

### Objectives

Build specialized input components for scientific computing and complete NumberInput's advanced precision controls. Inspired by [Jupyter Widgets](https://ipywidgets.readthedocs.io/), [Mathematica Manipulate](https://reference.wolfram.com/language/ref/Manipulate.html), and [PyQtGraph](https://www.pyqtgraph.org/).

### NumberInput Advanced Controls

Complete the precision-aware interactive controls for NumberInput.

#### Increment/Decrement Controls
- [x] +/- buttons with precision-aware stepping
- [x] Configurable step size (e.g., step="0.0001")
- [x] Keyboard shortcuts (Arrow up/down)
- [x] Modifier keys (Shift = 10x, Ctrl = 100x)
- [x] Mouse wheel support (`allow_mouse_wheel` prop)

#### Enhanced Input Handling
- [x] Copy/paste with automatic format detection
- [x] Drag-to-select precision indicators (`show_selection_info` prop)
- [x] Context menu for format conversion (`show_context_menu` prop)
- [x] Undo/redo for value changes

```rust
<NumberInput
    precision=NumberInputPrecision::Decimal(14)

    // Increment controls
    show_controls=true
    step="0.01"
    shift_step="0.1"
    ctrl_step="1.0"

    // Enhanced input
    allow_paste_format_detection=true
    convert_scientific_on_paste=true
/>
```

### Core Scientific Inputs

#### Deliverables
- [x] **AngleInput** - Degrees, radians, gradians with visual arc preview
- [x] **FractionInput** - Numerator/denominator with auto-simplification
- [x] **UnitInput** - Physical units with conversion (Length, Mass, Time, Temperature, Data)
- [x] **ComplexNumberInput** - Rectangular and polar form support
- [x] **UncertaintyInput** - Value ± error with multiple display formats
- [x] **IntervalInput** - Min/max bounds with open/closed notation
- [x] **CoordinateInput** - 2D/3D Cartesian, Polar, Spherical, Cylindrical
- [x] **PointLocator** - Drag-and-drop point positioning on canvas

#### UnitInput
```rust
<UnitInput
    value=signal
    unit="m/s"
    category=UnitCategory::Velocity
    allow_conversion=true
    on_change=Callback::new(|(value, unit): (Decimal, Unit)| { ... })
/>
```

**Features**:
- Integrated unit selection dropdown
- Automatic unit conversion (SI, Imperial, CGS)
- Engineering notation with SI prefixes (k, M, G, μ, n, p)
- Dimensional analysis validation
- Common unit categories: Length, Mass, Time, Temperature, Energy, etc.

#### ComplexNumberInput
```rust
<ComplexNumberInput
    precision=NumberInputPrecision::Decimal(10)
    format=ComplexFormat::Rectangular  // or Polar, Euler
    on_change=Callback::new(|z: Complex| { ... })
/>
```

**Features**:
- Rectangular (a + bi), Polar (r∠θ), Euler (re^iθ) forms
- Toggle between representations
- Argand diagram visualization option
- Conjugate and modulus display
- Phase angle in degrees or radians

#### FractionInput
```rust
<FractionInput
    allow_mixed=true
    auto_simplify=true
    precision=NumberInputPrecision::I128
    on_change=Callback::new(|frac: Fraction| { ... })
/>
```

**Features**:
- Numerator/denominator entry
- Mixed number support (2 ³⁄₄)
- Automatic simplification
- Exact rational arithmetic
- Decimal conversion display

#### UncertaintyInput
```rust
<UncertaintyInput
    precision=NumberInputPrecision::Decimal(6)
    format=UncertaintyFormat::PlusMinus  // or Parentheses, Percent
    on_change=Callback::new(|(value, error): (Decimal, Decimal)| { ... })
/>
```

**Features**:
- Value ± uncertainty entry (scientific measurements)
- Percentage uncertainty display
- Significant figures handling
- Error propagation preview
- Confidence interval display

#### IntervalInput
```rust
<IntervalInput
    precision=NumberInputPrecision::Decimal(8)
    bounds=IntervalBounds::Closed  // or Open, HalfOpen
    on_change=Callback::new(|interval: Interval| { ... })
/>
```

**Features**:
- Min/max bounds with open/closed indicators
- Mathematical notation display [a, b] or (a, b)
- Intersection/union preview
- Infinity support
- Set notation mode

---

### Coordinate & Geometry Inputs

#### CoordinateInput
```rust
<CoordinateInput
    system=CoordinateSystem::Cartesian3D  // or Polar, Spherical, Cylindrical
    precision=NumberInputPrecision::Decimal(10)
    on_change=Callback::new(|coords: Coordinates| { ... })
/>
```

**Features**:
- 2D/3D Cartesian (x, y, z)
- Polar (r, θ) / Cylindrical (r, θ, z) / Spherical (r, θ, φ)
- Automatic conversion between systems
- Angle units (degrees, radians, gradians)
- Visual coordinate preview

#### PointLocator (Mathematica-style)
```rust
<PointLocator
    bounds=((-10.0, 10.0), (-10.0, 10.0))
    precision=NumberInputPrecision::Decimal(4)
    snap_to_grid=Some(0.5)
    on_change=Callback::new(|point: (Decimal, Decimal)| { ... })
/>
```

**Features**:
- Drag-and-drop point positioning
- Grid snapping with configurable precision
- Multi-point selection mode
- Crosshair cursor with coordinate display
- Zoom and pan support

---

### Domain-Specific Components

#### CurrencyInput
```rust
<CurrencyInput
    currency="USD"
    precision=CurrencyPrecision::Cents  // or Satoshis, Wei
    show_symbol=true
    on_change=Callback::new(|amount: Currency| { ... })
/>
```

**Features**:
- Multi-currency with ISO 4217 codes
- Cryptocurrency micro-units (Satoshis, Wei, Gwei)
- Exact decimal arithmetic
- Locale-aware formatting
- Exchange rate integration ready

#### ScientificNotationInput
```rust
<ScientificNotationInput
    mantissa_precision=NumberInputPrecision::Decimal(6)
    exponent_range=(-308, 308)
    notation=ScientificNotation::Engineering  // exponents divisible by 3
    on_change=Callback::new(|value: Scientific| { ... })
/>
```

**Features**:
- Separate mantissa × 10^exponent entry
- Engineering notation (k, M, G prefixes)
- Automatic normalization
- Subnormal number handling
- Copy as LaTeX or plain text

#### DateTimeInput
```rust
<DateTimeInput
    precision=TimePrecision::Nanoseconds
    timezone="UTC"
    on_change=Callback::new(|dt: DateTime| { ... })
/>
```

**Features**:
- Nanosecond precision timestamps
- Timezone-aware with DST handling
- Leap second support
- Julian/Modified Julian date conversion
- ISO 8601 and Unix timestamp formats

#### AngleInput
```rust
<AngleInput
    unit=AngleUnit::Degrees  // or Radians, Gradians, DMS
    precision=NumberInputPrecision::Decimal(6)
    wrap=true  // 360° wraps to 0°
    on_change=Callback::new(|angle: Angle| { ... })
/>
```

**Features**:
- Degrees, radians, gradians, turns
- Degrees-Minutes-Seconds (DMS) format
- Visual angle arc preview
- Normalization options (0-360° or -180°-180°)
- Trigonometric function preview

---

## Phase 5: Mathematical Expression & Data Entry ✅ COMPLETED

**Status**: Complete
**Completed**: February 2026
**Version**: 0.6.1

### Objectives

Advanced mathematical input components inspired by [MathLive](https://cortexjs.io/mathlive/), [Mathematica](https://www.wolfram.com/language/core-areas/user-interfaces/), and scientific data tools.

### Deliverables

#### Equation & Formula Components
- [x] **EquationEditor** - Visual equation editing with geometric algebra support
- [x] **FormulaInput** - Mathematical expression parser with evaluation
- [x] **SymbolPalette** - Greek letters, operators, and mathematical symbols

#### Matrix & Linear Algebra Components
- [x] **MatrixInput** - Spreadsheet-style matrix entry with operations
- [x] **VectorInput** - Row/column vectors with notation options
- [x] **TensorInput** - Multi-dimensional array entry with shape manipulation

#### Parameter Manipulation (Mathematica-style)
- [x] **ParameterSlider** - High-precision slider with exact decimal values
- [x] **ParameterGrid** - Multiple parameter sliders with presets
- [x] **ParameterTree** - PyQtGraph-style hierarchical parameter editing

---

### Component Details

### Equation & Formula Components

#### EquationEditor
```rust
<EquationEditor
    output_format=EquationFormat::LaTeX  // or MathML, AsciiMath
    symbols=SymbolPalette::Mathematics
    on_change=Callback::new(|latex: String| { ... })
/>
```

**Features**:
- Visual WYSIWYG math editing (like MathLive/MathQuill)
- LaTeX, MathML, and AsciiMath output
- Symbol palette with Greek letters, operators, relations
- Fraction, exponent, root, integral entry
- Keyboard shortcuts for common symbols
- Live KaTeX/MathJax preview

#### FormulaInput
```rust
<FormulaInput
    allow_symbolic=true
    precision=NumberInputPrecision::Arbitrary
    variables=vec!["x", "y", "z"]
    on_change=Callback::new(|expr: Expression| { ... })
/>
```

**Features**:
- Mathematical expression parser
- Symbolic variable support
- Function recognition (sin, cos, exp, ln, etc.)
- Automatic differentiation preview (Amari integration)
- Expression tree visualization
- Evaluation with substitution

#### SymbolPalette
```rust
<SymbolPalette
    categories=vec![SymbolCategory::Greek, SymbolCategory::Operators]
    on_select=Callback::new(|symbol: &str| { ... })
/>
```

**Features**:
- Greek letters (α, β, γ, δ, etc.)
- Mathematical operators (∑, ∏, ∫, ∂, ∇)
- Set theory (∈, ⊂, ∪, ∩, ∅)
- Logic (∀, ∃, ∧, ∨, ¬, ⇒)
- Arrows and relations (→, ↔, ≤, ≥, ≠, ≈)
- Searchable symbol picker

---

### Matrix & Linear Algebra Components

#### MatrixInput
```rust
<MatrixInput
    rows=3
    cols=3
    precision=NumberInputPrecision::Arbitrary
    show_operations=true
    on_change=Callback::new(|matrix: Matrix| { ... })
/>
```

**Features**:
- Spreadsheet-style matrix entry
- Keyboard navigation (Tab, Arrow keys)
- Row/column resize and insert
- Matrix operations preview (det, tr, rank, inverse)
- LaTeX display mode
- Import/export MATLAB, NumPy, Mathematica formats

#### VectorInput
```rust
<VectorInput
    dimensions=3
    notation=VectorNotation::Column  // or Row, Geometric
    precision=NumberInputPrecision::Decimal(10)
    on_change=Callback::new(|vec: Vector| { ... })
/>
```

**Features**:
- Row/column vector entry
- Geometric algebra multivector support (Amari)
- Magnitude and direction display
- Dot/cross product preview
- Unit vector normalization
- Basis vector decomposition

#### TensorInput
```rust
<TensorInput
    shape=vec![2, 3, 4]
    precision=NumberInputPrecision::Decimal(8)
    on_change=Callback::new(|tensor: Tensor| { ... })
/>
```

**Features**:
- Multi-dimensional array entry
- Slice and index selection
- Einstein notation support
- Contraction preview
- Shape manipulation (reshape, transpose)

---

### Parameter Manipulation (Mathematica-style)

#### ParameterSlider
```rust
<ParameterSlider
    min="-10"
    max="10"
    step="0.01"
    precision=NumberInputPrecision::Decimal(4)
    show_value=true
    on_change=Callback::new(|value: Decimal| { ... })
/>
```

**Features**:
- High-precision slider with exact values
- Logarithmic scale option
- Keyboard fine-tuning
- Value input alongside slider
- Animation/autoplay mode
- Modifier keys for step multipliers

#### ParameterGrid
```rust
<ParameterGrid
    parameters=vec![
        ("amplitude", 0.0..10.0, 0.1),
        ("frequency", 0.1..100.0, 0.1),
        ("phase", 0.0..TAU, 0.01),
    ]
    on_change=Callback::new(|params: HashMap<String, Decimal>| { ... })
/>
```

**Features**:
- Multiple parameter sliders in grid
- Linked parameter updates
- Preset save/load
- Reset to defaults
- Parameter grouping/collapsing
- Mathematica Manipulate-style layout

#### ParameterTree (PyQtGraph-style)
```rust
<ParameterTree
    schema=parameter_schema
    on_change=Callback::new(|path: &str, value: Value| { ... })
/>
```

**Features**:
- Hierarchical parameter editing
- Type-aware editors (number, bool, color, enum)
- Expand/collapse groups
- Search/filter parameters
- Save/load configurations
- Undo/redo support

---

## Phase 6: Visualization & Analysis

**Target**: 2027
**Version**: 0.8.0

### Components

#### PrecisionChart
- High-precision data visualization
- Zoom without precision loss
- Exact point coordinates
- Error bar precision handling

#### StatisticsPanel
- Precision-preserving statistical calculations
- Mean, variance, etc. with arbitrary precision
- Distribution fitting with exact parameters
- Hypothesis testing with exact p-values

#### TropicalAlgebraComponents
- Tropical semiring operations UI
- Min-plus/max-plus algebra widgets
- Path algebra visualization
- Integration with Amari's tropical algebra

---

## Phase 7: Node-Based Network UI

**Target**: 2027
**Version**: 0.9.0

### Objectives

Build a flexible, precision-aware node graph editor inspired by [ComfyUI](https://github.com/comfyanonymous/ComfyUI), [Blender's node editor](https://docs.blender.org/manual/en/latest/interface/controls/nodes/index.html), and [Rete.js](https://rete.js.org/). Enable visual programming workflows for data pipelines, mathematical computations, and creative applications.

### Core Components

#### NodeCanvas
```rust
<NodeCanvas
    graph=graph_signal
    on_connection=Callback::new(|conn: Connection| { ... })
    on_node_move=Callback::new(|id, pos| { ... })
    zoom_range=(0.1, 4.0)
    grid_snap=Some(20.0)
/>
```

**Features**:
- Infinite canvas with pan and zoom
- Grid snapping with configurable resolution
- Minimap for navigation
- Multi-node selection and group operations
- Undo/redo for all operations
- Touch and stylus support

#### Node
```rust
<Node
    id="node_001"
    title="Precision Multiply"
    position=(100.0, 200.0)
    inputs=vec![
        NodePort::new("a", PortType::Decimal),
        NodePort::new("b", PortType::Decimal),
    ]
    outputs=vec![
        NodePort::new("result", PortType::Decimal),
    ]
    on_input_change=Callback::new(|port, value| { ... })
/>
```

**Features**:
- Collapsible/expandable node bodies
- Custom node colors and icons
- Inline parameter editing (NumberInput integration!)
- Preview/thumbnail display
- Error state visualization
- Execution progress indicator

#### NodePort
```rust
<NodePort
    id="input_a"
    direction=PortDirection::Input
    port_type=PortType::Decimal { precision: NumberInputPrecision::Arbitrary }
    connected=is_connected_signal
    on_connect=Callback::new(|source_port| { ... })
/>
```

**Features**:
- Type-safe connections (prevent incompatible types)
- Multi-connection support (fan-in/fan-out)
- Visual type indicators (color-coded by data type)
- Hover previews of flowing data
- Optional/required port indicators
- Array/batch port support

#### NodeConnection
```rust
<NodeConnection
    from_node="node_001"
    from_port="output"
    to_node="node_002"
    to_port="input_a"
    style=ConnectionStyle::Bezier  // or Straight, Step
/>
```

**Features**:
- Bezier, straight, or stepped connection styles
- Animated data flow visualization
- Connection reroute points
- Highlight on hover/selection
- Delete on middle-click or backspace
- Precision-aware data flow (visualize precision loss warnings)

### Precision Integration

**Key Differentiator**: Unlike other node editors, Mingot's node graph is precision-aware:

- **Precision Propagation**: Track precision through the graph, warn when precision is lost
- **Type-Safe Ports**: Ports know their precision requirements (U64, U128, Decimal, Arbitrary)
- **Exact Arithmetic Nodes**: Built-in nodes for exact decimal operations
- **Validation Flow**: Real-time validation status propagates through connections
- **Precision Inspector**: Click any connection to see exact value with full precision

### Built-in Node Libraries

#### Arithmetic Nodes
- Add, Subtract, Multiply, Divide (precision-preserving)
- Power, Root, Logarithm
- Modulo, Floor, Ceiling, Round
- Comparison (with epsilon for decimals)

#### Mathematical Nodes
- Trigonometric (sin, cos, tan, etc.)
- Matrix operations (integrate with MatrixInput)
- Complex number operations
- Statistical aggregations

#### Control Flow Nodes
- Switch/Case
- Loop/Iterate
- Filter/Map
- Merge/Split

#### Data Nodes
- Constant values (with NumberInput)
- Input/Output terminals
- Variable references
- Buffer/Cache

### Graph Operations

```rust
// Graph management
let graph = use_node_graph();

graph.add_node(node_definition);
graph.remove_node(node_id);
graph.connect(from, to);
graph.disconnect(connection_id);

// Execution
graph.execute();  // Run the graph
graph.validate(); // Check for cycles, type mismatches

// Serialization
let json = graph.to_json();
graph.from_json(json);
```

### Use Cases

1. **Scientific Computing Pipelines**: Chain precision-critical calculations visually
2. **Data Transformation**: ETL workflows with exact numeric handling
3. **Generative AI Workflows**: ComfyUI-style image/audio generation pipelines
4. **Financial Modeling**: Build calculation graphs with audit trails
5. **Educational Tools**: Visual mathematics and physics simulations
6. **Game Development**: Shader graphs, behavior trees, dialogue systems

### API Design Goals

- **Declarative**: Define graphs in Leptos RSX or load from JSON
- **Reactive**: Graph changes trigger re-execution automatically (optional)
- **Extensible**: Custom node types via trait implementation
- **Serializable**: Full graph state to/from JSON for persistence
- **Accessible**: Keyboard navigation, screen reader support for node operations

---

## Phase 8: Theme System & Custom Themes

**Target**: 2027+
**Version**: 1.0.0

### Objectives

Expand Mingot's theming capabilities with production-ready themes and a flexible theme creation system. Some themes will be designed in Figma and translated to Mingot's theme system.

### Deliverables

#### Built-in Themes
- [ ] **Mingot Default** - Current theme, refined and polished
- [ ] **Mingot Dark** - Dark mode variant of default
- [ ] **Industrial** - Industrial Algebra brand theme
- [ ] **Scientific** - Clean, academic aesthetic for research applications
- [ ] **Financial** - Professional theme for fintech applications
- [ ] **TBD (Figma)** - Custom themes designed in Figma

#### Theme System Enhancements
- [ ] Theme switching without page reload
- [ ] CSS custom property extraction
- [ ] Theme inheritance (extend existing themes)
- [ ] Component-level theme overrides
- [ ] Theme validation and type safety

#### Figma Integration Workflow
- [ ] Figma design tokens export format
- [ ] Theme generator from Figma variables
- [ ] Documentation for Figma-to-Mingot workflow
- [ ] Example Figma component library

#### Theme API
```rust
// Define a custom theme
let my_theme = Theme::new()
    .extend(themes::MINGOT_DARK)
    .colors(ColorScheme {
        primary: "#6366f1",
        secondary: "#8b5cf6",
        accent: "#22d3ee",
        ..Default::default()
    })
    .typography(Typography {
        font_family: "'JetBrains Mono', monospace",
        ..Default::default()
    })
    .radius(Radius::rounded())  // All rounded corners
    .shadows(Shadows::subtle()); // Subtle shadow preset

// Use in app
view! {
    <ThemeProvider theme=my_theme>
        <App />
    </ThemeProvider>
}
```

#### Theme Presets
```rust
// Quick theme presets
<ThemeProvider theme=themes::SCIENTIFIC />
<ThemeProvider theme=themes::FINANCIAL />
<ThemeProvider theme=themes::INDUSTRIAL />

// Dynamic theming
let user_theme = create_signal(themes::MINGOT_DEFAULT);
<ThemeProvider theme=user_theme />
```

### Design Goals

- **Consistency**: All 50+ components respect theme values
- **Accessibility**: All themes meet WCAG 2.1 AA contrast requirements
- **Performance**: Theme switching is instantaneous (CSS variables)
- **Customization**: Override any theme value at any level
- **Documentation**: Each theme documented with usage guidelines

---

## Phase 9: VFX Extension (WGSL Shaders)

**Target**: 2027+
**Version**: 1.1.0

### Objectives

Add a visual effects framework to Mingot leveraging WGSL (WebGPU Shading Language) shaders for high-performance, GPU-accelerated visual effects. Inspired by effects seen on sites like Mercury Bank (plasma-shader buttons) and creative web experiences.

### Vision

Transform standard UI components into dynamic, visually stunning elements without sacrificing performance or accessibility. The VFX extension will be opt-in, ensuring zero overhead for applications that don't need visual effects.

### Core Architecture

#### ShaderCanvas
```rust
<ShaderCanvas
    shader=shaders::PLASMA
    uniforms=ShaderUniforms {
        time: time_signal,
        mouse: mouse_position,
        color1: "#6366f1",
        color2: "#22d3ee",
    }
    fallback=view! { <div class="gradient-fallback" /> }
/>
```

**Features**:
- WebGPU-first with WebGL2 fallback
- Automatic feature detection
- Graceful degradation for unsupported browsers
- CSS fallback for accessibility/reduced-motion

#### VFXButton
```rust
<VFXButton
    effect=ButtonEffect::Plasma
    intensity=0.8
    on_hover=ButtonEffect::Glow
    on_click=ButtonEffect::Ripple
>
    "Get Started"
</VFXButton>
```

**Built-in Effects**:
- **Plasma** - Animated plasma/gradient flow (Mercury Bank style)
- **Glow** - Soft, pulsing glow effect
- **Ripple** - Material Design-inspired ripple on click
- **Shimmer** - Loading/skeleton shimmer effect
- **Aurora** - Northern lights color flow
- **Noise** - Procedural noise textures
- **Holographic** - Iridescent/holographic sheen

### Built-in Shaders

#### Background Effects
```rust
<VFXBackground effect=BackgroundEffect::Gradient {
    colors: vec!["#1a1a2e", "#16213e", "#0f3460"],
    animation: GradientAnimation::Flow { speed: 0.5 },
} />

<VFXBackground effect=BackgroundEffect::Particles {
    count: 100,
    color: "#ffffff",
    connections: true,
} />

<VFXBackground effect=BackgroundEffect::Waves {
    amplitude: 20.0,
    frequency: 0.02,
    color: "#3b82f6",
} />
```

#### Component Effects
```rust
// Glow effect on any component
<VFXWrapper effect=Effect::Glow { color: "#6366f1", intensity: 0.6 }>
    <Card>...</Card>
</VFXWrapper>

// Hover-triggered effects
<VFXWrapper
    on_hover=Effect::Scale { amount: 1.05 }
    on_hover=Effect::Glow { color: "auto", intensity: 0.4 }
>
    <Button>Hover Me</Button>
</VFXWrapper>
```

### Custom Shader Support

```rust
// Define custom WGSL shader
const MY_SHADER: &str = r#"
    @fragment
    fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
        let time = uniforms.time;
        let color = vec3<f32>(
            sin(uv.x * 10.0 + time) * 0.5 + 0.5,
            cos(uv.y * 10.0 + time) * 0.5 + 0.5,
            sin((uv.x + uv.y) * 10.0 + time) * 0.5 + 0.5
        );
        return vec4<f32>(color, 1.0);
    }
"#;

<ShaderCanvas
    shader=CustomShader::new(MY_SHADER)
    uniforms=uniforms
/>
```

### Performance & Accessibility

#### Performance Guarantees
- **Zero CPU overhead**: All effects run on GPU
- **Lazy initialization**: Shaders compiled on first use
- **LOD system**: Reduce effect quality on lower-end devices
- **Battery-aware**: Reduce effects when on battery power
- **Frame budget**: Effects throttle to maintain 60fps

#### Accessibility
```rust
<VFXButton
    effect=ButtonEffect::Plasma
    reduced_motion_fallback=ButtonEffect::None  // Respects prefers-reduced-motion
    aria_label="Submit form"
>
    "Submit"
</VFXButton>
```

- **prefers-reduced-motion**: All effects respect user preference
- **Fallback styles**: CSS-only fallbacks for screen readers
- **Focus indicators**: VFX never obscures focus states
- **Color contrast**: Effects don't reduce text contrast below WCAG AA

### Integration with Existing Components

```rust
// Enable VFX on existing Mingot buttons
<Button
    variant=ButtonVariant::Filled
    vfx=Some(ButtonEffect::Plasma)  // Optional VFX enhancement
>
    "Enhanced Button"
</Button>

// VFX-enhanced loader
<Loader variant=LoaderVariant::Oval vfx=LoaderEffect::Glow />

// Shader-based skeleton
<Skeleton vfx=SkeletonEffect::Holographic />
```

### Shader Library

#### Included Shaders
| Shader | Description | Use Case |
|--------|-------------|----------|
| `plasma` | Flowing gradient plasma | Hero buttons, CTAs |
| `aurora` | Northern lights effect | Backgrounds |
| `noise` | Perlin/Simplex noise | Textures, backgrounds |
| `voronoi` | Cell-based patterns | Abstract backgrounds |
| `wave` | Sine wave distortion | Water effects |
| `glow` | Soft bloom/glow | Highlights, focus states |
| `particles` | GPU particle system | Celebrations, backgrounds |
| `holographic` | Iridescent sheen | Premium feel |
| `fire` | Procedural flames | Dramatic effects |
| `electric` | Lightning/electricity | Energy, power themes |

#### Shader Composition
```rust
// Combine multiple effects
<ShaderCanvas
    shaders=vec![
        shaders::NOISE.with_opacity(0.3),
        shaders::PLASMA.with_blend(BlendMode::Overlay),
    ]
/>
```

### Feature Flag

```toml
[dependencies]
mingot = { version = "1.1", features = ["vfx"] }
```

- VFX is opt-in via feature flag
- Zero binary size impact when disabled
- No WebGPU dependency unless feature enabled

### Browser Support

| Browser | Support Level |
|---------|--------------|
| Chrome 113+ | Full WebGPU |
| Edge 113+ | Full WebGPU |
| Firefox 115+ | WebGPU (flag) |
| Safari 17+ | WebGPU |
| Older browsers | WebGL2 fallback |
| No GPU | CSS fallback |

### Use Cases

1. **Marketing Sites**: Eye-catching hero sections, animated CTAs
2. **Creative Applications**: Design tools, creative software
3. **Gaming UIs**: Rich, dynamic interfaces
4. **Premium Products**: Luxury feel, high-end aesthetics
5. **Data Visualization**: GPU-accelerated charts and graphs
6. **Scientific Visualization**: Real-time simulation displays

---

## Long-Term Vision (2027+)

### Research Collaborations
- Partner with computational mathematics labs
- Integration with Jupyter/computational notebooks
- Support for mathematical research workflows

### Advanced Amari Integration
- Full geometric algebra component suite
- Automatic differentiation visualization
- Symbolic computation UI
- Computer algebra system integration

### Domain Expansion
- **Physics**: Quantum mechanics, relativity calculations
- **Chemistry**: Molecular dynamics, quantum chemistry
- **Biology**: Bioinformatics, genomics precision
- **Engineering**: CAD/CAM, finite element analysis
- **Finance**: Quantitative finance, risk modeling

### Performance Optimization
- GPU acceleration for Amari operations
- SIMD optimizations for array operations
- Parallel validation for large datasets
- WebWorker integration for non-blocking calculations

### Ecosystem Growth
- Mingot CLI tools for code generation
- Figma/Sketch plugins for design
- VS Code extension for component snippets
- Community component marketplace

---

## Versioning Strategy

### Pre-1.0 (0.x.y)
- **Minor bumps (0.X.0)**: New phase completion, new precision features
- **Patch bumps (0.x.Y)**: Bug fixes, documentation, minor enhancements
- **Breaking changes allowed**: API evolution prioritized over stability

### 1.0.0 Release Criteria
- [ ] All Phase 1-3 features complete and tested
- [ ] Comprehensive documentation
- [ ] Real-world usage in 3+ production applications
- [ ] Performance benchmarks meet targets
- [ ] Accessibility audit complete (WCAG 2.1 AA)
- [ ] Community adoption and feedback
- [ ] API stability commitment

### Post-1.0
- **Major (X.0.0)**: Breaking API changes
- **Minor (x.X.0)**: New features, backward compatible
- **Patch (x.x.X)**: Bug fixes, documentation

---

## Success Metrics

### Adoption Metrics
- **Stars on GitHub**: 1,000+ by 1.0 release
- **Downloads from crates.io**: 10,000+ by end of 2026
- **Production deployments**: 10+ known applications
- **Community contributions**: 20+ contributors

### Quality Metrics
- **Test coverage**: > 90% for precision components
- **Documentation coverage**: 100% of public API
- **Performance**: < 16ms input latency across all components
- **Accessibility**: WCAG 2.1 AA compliance
- **Browser compatibility**: Latest 2 versions of major browsers

### Precision Metrics
- **Zero precision loss**: No component loses precision silently
- **Validated range**: All precision types tested at boundaries
- **Amari integration**: Full coverage of Amari's Number type
- **Error handling**: All precision errors surfaced to users

---

## Community & Ecosystem

### Documentation Strategy
- Comprehensive API docs (docs.rs)
- Interactive examples (Leptos playground integration)
- Video tutorials for complex components
- Case studies from real applications
- Migration guides from other libraries

### Community Building
- Discord server for developers
- Monthly community calls
- Annual Mingot conference
- Contributor recognition program
- Mentorship for new contributors

### Industrial Algebra Ecosystem
- **Amari**: Core mathematical engine
- **Mingot**: UI components
- **Future**: Additional libraries for computation, visualization, etc.

---

## Risk Management

### Technical Risks
- **WASM binary size**: Mitigation through feature flags, tree shaking
- **Performance**: Continuous benchmarking, optimization sprints
- **Browser compatibility**: Automated cross-browser testing
- **Amari API changes**: Feature flags, version compatibility matrix

### Adoption Risks
- **Learning curve**: Extensive documentation, examples, tutorials
- **Competition**: Emphasize unique precision capabilities
- **Breaking changes in 0.x**: Clear migration guides, deprecation warnings

---

## Contributing to the Roadmap

The community is invited to shape Mingot's future:

1. **Vote on priorities**: GitHub Discussions for feature prioritization
2. **Propose new components**: RFC process for major additions
3. **Domain expertise**: Partner with domain experts (finance, science, etc.)
4. **Performance improvements**: Benchmarking and optimization contributions

**Contact**: Open an issue or discussion on [GitHub](https://github.com/Industrial-Algebra/Mingot)

---

**Last Updated**: January 2026
**Next Review**: April 2026

---

**Mingot: Precision without compromise.**
