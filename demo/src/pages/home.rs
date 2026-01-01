use leptos::prelude::*;
use mingot::prelude::*;

use crate::components::CodeBlock;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            // Hero section
            <div style="text-align: center; margin-bottom: 3rem;">
                <h1 style="font-size: 2.5rem; font-weight: 800; margin-bottom: 1rem;">
                    "Mingot"
                </h1>
                <p style="font-size: 1.25rem; color: #868e96; max-width: 600px; margin: 0 auto 1.5rem;">
                    "The Leptos UI library for applications demanding mathematical precision. "
                    "u64+ integers, arbitrary-precision decimals, zero precision loss."
                </p>
                <div style="display: flex; gap: 1rem; justify-content: center;">
                    <a href="/getting-started">
                        <Button variant=ButtonVariant::Filled>
                            "Get Started"
                        </Button>
                    </a>
                    <a href="https://github.com/Industrial-Algebra/Mingot" target="_blank" rel="noopener">
                        <Button variant=ButtonVariant::Outline>
                            "GitHub"
                        </Button>
                    </a>
                </div>
            </div>

            // Features
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1.5rem; margin-bottom: 3rem;">
                <FeatureCard
                    title="High Precision"
                    description="Support for u64, u128, i64, i128, and arbitrary-precision decimals via rust_decimal."
                    icon="🔢"
                />
                <FeatureCard
                    title="46 Components"
                    description="Buttons, inputs, modals, tables, and more. Everything you need for your app."
                    icon="🧩"
                />
                <FeatureCard
                    title="Theming"
                    description="Dark and light modes with customizable color schemes out of the box."
                    icon="🎨"
                />
                <FeatureCard
                    title="Type Safe"
                    description="Leverages Rust's type system. No silent precision loss or runtime surprises."
                    icon="✅"
                />
            </div>

            // Quick example
            <h2 class="section-title">"Quick Example"</h2>
            <CodeBlock
                code=r#"use mingot::prelude::*;

#[component]
fn App() -> impl IntoView {
    let value = RwSignal::new(String::new());

    view! {
        <MingotProvider>
            <Container>
                <NumberInput
                    precision=NumberInputPrecision::U64
                    label="Large Integer"
                    on_valid_change=Callback::new(move |result| {
                        if let Ok(val) = result {
                            value.set(val);
                        }
                    })
                />
                <Text>"Value: " {move || value.get()}</Text>
            </Container>
        </MingotProvider>
    }
}"#
                language="rust"
            />

            // Why Mingot
            <h2 class="section-title">"Why Mingot?"</h2>
            <p style="margin-bottom: 1rem;">
                "Standard HTML number inputs are limited by JavaScript's Number type, which can only safely represent integers up to 2"<sup>"53"</sup>"-1. For financial applications, scientific computing, or any domain requiring exact values, this is a critical limitation."
            </p>
            <CodeBlock
                code=r#"// JavaScript precision problems:
9007199254740992 + 1  // → 9007199254740992 (WRONG!)
0.1 + 0.2             // → 0.30000000000000004 (WRONG!)

// Mingot handles this correctly with rust_decimal
// Up to 28-29 significant digits with exact arithmetic"#
                language="rust"
            />
        </div>
    }
}

#[component]
fn FeatureCard(
    title: &'static str,
    description: &'static str,
    icon: &'static str,
) -> impl IntoView {
    view! {
        <div style="padding: 1.5rem; border: 1px solid #e9ecef; border-radius: 0.5rem;">
            <div style="font-size: 2rem; margin-bottom: 0.5rem;">{icon}</div>
            <h3 style="font-size: 1.125rem; font-weight: 600; margin-bottom: 0.5rem;">{title}</h3>
            <p style="color: #868e96; font-size: 0.875rem; margin: 0;">{description}</p>
        </div>
    }
}
