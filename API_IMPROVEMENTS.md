# Mingot API Improvements

This document outlines API improvements discovered during integration with Ultramarine-Red project.

## Latest Integration Attempt - Iteration 4

**Date:** 2025-11-18
**Status:** 15 compilation errors remaining (down from 20!)
**Progress:**
- ✅ String props fixed (Iteration 2)
- ✅ Alert & Card enum props fixed (Iteration 3)
- ❌ Callback props still failing - need generic type parameters

### Remaining Errors Summary:
- **15 errors**: Callback type mismatch - `expected Callback<T>, found closure`
  - 14× `Callback<String>` (Input `on_input`, Select `on_change`)
  - 1× `Callback<MouseEvent>` (Button `on_click`)

---

## CRITICAL: Callback Props Need Generic Type Parameters

**Status:** BLOCKING - 15 compilation errors
**Error:** `mismatched types: expected Callback<T>, found closure`

The current approach of using concrete `Callback<T>` types doesn't allow Leptos to automatically convert closures. In Leptos 0.7, the recommended pattern is to use generic type parameters with `Fn` trait bounds.

**Affected Props & Error Locations:**
- **Input `on_input`** (14 errors):
  - `greeks.rs:143, 153, 160, 168, 176, 183, 191, 199` (8 instances)
  - `login.rs:82, 91` (2 instances)
  - `register.rs:97, 106, 115, 127` (4 instances)
- **Button `on_click`** (1 error):
  - `dashboard.rs:26`

**Current Usage Pattern (Failing):**
```rust
// User code (what we want to write):
<Input on_input=move |val| email.set(val) />
<Button on_click=move |_| handle_click() />

// Current component signature (doesn't accept closures):
#[component]
pub fn Input(
    #[prop(optional)] on_input: Option<Callback<String>>,  // ❌ Too specific
    // ...
)
```

**Root Cause:**
When a component prop is typed as `Callback<T>`, Leptos doesn't automatically convert closures to Callback. The component must explicitly accept generic function types.

**Fix Required - Option 1 (Generic Type Parameter):**
```rust
#[component]
pub fn Input<F>(
    #[prop(optional)] on_input: Option<F>,
    #[prop(optional, into)] label: Option<String>,
    // ... other props
) -> impl IntoView
where
    F: Fn(String) + 'static,  // Accept any closure that takes String
{
    let on_input = on_input.map(Callback::new);  // Convert to Callback internally

    // In the view:
    view! {
        <input
            on:input=move |ev| {
                if let Some(cb) = &on_input {
                    cb(event_target_value(&ev));
                }
            }
        />
    }
}
```

**Fix Required - Option 2 (Multiple Generic Parameters):**
For components with multiple callbacks:
```rust
#[component]
pub fn Input<FI, FC>(
    #[prop(optional)] on_input: Option<FI>,
    #[prop(optional)] on_change: Option<FC>,
    #[prop(optional, into)] label: Option<String>,
    // ... other props
) -> impl IntoView
where
    FI: Fn(String) + 'static,
    FC: Fn(String) + 'static,
{
    let on_input = on_input.map(Callback::new);
    let on_change = on_change.map(Callback::new);
    // ...
}
```

**Fix Required - Option 3 (Use Into with Proper Bounds):**
If you want to keep using `Callback<T>`, use `into` with a generic that converts:
```rust
#[component]
pub fn Input<FI>(
    #[prop(optional, into)] on_input: Option<Callback<String, FI>>,
    // ...
) -> impl IntoView
where
    FI: Fn(String) + 'static,
{
    // Callback already constructed, use directly
}
```

**Recommended Approach:**
Use Option 1 or 2 (generic type parameters) as this is the most idiomatic Leptos 0.7 pattern and provides maximum flexibility.

**Components That Need This Fix:**
- `Input` - needs generics for `on_input`, `on_change`
- `Select` - needs generics for `on_change`
- `Button` - needs generics for `on_click`
- Any other component with callback props

---

## Alert and Card Enum Props

**Status:** RESOLVED ✅ (Iteration 3)

Alert's `color` prop and Card's `padding` prop now accept string values with `From<&str>` implementations and `#[prop(into)]`.

**Pattern Applied:**
```rust
// For Alert:
impl From<&str> for AlertColor {
    fn from(s: &str) -> Self {
        match s {
            "red" => AlertColor::Red,
            "blue" => AlertColor::Blue,
            "green" => AlertColor::Green,
            "yellow" => AlertColor::Yellow,
            _ => AlertColor::Blue,  // Default
        }
    }
}

#[component]
pub fn Alert(
    #[prop(optional, into)] color: Option<AlertColor>,
    #[prop(optional, into)] title: Option<String>,
    children: Children,
) -> impl IntoView {
    // ...
}
```

**Usage (Now Works):**
```rust
<Alert color="red" title="Error">
    {error_message}
</Alert>

<Card padding="lg" radius="md">
    {content}
</Card>
```

---

## CRITICAL: String Props Missing `#[prop(into)]`

**Status:** RESOLVED ✅ (Iteration 2)

Many components accept `String` props but don't have `#[prop(into)]`, which means users must call `.to_string()` on every string literal. This creates significant friction.

**Affected Components:**
- **Stack**: `spacing` expects `String` (should accept `&str`)
- **Paper**: `radius` expects `String`, `padding` expects `PaperPadding` enum
- **Button**: `button_type` expects `String` (should accept `&str`)
- **Input**: `label` expects `String` (should accept `&str`)
- **Select**: `label` expects `String` (should accept `&str`)
- **All components**: `style` and `class` expect `String`

**Impact:**
```rust
// CURRENT (frustrating):
<Stack spacing="md".to_string()>
<Input label="Email".to_string() />

// DESIRED (ergonomic):
<Stack spacing="md">
<Input label="Email" />
```

**Fix Required:** Add `#[prop(into)]` to all `String` parameters:
```rust
#[component]
pub fn Stack(
    #[prop(optional, into)] spacing: String,  // Add 'into'
    #[prop(optional, into)] class: String,     // Add 'into'
    #[prop(optional, into)] style: String,     // Add 'into'
    // ...
)
```

## Paper Padding Enum Support

**Status:** RESOLVED ✅ (Iteration 2) - Same pattern should apply to all enum props

Paper component's `padding` prop now accepts strings. The same pattern should be applied to Card, Alert, and other components with enum props like color, padding, size, etc.

**Pattern Applied:**
```rust
impl From<&str> for PaperPadding {
    fn from(s: &str) -> Self {
        match s {
            "xs" => PaperPadding::Xs,
            "sm" => PaperPadding::Sm,
            "lg" => PaperPadding::Lg,
            "xl" => PaperPadding::Xl,
            _ => PaperPadding::Md,
        }
    }
}

#[component]
pub fn Paper(
    #[prop(optional, into)] padding: Option<PaperPadding>,
    // ...
)
```

**Note:** Alert and Card still need this same pattern applied (see critical issues above).

---

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
