# Component Development Guidelines

This document captures key considerations and patterns for developing components in the Mingot library, informed by real-world integration testing and iterative development.

## Table of Contents
- [Callback Props Pattern](#callback-props-pattern)
- [HTML5 Attribute Support](#html5-attribute-support)
- [Component API Design](#component-api-design)
- [Testing Strategy](#testing-strategy)
- [Common Pitfalls](#common-pitfalls)

## Callback Props Pattern

### Use Concrete Callback Types

**DO**: Use Leptos's concrete `Callback<T>` type for all callback props:

```rust
#[component]
pub fn Button(
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let handle_click = move |ev: ev::MouseEvent| {
        if let Some(callback) = on_click {
            callback.run(ev);
        }
    };
    // ...
}
```

**DON'T**: Use generic type parameters for optional callbacks:

```rust
// ❌ This causes type inference failures
#[component]
pub fn Button<F>(
    #[prop(optional)] on_click: Option<F>,
) -> impl IntoView
where F: Fn(ev::MouseEvent) + Copy + Send + Sync + 'static
{
    // When on_click is None, Rust cannot infer type F
}
```

### Why This Matters

When a generic type parameter is only used in an optional prop that defaults to `None`, Rust's type inference cannot determine the concrete type. This leads to compilation errors requiring users to provide explicit type annotations even when not using the callback.

**User Impact**:
```rust
// With concrete Callback<T>:
<Button>"Click me"</Button>  // ✅ Works

// With generic F:
<Button>"Click me"</Button>  // ❌ Error: cannot infer type
<Button::<fn(MouseEvent)>>"Click me"</Button>  // Requires annotation
```

### Standard Callback Types by Component

| Component Type | Callback Parameter | Type |
|---------------|-------------------|------|
| Button | `on_click` | `Callback<ev::MouseEvent>` |
| Input/Textarea | `on_input`, `on_change` | `Callback<String>` |
| Checkbox/Switch | `on_change` | `Callback<bool>` |
| Select | `on_change` | `Callback<String>` |
| Modal/Drawer/Alert | `on_close` | `Callback<()>` |
| Menu Items | `on_click` | `Callback<()>` |
| Navigation Links | `on_click` | `Callback<ev::MouseEvent>` |

## HTML5 Attribute Support

### Be Comprehensive with Native Attributes

When creating form components, include support for all relevant HTML5 attributes to avoid blocking users who need native browser validation and accessibility features.

### Required HTML5 Attributes by Component

#### Input Component
```rust
#[component]
pub fn Input(
    // Core functionality
    #[prop(optional, into)] input_type: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional)] disabled: Signal<bool>,
    #[prop(optional)] required: bool,

    // Number validation
    #[prop(optional, into)] step: Option<String>,
    #[prop(optional, into)] min: Option<String>,
    #[prop(optional, into)] max: Option<String>,

    // Text validation
    #[prop(optional, into)] pattern: Option<String>,
    #[prop(optional, into)] maxlength: Option<String>,
    #[prop(optional, into)] minlength: Option<String>,

    // Accessibility & UX
    #[prop(optional, into)] autocomplete: Option<String>,
) -> impl IntoView
```

**Critical**: The `step` attribute is essential for number inputs. Without it, browsers default to `step="1"`, which rejects decimal values like `0.01`.

#### Textarea Component
```rust
#[component]
pub fn Textarea(
    // Text constraints
    #[prop(optional, into)] maxlength: Option<String>,
    #[prop(optional, into)] minlength: Option<String>,

    // Accessibility
    #[prop(optional, into)] autocomplete: Option<String>,

    // Layout control
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional)] auto_size: bool,
) -> impl IntoView
```

### Real-World Impact

Missing HTML5 attributes can cause silent failures in production:

**Example**: A financial application using number inputs for decimal values:
```rust
// Without step support - BREAKS in production
<Input input_type="number" label="Volatility" />
// Browser rejects "0.25" because step defaults to 1

// With step support - WORKS
<Input input_type="number" step="0.01" label="Volatility" />
```

### HTML5 Attribute Reference

| Attribute | Applies To | Purpose | Example |
|-----------|-----------|---------|---------|
| `step` | number, date, time | Increment granularity | `step="0.01"` |
| `min` | number, date | Minimum value | `min="0"` |
| `max` | number, date | Maximum value | `max="100"` |
| `pattern` | text, tel, email | Regex validation | `pattern="[0-9]{3}-[0-9]{4}"` |
| `maxlength` | text, textarea | Max character count | `maxlength="280"` |
| `minlength` | text, textarea | Min character count | `minlength="8"` |
| `autocomplete` | all inputs | Browser autocomplete | `autocomplete="email"` |

## Component API Design

### Prop Patterns

1. **Use `Signal<T>` for reactive values that change externally**:
   ```rust
   #[prop(optional, into)] value: Signal<String>,
   #[prop(optional, into)] disabled: Signal<bool>,
   ```

2. **Use `RwSignal<T>` for two-way binding**:
   ```rust
   #[prop(optional)] value: Option<RwSignal<String>>,
   ```

3. **Use `#[prop(optional, into)]` for string-like props**:
   ```rust
   #[prop(optional, into)] placeholder: Option<String>,
   #[prop(optional, into)] label: Option<String>,
   ```

4. **Use plain `Option<T>` for simple configuration**:
   ```rust
   #[prop(optional)] variant: Option<ButtonVariant>,
   #[prop(optional)] size: Option<ButtonSize>,
   ```

### Variant and Size Enums

Use consistent naming across components:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputVariant {
    Default,    // Border with background
    Filled,     // Subtle background, no border
    Unstyled,   // No styling (for custom designs)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputSize {
    Xs,  // Extra small
    Sm,  // Small
    Md,  // Medium (default)
    Lg,  // Large
    Xl,  // Extra large
}
```

### Label and Description Pattern

Form components should support consistent helper text:

```rust
#[component]
pub fn FormInput(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    view! {
        <div class="form-field">
            {label.map(|l| view! {
                <label>
                    {l}
                    {if required { " *" } else { "" }}
                </label>
            })}

            <input /* ... */ />

            {description.map(|d| view! {
                <div class="description">{d}</div>
            })}

            {error.map(|e| view! {
                <div class="error">{e}</div>
            })}
        </div>
    }
}
```

## Testing Strategy

### Integration Testing First

The most valuable feedback comes from real-world integration:

1. **Build actual applications** using the component library
2. **Document pain points** in API_IMPROVEMENTS.md
3. **Iterate based on actual usage** rather than theoretical design

### What to Test

1. **Component compilation** with minimal props:
   ```rust
   <Input />  // Should compile with all defaults
   ```

2. **Callback omission**:
   ```rust
   <Button>"Click me"</Button>  // Should work without on_click
   ```

3. **Type inference**:
   ```rust
   <Input label="Email" />  // Should not require type annotations
   ```

4. **HTML5 validation**:
   ```rust
   <Input input_type="number" step="0.01" min="0" max="100" />
   ```

### Pre-commit Checks

Ensure all commits pass:
- `cargo fmt` - Code formatting
- `cargo clippy` - Linting
- `cargo test` - All tests
- `cargo build` - Compilation

## Common Pitfalls

### 1. Callback Reference vs. Value

**Problem**: Attempting to call `&Callback<T>` instead of `Callback<T>`

```rust
// ❌ Wrong
let handle_click = move |_| {
    if let Some(callback) = &on_close {
        callback();  // Error: &Callback is not callable
    }
};

// ✅ Correct
let handle_click = move |_| {
    if let Some(callback) = on_close {
        callback.run(());  // Use .run() method
    }
};
```

### 2. Callback Wrapping in Templates

**Problem**: Passing closures directly to components expecting `Callback<T>`

```rust
// ❌ Wrong
<Button on_click=move |_| { /* ... */ }/>

// ✅ Correct
<Button on_click=Callback::new(move |_| { /* ... */ })/>
```

### 3. Missing HTML5 Attributes

**Problem**: Assuming browser defaults are sufficient

```rust
// ❌ Incomplete - breaks for decimal inputs
<Input input_type="number" />

// ✅ Complete - supports decimal values
<Input input_type="number" step="0.01" />
```

### 4. Ignoring Integration Feedback

**Problem**: Implementing features without user testing

**Solution**:
- Build real applications with the library
- Document issues in API_IMPROVEMENTS.md
- Prioritize blocking issues (like missing HTML5 attributes)
- Iterate based on actual pain points

## Versioning and Breaking Changes

### When Callback API Changed

The callback API change from generic `Option<F>` to concrete `Option<Callback<T>>` was a breaking change that affected usage:

**Before**:
```rust
<Button on_click=move |_| {}/>
```

**After**:
```rust
<Button on_click=Callback::new(move |_| {})/>
```

This trade-off was necessary for reliable type inference and is now the standard Leptos pattern.

## Future Considerations

### 1. Accessibility (ARIA)

Consider adding ARIA attribute support:
```rust
#[prop(optional, into)] aria_label: Option<String>,
#[prop(optional, into)] aria_describedby: Option<String>,
```

### 2. Form Validation Integration

Future integration with validation framework:
```rust
#[prop(optional)] validator: Option<Validator>,
```

### 3. Ref Support

For direct DOM access:
```rust
#[prop(optional)] node_ref: Option<NodeRef<html::Input>>,
```

## Summary

The key lessons from developing Mingot components:

1. **Use concrete `Callback<T>` types** - Generic type parameters fail with optional callbacks
2. **Support all relevant HTML5 attributes** - Missing attributes cause production failures
3. **Test with real applications** - Integration testing reveals issues theory misses
4. **Document breaking changes** - Help users migrate when APIs evolve
5. **Prioritize blocking issues** - Fix critical problems (like decimal input validation) immediately

These guidelines ensure components are both developer-friendly and production-ready.
