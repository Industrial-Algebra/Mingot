use leptos::prelude::*;

use crate::components::CodeBlock;

#[component]
pub fn GettingStartedPage() -> impl IntoView {
    view! {
        <div>
            <h1 class="page-title">"Getting Started"</h1>
            <p class="page-description">
                "Learn how to install and use Mingot in your Leptos project."
            </p>

            // Installation
            <h2 class="section-title">"Installation"</h2>
            <p>"Add Mingot to your " <code>"Cargo.toml"</code> ":"</p>
            <CodeBlock
                code=r#"[dependencies]
    mingot = "0.3.0""#
                language="toml"
            />

            <p>"For high-precision decimal support with rust_decimal:"</p>
            <CodeBlock
                code=r#"[dependencies]
    mingot = { version = "0.3.0", features = ["high-precision"] }"#
                language="toml"
            />

            // Basic usage
            <h2 class="section-title">"Basic Usage"</h2>
            <p>"Wrap your app with " <code>"MingotProvider"</code> " to enable theming:"</p>
            <CodeBlock
                code=r#"use leptos::prelude::*;
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
                    <Button
                        variant=ButtonVariant::Filled
                        on_click=Callback::new(|_| {
                            web_sys::window()
                                .unwrap()
                                .alert_with_message("Hello!")
                                .unwrap();
                        })
                    >
                        "Click me"
                    </Button>
                </Stack>
            </Container>
        </MingotProvider>
    }
}"#
                language="rust"
            />

            // Theme toggle
            <h2 class="section-title">"Theme Toggle"</h2>
            <p>"Mingot supports dark and light themes. Use the theme hooks to toggle:"</p>
            <CodeBlock
                code=r#"use mingot::prelude::*;

#[component]
fn ThemeToggle() -> impl IntoView {
    let toggle = use_color_scheme_toggle();
    let scheme = use_color_scheme();

    view! {
        <Button on_click=Callback::new(move |_| toggle())>
            {move || if scheme.get() == ColorSchemeMode::Dark {
                "Switch to Light"
            } else {
                "Switch to Dark"
            }}
        </Button>
    }
}"#
                language="rust"
            />

            // High precision
            <h2 class="section-title">"High-Precision Numbers"</h2>
            <p>
                "Mingot's " <code>"NumberInput"</code> " component supports precision beyond JavaScript's limits:"
            </p>
            <CodeBlock
                code=r#"use mingot::prelude::*;

#[component]
fn PrecisionInput() -> impl IntoView {
    view! {
        // U64: up to 18,446,744,073,709,551,615
        <NumberInput
            precision=NumberInputPrecision::U64
            label="U64 Integer"
        />

        // U128: up to 340 undecillion
        <NumberInput
            precision=NumberInputPrecision::U128
            label="U128 Integer"
        />

        // Decimal with 6 decimal places
        <NumberInput
            precision=NumberInputPrecision::Decimal(6)
            label="Fixed Decimal"
        />

        // Arbitrary precision with rust_decimal (requires feature)
        #[cfg(feature = "high-precision")]
        <NumberInput
            precision=NumberInputPrecision::Arbitrary
            label="Arbitrary Precision"
        />
    }
}"#
                language="rust"
            />

            // Next steps
            <h2 class="section-title">"Next Steps"</h2>
            <ul style="line-height: 1.8;">
                <li>"Explore the " <a href="/core/button">"Button"</a> " component to learn about variants and sizes"</li>
                <li>"Check out " <a href="/form/number-input">"NumberInput"</a> " for the flagship precision component"</li>
                <li>"Browse all 46 components in the sidebar"</li>
            </ul>
        </div>
    }
}
