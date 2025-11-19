# High-Precision Number Input Support

## Context

Industrial Algebra's product ecosystem heavily relies on Amari (u64+ precision mathematical computing library with geometric algebra, tropical algebra, and automatic differentiation). Mingot should provide first-class support for high-precision number inputs rather than relying on HTML5's limited number input capabilities.

## Problem Statement

### HTML5 Number Input Limitations

1. **JavaScript Number Precision**: HTML5 `<input type="number">` is constrained by JavaScript's 64-bit floating-point representation:
   - Maximum safe integer: `2^53 - 1` (9,007,199,254,740,991)
   - Precision loss beyond ~15-17 significant digits
   - Cannot represent u64, u128, or arbitrary-precision numbers accurately

2. **Scientific Applications**: Industrial Algebra products require:
   - u64+ precision for mathematical computations
   - Exact decimal representation (no floating-point errors)
   - Integration with Amari's geometric algebra types
   - Support for very large and very small numbers

3. **Current Workaround**: Using `<input type="text">` with manual validation loses:
   - Semantic meaning
   - Browser accessibility features for numeric input
   - Consistent UX patterns

## Proposed Solution

### New Component: `NumberInput`

A specialized component for high-precision number input that:
- Uses text-based input for unlimited precision
- Validates against configurable precision requirements
- Integrates with Amari types
- Provides formatting and display options
- Maintains accessibility

### Component API Design

```rust
use amari::Number; // or appropriate Amari type

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumberInputPrecision {
    U64,           // Unsigned 64-bit
    U128,          // Unsigned 128-bit
    I64,           // Signed 64-bit
    I128,          // Signed 128-bit
    Decimal(u32),  // Fixed decimal places
    Arbitrary,     // Arbitrary precision (uses Amari's Number type)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumberInputFormat {
    Standard,      // 123456789
    Thousand,      // 123,456,789
    Scientific,    // 1.23e8
    Engineering,   // 123.4e6
}

#[component]
pub fn NumberInput(
    // Core value handling
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_valid_change: Option<Callback<Result<Number, ParseError>>>,

    // Precision configuration
    #[prop(optional)] precision: Option<NumberInputPrecision>,
    #[prop(optional, into)] min: Option<String>,
    #[prop(optional, into)] max: Option<String>,

    // Display formatting
    #[prop(optional)] format: Option<NumberInputFormat>,
    #[prop(optional)] decimal_separator: Option<char>, // Default: '.'
    #[prop(optional)] thousand_separator: Option<char>, // Default: ','

    // Validation
    #[prop(optional)] allow_negative: bool,
    #[prop(optional)] allow_decimal: bool,
    #[prop(optional)] allow_scientific: bool,

    // Standard form input props
    #[prop(optional)] variant: Option<InputVariant>,
    #[prop(optional)] size: Option<InputSize>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
) -> impl IntoView
```

### Usage Examples

#### Example 1: U64 Integer Input
```rust
use mingot::prelude::*;

let value = RwSignal::new(String::from("9223372036854775807"));

view! {
    <NumberInput
        value=value
        precision=NumberInputPrecision::U64
        format=NumberInputFormat::Thousand
        label="Large Integer Value"
        description="Enter a 64-bit unsigned integer"
    />
}
```

#### Example 2: High-Precision Decimal
```rust
let volatility = RwSignal::new(String::from("0.00000123456789"));

view! {
    <NumberInput
        value=volatility
        precision=NumberInputPrecision::Decimal(14)
        allow_decimal=true
        min="0"
        max="1"
        label="Volatility"
        description="14 decimal places of precision"
    />
}
```

#### Example 3: Amari Integration
```rust
use amari::Number;

let (amari_value, set_amari_value) = create_signal(None::<Number>);

view! {
    <NumberInput
        precision=NumberInputPrecision::Arbitrary
        on_valid_change=Callback::new(move |result: Result<Number, ParseError>| {
            if let Ok(num) = result {
                set_amari_value.set(Some(num));
            }
        })
        label="Arbitrary Precision Value"
        description="Powered by Amari"
    />
}
```

#### Example 4: Scientific Notation
```rust
view! {
    <NumberInput
        precision=NumberInputPrecision::Arbitrary
        format=NumberInputFormat::Scientific
        allow_scientific=true
        allow_negative=true
        label="Scientific Value"
        placeholder="1.23e-8"
    />
}
```

## Implementation Strategy

### Phase 1: Foundation (Without Amari)
Build core NumberInput component with:
- Text-based input validation
- Support for u64, u128, i64, i128 via Rust's standard library
- Formatting options (thousand separators, etc.)
- Range validation
- Accessibility features

**Dependencies**: None new (use Rust stdlib)

### Phase 2: Amari Integration
Add Amari as optional dependency:
- `NumberInputPrecision::Arbitrary` mode
- Integration with Amari's `Number` type
- Callbacks that return parsed Amari types
- Validation using Amari's precision capabilities

**Dependencies**:
```toml
[dependencies]
amari = { version = "0.9.10", optional = true }

[features]
default = ["csr"]
high-precision = ["amari"]
```

### Phase 3: Advanced Features
- Auto-formatting on blur
- Increment/decrement controls for high-precision values
- Copy/paste handling with format detection
- Keyboard shortcuts (scientific notation toggle, etc.)
- Integration with Mingot's validation framework

## Technical Considerations

### 1. String-Based Value Storage

```rust
// Store as string to preserve exact input
let raw_value = RwSignal::new(String::new());

// Parse on demand
let parsed_value = move || {
    match precision {
        NumberInputPrecision::U64 => raw_value.get().parse::<u64>(),
        NumberInputPrecision::Arbitrary => {
            #[cfg(feature = "high-precision")]
            amari::Number::from_str(&raw_value.get())
        }
        // ...
    }
};
```

### 2. Real-Time Validation

```rust
let validate_input = move |input: String| -> ValidationResult {
    match precision {
        NumberInputPrecision::U64 => {
            input.parse::<u64>()
                .map_err(|e| ValidationError::new("Invalid u64"))
        }
        NumberInputPrecision::Decimal(places) => {
            validate_decimal_places(&input, places)
        }
        // ...
    }
};
```

### 3. Formatting Display

```rust
let format_display = move |value: &str| -> String {
    match format {
        NumberInputFormat::Thousand => {
            add_thousand_separators(value, thousand_separator.unwrap_or(','))
        }
        NumberInputFormat::Scientific => {
            convert_to_scientific(value)
        }
        // ...
    }
};
```

### 4. Input Filtering

```rust
// Only allow valid characters based on configuration
let filter_input = move |char: char| -> bool {
    match char {
        '0'..='9' => true,
        '-' => allow_negative && /* is first char */,
        '.' => allow_decimal && /* not already present */,
        'e' | 'E' => allow_scientific && /* valid position */,
        _ if thousand_separator == Some(char) => format == NumberInputFormat::Thousand,
        _ => false,
    }
};
```

## Accessibility Considerations

1. **ARIA Attributes**:
   ```rust
   <input
       type="text"
       inputmode="decimal"  // Mobile keyboard optimization
       role="spinbutton"    // Semantic role for screen readers
       aria-valuemin=min
       aria-valuemax=max
       aria-valuenow=current_value
       aria-invalid=is_error
   />
   ```

2. **Keyboard Navigation**:
   - Arrow up/down: Increment/decrement by step size
   - Shift + Arrow: Increment/decrement by 10x step
   - Ctrl + Arrow: Increment/decrement by 100x step

3. **Screen Reader Support**:
   - Announce validation errors
   - Announce value changes
   - Describe precision and range constraints

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_u64_validation() {
        assert!(validate_u64("18446744073709551615").is_ok());
        assert!(validate_u64("18446744073709551616").is_err()); // overflow
    }

    #[test]
    fn test_decimal_precision() {
        assert!(validate_decimal("0.123456", 6).is_ok());
        assert!(validate_decimal("0.1234567", 6).is_err()); // too many decimals
    }

    #[test]
    fn test_thousand_separator_formatting() {
        assert_eq!(format_thousands("1234567"), "1,234,567");
    }

    #[cfg(feature = "high-precision")]
    #[test]
    fn test_amari_integration() {
        let num = parse_to_amari("123.456789012345678901234567890");
        assert!(num.is_ok());
    }
}
```

### Integration Tests
- Test with real Amari types in actual applications
- Verify WASM compatibility
- Performance testing with very large numbers
- Edge case testing (overflow, underflow, precision limits)

## Migration Path

### Existing Input Usage
```rust
// Before: Limited to JavaScript Number precision
<Input input_type="number" step="0.01" label="Value" />
```

### After: High-Precision Support
```rust
// After: Unlimited precision
<NumberInput
    precision=NumberInputPrecision::Decimal(14)
    allow_decimal=true
    label="Value"
/>
```

### Compatibility
- Keep existing `Input` component for standard use cases
- `NumberInput` is opt-in for high-precision requirements
- Gradual migration path - components coexist

## Open Questions

1. **Amari Type Selection**: Which Amari types should we support?
   - `amari::Number`?
   - Geometric algebra types?
   - Tropical algebra types?

2. **Feature Flag Strategy**:
   - Should Amari integration be optional via feature flag?
   - Or should we make Amari a required dependency for Mingot?

3. **Performance**:
   - String parsing overhead vs native number types
   - WASM binary size impact
   - Real-time validation performance for very large numbers

4. **Browser Compatibility**:
   - Clipboard API for copy/paste
   - IME (Input Method Editor) handling
   - Mobile keyboard optimization

5. **Validation Framework Integration**:
   - Should NumberInput integrate with Mingot's existing validation system?
   - Custom validators for domain-specific number constraints?

## Next Steps

1. **Gather Requirements**: Survey Industrial Algebra products for specific precision needs
2. **Prototype Phase 1**: Build basic NumberInput without Amari integration
3. **Validate Approach**: Test with real-world use cases from Ultramarine-Red or other products
4. **Phase 2 Implementation**: Add Amari integration after Phase 1 validation
5. **Documentation**: Comprehensive examples and migration guide
6. **Performance Testing**: Benchmark against requirements

## Success Criteria

- ✅ Support u64, u128, i64, i128 precision
- ✅ Support arbitrary precision via Amari integration
- ✅ No precision loss in input/output
- ✅ Accessible (WCAG 2.1 AA compliance)
- ✅ Performant (< 16ms input latency)
- ✅ WASM-compatible
- ✅ Integration tests with real Amari types
- ✅ Clear migration path from existing Input component

## References

- [Amari Documentation](https://docs.rs/amari/0.9.10)
- [Amari Repository](https://github.com/justinelliottcobb/Amari)
- [MDN: Input Type Number Limitations](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/number)
- [WCAG 2.1 Spinbutton Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/spinbutton/)
