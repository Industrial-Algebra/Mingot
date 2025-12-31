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

## Phase 3: Advanced Precision Features

**Target**: Q1 2026
**Version**: 0.4.0

### Objectives

Enhance NumberInput with professional-grade features for production applications.

### Deliverables

#### Auto-Formatting
- [ ] Thousand separators on blur (1,234,567)
- [ ] Scientific notation formatting (1.23e8)
- [ ] Engineering notation (123.4e6)
- [ ] Custom format strings
- [ ] Locale-aware formatting (European: 1.234.567,89)

#### Increment/Decrement Controls
- [ ] +/- buttons with precision-aware stepping
- [ ] Configurable step size (e.g., step="0.0001")
- [ ] Keyboard shortcuts (Arrow up/down)
- [ ] Modifier keys (Shift = 10x, Ctrl = 100x)
- [ ] Mouse wheel support

#### Enhanced Input Handling
- [ ] Copy/paste with automatic format detection
- [ ] Drag-to-select precision indicators
- [ ] Context menu for format conversion
- [ ] Undo/redo for value changes

#### Visual Enhancements
- [ ] Precision indicators (e.g., "14 decimal places")
- [ ] Real-time validation feedback
- [ ] Overflow warning (approaching limits)
- [ ] Scientific notation auto-switch for large values

#### Accessibility
- [ ] Screen reader precision announcements
- [ ] ARIA labels for precision indicators
- [ ] Keyboard-only operation
- [ ] High-contrast mode support

### API Additions

```rust
<NumberInput
    precision=NumberInputPrecision::Decimal(14)

    // Auto-formatting
    format_on_blur=true
    format=NumberInputFormat::Thousand
    locale="en-US"

    // Increment controls
    show_controls=true
    step="0.01"
    shift_step="0.1"

    // Enhanced input
    allow_paste_format_detection=true
    convert_scientific_on_paste=true

    // Visual
    show_precision_indicator=true
    show_overflow_warning=true
/>
```

---

## Phase 4: Domain-Specific Components

**Target**: Q2 2026
**Version**: 0.5.0

### Objectives

Build specialized precision components for common domains.

### Components

#### CurrencyInput
```rust
<CurrencyInput
    currency="USD"
    precision=CurrencyPrecision::Cents  // or Satoshis, Wei, etc.
    show_symbol=true
    allow_negative=true
    on_value_change=Callback::new(|amount: Currency| { ... })
/>
```

**Features**:
- Multi-currency support with exchange rates
- Exact decimal arithmetic (no floating point)
- Support for cryptocurrency micro-units
- Automatic rounding rules per currency
- Tax and tip calculations

#### ScientificInput
```rust
<ScientificInput
    mantissa_precision=NumberInputPrecision::Decimal(6)
    exponent_range=(-308, 308)
    allow_subnormal=true
    notation=ScientificNotation::Engineering
/>
```

**Features**:
- Separate mantissa and exponent validation
- Engineering notation (exponents divisible by 3)
- Subnormal/denormal number handling
- Unit prefixes (k, M, G, etc.)

#### DateTimeInput
```rust
<DateTimeInput
    precision=TimePrecision::Nanoseconds
    timezone="UTC"
    on_timestamp_change=Callback::new(|nanos: i128| { ... })
/>
```

**Features**:
- Nanosecond-precision timestamps
- Timezone-aware with no precision loss
- Leap second handling
- High-frequency trading time accuracy

#### RangeInput
```rust
<RangeInput
    precision=NumberInputPrecision::Decimal(8)
    min="0"
    max="1"
    step="0.00000001"
    show_exact_value=true
/>
```

**Features**:
- Slider with exact value display
- Precision-preserving range selection
- Logarithmic scales for wide ranges
- Two-handle range selection

#### CalculatorInput
```rust
<CalculatorInput
    precision=NumberInputPrecision::Arbitrary
    allow_expressions=true
    on_result=Callback::new(|result: Number| { ... })
/>
```

**Features**:
- Expression evaluation (e.g., "1.5 * 2.3 + 4")
- Arbitrary precision throughout calculation
- Support for mathematical functions
- Memory and history features

---

## Phase 5: Advanced Mathematics

**Target**: Q3-Q4 2026
**Version**: 0.6.0

### Objectives

Deep integration with Amari's advanced mathematical capabilities.

### Components

#### MatrixInput
```rust
<MatrixInput
    rows=3
    cols=3
    precision=NumberInputPrecision::Arbitrary
    on_matrix_change=Callback::new(|matrix: Matrix<Number>| { ... })
/>
```

**Features**:
- Precision matrix entry with keyboard navigation
- Matrix operations preview (determinant, inverse, etc.)
- LaTeX display mode
- Copy/paste from MATLAB, NumPy, etc.

#### VectorInput (Geometric Algebra)
```rust
<VectorInput
    dimensions=3
    algebra=GeometricAlgebra::Euclidean3D
    on_vector_change=Callback::new(|vec: MultiVector| { ... })
/>
```

**Features**:
- Geometric algebra multivector input
- Blade visualization
- Wedge/dot product preview
- Integration with Amari's geometric algebra

#### GraphInput
```rust
<GraphInput
    coordinate_system=CoordinateSystem::Cartesian
    x_precision=NumberInputPrecision::Decimal(10)
    y_precision=NumberInputPrecision::Decimal(10)
    snap_to_grid=false
/>
```

**Features**:
- Precision coordinate input
- Grid snapping with configurable precision
- Polar/cylindrical/spherical coordinates
- Function plotting with exact points

#### FormulaInput
```rust
<FormulaInput
    allow_symbolic=true
    precision=NumberInputPrecision::Arbitrary
    on_expression_change=Callback::new(|expr: Expression| { ... })
/>
```

**Features**:
- Mathematical expression editor
- Symbolic mathematics support
- Automatic differentiation preview
- Integration with Amari's AD capabilities

---

## Phase 6: Visualization & Analysis

**Target**: 2027
**Version**: 0.7.0

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

**Last Updated**: December 2025
**Next Review**: March 2026

---

**Mingot: Precision without compromise.**
