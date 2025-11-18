# Mingot

A Leptos component library inspired by Mantine UI.

## Overview

Mingot provides a comprehensive set of UI components for Leptos applications, following the design principles and component API of Mantine UI. Built with Rust and WebAssembly, it offers type-safe, reactive components with a powerful theming system.

## Features

- **Type-safe Components**: Built with Rust for compile-time safety
- **Theming System**: Comprehensive theme support with colors, spacing, typography, and more
- **Mantine-inspired API**: Familiar API for developers coming from Mantine UI
- **Reactive**: Leverages Leptos's fine-grained reactivity
- **Customizable**: Easy to customize through props and theming

## Installation

Add Mingot to your `Cargo.toml`:

```toml
[dependencies]
mingot = "0.1.0"
leptos = "0.7"
```

## Quick Start

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

## Components

### MingotProvider

Wraps your application and provides theme context to all components.

```rust
use mingot::{MingotProvider, Theme};

view! {
    <MingotProvider theme=Some(Theme::default())>
        // Your app components
    </MingotProvider>
}
```

### Button

A versatile button component with multiple variants and sizes.

```rust
use mingot::{Button, ButtonVariant, ButtonSize};

// Filled button (default)
view! {
    <Button>"Click me"</Button>
}

// Outline button
view! {
    <Button variant=ButtonVariant::Outline>"Outline"</Button>
}

// Different sizes
view! {
    <Button size=ButtonSize::Lg>"Large Button"</Button>
}

// With custom color
view! {
    <Button color="red">"Danger"</Button>
}

// Full width
view! {
    <Button full_width=true>"Full Width"</Button>
}

// With click handler
view! {
    <Button on_click=move |_| {
        logging::log!("Button clicked!");
    }>
        "Click Handler"
    </Button>
}
```

**Props:**
- `variant`: `ButtonVariant` - Filled, Outline, Light, Subtle, or Default
- `size`: `ButtonSize` - Xs, Sm, Md (default), Lg, or Xl
- `color`: `String` - Color from theme (e.g., "blue", "red", "green")
- `radius`: `String` - Border radius override
- `full_width`: `bool` - Makes button take full width
- `disabled`: `bool` - Disables the button
- `loading`: `bool` - Shows loading state
- `on_click`: `Callback<MouseEvent>` - Click handler

### Container

A responsive container for page content with max-width constraints.

```rust
use mingot::{Container, ContainerSize};

// Default container
view! {
    <Container>
        "Your content here"
    </Container>
}

// Fluid container (no max-width)
view! {
    <Container fluid=true>
        "Full width content"
    </Container>
}

// Custom size
view! {
    <Container size=ContainerSize::Lg>
        "Large container"
    </Container>
}
```

### Text

Typography component for styling text.

```rust
use mingot::{Text, TextSize, TextWeight};

view! {
    <Text size=TextSize::Lg weight=TextWeight::Bold>
        "Bold large text"
    </Text>

    <Text color="red" italic=true>
        "Red italic text"
    </Text>

    <Text align="center">
        "Centered text"
    </Text>
}
```

**Props:**
- `size`: `TextSize` - Xs, Sm, Md (default), Lg, or Xl
- `weight`: `TextWeight` - Normal, Medium, Semibold, or Bold
- `color`: `String` - Text color (theme color or CSS color)
- `italic`: `bool` - Italic text
- `underline`: `bool` - Underlined text
- `align`: `String` - Text alignment (left, center, right)

### Stack

Layout component that arranges children vertically.

```rust
use mingot::{Stack, StackAlign, StackJustify};

view! {
    <Stack spacing="lg" align=StackAlign::Center>
        <div>"Item 1"</div>
        <div>"Item 2"</div>
        <div>"Item 3"</div>
    </Stack>
}
```

**Props:**
- `spacing`: `String` - Gap between children
- `align`: `StackAlign` - Horizontal alignment (Start, Center, End, Stretch)
- `justify`: `StackJustify` - Vertical distribution (Start, Center, End, SpaceBetween, SpaceAround)

### Group

Layout component that arranges children horizontally.

```rust
use mingot::{Group, GroupAlign, GroupJustify};

view! {
    <Group spacing="md" justify=GroupJustify::SpaceBetween>
        <div>"Left"</div>
        <div>"Right"</div>
    </Group>
}

// Wrapping group
view! {
    <Group wrap=true>
        <Button>"Button 1"</Button>
        <Button>"Button 2"</Button>
        <Button>"Button 3"</Button>
    </Group>
}
```

**Props:**
- `spacing`: `String` - Gap between children
- `align`: `GroupAlign` - Vertical alignment (Start, Center, End, Baseline)
- `justify`: `GroupJustify` - Horizontal distribution (Start, Center, End, SpaceBetween, SpaceAround)
- `wrap`: `bool` - Allow wrapping to multiple lines

## Theming

### Default Theme

Mingot comes with a default theme that includes:

- **Colors**: Blue (primary), Gray, Red, Green, Yellow with 10 shades each
- **Spacing**: xs (10px), sm (12px), md (16px), lg (20px), xl (32px)
- **Font Sizes**: xs (12px), sm (14px), md (16px), lg (18px), xl (20px)
- **Border Radius**: xs-xl scale
- **Shadows**: xs-xl scale
- **Breakpoints**: xs (36em), sm (48em), md (62em), lg (75em), xl (88em)
- **Dark Mode**: Full support with separate light and dark color palettes

### Dark Mode

Mingot has built-in dark mode support with separate color palettes for light and dark themes.

#### Using Dark Mode

```rust
use mingot::prelude::*;

#[component]
fn App() -> impl IntoView {
    // Start with dark mode
    let mut theme = Theme::default();
    theme.color_scheme = ColorSchemeMode::Dark;

    view! {
        <MingotProvider theme=Some(theme)>
            <YourApp />
        </MingotProvider>
    }
}
```

#### Toggling Dark Mode

Use the `use_color_scheme_toggle` hook to toggle between light and dark modes:

```rust
use mingot::prelude::*;

#[component]
fn ThemeToggleButton() -> impl IntoView {
    let toggle_theme = use_color_scheme_toggle();

    view! {
        <Button on_click=move |_| toggle_theme()>
            "Toggle Dark Mode"
        </Button>
    }
}
```

#### Setting Color Scheme

You can explicitly set the color scheme using `use_set_color_scheme`:

```rust
use mingot::prelude::*;

#[component]
fn ThemeSelector() -> impl IntoView {
    let set_color_scheme = use_set_color_scheme();

    view! {
        <Group>
            <Button on_click=move |_| set_color_scheme(ColorSchemeMode::Light)>
                "Light"
            </Button>
            <Button on_click=move |_| set_color_scheme(ColorSchemeMode::Dark)>
                "Dark"
            </Button>
            <Button on_click=move |_| set_color_scheme(ColorSchemeMode::Auto)>
                "Auto"
            </Button>
        </Group>
    }
}
```

#### Getting Current Color Scheme

```rust
use mingot::prelude::*;

#[component]
fn CurrentTheme() -> impl IntoView {
    let get_color_scheme = use_color_scheme();

    view! {
        <Text>
            {move || format!("Current theme: {:?}", get_color_scheme())}
        </Text>
    }
}
```

#### Color Scheme Modes

- **Light**: Always use light mode
- **Dark**: Always use dark mode
- **Auto**: Use system preference (currently defaults to light, system detection coming soon)

### Custom Theme

You can create a custom theme:

```rust
use mingot::{Theme, ColorSchemeMode};

let mut custom_theme = Theme::default();
custom_theme.colors.primary_color = "green".to_string();
custom_theme.color_scheme = ColorSchemeMode::Dark;

view! {
    <MingotProvider theme=Some(custom_theme)>
        // Your app
    </MingotProvider>
}
```

### Using Theme in Components

```rust
use mingot::prelude::*;

#[component]
fn MyComponent() -> impl IntoView {
    let theme = use_theme();

    let custom_style = move || {
        let theme_val = theme.get();
        let scheme_colors = mingot::theme::get_scheme_colors(&theme_val);
        let primary = scheme_colors.get_color(&theme_val.colors.primary_color, 6)
            .unwrap_or_else(|| "#228be6".to_string());
        format!("color: {}", primary)
    };

    view! {
        <div style=custom_style>
            "Themed content"
        </div>
    }
}
```

## Examples

### Login Form

```rust
use leptos::prelude::*;
use mingot::prelude::*;

#[component]
fn LoginForm() -> impl IntoView {
    view! {
        <MingotProvider>
            <Container size=ContainerSize::Xs>
                <Stack spacing="lg">
                    <Text size=TextSize::Xl weight=TextWeight::Bold align="center">
                        "Login"
                    </Text>

                    <Stack spacing="sm">
                        // Input fields would go here
                        <Button full_width=true>"Sign In"</Button>
                        <Button variant=ButtonVariant::Subtle full_width=true>
                            "Forgot Password?"
                        </Button>
                    </Stack>
                </Stack>
            </Container>
        </MingotProvider>
    }
}
```

### Button Group

```rust
view! {
    <Group spacing="sm">
        <Button variant=ButtonVariant::Filled>"Save"</Button>
        <Button variant=ButtonVariant::Outline>"Cancel"</Button>
        <Button variant=ButtonVariant::Subtle color="red">"Delete"</Button>
    </Group>
}
```

### Dark Mode Toggle

```rust
use leptos::prelude::*;
use mingot::prelude::*;

#[component]
fn AppWithDarkMode() -> impl IntoView {
    view! {
        <MingotProvider>
            <Container>
                <Stack spacing="lg">
                    <Group justify=GroupJustify::SpaceBetween>
                        <Text size=TextSize::Xl weight=TextWeight::Bold>
                            "My Application"
                        </Text>
                        <ThemeToggle />
                    </Group>

                    <Text>"This content adapts to the theme automatically!"</Text>

                    <Group>
                        <Button>"Primary Action"</Button>
                        <Button variant=ButtonVariant::Outline>"Secondary"</Button>
                    </Group>
                </Stack>
            </Container>
        </MingotProvider>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let toggle_theme = use_color_scheme_toggle();
    let get_scheme = use_color_scheme();

    view! {
        <Button
            variant=ButtonVariant::Subtle
            on_click=move |_| toggle_theme()
        >
            {move || match get_scheme() {
                ColorSchemeMode::Light => "🌙 Dark",
                ColorSchemeMode::Dark => "☀️ Light",
                ColorSchemeMode::Auto => "🔄 Auto",
            }}
        </Button>
    }
}
```

## Roadmap

- [ ] More components (Input, Select, Modal, Drawer, etc.)
- [x] Dark mode support
- [ ] System dark mode detection (prefers-color-scheme)
- [ ] CSS-in-Rust styling with style generation
- [ ] More comprehensive theming options
- [ ] Accessibility improvements
- [ ] Component composition utilities
- [ ] Form components and validation
- [ ] Data display components (Table, Card, etc.)
- [ ] Smooth transitions between themes
- [ ] LocalStorage persistence for theme preference

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

Inspired by [Mantine UI](https://mantine.dev/) - A fully featured React components library.
