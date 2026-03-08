use leptos::prelude::*;
use mingot::prelude::*;
use mingot::theme::use_theme;

use crate::components::{CodeBlock, DemoBlock};

#[component]
pub fn ThemingOverviewPage() -> impl IntoView {
    view! {
        <div>
            <h1 class="page-title">"Theming Overview"</h1>
            <p class="page-description">
                "Mingot's theme system provides automatic CSS variable injection, "
                "light/dark/auto color schemes, scoped overrides, and a complete set of "
                <code>"--mingot-*"</code> " design tokens."
            </p>

            // MingotProvider section
            <h2 class="section-title">"MingotProvider"</h2>
            <p>
                "Wrap your app with " <code>"MingotProvider"</code>
                " to enable theming. It automatically injects "
                <code>"--mingot-*"</code> " CSS variables onto the document root, "
                "so all descendant elements can use them."
            </p>
            <CodeBlock
                code=r#"use mingot::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <MingotProvider>
            // All --mingot-* CSS vars are now available
            <div style="color: var(--mingot-primary)">
                "Themed content"
            </div>
        </MingotProvider>
    }
}"#
                language="rust"
            />
            <p>
                "To disable CSS variable injection (e.g. if you manage vars yourself), "
                "pass " <code>"inject_css_vars=false"</code> ":"
            </p>
            <CodeBlock
                code=r#"<MingotProvider inject_css_vars=false>
    // No --mingot-* vars injected
</MingotProvider>"#
                language="rust"
            />

            // Color Scheme Modes
            <h2 class="section-title">"Color Scheme Modes"</h2>
            <p>
                "Mingot supports three color scheme modes: "
                <code>"Light"</code> ", " <code>"Dark"</code> ", and " <code>"Auto"</code>
                " (follows system preference). Use the hooks below to read and change the scheme."
            </p>
            <ColorSchemeDemo />
            <CodeBlock
                code=r#"use mingot::prelude::*;

#[component]
fn ThemeControls() -> impl IntoView {
    let scheme = use_color_scheme();
    let set_scheme = use_set_color_scheme();
    let toggle = use_color_scheme_toggle();

    view! {
        // Read current scheme
        <Text>{move || format!("Current: {:?}", scheme())}</Text>

        // Set a specific scheme
        <Button on_click=Callback::new(move |_| set_scheme(ColorSchemeMode::Dark))>
            "Dark Mode"
        </Button>

        // Toggle between light and dark
        <Button on_click=Callback::new(move |_| toggle())>
            "Toggle"
        </Button>
    }
}"#
                language="rust"
            />

            // ThemeOverride section
            <h2 class="section-title">"ThemeOverride"</h2>
            <p>
                "Use " <code>"ThemeOverride"</code>
                " to create scoped theme regions. Any prop you pass overrides "
                "that aspect of the theme for all children. Unset props inherit from the parent."
            </p>
            <ThemeOverrideDemo />
            <CodeBlock
                code=r#"use mingot::prelude::*;

view! {
    // A dark island inside a light page
    <ThemeOverride color_scheme=ColorSchemeMode::Dark>
        <Card>
            <Text>"This card uses dark mode colors"</Text>
            <Button variant=ButtonVariant::Filled>"Dark Button"</Button>
        </Card>
    </ThemeOverride>

    // Override just the primary color
    <ThemeOverride primary_color="red".to_string()>
        <Button variant=ButtonVariant::Filled>"Red Primary"</Button>
    </ThemeOverride>
}"#
                language="rust"
            />

            // CSS Variables Reference
            <h2 class="section-title">"CSS Variables Reference"</h2>
            <p>
                "The following " <code>"--mingot-*"</code>
                " variables are injected by " <code>"MingotProvider"</code> ":"
            </p>
            <CssVarsTable />
        </div>
    }
}

#[component]
fn ColorSchemeDemo() -> impl IntoView {
    let scheme = use_color_scheme();
    let theme = use_theme();

    let set_light = move |_| {
        theme.update(|t| t.color_scheme = ColorSchemeMode::Light);
    };
    let set_dark = move |_| {
        theme.update(|t| t.color_scheme = ColorSchemeMode::Dark);
    };
    let set_auto = move |_| {
        theme.update(|t| t.color_scheme = ColorSchemeMode::Auto);
    };

    view! {
        <DemoBlock title="Color Scheme Switcher">
            <Group>
                <Button
                    variant=ButtonVariant::Outline
                    on_click=Callback::new(set_light)
                >
                    "Light"
                </Button>
                <Button
                    variant=ButtonVariant::Outline
                    on_click=Callback::new(set_dark)
                >
                    "Dark"
                </Button>
                <Button
                    variant=ButtonVariant::Outline
                    on_click=Callback::new(set_auto)
                >
                    "Auto"
                </Button>
            </Group>
            <Text size=TextSize::Sm>
                {move || format!("Current scheme: {:?}", scheme())}
            </Text>
        </DemoBlock>
    }
}

#[component]
fn ThemeOverrideDemo() -> impl IntoView {
    let theme = use_theme();

    // Determine the opposite scheme for the island demo
    let opposite_scheme = move || {
        let current = theme.get().color_scheme;
        if current == ColorSchemeMode::Dark {
            ColorSchemeMode::Light
        } else {
            ColorSchemeMode::Dark
        }
    };

    view! {
        <DemoBlock title="Scoped Theme Override">
            <Stack spacing="md">
                <Text size=TextSize::Sm>"Current page theme:"</Text>
                <Group>
                    <Button variant=ButtonVariant::Filled>"Normal Button"</Button>
                    <Badge>"Normal Badge"</Badge>
                </Group>

                <Divider />

                <Text size=TextSize::Sm>"Opposite-scheme island (ThemeOverride):"</Text>
                <ThemeOverride color_scheme=opposite_scheme()>
                    <div style="padding: 1rem; border-radius: 0.5rem; background: var(--mingot-surface-0); border: 1px solid var(--mingot-border);">
                        <Stack spacing="sm">
                            <Group>
                                <Button variant=ButtonVariant::Filled>"Overridden Button"</Button>
                                <Badge>"Overridden Badge"</Badge>
                            </Group>
                            <Text size=TextSize::Sm>"This region uses the opposite color scheme."</Text>
                        </Stack>
                    </div>
                </ThemeOverride>

                <Divider />

                <Text size=TextSize::Sm>"Red primary override:"</Text>
                <ThemeOverride primary_color="red".to_string()>
                    <Group>
                        <Button variant=ButtonVariant::Filled>"Red Primary"</Button>
                        <Badge>"Red Badge"</Badge>
                    </Group>
                </ThemeOverride>
            </Stack>
        </DemoBlock>
    }
}

#[component]
fn CssVarsTable() -> impl IntoView {
    let vars = vec![
        ("--mingot-background", "Page background color"),
        ("--mingot-text", "Primary text color"),
        ("--mingot-border", "Border color"),
        ("--mingot-white", "White (#ffffff)"),
        ("--mingot-black", "Black (#000000)"),
        ("--mingot-primary", "Primary brand color"),
        ("--mingot-primary-light", "Light primary tint"),
        ("--mingot-success", "Success/green color"),
        ("--mingot-error", "Error/red color"),
        ("--mingot-warning", "Warning/yellow color"),
        ("--mingot-surface-0", "Base surface"),
        ("--mingot-surface-1", "Elevated surface"),
        ("--mingot-surface-2", "Highest surface"),
        ("--mingot-hover-bg", "Hover background"),
        ("--mingot-text-dimmed", "Secondary/dimmed text"),
        ("--mingot-spacing-xs", "Extra small spacing (0.625rem)"),
        ("--mingot-spacing-sm", "Small spacing (0.75rem)"),
        ("--mingot-spacing-md", "Medium spacing (1rem)"),
        ("--mingot-spacing-lg", "Large spacing (1.25rem)"),
        ("--mingot-spacing-xl", "Extra large spacing (2rem)"),
        ("--mingot-radius-xs", "Extra small radius"),
        ("--mingot-radius-sm", "Small radius"),
        ("--mingot-radius-md", "Medium radius"),
        ("--mingot-radius-lg", "Large radius"),
        ("--mingot-radius-xl", "Extra large radius"),
        ("--mingot-font-family", "Primary font family"),
        ("--mingot-font-family-mono", "Monospace font family"),
        ("--mingot-font-size-xs", "Extra small font (0.75rem)"),
        ("--mingot-font-size-sm", "Small font (0.875rem)"),
        ("--mingot-font-size-md", "Medium font (1rem)"),
        ("--mingot-font-size-lg", "Large font (1.125rem)"),
        ("--mingot-font-size-xl", "Extra large font (1.25rem)"),
        ("--mingot-font-size-xxl", "Display font (2rem)"),
    ];

    view! {
        <table class="props-table">
            <thead>
                <tr>
                    <th>"Variable"</th>
                    <th>"Description"</th>
                </tr>
            </thead>
            <tbody>
                {vars.into_iter().map(|(name, desc)| view! {
                    <tr>
                        <td><code>{name}</code></td>
                        <td>{desc}</td>
                    </tr>
                }).collect_view()}
            </tbody>
        </table>
    }
}
