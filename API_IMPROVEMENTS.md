# Mingot API Improvements

This document outlines API improvements discovered during integration with Ultramarine-Red project.

## Critical Issues (Blocking Integration)

### 1. Button: Missing `type` Attribute Support
**Current:** Button component doesn't expose HTML `type` attribute
**Impact:** Cannot create submit buttons for forms
**Use Case:**
```rust
// Desired API:
<Button button_type="submit">Submit</Button>
<Button button_type="button">Cancel</Button>

// Current workaround: Have to use raw <button> tags
```

**Suggested Fix:**
```rust
#[component]
pub fn Button(
    // ... existing props
    #[prop(optional)] button_type: Option<String>,  // "button" | "submit" | "reset"
    // ...
) -> impl IntoView {
    let button_type = button_type.unwrap_or_else(|| "button".to_string());

    view! {
        <button
            type=button_type
            // ... rest
        />
    }
}
```

### 2. Button: No Support for Link/Anchor Rendering
**Current:** Button only renders `<button>` elements
**Impact:** Navigation buttons require wrapper `<a>` tags or separate component
**Use Case:**
```rust
// Desired API:
<Button as_="a" href="/dashboard">Go to Dashboard</Button>

// Current workaround:
<a href="/dashboard"><Button>Go to Dashboard</Button></a>
// (not semantically correct, button inside anchor is poor HTML)
```

**Suggested Fix:**
```rust
#[component]
pub fn Button(
    // ... existing props
    #[prop(optional)] as_: Option<String>,  // "button" | "a"
    #[prop(optional)] href: Option<String>,
    // ...
) -> impl IntoView {
    let is_link = as_.as_ref().map(|s| s == "a").unwrap_or(false);

    if is_link {
        view! {
            <a
                href=href.unwrap_or_default()
                class=class_str
                style=move || button_styles()
            >
                {children()}
            </a>
        }
    } else {
        view! {
            <button
                type=button_type
                // ... existing
            >
                {children()}
            </button>
        }
    }
}
```

### 3. Select: Cannot Use Native HTML Children Syntax
**Current:** Requires `Vec<SelectOption>` via `options` prop
**Impact:** Cannot use familiar `<option>` syntax, harder migration from plain HTML
**Use Case:**
```rust
// Desired API (native HTML style):
<Select>
    <option value="free">"Free (60 req/min)"</option>
    <option value="pro">"Pro (600 req/min)"</option>
</Select>

// Current API (programmatic):
<Select options=vec![
    SelectOption::new("free", "Free (60 req/min)"),
    SelectOption::new("pro", "Pro (600 req/min)"),
]/>
```

**Suggested Fix:** Support both APIs
```rust
#[component]
pub fn Select(
    // ... existing props
    #[prop(optional)] options: Option<Vec<SelectOption>>,
    #[prop(optional)] children: Option<Children>,
    // ...
) -> impl IntoView {
    // If options provided, use them; otherwise parse children
    // This allows both programmatic and declarative syntax
}
```

### 4. Input/Select: Value Binding Not Flexible
**Current:** `value: Option<RwSignal<String>>`, creates own signal if None
**Impact:** Cannot easily use derived signals or one-way bindings
**Use Case:**
```rust
let email = RwSignal::new(String::new());

// Desired API (flexible):
<Input value=Signal::derive(move || email.get()) on_input=move |val| email.set(val) />
<Input value=email />  // Also works

// Current API (less flexible):
<Input value=email />  // Only this works
```

**Suggested Fix:** Accept `MaybeSignal<String>` for reads, separate callback for writes
```rust
#[component]
pub fn Input(
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional)] on_input: Option<Callback<String>>,
    // ...
)
```

### 5. Text: Missing `Xxl` Size Variant
**Current:** TextSize only has Xs, Sm, Md, Lg, Xl
**Impact:** Cannot create extra-large headings
**Use Case:**
```rust
// Desired for page titles:
<Text size=TextSize::Xxl weight=TextWeight::Bold>"Ultramarine-Red"</Text>
```

**Suggested Fix:** Add Xxl variant
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,  // Add this
}

// In theme:
pub struct FontSizes {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
    pub xxl: &'static str,  // e.g., "2rem" or "32px"
}
```

## High Priority Issues (Limits Reactivity)

### 6. Boolean Props Not Reactive
**Current:** Props like `disabled`, `loading`, `full_width` are plain `bool`
**Impact:** Cannot dynamically change these based on state
**Use Case:**
```rust
let loading = RwSignal::new(false);

// Desired API:
<Button loading=loading>Submit</Button>
<Button loading=Signal::derive(move || loading.get())>Submit</Button>

// Current API:
// Must recreate entire component when loading changes
```

**Suggested Fix:** Use `MaybeSignal<bool>` or `MaybeProp<bool>`
```rust
#[component]
pub fn Button(
    // ... existing props
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] loading: MaybeSignal<bool>,
    #[prop(optional, into)] full_width: MaybeSignal<bool>,
    // ...
) -> impl IntoView {
    let button_styles = move || {
        let is_disabled = disabled.get();
        let is_loading = loading.get();
        let is_full_width = full_width.get();
        // ... use these values reactively
    };
}
```

### 7. Text Align Should Be Type-Safe
**Current:** `align: Option<String>`
**Impact:** Typos cause silent failures, no autocomplete
**Use Case:**
```rust
// Current (error-prone):
<Text align="center">"Title"</Text>
<Text align="cneter">"Title"</Text>  // Silent failure!

// Desired (type-safe):
<Text align=TextAlign::Center>"Title"</Text>
```

**Suggested Fix:** Use enum with string fallback
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

#[component]
pub fn Text(
    #[prop(optional)] align: Option<TextAlign>,
    #[prop(optional)] align_custom: Option<String>,  // Escape hatch
    // ...
)
```

### 8. Input: Missing `on_change` Callback Implementation
**Current:** `on_change` prop exists but might not be wired up correctly
**Impact:** Cannot distinguish between input events and change events
**Use Case:**
```rust
// on_input: Fires on every keystroke
// on_change: Fires on blur/enter (for validation)
<Input
    on_input=move |val| email.set(val)  // Live update
    on_change=move |val| validate_email(val)  // Validate on blur
/>
```

**Verification Needed:** Check if on_change is properly connected to HTML change event

## Medium Priority Issues (Convenience)

### 9. Alert: Missing `title` Prop Type
**Current:** Alert component signature unclear
**Impact:** Used in integration but need to verify API
**Verification Needed:**
```rust
// Used this way in integration:
<Alert color="red" title="Error">
    {error_message}
</Alert>

// Verify Alert supports:
// - color prop
// - title prop
// - children for message
```

### 10. Badge: Size Enum Missing Documentation
**Current:** BadgeSize enum exists but available sizes unclear
**Impact:** Hard to know what sizes are available
**Use Case:**
```rust
<Badge size=BadgeSize::Lg color="blue">Pro</Badge>

// What sizes exist? Xs, Sm, Md, Lg, Xl?
```

**Suggested Fix:** Add doc comments to enum
```rust
/// Badge size variants
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BadgeSize {
    /// Extra small badge (height: 1rem)
    Xs,
    /// Small badge (height: 1.25rem)
    Sm,
    /// Medium badge (default) (height: 1.5rem)
    Md,
    /// Large badge (height: 1.75rem)
    Lg,
    /// Extra large badge (height: 2rem)
    Xl,
}
```

### 11. Grid: Span API Clarity
**Current:** `span=Span::Fixed(6)` works, but `md=Span::Fixed(12)` syntax unclear
**Impact:** Responsive grid usage not intuitive
**Use Case:**
```rust
// Used this way:
<GridCol span=Span::Fixed(6) md=Span::Fixed(12)>

// Questions:
// - Is md a prop on GridCol?
// - What's the full responsive API?
// - Can we do span={6} or must use Span::Fixed(6)?
```

**Suggested Fix:** Improve documentation and consider shorthand
```rust
// Option 1: Keep current, document well
<GridCol span=Span::Fixed(6) md=Span::Fixed(12)>

// Option 2: Add convenience conversions
<GridCol span=6 md=12>  // If From<u32> implemented
```

### 12. Card: `as_` Prop for Semantic HTML
**Current:** Used `<Card as_="a" href="/greeks">` in integration
**Impact:** Need to verify this works like Button should
**Verification Needed:**
```rust
// Used this way:
<Card as_="a" href="/greeks" padding="lg">
    <Text>"Greeks Calculator"</Text>
</Card>

// Verify Card supports:
// - as_ prop for rendering as different element
// - href prop when as_="a"
```

## Low Priority Issues (Polish)

### 13. Loading State Indicator
**Current:** Button accepts `loading` but unclear what it displays
**Impact:** Need to know if spinner is built-in or need custom indicator
**Verification Needed:** Does loading=true show a spinner automatically?

### 14. Form Components Validation Styling
**Current:** `error` prop exists on Input/Select
**Impact:** Need to verify error styling is automatic
**Verification Needed:**
```rust
<Input error="Email is required" />
// Does this automatically style the input red and show the message?
```

### 15. Divider Component Props
**Current:** Used `<Divider />` with no props
**Impact:** May need orientation, spacing, or style options
**Suggested Enhancement:**
```rust
<Divider orientation="horizontal" spacing="md" />
<Divider orientation="vertical" color="gray" />
```

## API Consistency Issues

### 16. Prop Naming Inconsistency
**Observation:** Some components use different naming for similar concepts
- Button: `full_width`
- Input: `width` implied by container
- Text: `align` as string vs other enums

**Suggested Fix:** Standardize naming patterns across components

### 17. Color Prop Consistency
**Observation:** Color props are strings, some components may have limited palette
- Button: `color: Option<String>` - seems to accept theme colors
- Badge: `color` - what values are valid?
- Alert: `color` - what values are valid?

**Suggested Fix:** Document valid color values or use enum with escape hatch
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum ThemeColor {
    Blue,
    Red,
    Green,
    Yellow,
    Gray,
    Custom(String),
}
```

## Testing Required

The following components were used but need verification:
1. ✅ Container - basic usage worked
2. ✅ Stack - basic usage worked
3. ✅ Paper - basic usage worked
4. ✅ Grid/GridCol - basic usage worked
5. ✅ Group - basic usage worked
6. ❓ Alert - need to verify title prop
7. ❓ Badge - need to verify size enum
8. ❓ Card - need to verify as_ prop
9. ❓ Divider - need to verify it exists
10. ❓ Select - need to verify options API

## Implementation Priority

### Phase 1: Critical (Needed for Integration)
1. Button: Add `button_type` prop
2. Button: Add `as_` and `href` props for links
3. Select: Support native children OR improve current API docs
4. Input/Select: More flexible value binding
5. Text: Add Xxl size

### Phase 2: High Priority (Reactivity)
6. Make boolean props reactive (disabled, loading, etc.)
7. Add TextAlign enum
8. Verify/fix on_change callbacks

### Phase 3: Medium Priority (Convenience)
9-12. Documentation and verification of existing components

### Phase 4: Low Priority (Polish)
13-15. Enhanced features and indicators

### Phase 5: Consistency
16-17. API standardization across components

## Examples of Desired Final API

```rust
// Login Form
view! {
    <Container>
        <Stack spacing="md">
            <Text size=TextSize::Xxl align=TextAlign::Center weight=TextWeight::Bold>
                "Login"
            </Text>

            <Alert color="red" title="Error" show=Signal::derive(move || error.get().is_some())>
                {move || error.get().unwrap_or_default()}
            </Alert>

            <form on:submit=handle_submit>
                <Stack spacing="md">
                    <Input
                        input_type="email"
                        label="Email"
                        value=email
                        on_input=move |val| email.set(val)
                        disabled=loading
                    />

                    <Input
                        input_type="password"
                        label="Password"
                        value=password
                        on_input=move |val| password.set(val)
                        disabled=loading
                    />

                    <Button
                        button_type="submit"
                        variant=ButtonVariant::Filled
                        loading=loading
                        full_width=true
                    >
                        "Login"
                    </Button>
                </Stack>
            </form>

            <Text size=TextSize::Sm align=TextAlign::Center>
                "Don't have an account? "
                <Button as_="a" href="/register" variant=ButtonVariant::Subtle>
                    "Sign up"
                </Button>
            </Text>
        </Stack>
    </Container>
}
```

## Additional Notes

- Mingot is inspired by Mantine, which has excellent documentation. Consider referencing Mantine's component APIs for additional inspiration.
- Leptos 0.7's signal system with `MaybeSignal` and `MaybeProp` makes reactive props very ergonomic.
- Consider reviewing other Leptos component libraries (leptonic, thaw-ui) for API patterns.

## References

- [Mantine Documentation](https://mantine.dev/)
- [Leptos Book - Component Props](https://book.leptos.dev/)
- [Leptos MaybeSignal](https://docs.rs/leptos/latest/leptos/reactive/signal/struct.MaybeSignal.html)
