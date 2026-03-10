use leptos::prelude::*;
use mingot::prelude::*;
use mingot::theme::{get_scheme_colors, presets, use_theme};

use crate::components::{CodeBlock, DemoBlock};

#[component]
pub fn ThemingPresetsPage() -> impl IntoView {
    view! {
        <div>
            <h1 class="page-title">"Theme Presets"</h1>
            <p class="page-description">
                "Mingot ships with 5 built-in theme presets. Each is validated for "
                "WCAG AA contrast and can be used as a starting point for customization."
            </p>

            // Available Presets
            <h2 class="section-title">"Available Presets"</h2>
            <table class="props-table">
                <thead>
                    <tr>
                        <th>"Preset"</th>
                        <th>"Primary"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>"mingot_default()"</code></td>
                        <td>"Blue"</td>
                        <td>"Default light theme with blue primary"</td>
                    </tr>
                    <tr>
                        <td><code>"mingot_dark()"</code></td>
                        <td>"Blue"</td>
                        <td>"Dark mode optimized with elevated shadows"</td>
                    </tr>
                    <tr>
                        <td><code>"industrial()"</code></td>
                        <td>"Gray"</td>
                        <td>"Monospace font, tight spacing, neutral palette"</td>
                    </tr>
                    <tr>
                        <td><code>"scientific()"</code></td>
                        <td>"Indigo"</td>
                        <td>"Serif font, generous spacing, minimal shadows"</td>
                    </tr>
                    <tr>
                        <td><code>"financial()"</code></td>
                        <td>"Indigo"</td>
                        <td>"Conservative spacing, professional typography"</td>
                    </tr>
                </tbody>
            </table>

            // Live Preview
            <h2 class="section-title">"Live Preview"</h2>
            <p>"Each card below renders with a different preset's colors and typography applied."</p>
            <PresetPreviewGrid />

            // Using a Preset
            <h2 class="section-title">"Using a Preset"</h2>
            <p>"Pass a preset to " <code>"MingotProvider"</code> " to apply it globally:"</p>
            <CodeBlock
                code=r#"use mingot::prelude::*;
use mingot::theme::presets;

#[component]
fn App() -> impl IntoView {
    view! {
        <MingotProvider theme=presets::industrial()>
            // Entire app uses the Industrial preset
        </MingotProvider>
    }
}"#
                language="rust"
            />

            // Interactive Switcher
            <h2 class="section-title">"Interactive Switcher"</h2>
            <p>"Click a button to switch the entire page to that preset:"</p>
            <PresetSwitcher />
            <CodeBlock
                code=r#"use mingot::prelude::*;
use mingot::theme::{presets, use_theme};

#[component]
fn PresetPicker() -> impl IntoView {
    let theme = use_theme();

    view! {
        <Button on_click=Callback::new(move |_| {
            theme.set(presets::scientific());
        })>
            "Scientific"
        </Button>
    }
}"#
                language="rust"
            />
        </div>
    }
}

#[component]
fn PresetPreviewGrid() -> impl IntoView {
    let preset_cards: Vec<(&str, Theme)> = vec![
        ("Default", presets::mingot_default()),
        ("Dark", presets::mingot_dark()),
        ("Industrial", presets::industrial()),
        ("Scientific", presets::scientific()),
        ("Financial", presets::financial()),
    ];

    view! {
        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 1rem; margin: 1.5rem 0;">
            {preset_cards.into_iter().map(|(name, preset)| {
                // Compute colors directly from the preset's scheme
                let scheme = get_scheme_colors(&preset);
                let primary_key = preset.colors.primary_color.clone();
                let bg = scheme.get_color("gray", if preset.color_scheme == ColorSchemeMode::Dark { 1 } else { 0 }).unwrap_or_default();
                let border = scheme.border.clone();
                let text_color = scheme.text.clone();
                let font = preset.typography.font_family.to_string();
                let card_style = format!(
                    "padding: 1.25rem; border-radius: 0.5rem; background: {}; border: 1px solid {}; color: {}; font-family: {};",
                    bg, border, text_color, font
                );

                let primary_for_buttons = primary_key.clone();
                let primary_for_outline_btn = primary_key.clone();
                let primary_for_badges = primary_key.clone();
                let primary_for_outline_badge = primary_key.clone();
                let color_scheme = preset.color_scheme;

                view! {
                    <ThemeOverride color_scheme=color_scheme primary_color=primary_key>
                        <div style=card_style>
                            <Stack spacing="sm">
                                <Text size=TextSize::Lg weight=TextWeight::Bold>{name}</Text>
                                <Group>
                                    <Button variant=ButtonVariant::Filled size=ButtonSize::Xs color=primary_for_buttons>"Filled"</Button>
                                    <Button variant=ButtonVariant::Outline size=ButtonSize::Xs color=primary_for_outline_btn>"Outline"</Button>
                                </Group>
                                <Group>
                                    <Badge color=primary_for_badges>"Badge"</Badge>
                                    <Badge variant=BadgeVariant::Outline color=primary_for_outline_badge>"Outline"</Badge>
                                </Group>
                                <Text size=TextSize::Sm>"Sample body text in this preset."</Text>
                            </Stack>
                        </div>
                    </ThemeOverride>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn PresetSwitcher() -> impl IntoView {
    let theme = use_theme();

    view! {
        <DemoBlock title="Switch Page Preset">
            <Stack spacing="sm">
                <Group>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(move |_| theme.set(presets::mingot_default()))
                    >
                        "Default"
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(move |_| theme.set(presets::mingot_dark()))
                    >
                        "Dark"
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(move |_| theme.set(presets::industrial()))
                    >
                        "Industrial"
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(move |_| theme.set(presets::scientific()))
                    >
                        "Scientific"
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(move |_| theme.set(presets::financial()))
                    >
                        "Financial"
                    </Button>
                </Group>
                <Text size=TextSize::Sm>
                    {move || format!("Active: {:?} scheme, primary = \"{}\"", theme.get().color_scheme, theme.get().colors.primary_color)}
                </Text>
            </Stack>
        </DemoBlock>
    }
}
