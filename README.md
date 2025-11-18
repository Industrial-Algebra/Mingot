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

### Grid

A powerful responsive grid layout system based on CSS Grid with 12-column default.

```rust
use mingot::{Grid, GridCol};

// Basic grid with equal columns
view! {
    <Grid>
        <GridCol span=6>
            <div>"Half width column"</div>
        </GridCol>
        <GridCol span=6>
            <div>"Half width column"</div>
        </GridCol>
    </Grid>
}

// Responsive grid - different spans at different breakpoints
view! {
    <Grid>
        <GridCol span=12 md=6 lg=4>
            <div>"Full width on mobile, half on tablet, third on desktop"</div>
        </GridCol>
        <GridCol span=12 md=6 lg=4>
            <div>"Full width on mobile, half on tablet, third on desktop"</div>
        </GridCol>
        <GridCol span=12 md=12 lg=4>
            <div>"Full width on mobile and tablet, third on desktop"</div>
        </GridCol>
    </Grid>
}

// Grid with custom columns and gutter
view! {
    <Grid columns=24 gutter="xl">
        <GridCol span=8>
            <div>"8 of 24 columns"</div>
        </GridCol>
        <GridCol span=16>
            <div>"16 of 24 columns"</div>
        </GridCol>
    </Grid>
}

// Grid with offset
view! {
    <Grid>
        <GridCol span=4 offset=4>
            <div>"4 columns wide, offset by 4"</div>
        </GridCol>
    </Grid>
}

// Aligned grid
view! {
    <Grid align=GridAlign::Center justify=GridJustify::SpaceBetween>
        <GridCol span=3>
            <div style="height: 100px;">"Tall content"</div>
        </GridCol>
        <GridCol span=3>
            <div>"Short"</div>
        </GridCol>
        <GridCol span=3>
            <div>"Centered vertically"</div>
        </GridCol>
    </Grid>
}
```

**Grid Props:**
- `columns`: `u32` - Number of grid columns (default: 12)
- `gutter`: `String` - Gap between grid items (default: theme.spacing.md)
- `align`: `GridAlign` - Vertical alignment (Start, Center, End, Stretch)
- `justify`: `GridJustify` - Horizontal distribution (Start, Center, End, SpaceBetween, SpaceAround)
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**GridCol Props:**
- `span`: `u32` - Number of columns to span (default: 12)
- `offset`: `u32` - Number of columns to skip before this column
- `xs`: `u32` - Span at xs breakpoint (≥36em)
- `sm`: `u32` - Span at sm breakpoint (≥48em)
- `md`: `u32` - Span at md breakpoint (≥62em)
- `lg`: `u32` - Span at lg breakpoint (≥75em)
- `xl`: `u32` - Span at xl breakpoint (≥88em)
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**Responsive Behavior:**

The responsive props (`xs`, `sm`, `md`, `lg`, `xl`) define the column span at different screen sizes:
- Mobile-first approach: spans cascade upward
- If only `span` is set, it applies to all breakpoints
- Each breakpoint prop overrides the previous one at that screen size

### SimpleGrid

A simpler grid component for auto-fit layouts without manual column spans.

```rust
use mingot::SimpleGrid;

// Fixed number of columns
view! {
    <SimpleGrid cols=3 spacing="lg">
        <div>"Column 1"</div>
        <div>"Column 2"</div>
        <div>"Column 3"</div>
        <div>"Column 4"</div>
        <div>"Column 5"</div>
        <div>"Column 6"</div>
    </SimpleGrid>
}

// Auto-fit based on minimum child width
view! {
    <SimpleGrid min_child_width="250px" spacing="md">
        <div>"Item 1"</div>
        <div>"Item 2"</div>
        <div>"Item 3"</div>
        <div>"Item 4"</div>
    </SimpleGrid>
}
```

**SimpleGrid Props:**
- `cols`: `u32` - Fixed number of columns (default: 1)
- `spacing`: `String` - Gap between items (default: theme.spacing.md)
- `min_child_width`: `String` - Minimum width for auto-fit layout (e.g., "200px")
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**Note:** Use `cols` for a fixed grid or `min_child_width` for a responsive auto-fit grid. `min_child_width` takes precedence if both are provided.

### Header

A header component for page layouts with positioning options.

```rust
use mingot::{Header, HeaderHeight, HeaderPosition, Navbar, NavbarLink, NavbarBrand};

// Basic header
view! {
    <Header with_border=true>
        <Container>
            "My Application"
        </Container>
    </Header>
}

// Sticky header with navigation
view! {
    <Header
        height=HeaderHeight::Md
        position=HeaderPosition::Sticky
        with_border=true
    >
        <Container>
            <Group justify=GroupJustify::SpaceBetween>
                <NavbarBrand href="/">
                    "MyApp"
                </NavbarBrand>
                <Navbar>
                    <NavbarLink href="/" active=true>"Home"</NavbarLink>
                    <NavbarLink href="/about">"About"</NavbarLink>
                    <NavbarLink href="/contact">"Contact"</NavbarLink>
                </Navbar>
            </Group>
        </Container>
    </Header>
}

// Fixed header
view! {
    <Header
        position=HeaderPosition::Fixed
        height=HeaderHeight::Sm
        with_border=true
    >
        "Fixed header content"
    </Header>
}
```

**Header Props:**
- `height`: `HeaderHeight` - Xs (48px), Sm (60px), Md (72px, default), Lg (84px), Xl (96px)
- `position`: `HeaderPosition` - Static (default), Fixed, or Sticky
- `with_border`: `bool` - Add bottom border (default: false)
- `padding`: `String` - Custom padding (default: "0 {theme.spacing.md}")
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

### Footer

A footer component for page layouts.

```rust
use mingot::{Footer, FooterHeight, FooterPosition};

// Basic footer
view! {
    <Footer with_border=true>
        <Container>
            <Text size=TextSize::Sm>
                "© 2024 MyApp. All rights reserved."
            </Text>
        </Container>
    </Footer>
}

// Footer with multiple sections
view! {
    <Footer
        height=FooterHeight::Lg
        with_border=true
    >
        <Container>
            <Grid>
                <GridCol span=4>
                    <Stack spacing="xs">
                        <Text weight=TextWeight::Bold>"Company"</Text>
                        <Text size=TextSize::Sm>"About Us"</Text>
                        <Text size=TextSize::Sm>"Careers"</Text>
                    </Stack>
                </GridCol>
                <GridCol span=4>
                    <Stack spacing="xs">
                        <Text weight=TextWeight::Bold>"Support"</Text>
                        <Text size=TextSize::Sm>"Help Center"</Text>
                        <Text size=TextSize::Sm>"Contact"</Text>
                    </Stack>
                </GridCol>
                <GridCol span=4>
                    <Stack spacing="xs">
                        <Text weight=TextWeight::Bold>"Legal"</Text>
                        <Text size=TextSize::Sm>"Privacy"</Text>
                        <Text size=TextSize::Sm>"Terms"</Text>
                    </Stack>
                </GridCol>
            </Grid>
        </Container>
    </Footer>
}

// Fixed footer
view! {
    <Footer
        position=FooterPosition::Fixed
        height=FooterHeight::Md
        with_border=true
    >
        "Footer content"
    </Footer>
}
```

**Footer Props:**
- `height`: `FooterHeight` - Xs (48px), Sm (60px), Md (72px, default), Lg (84px), Xl (96px)
- `position`: `FooterPosition` - Static (default) or Fixed
- `with_border`: `bool` - Add top border (default: false)
- `padding`: `String` - Custom padding (default: "0 {theme.spacing.md}")
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

### Navbar

A navigation bar component with link support and active states.

```rust
use mingot::{Navbar, NavbarLink, NavbarBrand, NavbarOrientation, NavbarVariant};

// Horizontal navbar (default)
view! {
    <Navbar>
        <NavbarBrand href="/">"MyApp"</NavbarBrand>
        <NavbarLink href="/" active=true>"Home"</NavbarLink>
        <NavbarLink href="/products">"Products"</NavbarLink>
        <NavbarLink href="/about">"About"</NavbarLink>
        <NavbarLink href="/contact">"Contact"</NavbarLink>
    </Navbar>
}

// Vertical navbar
view! {
    <Navbar orientation=NavbarOrientation::Vertical>
        <NavbarLink href="/" active=true>"Dashboard"</NavbarLink>
        <NavbarLink href="/users">"Users"</NavbarLink>
        <NavbarLink href="/settings">"Settings"</NavbarLink>
    </Navbar>
}

// Different navbar variants
view! {
    <Stack spacing="lg">
        // Default variant (underline)
        <Navbar>
            <NavbarLink href="/" active=true variant=NavbarVariant::Default>
                "Home"
            </NavbarLink>
            <NavbarLink href="/about" variant=NavbarVariant::Default>
                "About"
            </NavbarLink>
        </Navbar>

        // Subtle variant (background highlight)
        <Navbar>
            <NavbarLink href="/" active=true variant=NavbarVariant::Subtle>
                "Home"
            </NavbarLink>
            <NavbarLink href="/about" variant=NavbarVariant::Subtle>
                "About"
            </NavbarLink>
        </Navbar>

        // Pills variant (rounded background)
        <Navbar>
            <NavbarLink href="/" active=true variant=NavbarVariant::Pills>
                "Home"
            </NavbarLink>
            <NavbarLink href="/about" variant=NavbarVariant::Pills>
                "About"
            </NavbarLink>
        </Navbar>
    </Stack>
}

// Navbar with click handlers
view! {
    <Navbar>
        <NavbarLink
            href="/dashboard"
            on_click=Callback::new(|ev| {
                ev.prevent_default();
                // Handle navigation
                logging::log!("Navigate to dashboard");
            })
        >
            "Dashboard"
        </NavbarLink>
    </Navbar>
}

// Disabled link
view! {
    <Navbar>
        <NavbarLink href="/" active=true>"Home"</NavbarLink>
        <NavbarLink href="/coming-soon" disabled=true>"Coming Soon"</NavbarLink>
    </Navbar>
}
```

**Navbar Props:**
- `orientation`: `NavbarOrientation` - Horizontal (default) or Vertical
- `spacing`: `String` - Gap between nav items (default: theme.spacing.sm)
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**NavbarLink Props:**
- `href`: `String` - Link destination
- `active`: `bool` - Whether this link is active (default: false)
- `variant`: `NavbarVariant` - Default, Subtle, or Pills (default: Default)
- `disabled`: `bool` - Disable the link (default: false)
- `on_click`: `Callback<MouseEvent>` - Click handler
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**NavbarBrand Props:**
- `href`: `String` - Link destination (optional, renders as div if not provided)
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

### Banner

An announcement banner component for important messages at the top or bottom of the page.

```rust
use mingot::{Banner, BannerVariant, BannerPosition};

// Basic info banner
view! {
    <Banner variant=BannerVariant::Info>
        "Welcome to our new website! Check out our latest features."
    </Banner>
}

// Success banner with icon
view! {
    <Banner variant=BannerVariant::Success icon="✓">
        "Your changes have been saved successfully!"
    </Banner>
}

// Warning banner
view! {
    <Banner variant=BannerVariant::Warning icon="⚠">
        "This feature will be deprecated in the next release."
    </Banner>
}

// Error banner
view! {
    <Banner variant=BannerVariant::Error icon="✕">
        "An error occurred while processing your request."
    </Banner>
}

// Dismissible banner with controlled state
let banner_open = RwSignal::new(true);
view! {
    <Banner
        variant=BannerVariant::Info
        dismissible=true
        opened=banner_open
        on_close=Callback::new(move |_| {
            logging::log!("Banner closed");
        })
    >
        "This banner can be dismissed!"
    </Banner>
}

// Sticky banner
view! {
    <Banner
        variant=BannerVariant::Success
        position=BannerPosition::Sticky
        with_border=true
    >
        "Limited time offer! Get 50% off on all products."
    </Banner>
}

// Fixed banner
view! {
    <Banner
        variant=BannerVariant::Info
        position=BannerPosition::Fixed
    >
        "🎉 New version 2.0 is now available!"
    </Banner>
}
```

**Banner Props:**
- `variant`: `BannerVariant` - Info (default), Success, Warning, Error, or Default
- `position`: `BannerPosition` - Static (default), Fixed, or Sticky
- `with_border`: `bool` - Add bottom border (default: false)
- `dismissible`: `bool` - Show close button (default: false)
- `opened`: `RwSignal<bool>` - Control visibility (default: internal signal set to true)
- `on_close`: `Callback<()>` - Called when banner is closed
- `icon`: `String` - Optional icon to display before content
- `padding`: `String` - Custom padding (default: "{theme.spacing.sm} {theme.spacing.md}")
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**Variants:**
- **Info**: Blue color scheme for informational messages
- **Success**: Green color scheme for success messages
- **Warning**: Yellow color scheme for warnings
- **Error**: Red color scheme for errors
- **Default**: Gray color scheme for neutral messages

### Hero

A large hero section component for landing pages with title, subtitle, and call-to-action buttons.

```rust
use mingot::{Hero, HeroTitle, HeroSubtitle, HeroActions, HeroHeight, HeroAlign};

// Basic hero
view! {
    <Hero>
        <HeroTitle>"Welcome to Mingot"</HeroTitle>
        <HeroSubtitle>
            "A beautiful component library for Leptos applications"
        </HeroSubtitle>
        <HeroActions>
            <Button size=ButtonSize::Lg>"Get Started"</Button>
            <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                "Learn More"
            </Button>
        </HeroActions>
    </Hero>
}

// Hero with custom height
view! {
    <Hero height=HeroHeight::Lg>
        <HeroTitle>"Build Amazing Apps"</HeroTitle>
        <HeroSubtitle>
            "Fast, type-safe, and beautiful components for your next project"
        </HeroSubtitle>
    </Hero>
}

// Left-aligned hero
view! {
    <Hero align=HeroAlign::Left>
        <Container>
            <HeroTitle>"Start Building Today"</HeroTitle>
            <HeroSubtitle>
                "Create stunning user interfaces with our component library"
            </HeroSubtitle>
            <HeroActions>
                <Button>"Try it now"</Button>
            </HeroActions>
        </Container>
    </Hero>
}

// Hero with background color
view! {
    <Hero
        height=HeroHeight::Full
        background_color="#1c7ed6"
        align=HeroAlign::Center
    >
        <HeroTitle color="#ffffff">
            "Transform Your Ideas"
        </HeroTitle>
        <HeroSubtitle color="#ffffff">
            "Into beautiful web applications"
        </HeroSubtitle>
        <HeroActions>
            <Button size=ButtonSize::Xl>"Get Started Free"</Button>
        </HeroActions>
    </Hero>
}

// Hero with background image and overlay
view! {
    <Hero
        height=HeroHeight::Xl
        background_image="/hero-bg.jpg"
        overlay=true
        overlay_opacity=0.5
    >
        <HeroTitle color="#ffffff">
            "Beautiful Backgrounds"
        </HeroTitle>
        <HeroSubtitle color="#ffffff">
            "Create stunning hero sections with custom backgrounds"
        </HeroSubtitle>
        <HeroActions>
            <Button variant=ButtonVariant::Light size=ButtonSize::Lg>
                "Explore"
            </Button>
        </HeroActions>
    </Hero>
}

// Complex hero with custom layout
view! {
    <Hero height=HeroHeight::Lg background_color="#f8f9fa">
        <Container size=ContainerSize::Md>
            <Stack spacing="xl" align=StackAlign::Center>
                <HeroTitle>"Supercharge Your Development"</HeroTitle>
                <HeroSubtitle>
                    "Join thousands of developers building with Mingot. \
                     Fast, reliable, and easy to use."
                </HeroSubtitle>
                <HeroActions>
                    <Button size=ButtonSize::Lg color="blue">
                        "Start Free Trial"
                    </Button>
                    <Button variant=ButtonVariant::Subtle size=ButtonSize::Lg>
                        "View Demo"
                    </Button>
                </HeroActions>
                <Text size=TextSize::Sm color="gray">
                    "No credit card required • 14-day free trial"
                </Text>
            </Stack>
        </Container>
    </Hero>
}
```

**Hero Props:**
- `height`: `HeroHeight` - Sm (300px), Md (400px, default), Lg (500px), Xl (600px), or Full (100vh)
- `align`: `HeroAlign` - Left, Center (default), or Right
- `background_color`: `String` - Background color (CSS color value)
- `background_image`: `String` - Background image URL
- `overlay`: `bool` - Add dark overlay on top of background (default: false)
- `overlay_opacity`: `f32` - Overlay opacity 0.0-1.0 (default: 0.6)
- `padding`: `String` - Custom padding (default: "{theme.spacing.xl} {theme.spacing.md}")
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**HeroTitle Props:**
- `color`: `String` - Text color (CSS color value)
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**HeroSubtitle Props:**
- `color`: `String` - Text color (CSS color value)
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**HeroActions Props:**
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**Usage Tips:**
- Use `HeroTitle` for the main heading (renders as `<h1>`)
- Use `HeroSubtitle` for supporting text (renders as `<p>`)
- Use `HeroActions` to group call-to-action buttons
- Combine with `Container` for constrained content width
- Set `overlay=true` when using background images to improve text readability
- Use custom `color` props on HeroTitle/HeroSubtitle when using dark backgrounds

### Avatar

A user profile picture component with image support, initials fallback, and grouping.

```rust
use mingot::{Avatar, AvatarGroup, AvatarSize, AvatarRadius};

// Basic avatar with image
view! {
    <Avatar
        src="/user-photo.jpg"
        alt="John Doe"
    />
}

// Avatar with initials fallback
view! {
    <Avatar
        initials="JD"
        color="blue"
    />
}

// Different sizes
view! {
    <Stack spacing="sm">
        <Avatar src="/user.jpg" size=AvatarSize::Xs />
        <Avatar src="/user.jpg" size=AvatarSize::Sm />
        <Avatar src="/user.jpg" size=AvatarSize::Md />
        <Avatar src="/user.jpg" size=AvatarSize::Lg />
        <Avatar src="/user.jpg" size=AvatarSize::Xl />
    </Stack>
}

// Different radius
view! {
    <Group>
        <Avatar initials="JD" radius=AvatarRadius::Xs />
        <Avatar initials="JD" radius=AvatarRadius::Md />
        <Avatar initials="JD" radius=AvatarRadius::Full />
    </Group>
}

// Different colors
view! {
    <Group>
        <Avatar initials="AB" color="blue" />
        <Avatar initials="CD" color="red" />
        <Avatar initials="EF" color="green" />
    </Group>
}

// Avatar group (overlapping avatars)
view! {
    <AvatarGroup>
        <Avatar src="/user1.jpg" />
        <Avatar src="/user2.jpg" />
        <Avatar src="/user3.jpg" />
        <Avatar initials="+5" />
    </AvatarGroup>
}

// Avatar group with spacing
view! {
    <AvatarGroup spacing="sm">
        <Avatar initials="AB" color="blue" />
        <Avatar initials="CD" color="red" />
        <Avatar initials="EF" color="green" />
    </AvatarGroup>
}
```

**Avatar Props:**
- `src`: `String` - Image source URL
- `alt`: `String` - Alt text for image
- `size`: `AvatarSize` - Xs (16px), Sm (26px), Md (38px, default), Lg (56px), Xl (84px)
- `radius`: `AvatarRadius` - Xs, Sm, Md, Lg, Xl, or Full (default: Full)
- `color`: `String` - Background color for initials (theme color name, default: "blue")
- `initials`: `String` - Fallback text when no image (usually 1-2 characters)
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**AvatarGroup Props:**
- `spacing`: `String` - Gap between avatars (default: overlapping with negative margin)
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

### Badge

A small labeled status indicator component.

```rust
use mingot::{Badge, BadgeVariant, BadgeSize};

// Basic badge
view! {
    <Badge>"New"</Badge>
}

// Different variants
view! {
    <Group>
        <Badge variant=BadgeVariant::Filled>"Filled"</Badge>
        <Badge variant=BadgeVariant::Light>"Light"</Badge>
        <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
        <Badge variant=BadgeVariant::Dot>"Dot"</Badge>
    </Group>
}

// Different colors
view! {
    <Group>
        <Badge color="blue">"Blue"</Badge>
        <Badge color="red">"Red"</Badge>
        <Badge color="green">"Green"</Badge>
        <Badge color="yellow">"Yellow"</Badge>
    </Group>
}

// Different sizes
view! {
    <Group>
        <Badge size=BadgeSize::Xs>"Extra Small"</Badge>
        <Badge size=BadgeSize::Sm>"Small"</Badge>
        <Badge size=BadgeSize::Md>"Medium"</Badge>
        <Badge size=BadgeSize::Lg>"Large"</Badge>
        <Badge size=BadgeSize::Xl>"Extra Large"</Badge>
    </Group>
}

// With left/right sections
view! {
    <Badge
        left_section={|| view! { <span>"✓"</span> }}
    >
        "Verified"
    </Badge>
}

// Full width
view! {
    <Badge full_width=true>"Full Width Badge"</Badge>
}

// Dot variant for status indicators
view! {
    <Stack spacing="xs">
        <Badge variant=BadgeVariant::Dot color="green">"Active"</Badge>
        <Badge variant=BadgeVariant::Dot color="red">"Offline"</Badge>
        <Badge variant=BadgeVariant::Dot color="yellow">"Away"</Badge>
    </Stack>
}
```

**Badge Props:**
- `variant`: `BadgeVariant` - Filled (default), Light, Outline, or Dot
- `size`: `BadgeSize` - Xs, Sm, Md (default), Lg, or Xl
- `color`: `String` - Theme color name (default: "blue")
- `radius`: `String` - Border radius (default: "9999px" for pill shape)
- `full_width`: `bool` - Expand to full width (default: false)
- `left_section`: `Children` - Content to display on the left
- `right_section`: `Children` - Content to display on the right
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**Variants:**
- **Filled**: Solid background with white text
- **Light**: Light background with colored text
- **Outline**: Transparent with colored border
- **Dot**: Small colored dot with text (no uppercase transform)

### Card

A container component for content with optional sections and borders.

```rust
use mingot::{Card, CardSection, CardPadding};

// Basic card
view! {
    <Card>
        <Text weight=TextWeight::Bold>"Card Title"</Text>
        <Text size=TextSize::Sm>
            "This is some card content that goes inside the card."
        </Text>
    </Card>
}

// Card with sections
view! {
    <Card padding=CardPadding::Lg>
        <CardSection>
            <img src="/image.jpg" alt="Card image" style="width: 100%; height: 200px; object-fit: cover;" />
        </CardSection>

        <Stack spacing="sm">
            <Text weight=TextWeight::Bold size=TextSize::Lg>
                "Card with Image"
            </Text>
            <Text size=TextSize::Sm>
                "This card has an image section at the top."
            </Text>
        </Stack>

        <CardSection with_border=true>
            <Group justify=GroupJustify::End>
                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                <Button>"Save"</Button>
            </Group>
        </CardSection>
    </Card>
}

// Card without border
view! {
    <Card with_border=false shadow="lg">
        "Card with shadow instead of border"
    </Card>
}

// Different padding sizes
view! {
    <Grid>
        <GridCol span=6>
            <Card padding=CardPadding::Xs>"Extra Small Padding"</Card>
        </GridCol>
        <GridCol span=6>
            <Card padding=CardPadding::Xl>"Extra Large Padding"</Card>
        </GridCol>
    </Grid>
}

// User profile card example
view! {
    <Card>
        <Stack spacing="md" align=StackAlign::Center>
            <Avatar src="/user.jpg" size=AvatarSize::Xl />
            <Stack spacing="xs" align=StackAlign::Center>
                <Text weight=TextWeight::Bold size=TextSize::Lg>
                    "John Doe"
                </Text>
                <Badge variant=BadgeVariant::Light color="blue">
                    "Admin"
                </Badge>
            </Stack>
            <Text size=TextSize::Sm align="center">
                "Full-stack developer and open source contributor"
            </Text>
        </Stack>

        <CardSection with_border=true>
            <Group justify=GroupJustify::SpaceAround>
                <Stack spacing="xs" align=StackAlign::Center>
                    <Text weight=TextWeight::Bold>"1.2k"</Text>
                    <Text size=TextSize::Sm>"Followers"</Text>
                </Stack>
                <Stack spacing="xs" align=StackAlign::Center>
                    <Text weight=TextWeight::Bold>"847"</Text>
                    <Text size=TextSize::Sm>"Following"</Text>
                </Stack>
            </Group>
        </CardSection>
    </Card>
}
```

**Card Props:**
- `padding`: `CardPadding` - Xs, Sm, Md (default), Lg, or Xl
- `radius`: `String` - Border radius (default: theme.radius.md)
- `with_border`: `bool` - Add border (default: true)
- `shadow`: `String` - Box shadow (default: theme.shadows.sm)
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**CardSection Props:**
- `with_border`: `bool` - Add top and bottom borders (default: false)
- `inherit_padding`: `bool` - Inherit padding from Card (default: false, removes padding)
- `padding`: `String` - Custom padding
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**Usage Tips:**
- Use `CardSection` without `inherit_padding` for full-width content like images
- Use `with_border=true` on sections to visually separate content
- Combine Card with Grid/Stack for layouts

### Divider

A horizontal or vertical line separator with optional labels.

```rust
use mingot::{Divider, DividerOrientation, DividerLabelPosition, DividerVariant};

// Basic horizontal divider
view! {
    <Stack>
        <Text>"Content above"</Text>
        <Divider />
        <Text>"Content below"</Text>
    </Stack>
}

// Divider with label
view! {
    <Stack>
        <Text>"Section 1"</Text>
        <Divider label="Or" />
        <Text>"Section 2"</Text>
    </Stack>
}

// Label positions
view! {
    <Stack>
        <Divider label="Left" label_position=DividerLabelPosition::Left />
        <Divider label="Center" label_position=DividerLabelPosition::Center />
        <Divider label="Right" label_position=DividerLabelPosition::Right />
    </Stack>
}

// Different variants
view! {
    <Stack>
        <Divider variant=DividerVariant::Solid label="Solid" />
        <Divider variant=DividerVariant::Dashed label="Dashed" />
        <Divider variant=DividerVariant::Dotted label="Dotted" />
    </Stack>
}

// Vertical divider
view! {
    <Group style="height: 100px;">
        <Text>"Left"</Text>
        <Divider orientation=DividerOrientation::Vertical />
        <Text>"Right"</Text>
    </Group>
}

// Custom color and size
view! {
    <Divider
        color="#ff6b6b"
        size="2px"
        label="Custom Divider"
    />
}

// Login form separator example
view! {
    <Stack spacing="md">
        <Input label="Email" />
        <Input label="Password" input_type="password" />
        <Button full_width=true>"Sign In"</Button>

        <Divider label="Or continue with" />

        <Group>
            <Button variant=ButtonVariant::Outline full_width=true>
                "Google"
            </Button>
            <Button variant=ButtonVariant::Outline full_width=true>
                "GitHub"
            </Button>
        </Group>
    </Stack>
}
```

**Divider Props:**
- `orientation`: `DividerOrientation` - Horizontal (default) or Vertical
- `label`: `String` - Optional label text (only for horizontal)
- `label_position`: `DividerLabelPosition` - Left, Center (default), or Right
- `size`: `String` - Border thickness (default: "1px")
- `color`: `String` - Border color (default: theme border color)
- `variant`: `DividerVariant` - Solid (default), Dashed, or Dotted
- `class`: `String` - Additional CSS class
- `style`: `String` - Additional inline styles

**Usage Tips:**
- Use horizontal dividers to separate content sections
- Use vertical dividers in Group or horizontal layouts
- Add labels for "or" separators in forms
- Vertical dividers require a parent with defined height

### Input

A text input component with multiple variants and validation support.

```rust
use mingot::{Input, InputVariant, InputSize};

// Basic input
let value = RwSignal::new(String::new());
view! {
    <Input
        placeholder="Enter your name"
        value=Some(value)
    />
}

// With label and description
view! {
    <Input
        label="Email"
        placeholder="you@example.com"
        description="We'll never share your email"
        input_type="email"
        required=true
    />
}

// With error state
view! {
    <Input
        label="Username"
        error="Username is already taken"
    />
}

// Different variants
view! {
    <Stack>
        <Input variant=InputVariant::Default placeholder="Default" />
        <Input variant=InputVariant::Filled placeholder="Filled" />
        <Input variant=InputVariant::Unstyled placeholder="Unstyled" />
    </Stack>
}

// Different sizes
view! {
    <Input size=InputSize::Xs placeholder="Extra small" />
}

// With input handler
view! {
    <Input
        placeholder="Type something..."
        on_input=Some(Callback::new(move |val: String| {
            logging::log!("Input: {}", val);
        }))
    />
}
```

**Props:**
- `variant`: `InputVariant` - Default, Filled, or Unstyled
- `size`: `InputSize` - Xs, Sm, Md (default), Lg, or Xl
- `placeholder`: `String` - Placeholder text
- `value`: `RwSignal<String>` - Controlled value
- `disabled`: `bool` - Disabled state
- `error`: `String` - Error message (also styles input red)
- `required`: `bool` - Mark as required
- `input_type`: `String` - HTML input type (text, email, password, etc.)
- `on_input`: `Callback<String>` - Called on every keystroke
- `on_change`: `Callback<String>` - Called when input loses focus
- `label`: `String` - Label text
- `description`: `String` - Helper text below input

### Textarea

Multi-line text input component.

```rust
use mingot::{Textarea, TextareaVariant, TextareaSize};

let value = RwSignal::new(String::new());
view! {
    <Textarea
        label="Bio"
        placeholder="Tell us about yourself..."
        value=Some(value)
        rows=5
    />
}

// With validation
view! {
    <Textarea
        label="Message"
        error="Message must be at least 10 characters"
        required=true
    />
}
```

**Props:**
- Similar to Input, plus:
- `rows`: `u32` - Number of visible rows (default: 3)
- `auto_size`: `bool` - Auto-grow with content (disables resize)

### Select

Dropdown select component.

```rust
use mingot::{Select, SelectOption, SelectVariant, SelectSize};

let value = RwSignal::new(String::new());
view! {
    <Select
        label="Country"
        placeholder="Select a country"
        value=Some(value)
        options=vec![
            SelectOption::new("us", "United States"),
            SelectOption::new("ca", "Canada"),
            SelectOption::new("mx", "Mexico"),
        ]
    />
}

// With disabled options
view! {
    <Select
        options=vec![
            SelectOption::new("opt1", "Option 1"),
            SelectOption::new("opt2", "Option 2").disabled(true),
            SelectOption::new("opt3", "Option 3"),
        ]
    />
}

// With change handler
view! {
    <Select
        options=vec![/* ... */]
        on_change=Some(Callback::new(move |val: String| {
            logging::log!("Selected: {}", val);
        }))
    />
}
```

**Props:**
- Similar to Input, plus:
- `options`: `Vec<SelectOption>` - List of options
- `on_change`: `Callback<String>` - Called when selection changes

### Checkbox

Checkbox input with label support.

```rust
use mingot::{Checkbox, CheckboxSize};

let checked = RwSignal::new(false);
view! {
    <Checkbox
        label="I agree to terms and conditions"
        checked=Some(checked)
    />
}

// With description
view! {
    <Checkbox
        label="Subscribe to newsletter"
        description="Get weekly updates about new features"
        checked=Some(checked)
    />
}

// Different sizes and colors
view! {
    <Stack>
        <Checkbox size=CheckboxSize::Sm label="Small" />
        <Checkbox size=CheckboxSize::Md label="Medium" />
        <Checkbox size=CheckboxSize::Lg label="Large" />
        <Checkbox color="green" label="Green checkbox" />
    </Stack>
}

// With change handler
view! {
    <Checkbox
        label="Enable notifications"
        on_change=Some(Callback::new(move |checked: bool| {
            logging::log!("Checked: {}", checked);
        }))
    />
}
```

**Props:**
- `checked`: `RwSignal<bool>` - Controlled checked state
- `size`: `CheckboxSize` - Xs, Sm, Md (default), Lg, or Xl
- `color`: `String` - Theme color (default: blue)
- `label`: `String` - Label text
- `description`: `String` - Helper text
- `disabled`: `bool` - Disabled state
- `error`: `String` - Error message
- `on_change`: `Callback<bool>` - Called when checkbox is toggled

### Radio

Radio button input with label support.

```rust
use mingot::{Radio, RadioGroup, RadioSize};

let selected = RwSignal::new("option1".to_string());

view! {
    <RadioGroup label="Choose an option">
        <Radio
            value="option1"
            label="Option 1"
            name="my-radio"
        />
        <Radio
            value="option2"
            label="Option 2"
            name="my-radio"
        />
        <Radio
            value="option3"
            label="Option 3"
            description="This is the recommended option"
            name="my-radio"
        />
    </RadioGroup>
}

// With change handler
view! {
    <Radio
        value="yes"
        label="Yes, I agree"
        on_change=Some(Callback::new(move |val: String| {
            logging::log!("Selected: {}", val);
        }))
    />
}
```

**Props:**
- `value`: `String` - The value of this radio button
- `checked`: `RwSignal<bool>` - Controlled checked state
- `name`: `String` - Radio group name (for native grouping)
- `size`: `RadioSize` - Xs, Sm, Md (default), Lg, or Xl
- `color`: `String` - Theme color (default: blue)
- `label`: `String` - Label text
- `description`: `String` - Helper text
- `disabled`: `bool` - Disabled state
- `error`: `String` - Error message
- `on_change`: `Callback<String>` - Called when radio is selected

### Modal

Overlay modal dialog component.

```rust
use mingot::{Modal, ModalSize, Button};

let opened = RwSignal::new(false);

view! {
    <>
        <Button on_click=move |_| opened.set(true)>
            "Open Modal"
        </Button>

        <Modal
            opened=opened.into()
            on_close=Some(Callback::new(move |_| opened.set(false)))
            title="Modal Title"
        >
            <Stack spacing="md">
                <Text>"This is the modal content"</Text>
                <Group justify=GroupJustify::End>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=move |_| opened.set(false)
                    >
                        "Cancel"
                    </Button>
                    <Button on_click=move |_| {
                        // Handle save
                        opened.set(false);
                    }>
                        "Save"
                    </Button>
                </Group>
            </Stack>
        </Modal>
    </>
}

// Centered modal
view! {
    <Modal
        opened=opened.into()
        centered=true
        size=ModalSize::Lg
    >
        "Large centered modal"
    </Modal>
}

// Without close button
view! {
    <Modal
        opened=opened.into()
        with_close_button=false
        close_on_click_outside=false
    >
        "Must use a button to close"
    </Modal>
}
```

**Props:**
- `opened`: `Signal<bool>` - Controls modal visibility
- `on_close`: `Callback<()>` - Called when modal should close
- `size`: `ModalSize` - Xs, Sm, Md (default), Lg, Xl, or Full
- `title`: `String` - Modal title
- `centered`: `bool` - Center modal vertically (default: false)
- `close_on_click_outside`: `bool` - Close when clicking overlay (default: true)
- `close_on_escape`: `bool` - Close on Escape key (default: true)
- `with_close_button`: `bool` - Show X button (default: true)
- `padding`: `String` - Custom padding (default: theme.spacing.lg)

### Table

A comprehensive data table component with sortable columns, pagination, and customizable styling.

```rust
use mingot::{Table, TableColumn, SortDirection};
use leptos::prelude::*;

#[derive(Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: String,
}

#[component]
fn UserTable() -> impl IntoView {
    let users = Signal::derive(move || vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string(), role: "Admin".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string(), role: "User".to_string() },
        User { id: 3, name: "Charlie".to_string(), email: "charlie@example.com".to_string(), role: "User".to_string() },
    ]);

    let columns = vec![
        TableColumn::new("id", "ID", |user: &User| view! { {user.id.to_string()} })
            .sortable(true)
            .width("80px"),
        TableColumn::new("name", "Name", |user: &User| view! { {user.name.clone()} })
            .sortable(true),
        TableColumn::new("email", "Email", |user: &User| view! { {user.email.clone()} })
            .sortable(true),
        TableColumn::new("role", "Role", |user: &User| view! { {user.role.clone()} }),
    ];

    view! {
        <Table
            columns=columns
            data=users
            striped=true
            highlight_on_hover=true
            with_border=true
        />
    }
}
```

**Sortable Table:**

```rust
let sort_column = RwSignal::new(None);
let sort_direction = RwSignal::new(SortDirection::None);

view! {
    <Table
        columns=columns
        data=users
        sort_column=sort_column
        sort_direction=sort_direction
        on_sort=Callback::new(move |(col, dir): (String, SortDirection)| {
            // Handle sorting logic here
            logging::log!("Sort by {} in {:?} order", col, dir);
        })
    />
}
```

**Table Props:**
- `columns`: `Vec<TableColumn<T>>` - Column definitions
- `data`: `Signal<Vec<T>>` - Table data
- `striped`: `bool` - Alternate row colors (default: false)
- `highlight_on_hover`: `bool` - Highlight rows on hover (default: true)
- `with_border`: `bool` - Add border around table (default: false)
- `with_column_borders`: `bool` - Add borders between columns (default: false)
- `sort_column`: `RwSignal<Option<String>>` - Currently sorted column
- `sort_direction`: `RwSignal<SortDirection>` - Current sort direction
- `on_sort`: `Callback<(String, SortDirection)>` - Called when column header is clicked
- `empty_message`: `String` - Message to show when table is empty (default: "No data available")

**TableColumn Methods:**
- `new(key, header, render)` - Create a new column
  - `key`: Unique identifier for the column
  - `header`: Display name
  - `render`: Function to render cell content from data item
- `.sortable(bool)` - Make column sortable (default: false)
- `.width(String)` - Set column width (e.g., "100px", "20%")

### Pagination

A pagination control component for navigating through pages.

```rust
use mingot::{Pagination, Button};

let current_page = RwSignal::new(1);
let total_pages = Signal::derive(move || 10);

view! {
    <Pagination
        current_page=Signal::from(current_page)
        total_pages=total_pages
        on_page_change=Callback::new(move |page: usize| {
            current_page.set(page);
        })
    />
}
```

**Props:**
- `current_page`: `Signal<usize>` - Current page number (1-indexed)
- `total_pages`: `Signal<usize>` - Total number of pages
- `on_page_change`: `Callback<usize>` - Called when page changes
- `show_edges`: `bool` - Show first and last page (default: true)
- `siblings`: `usize` - Number of pages to show on each side of current (default: 1)

### TableWithPagination

A helper component that combines Table with Pagination for easy data pagination.

```rust
use mingot::{TableWithPagination, TableColumn};

#[component]
fn PaginatedUserTable() -> impl IntoView {
    let users = Signal::derive(move || vec![
        // ... lots of users
    ]);

    let columns = vec![
        TableColumn::new("id", "ID", |user: &User| view! { {user.id.to_string()} }),
        TableColumn::new("name", "Name", |user: &User| view! { {user.name.clone()} }),
        // ... more columns
    ];

    let current_page = RwSignal::new(1);
    let page_size = Signal::derive(move || 10);

    view! {
        <TableWithPagination
            columns=columns
            data=users
            current_page=current_page
            page_size=page_size
            striped=true
            highlight_on_hover=true
            with_border=true
        />
    }
}
```

**Props:**
- All props from `Table` component, plus:
- `current_page`: `RwSignal<usize>` - Current page (1-indexed)
- `page_size`: `Signal<usize>` - Number of items per page

**Features:**
- Automatically paginates data
- Shows pagination controls only when there's data
- Calculates total pages based on data length and page size
- Handles empty states gracefully

**Custom Empty State:**

```rust
view! {
    <Table
        columns=columns
        data=empty_data
        empty_message="No users found. Try adjusting your filters."
    />
}
```

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

- [x] Basic form components (Input, Textarea, Select, Checkbox, Radio)
- [x] Modal/Dialog component
- [x] Dark mode support
- [x] Table component with sortable columns and pagination
- [x] Responsive grid layout system (Grid, GridCol, SimpleGrid)
- [x] Navigation components (Header, Footer, Navbar)
- [x] Banner and Hero components
- [x] User info components (Avatar, Badge, Card, Divider)
- [ ] Additional overlay components (Drawer, Popover, Tooltip)
- [ ] System dark mode detection (prefers-color-scheme)
- [ ] More form components (Switch, Slider, File Input, Date Picker)
- [ ] More navigation components (Tabs, Menu, Breadcrumbs)
- [ ] Feedback components (Alert, Notification, Progress)
- [ ] CSS-in-Rust styling with style generation
- [ ] More comprehensive theming options
- [ ] Accessibility improvements (ARIA labels, keyboard navigation)
- [ ] Form validation utilities
- [ ] Smooth transitions between themes
- [ ] LocalStorage persistence for theme preference
- [ ] Component testing utilities

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

Inspired by [Mantine UI](https://mantine.dev/) - A fully featured React components library.
