use leptos::prelude::*;
use mingot::prelude::*;
use mingot::theme::tokens::DesignTokens;
use mingot::theme::validation::{contrast_ratio, meets_wcag_aa, validate_theme};
use mingot::theme::{presets, use_theme};

use crate::components::{CodeBlock, DemoBlock};

#[component]
pub fn ThemingCustomPage() -> impl IntoView {
    view! {
        <div>
            <h1 class="page-title">"Custom Themes"</h1>
            <p class="page-description">
                "Build your own themes with the fluent ThemeBuilder API, validate contrast ratios, "
                "and export/import themes as design tokens."
            </p>

            // ThemeBuilder
            <h2 class="section-title">"ThemeBuilder"</h2>
            <p>
                "Use " <code>"ThemeBuilder"</code>
                " to create themes with a fluent API. Start from scratch or from an existing preset."
            </p>
            <CodeBlock
                code=r#"use mingot::prelude::*;

let theme = ThemeBuilder::new()
    .primary_color("indigo")
    .color_scheme(ColorSchemeMode::Light)
    .font_family("Georgia, serif")
    .spacing_md("1.25rem")
    .radius_md("0.75rem")
    .build();"#
                language="rust"
            />
            <p>"Or start from an existing preset and customize:"</p>
            <CodeBlock
                code=r#"use mingot::theme::presets;

let theme = ThemeBuilder::from(presets::industrial())
    .primary_color("blue")
    .color_scheme(ColorSchemeMode::Dark)
    .build();"#
                language="rust"
            />
            <ThemeBuilderDemo />

            // Theme Validation
            <h2 class="section-title">"Theme Validation"</h2>
            <p>
                <code>"validate_theme()"</code>
                " checks a theme for common issues: missing primary color, insufficient "
                "WCAG contrast, incomplete shade palettes, and invalid CSS values."
            </p>
            <CodeBlock
                code=r#"use mingot::theme::validation::validate_theme;

let warnings = validate_theme(&theme);
if warnings.is_empty() {
    println!("Theme is valid!");
} else {
    for w in &warnings {
        println!("Warning: {:?}", w);
    }
}"#
                language="rust"
            />
            <ValidationDemo />

            // WCAG Contrast Checker
            <h2 class="section-title">"WCAG Contrast Checker"</h2>
            <p>
                "Use " <code>"contrast_ratio()"</code> " and " <code>"meets_wcag_aa()"</code>
                " to verify that your color combinations meet accessibility standards."
            </p>
            <CodeBlock
                code=r##"use mingot::theme::validation::{contrast_ratio, meets_wcag_aa};

let ratio = contrast_ratio("#ffffff", "#228be6");
let passes = meets_wcag_aa("#ffffff", "#228be6");
// ratio ≈ 3.58, passes = false (needs ≥ 4.5 for normal text)"##
                language="rust"
            />
            <ContrastCheckerDemo />

            // Design Tokens
            <h2 class="section-title">"Design Tokens"</h2>
            <p>
                "Export any theme as JSON design tokens with " <code>"DesignTokens::from_theme()"</code>
                ", and import back with " <code>"DesignTokens::from_json()"</code>
                ". Requires the " <code>"theme-tokens"</code> " feature."
            </p>
            <CodeBlock
                code=r#"# Cargo.toml
[dependencies]
mingot = { version = "0.7.0", features = ["theme-tokens"] }"#
                language="toml"
            />
            <CodeBlock
                code=r#"use mingot::theme::tokens::DesignTokens;
use mingot::theme::presets;

// Export
let tokens = DesignTokens::from_theme(&presets::scientific());
let json = tokens.to_json().unwrap();

// Import
let tokens = DesignTokens::from_json(&json).unwrap();
let theme = tokens.to_theme();"#
                language="rust"
            />
            <DesignTokensDemo />
        </div>
    }
}

#[component]
fn ThemeBuilderDemo() -> impl IntoView {
    let theme_ctx = use_theme();
    let primary_color = RwSignal::new(String::from("blue"));
    let font_choice = RwSignal::new(String::from("system"));

    let apply_theme = move |_| {
        let primary = primary_color.get();
        let font = match font_choice.get().as_str() {
            "serif" => "Georgia, 'Times New Roman', serif",
            "mono" => "ui-monospace, SFMono-Regular, 'SF Mono', Menlo, monospace",
            _ => "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
        };

        let new_theme = ThemeBuilder::new()
            .primary_color(primary)
            .font_family(font)
            .build();
        theme_ctx.set(new_theme);
    };

    view! {
        <DemoBlock title="Interactive ThemeBuilder">
            <Stack spacing="md">
                <Group>
                    <div>
                        <Text size=TextSize::Sm weight=TextWeight::Medium>"Primary Color"</Text>
                        <select
                            style="padding: 0.5rem; border: 1px solid var(--mingot-border); border-radius: 0.25rem; background: var(--mingot-surface-0); color: var(--mingot-text);"
                            on:change=move |ev| {
                                primary_color.set(event_target_value(&ev));
                            }
                        >
                            <option value="blue" selected=true>"Blue"</option>
                            <option value="red">"Red"</option>
                            <option value="green">"Green"</option>
                            <option value="indigo">"Indigo"</option>
                            <option value="yellow">"Yellow"</option>
                            <option value="gray">"Gray"</option>
                        </select>
                    </div>
                    <div>
                        <Text size=TextSize::Sm weight=TextWeight::Medium>"Font Family"</Text>
                        <select
                            style="padding: 0.5rem; border: 1px solid var(--mingot-border); border-radius: 0.25rem; background: var(--mingot-surface-0); color: var(--mingot-text);"
                            on:change=move |ev| {
                                font_choice.set(event_target_value(&ev));
                            }
                        >
                            <option value="system" selected=true>"System (sans-serif)"</option>
                            <option value="serif">"Georgia (serif)"</option>
                            <option value="mono">"Monospace"</option>
                        </select>
                    </div>
                </Group>
                <Button
                    variant=ButtonVariant::Filled
                    on_click=Callback::new(apply_theme)
                >
                    "Apply Theme"
                </Button>
                <Button
                    variant=ButtonVariant::Subtle
                    on_click=Callback::new(move |_| theme_ctx.set(Theme::default()))
                >
                    "Reset to Default"
                </Button>
                // Preview showing how the primary color affects components
                <Text size=TextSize::Sm weight=TextWeight::Medium>"Preview:"</Text>
                <div style="padding: 1rem; border: 1px solid var(--mingot-border); border-radius: 0.5rem;">
                    <Group>
                        {move || {
                            let color = theme_ctx.get().colors.primary_color;
                            view! {
                                <Button variant=ButtonVariant::Filled color=color.clone()>"Filled"</Button>
                                <Button variant=ButtonVariant::Outline color=color.clone()>"Outline"</Button>
                                <Button variant=ButtonVariant::Light color=color.clone()>"Light"</Button>
                                <Badge color=color.clone()>"Badge"</Badge>
                                <Badge variant=BadgeVariant::Outline color=color>"Outline"</Badge>
                            }
                        }}
                    </Group>
                </div>
            </Stack>
        </DemoBlock>
    }
}

#[component]
fn ValidationDemo() -> impl IntoView {
    let warnings = RwSignal::new(Vec::<String>::new());

    let validate_bad_theme = move |_| {
        // Build a theme with low contrast (white text on near-white background)
        let bad_theme = ThemeBuilder::new()
            .background("#ffffff".to_string(), "#fafafa".to_string())
            .text("#f0f0f0".to_string(), "#f8f8f8".to_string())
            .build();
        let result = validate_theme(&bad_theme);
        let msgs: Vec<String> = result.iter().map(|w| format!("{:?}", w)).collect();
        warnings.set(msgs);
    };

    let validate_good_theme = move |_| {
        let good_theme = presets::mingot_default();
        let result = validate_theme(&good_theme);
        if result.is_empty() {
            warnings.set(vec!["No warnings — theme is valid!".to_string()]);
        } else {
            let msgs: Vec<String> = result.iter().map(|w| format!("{:?}", w)).collect();
            warnings.set(msgs);
        }
    };

    view! {
        <DemoBlock title="Theme Validation">
            <Stack spacing="md">
                <Group>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(validate_bad_theme)
                    >
                        "Validate Bad Theme"
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(validate_good_theme)
                    >
                        "Validate Default Theme"
                    </Button>
                </Group>
                {move || {
                    let w = warnings.get();
                    if w.is_empty() {
                        view! { <div></div> }.into_any()
                    } else {
                        view! {
                            <div>
                                {w.into_iter().map(|msg| {
                                    let is_good = msg.contains("valid");
                                    let color = if is_good { AlertColor::Success } else { AlertColor::Warning };
                                    view! {
                                        <div style="margin-bottom: 0.5rem;">
                                            <Alert color=color>
                                                {msg}
                                            </Alert>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                }}
            </Stack>
        </DemoBlock>
    }
}

#[component]
fn ContrastCheckerDemo() -> impl IntoView {
    let bg_color = RwSignal::new(String::from("#ffffff"));
    let fg_color = RwSignal::new(String::from("#228be6"));
    let ratio = RwSignal::new(0.0_f64);
    let passes_aa = RwSignal::new(false);

    // Compute on mount and on change
    let compute = move || {
        let bg = bg_color.get();
        let fg = fg_color.get();
        let r = contrast_ratio(&bg, &fg);
        ratio.set(r);
        passes_aa.set(meets_wcag_aa(&bg, &fg));
    };

    // Initial computation
    Effect::new(move || {
        let _ = bg_color.get();
        let _ = fg_color.get();
        compute();
    });

    view! {
        <DemoBlock title="WCAG Contrast Checker">
            <Stack spacing="md">
                <Group>
                    <div>
                        <Text size=TextSize::Sm weight=TextWeight::Medium>"Background"</Text>
                        <input
                            type="color"
                            prop:value=move || bg_color.get()
                            on:input=move |ev| {
                                bg_color.set(event_target_value(&ev));
                            }
                            style="width: 4rem; height: 2.5rem; border: 1px solid var(--mingot-border); border-radius: 0.25rem; cursor: pointer;"
                        />
                        <Text size=TextSize::Xs>{move || bg_color.get()}</Text>
                    </div>
                    <div>
                        <Text size=TextSize::Sm weight=TextWeight::Medium>"Foreground"</Text>
                        <input
                            type="color"
                            prop:value=move || fg_color.get()
                            on:input=move |ev| {
                                fg_color.set(event_target_value(&ev));
                            }
                            style="width: 4rem; height: 2.5rem; border: 1px solid var(--mingot-border); border-radius: 0.25rem; cursor: pointer;"
                        />
                        <Text size=TextSize::Xs>{move || fg_color.get()}</Text>
                    </div>
                </Group>
                <div
                    style=move || format!(
                        "padding: 1rem; border-radius: 0.5rem; background: {}; color: {}; border: 1px solid var(--mingot-border);",
                        bg_color.get(), fg_color.get()
                    )
                >
                    "Sample text with these colors"
                </div>
                <Group>
                    <Text size=TextSize::Sm weight=TextWeight::Bold>
                        {move || format!("Ratio: {:.2}:1", ratio.get())}
                    </Text>
                    {move || {
                        if passes_aa.get() {
                            view! { <Badge variant=BadgeVariant::Filled>"WCAG AA Pass"</Badge> }.into_any()
                        } else {
                            view! { <Badge variant=BadgeVariant::Filled color="red">"WCAG AA Fail"</Badge> }.into_any()
                        }
                    }}
                </Group>
            </Stack>
        </DemoBlock>
    }
}

#[component]
fn DesignTokensDemo() -> impl IntoView {
    let theme_ctx = use_theme();
    let json_output = RwSignal::new(String::new());

    let export_tokens = move |_| {
        let current = theme_ctx.get();
        let tokens = DesignTokens::from_theme(&current);
        match tokens.to_json() {
            Ok(json) => json_output.set(json),
            Err(e) => json_output.set(format!("Error: {e}")),
        }
    };

    view! {
        <DemoBlock title="Design Token Export">
            <Stack spacing="md">
                <Button
                    variant=ButtonVariant::Filled
                    on_click=Callback::new(export_tokens)
                >
                    "Export Current Theme as JSON"
                </Button>
                {move || {
                    let output = json_output.get();
                    if output.is_empty() {
                        view! { <div></div> }.into_any()
                    } else {
                        view! {
                            <div>
                                <CodeBlock code=output.leak() language="json" />
                            </div>
                        }.into_any()
                    }
                }}
            </Stack>
        </DemoBlock>
    }
}
