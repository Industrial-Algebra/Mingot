use leptos::prelude::*;
use mingot::prelude::*;

use crate::components::DemoBlock;

/// Documentation for a component prop
#[derive(Clone)]
pub struct PropDoc {
    pub name: &'static str,
    pub prop_type: &'static str,
    pub default: Option<&'static str>,
    pub description: &'static str,
    pub required: bool,
}

/// Documentation for a component
pub struct ComponentDoc {
    pub name: &'static str,
    pub import_name: &'static str,
    pub description: &'static str,
    pub props: Vec<PropDoc>,
    pub demo: fn() -> AnyView,
}

/// Get component documentation by slug
pub fn get_component_doc(slug: &str) -> Option<ComponentDoc> {
    match slug {
        "button" => Some(button_doc()),
        "number-input" => Some(number_input_doc()),
        "input" => Some(input_doc()),
        "text" => Some(text_doc()),
        "stack" => Some(stack_doc()),
        "container" => Some(container_doc()),
        _ => None,
    }
}

fn button_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Button",
        import_name: "Button, ButtonVariant, ButtonSize",
        description: "A button component with multiple variants, sizes, and states.",
        props: vec![
            PropDoc {
                name: "variant",
                prop_type: "Option<ButtonVariant>",
                default: Some("Filled"),
                description: "Visual style: Filled, Outline, Light, Subtle, or Default",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<ButtonSize>",
                default: Some("Md"),
                description: "Size of the button: Xs, Sm, Md, Lg, or Xl",
                required: false,
            },
            PropDoc {
                name: "color",
                prop_type: "Option<String>",
                default: Some("\"blue\""),
                description: "Theme color key for the button",
                required: false,
            },
            PropDoc {
                name: "on_click",
                prop_type: "Option<Callback<MouseEvent>>",
                default: None,
                description: "Callback fired when button is clicked",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "Signal<bool>",
                default: Some("false"),
                description: "Whether the button is disabled",
                required: false,
            },
            PropDoc {
                name: "loading",
                prop_type: "Signal<bool>",
                default: Some("false"),
                description: "Shows loading state and disables interaction",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Button content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <div>
                    <DemoBlock title="Variants" code=r#"<Button variant=ButtonVariant::Filled>"Filled"</Button>
<Button variant=ButtonVariant::Outline>"Outline"</Button>
<Button variant=ButtonVariant::Light>"Light"</Button>
<Button variant=ButtonVariant::Subtle>"Subtle"</Button>"#>
                        <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                            <Button variant=ButtonVariant::Filled>"Filled"</Button>
                            <Button variant=ButtonVariant::Outline>"Outline"</Button>
                            <Button variant=ButtonVariant::Light>"Light"</Button>
                            <Button variant=ButtonVariant::Subtle>"Subtle"</Button>
                        </div>
                    </DemoBlock>

                    <DemoBlock title="Sizes" code=r#"<Button size=ButtonSize::Xs>"Xs"</Button>
<Button size=ButtonSize::Sm>"Sm"</Button>
<Button size=ButtonSize::Md>"Md"</Button>
<Button size=ButtonSize::Lg>"Lg"</Button>
<Button size=ButtonSize::Xl>"Xl"</Button>"#>
                        <div style="display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap;">
                            <Button size=ButtonSize::Xs>"Xs"</Button>
                            <Button size=ButtonSize::Sm>"Sm"</Button>
                            <Button size=ButtonSize::Md>"Md"</Button>
                            <Button size=ButtonSize::Lg>"Lg"</Button>
                            <Button size=ButtonSize::Xl>"Xl"</Button>
                        </div>
                    </DemoBlock>

                    <DemoBlock title="Colors" code=r#"<Button color="blue">"Blue"</Button>
<Button color="red">"Red"</Button>
<Button color="green">"Green"</Button>
<Button color="orange">"Orange"</Button>"#>
                        <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                            <Button color="blue">"Blue"</Button>
                            <Button color="red">"Red"</Button>
                            <Button color="green">"Green"</Button>
                            <Button color="orange">"Orange"</Button>
                        </div>
                    </DemoBlock>
                </div>
            }.into_any()
        },
    }
}

fn number_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "NumberInput",
        import_name: "NumberInput, NumberInputPrecision, ParseError",
        description: "High-precision numeric input supporting u64, u128, i64, i128, fixed decimals, and arbitrary precision via rust_decimal.",
        props: vec![
            PropDoc {
                name: "precision",
                prop_type: "NumberInputPrecision",
                default: Some("U64"),
                description: "Precision type: U64, U128, I64, I128, Decimal(u32), or Arbitrary",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text displayed above the input",
                required: false,
            },
            PropDoc {
                name: "placeholder",
                prop_type: "Option<String>",
                default: None,
                description: "Placeholder text when empty",
                required: false,
            },
            PropDoc {
                name: "on_valid_change",
                prop_type: "Option<Callback<Result<String, ParseError>>>",
                default: None,
                description: "Callback fired with validation result on each change",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "Signal<bool>",
                default: Some("false"),
                description: "Whether the input is disabled",
                required: false,
            },
            PropDoc {
                name: "error",
                prop_type: "Option<String>",
                default: None,
                description: "Error message to display",
                required: false,
            },
        ],
        demo: || {
            let u64_value = RwSignal::new(String::new());
            let u128_value = RwSignal::new(String::new());
            let decimal_value = RwSignal::new(String::new());

            view! {
                <div>
                    <DemoBlock title="Precision Types">
                        <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 400px;">
                            <div>
                                <NumberInput
                                    precision=NumberInputPrecision::U64
                                    label="U64 (up to 18,446,744,073,709,551,615)"
                                    placeholder="Enter a large integer"
                                    on_valid_change=Callback::new(move |result: Result<String, ParseError>| {
                                        if let Ok(val) = result {
                                            u64_value.set(val);
                                        }
                                    })
                                />
                                <div style="font-size: 0.75rem; color: #868e96; margin-top: 0.25rem;">
                                    "Value: " {move || u64_value.get()}
                                </div>
                            </div>

                            <div>
                                <NumberInput
                                    precision=NumberInputPrecision::U128
                                    label="U128 (up to 340 undecillion)"
                                    placeholder="Enter a very large integer"
                                    on_valid_change=Callback::new(move |result: Result<String, ParseError>| {
                                        if let Ok(val) = result {
                                            u128_value.set(val);
                                        }
                                    })
                                />
                                <div style="font-size: 0.75rem; color: #868e96; margin-top: 0.25rem;">
                                    "Value: " {move || u128_value.get()}
                                </div>
                            </div>

                            <div>
                                <NumberInput
                                    precision=NumberInputPrecision::Decimal(6)
                                    label="Decimal (6 places)"
                                    placeholder="0.000000"
                                    on_valid_change=Callback::new(move |result: Result<String, ParseError>| {
                                        if let Ok(val) = result {
                                            decimal_value.set(val);
                                        }
                                    })
                                />
                                <div style="font-size: 0.75rem; color: #868e96; margin-top: 0.25rem;">
                                    "Value: " {move || decimal_value.get()}
                                </div>
                            </div>
                        </div>
                    </DemoBlock>

                    <h2 class="section-title">"Why NumberInput?"</h2>
                    <p style="margin-bottom: 1rem;">
                        "Standard HTML number inputs are limited by JavaScript's Number type (max safe integer: 2"<sup>"53"</sup>"-1). "
                        "NumberInput handles values up to 2"<sup>"128"</sup>" and beyond with arbitrary precision decimals."
                    </p>

                    <DemoBlock title="Real-World Use Cases">
                        <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 400px;">
                            <NumberInput
                                precision=NumberInputPrecision::U64
                                label="Cryptocurrency: Satoshi Amount"
                                placeholder="e.g., 100000000 (1 BTC)"
                            />
                            <NumberInput
                                precision=NumberInputPrecision::Decimal(8)
                                label="Financial: Exact Currency"
                                placeholder="e.g., 1234567.89012345"
                            />
                            <NumberInput
                                precision=NumberInputPrecision::I128
                                label="Scientific: Large Signed Values"
                                placeholder="Positive or negative"
                            />
                        </div>
                    </DemoBlock>
                </div>
            }.into_any()
        },
    }
}

fn input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Input",
        import_name: "Input, InputVariant, InputSize",
        description: "A text input component with variants, sizes, and HTML5 attribute support.",
        props: vec![
            PropDoc {
                name: "variant",
                prop_type: "Option<InputVariant>",
                default: Some("Default"),
                description: "Visual style: Default, Filled, or Unstyled",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<InputSize>",
                default: Some("Md"),
                description: "Size: Xs, Sm, Md, Lg, or Xl",
                required: false,
            },
            PropDoc {
                name: "placeholder",
                prop_type: "Option<String>",
                default: None,
                description: "Placeholder text",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text",
                required: false,
            },
            PropDoc {
                name: "on_input",
                prop_type: "Option<Callback<String>>",
                default: None,
                description: "Callback fired on input",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "Signal<bool>",
                default: Some("false"),
                description: "Whether the input is disabled",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Basic Usage">
                    <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 300px;">
                        <Input label="Default" placeholder="Enter text..." />
                        <Input variant=InputVariant::Filled label="Filled" placeholder="Filled variant" />
                    </div>
                </DemoBlock>
            }.into_any()
        },
    }
}

fn text_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Text",
        import_name: "Text, TextSize, TextWeight",
        description: "A typography component for displaying text with various sizes and weights.",
        props: vec![
            PropDoc {
                name: "size",
                prop_type: "Option<TextSize>",
                default: Some("Md"),
                description: "Font size: Xs, Sm, Md, Lg, or Xl",
                required: false,
            },
            PropDoc {
                name: "weight",
                prop_type: "Option<TextWeight>",
                default: Some("Normal"),
                description: "Font weight: Normal, Medium, Semibold, or Bold",
                required: false,
            },
            PropDoc {
                name: "color",
                prop_type: "Option<String>",
                default: None,
                description: "Text color (theme color key or 'dimmed')",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Text content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Sizes and Weights">
                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                        <Text size=TextSize::Xl weight=TextWeight::Bold>"Extra Large Bold"</Text>
                        <Text size=TextSize::Lg weight=TextWeight::Semibold>"Large Semibold"</Text>
                        <Text size=TextSize::Md>"Medium (default)"</Text>
                        <Text size=TextSize::Sm color="dimmed">"Small Dimmed"</Text>
                        <Text size=TextSize::Xs>"Extra Small"</Text>
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn stack_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Stack",
        import_name: "Stack",
        description: "A vertical layout component that stacks children with consistent spacing.",
        props: vec![
            PropDoc {
                name: "spacing",
                prop_type: "Option<&str>",
                default: Some("\"md\""),
                description: "Spacing between items: xs, sm, md, lg, xl, or custom CSS value",
                required: false,
            },
            PropDoc {
                name: "align",
                prop_type: "Option<&str>",
                default: Some("\"stretch\""),
                description: "Horizontal alignment: stretch, start, center, end",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Stack content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Basic Stack">
                    <Stack spacing="md">
                        <div style="padding: 1rem; background: #e7f5ff; border-radius: 0.25rem;">"Item 1"</div>
                        <div style="padding: 1rem; background: #e7f5ff; border-radius: 0.25rem;">"Item 2"</div>
                        <div style="padding: 1rem; background: #e7f5ff; border-radius: 0.25rem;">"Item 3"</div>
                    </Stack>
                </DemoBlock>
            }.into_any()
        },
    }
}

fn container_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Container",
        import_name: "Container",
        description: "A wrapper component that centers content and constrains its maximum width.",
        props: vec![
            PropDoc {
                name: "size",
                prop_type: "Option<&str>",
                default: Some("\"md\""),
                description: "Maximum width: xs, sm, md, lg, xl, or custom CSS value",
                required: false,
            },
            PropDoc {
                name: "padding",
                prop_type: "Option<&str>",
                default: Some("\"md\""),
                description: "Horizontal padding",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Container content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Container Sizes">
                    <div style="width: 100%;">
                        <div style="margin-bottom: 1rem;">
                            <Text size=TextSize::Sm color="dimmed">"xs (540px max)"</Text>
                            <div style="max-width: 540px; background: #e7f5ff; padding: 0.5rem; border-radius: 0.25rem;">"Content"</div>
                        </div>
                        <div style="margin-bottom: 1rem;">
                            <Text size=TextSize::Sm color="dimmed">"sm (720px max)"</Text>
                            <div style="max-width: 720px; background: #d0ebff; padding: 0.5rem; border-radius: 0.25rem;">"Content"</div>
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"md (960px max)"</Text>
                            <div style="max-width: 960px; background: #a5d8ff; padding: 0.5rem; border-radius: 0.25rem;">"Content"</div>
                        </div>
                    </div>
                </DemoBlock>
            }.into_any()
        },
    }
}
