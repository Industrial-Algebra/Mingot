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
        // Core
        "button" => Some(button_doc()),
        "action-icon" => Some(action_icon_doc()),
        "divider" => Some(divider_doc()),
        "group" => Some(group_doc()),
        "text" => Some(text_doc()),
        "stack" => Some(stack_doc()),
        "container" => Some(container_doc()),
        // Layout
        "app-shell" => Some(app_shell_doc()),
        "card" => Some(card_doc()),
        "grid" => Some(grid_doc()),
        "header" => Some(header_doc()),
        "paper" => Some(paper_doc()),
        // Navigation
        "breadcrumbs" => Some(breadcrumbs_doc()),
        "burger" => Some(burger_doc()),
        "navbar" => Some(navbar_doc()),
        "pagination" => Some(pagination_doc()),
        "tabs" => Some(tabs_doc()),
        // Form
        "input" => Some(input_doc()),
        "number-input" => Some(number_input_doc()),
        "angle-input" => Some(angle_input_doc()),
        "fraction-input" => Some(fraction_input_doc()),
        "unit-input" => Some(unit_input_doc()),
        "complex-number-input" => Some(complex_number_input_doc()),
        "uncertainty-input" => Some(uncertainty_input_doc()),
        "interval-input" => Some(interval_input_doc()),
        "coordinate-input" => Some(coordinate_input_doc()),
        "point-locator" => Some(point_locator_doc()),
        "matrix-input" => Some(matrix_input_doc()),
        "vector-input" => Some(vector_input_doc()),
        "tensor-input" => Some(tensor_input_doc()),
        "symbol-palette" => Some(symbol_palette_doc()),
        "formula-input" => Some(formula_input_doc()),
        "equation-editor" => Some(equation_editor_doc()),
        "checkbox" => Some(checkbox_doc()),
        "file-input" => Some(file_input_doc()),
        "password-input" => Some(password_input_doc()),
        "pin-input" => Some(pin_input_doc()),
        "radio" => Some(radio_doc()),
        "select" => Some(select_doc()),
        "slider" => Some(slider_doc()),
        "range-slider" => Some(range_slider_doc()),
        "segmented-control" => Some(segmented_control_doc()),
        "switch" => Some(switch_doc()),
        "textarea" => Some(textarea_doc()),
        "parameter-slider" => Some(parameter_slider_doc()),
        "parameter-grid" => Some(parameter_grid_doc()),
        "parameter-tree" => Some(parameter_tree_doc()),
        // Overlay
        "drawer" => Some(drawer_doc()),
        "loading-overlay" => Some(loading_overlay_doc()),
        "modal" => Some(modal_doc()),
        "popover" => Some(popover_doc()),
        "tooltip" => Some(tooltip_doc()),
        // Feedback
        "alert" => Some(alert_doc()),
        "loader" => Some(loader_doc()),
        "notification" => Some(notification_doc()),
        "progress" => Some(progress_doc()),
        "skeleton" => Some(skeleton_doc()),
        // Data Display
        "accordion" => Some(accordion_doc()),
        "avatar" => Some(avatar_doc()),
        "badge" => Some(badge_doc()),
        "ring-progress" => Some(ring_progress_doc()),
        "stats" => Some(stats_doc()),
        "table" => Some(table_doc()),
        // Misc
        "error-page" => Some(error_page_doc()),
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
        import_name: "NumberInput, NumberInputPrecision, NumberInputFormat, NumberInputLocale, ParseError",
        description: "High-precision numeric input supporting u64, u128, i64, i128, fixed decimals, and arbitrary precision via rust_decimal. Features increment controls, locale formatting, and precision indicators.",
        props: vec![
            PropDoc {
                name: "precision",
                prop_type: "NumberInputPrecision",
                default: Some("I64"),
                description: "Precision type: U64, U128, I64, I128, Decimal(u32), or Arbitrary",
                required: false,
            },
            PropDoc {
                name: "show_controls",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to show +/- increment/decrement controls",
                required: false,
            },
            PropDoc {
                name: "step",
                prop_type: "Option<String>",
                default: Some("\"1\""),
                description: "Step size for increment/decrement",
                required: false,
            },
            PropDoc {
                name: "shift_step",
                prop_type: "Option<String>",
                default: Some("10x step"),
                description: "Step size when Shift key is held (default: 10x step)",
                required: false,
            },
            PropDoc {
                name: "ctrl_step",
                prop_type: "Option<String>",
                default: Some("100x step"),
                description: "Step size when Ctrl key is held (default: 100x step)",
                required: false,
            },
            PropDoc {
                name: "format_on_blur",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to auto-format value on blur",
                required: false,
            },
            PropDoc {
                name: "format",
                prop_type: "Option<NumberInputFormat>",
                default: Some("Standard"),
                description: "Format type: Standard, Thousand, Scientific, Engineering",
                required: false,
            },
            PropDoc {
                name: "locale",
                prop_type: "Option<NumberInputLocale>",
                default: None,
                description: "Locale preset: US, EU, Swiss, Indian",
                required: false,
            },
            PropDoc {
                name: "show_precision_indicator",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to show precision type indicator",
                required: false,
            },
            PropDoc {
                name: "show_overflow_warning",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to show warning when approaching limits",
                required: false,
            },
            PropDoc {
                name: "allow_mouse_wheel",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether mouse wheel can change value when focused",
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
                name: "on_valid_change",
                prop_type: "Option<Callback<Result<String, ParseError>>>",
                default: None,
                description: "Callback fired with validation result on each change",
                required: false,
            },
        ],
        demo: || {
            let u64_value = RwSignal::new(String::new());
            let controls_value = RwSignal::new("100".to_string());
            let format_value = RwSignal::new("1234567.89".to_string());

            view! {
                <div>
                    <DemoBlock title="Precision Types">
                        <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 400px;">
                            <NumberInput
                                precision=NumberInputPrecision::U64
                                label="U64 (up to 18.4 quintillion)"
                                placeholder="Enter a large integer"
                                show_precision_indicator=true
                                on_valid_change=Callback::new(move |result: Result<String, ParseError>| {
                                    if let Ok(val) = result {
                                        u64_value.set(val);
                                    }
                                })
                            />
                            <NumberInput
                                precision=NumberInputPrecision::Decimal(6)
                                label="Decimal (6 places)"
                                placeholder="0.000000"
                                show_precision_indicator=true
                            />
                        </div>
                    </DemoBlock>

                    <h2 class="section-title">"Increment/Decrement Controls"</h2>
                    <DemoBlock title="With +/- Controls and Modifier Keys">
                        <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 400px;">
                            <NumberInput
                                value=controls_value
                                precision=NumberInputPrecision::I64
                                label="Step Modifiers (click +/- or use keys)"
                                show_controls=true
                                step="1"
                                shift_step="10"
                                ctrl_step="100"
                                allow_mouse_wheel=true
                                description="Click/Arrow: ±1, Shift: ±10, Ctrl: ±100"
                            />
                            <NumberInput
                                precision=NumberInputPrecision::Decimal(2)
                                label="Decimal stepping with modifiers"
                                placeholder="0.00"
                                show_controls=true
                                step="0.01"
                                shift_step="0.10"
                                ctrl_step="1.00"
                                description="Normal: ±0.01, Shift: ±0.10, Ctrl: ±1.00"
                            />
                            <NumberInput
                                precision=NumberInputPrecision::U64
                                label="Large value stepping"
                                placeholder="0"
                                show_controls=true
                                step="1000"
                                shift_step="10000"
                                ctrl_step="100000"
                                format_on_blur=true
                                format=NumberInputFormat::Thousand
                                description="Normal: ±1K, Shift: ±10K, Ctrl: ±100K"
                            />
                        </div>
                    </DemoBlock>

                    <h2 class="section-title">"Number Formatting"</h2>
                    <DemoBlock title="Auto-formatting on blur">
                        <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 400px;">
                            <NumberInput
                                value=format_value
                                precision=NumberInputPrecision::Decimal(2)
                                label="Thousand separators (US format)"
                                format_on_blur=true
                                format=NumberInputFormat::Thousand
                                locale=NumberInputLocale::US
                            />
                            <NumberInput
                                precision=NumberInputPrecision::Decimal(2)
                                label="EU format (1.234.567,89)"
                                placeholder="Enter number"
                                format_on_blur=true
                                format=NumberInputFormat::Thousand
                                locale=NumberInputLocale::EU
                            />
                            <NumberInput
                                precision=NumberInputPrecision::I64
                                label="Engineering notation (exponents ÷ 3)"
                                placeholder="e.g., 1234567"
                                format_on_blur=true
                                format=NumberInputFormat::Engineering
                            />
                        </div>
                    </DemoBlock>

                    <h2 class="section-title">"Overflow Warnings"</h2>
                    <DemoBlock title="Approaching limits">
                        <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 400px;">
                            <NumberInput
                                precision=NumberInputPrecision::U64
                                label="Try entering a value near 18,446,744,073,709,551,615"
                                placeholder="Enter large value"
                                show_overflow_warning=true
                                show_precision_indicator=true
                            />
                        </div>
                    </DemoBlock>

                    <h2 class="section-title">"Real-World Use Cases"</h2>
                    <DemoBlock title="Domain-specific examples">
                        <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 400px;">
                            <NumberInput
                                precision=NumberInputPrecision::U64
                                label="Cryptocurrency: Satoshi Amount"
                                placeholder="e.g., 100000000 (1 BTC)"
                                show_controls=true
                                step="100000"
                                format_on_blur=true
                                format=NumberInputFormat::Thousand
                            />
                            <NumberInput
                                precision=NumberInputPrecision::Decimal(8)
                                label="Financial: Exact Currency"
                                placeholder="e.g., 1234567.89"
                                format_on_blur=true
                                locale=NumberInputLocale::US
                            />
                            <NumberInput
                                precision=NumberInputPrecision::I128
                                label="Scientific: Large Values (auto-scientific)"
                                placeholder="Values > 1 trillion become scientific"
                                auto_scientific_threshold=1000000000000.0
                                format_on_blur=true
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

// ============================================================================
// Core Components
// ============================================================================

fn action_icon_doc() -> ComponentDoc {
    ComponentDoc {
        name: "ActionIcon",
        import_name: "ActionIcon, ActionIconVariant, ActionIconSize",
        description:
            "An icon-only button component for actions like close, settings, or menu toggles.",
        props: vec![
            PropDoc {
                name: "variant",
                prop_type: "Option<ActionIconVariant>",
                default: Some("Filled"),
                description: "Visual style: Filled, Outline, Light, Subtle, or Transparent",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<ActionIconSize>",
                default: Some("Md"),
                description: "Size: Xs, Sm, Md, Lg, or Xl",
                required: false,
            },
            PropDoc {
                name: "color",
                prop_type: "Option<String>",
                default: Some("\"blue\""),
                description: "Theme color for the button",
                required: false,
            },
            PropDoc {
                name: "on_click",
                prop_type: "Option<Callback<MouseEvent>>",
                default: None,
                description: "Click handler",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Icon content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Variants">
                    <Group spacing="md">
                        <ActionIcon variant=ActionIconVariant::Filled>"✓"</ActionIcon>
                        <ActionIcon variant=ActionIconVariant::Outline>"✓"</ActionIcon>
                        <ActionIcon variant=ActionIconVariant::Light>"✓"</ActionIcon>
                        <ActionIcon variant=ActionIconVariant::Subtle>"✓"</ActionIcon>
                    </Group>
                </DemoBlock>
                <DemoBlock title="Sizes">
                    <Group spacing="md" align=GroupAlign::Center>
                        <ActionIcon size=ActionIconSize::Xs>"×"</ActionIcon>
                        <ActionIcon size=ActionIconSize::Sm>"×"</ActionIcon>
                        <ActionIcon size=ActionIconSize::Md>"×"</ActionIcon>
                        <ActionIcon size=ActionIconSize::Lg>"×"</ActionIcon>
                        <ActionIcon size=ActionIconSize::Xl>"×"</ActionIcon>
                    </Group>
                </DemoBlock>
                <DemoBlock title="Colors">
                    <Group spacing="md">
                        <ActionIcon color="blue">"★"</ActionIcon>
                        <ActionIcon color="red">"★"</ActionIcon>
                        <ActionIcon color="green">"★"</ActionIcon>
                        <ActionIcon color="orange">"★"</ActionIcon>
                    </Group>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn divider_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Divider",
        import_name: "Divider, DividerOrientation, DividerVariant, DividerLabelPosition",
        description: "A horizontal or vertical line to separate content sections.",
        props: vec![
            PropDoc {
                name: "orientation",
                prop_type: "Option<DividerOrientation>",
                default: Some("Horizontal"),
                description: "Orientation: Horizontal or Vertical",
                required: false,
            },
            PropDoc {
                name: "variant",
                prop_type: "Option<DividerVariant>",
                default: Some("Solid"),
                description: "Line style: Solid, Dashed, or Dotted",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Optional label text (horizontal only)",
                required: false,
            },
            PropDoc {
                name: "label_position",
                prop_type: "Option<DividerLabelPosition>",
                default: Some("Center"),
                description: "Label position: Left, Center, or Right",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Basic Divider">
                    <div style="width: 100%;">
                        <Text>"Content above"</Text>
                        <Divider />
                        <Text>"Content below"</Text>
                    </div>
                </DemoBlock>
                <DemoBlock title="With Label">
                    <div style="width: 100%;">
                        <Divider label="OR" />
                        <Divider label="Section" label_position=DividerLabelPosition::Left />
                    </div>
                </DemoBlock>
                <DemoBlock title="Variants">
                    <div style="width: 100%;">
                        <Divider variant=DividerVariant::Solid />
                        <Divider variant=DividerVariant::Dashed />
                        <Divider variant=DividerVariant::Dotted />
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn group_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Group",
        import_name: "Group, GroupAlign, GroupJustify",
        description: "A horizontal layout component that groups children with consistent spacing.",
        props: vec![
            PropDoc {
                name: "spacing",
                prop_type: "Option<String>",
                default: Some("\"md\""),
                description: "Gap between items",
                required: false,
            },
            PropDoc {
                name: "align",
                prop_type: "Option<GroupAlign>",
                default: Some("Center"),
                description: "Vertical alignment: Start, Center, End, Baseline",
                required: false,
            },
            PropDoc {
                name: "justify",
                prop_type: "Option<GroupJustify>",
                default: Some("Start"),
                description:
                    "Horizontal distribution: Start, Center, End, SpaceBetween, SpaceAround",
                required: false,
            },
            PropDoc {
                name: "wrap",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether items should wrap to next line",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Group content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Basic Group">
                    <Group spacing="md">
                        <Button>"One"</Button>
                        <Button>"Two"</Button>
                        <Button>"Three"</Button>
                    </Group>
                </DemoBlock>
                <DemoBlock title="Justify">
                    <div style="width: 100%;">
                        <Group justify=GroupJustify::SpaceBetween>
                            <Button>"Left"</Button>
                            <Button>"Right"</Button>
                        </Group>
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

// ============================================================================
// Layout Components
// ============================================================================

fn app_shell_doc() -> ComponentDoc {
    ComponentDoc {
        name: "AppShell",
        import_name: "AppShell",
        description: "A layout component for building application shells with header, navbar, and main content areas.",
        props: vec![
            PropDoc {
                name: "header",
                prop_type: "Option<Children>",
                default: None,
                description: "Header content",
                required: false,
            },
            PropDoc {
                name: "navbar",
                prop_type: "Option<Children>",
                default: None,
                description: "Navbar/sidebar content",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Main content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="AppShell Structure">
                    <div style="border: 1px solid #dee2e6; border-radius: 0.25rem; overflow: hidden; height: 200px;">
                        <div style="background: #228be6; color: white; padding: 0.5rem 1rem; font-weight: 500;">"Header"</div>
                        <div style="display: flex; height: calc(100% - 40px);">
                            <div style="width: 200px; background: #f1f3f5; padding: 1rem; border-right: 1px solid #dee2e6;">"Navbar"</div>
                            <div style="flex: 1; padding: 1rem;">"Main Content"</div>
                        </div>
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn card_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Card",
        import_name: "Card",
        description: "A container component with a border, background, and optional shadow.",
        props: vec![
            PropDoc {
                name: "shadow",
                prop_type: "Option<String>",
                default: Some("\"sm\""),
                description: "Shadow size: xs, sm, md, lg, xl",
                required: false,
            },
            PropDoc {
                name: "padding",
                prop_type: "Option<String>",
                default: Some("\"md\""),
                description: "Internal padding",
                required: false,
            },
            PropDoc {
                name: "radius",
                prop_type: "Option<String>",
                default: Some("\"sm\""),
                description: "Border radius",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Card content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Basic Card">
                    <Card>
                        <Text weight=TextWeight::Bold>"Card Title"</Text>
                        <Text size=TextSize::Sm color="dimmed">"Card content goes here."</Text>
                    </Card>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn grid_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Grid",
        import_name: "Grid, GridCol",
        description: "A responsive grid layout component based on CSS Grid.",
        props: vec![
            PropDoc {
                name: "columns",
                prop_type: "Option<u32>",
                default: Some("12"),
                description: "Number of columns",
                required: false,
            },
            PropDoc {
                name: "gutter",
                prop_type: "Option<String>",
                default: Some("\"md\""),
                description: "Gap between grid items",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Grid content (GridCol components)",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Grid Layout">
                    <Grid>
                        <GridCol span=6>
                            <div style="background: #e7f5ff; padding: 1rem; text-align: center;">"span=6"</div>
                        </GridCol>
                        <GridCol span=6>
                            <div style="background: #d0ebff; padding: 1rem; text-align: center;">"span=6"</div>
                        </GridCol>
                        <GridCol span=4>
                            <div style="background: #a5d8ff; padding: 1rem; text-align: center;">"span=4"</div>
                        </GridCol>
                        <GridCol span=4>
                            <div style="background: #74c0fc; padding: 1rem; text-align: center;">"span=4"</div>
                        </GridCol>
                        <GridCol span=4>
                            <div style="background: #4dabf7; padding: 1rem; text-align: center;">"span=4"</div>
                        </GridCol>
                    </Grid>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn header_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Header",
        import_name: "Header",
        description: "A fixed or static header component for application layouts.",
        props: vec![
            PropDoc {
                name: "height",
                prop_type: "Option<String>",
                default: Some("\"60px\""),
                description: "Header height",
                required: false,
            },
            PropDoc {
                name: "fixed",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether header is fixed to top",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Header content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Header Example">
                    <div style="border: 1px solid #dee2e6; border-radius: 0.25rem;">
                        <Header>
                            <Group justify=GroupJustify::SpaceBetween style="padding: 0 1rem; height: 100%;">
                                <Text weight=TextWeight::Bold>"Logo"</Text>
                                <Group spacing="md">
                                    <Text>"Home"</Text>
                                    <Text>"About"</Text>
                                    <Text>"Contact"</Text>
                                </Group>
                            </Group>
                        </Header>
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn paper_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Paper",
        import_name: "Paper",
        description:
            "A container with a background color and optional shadow, useful for elevated surfaces.",
        props: vec![
            PropDoc {
                name: "shadow",
                prop_type: "Option<String>",
                default: None,
                description: "Shadow size: xs, sm, md, lg, xl",
                required: false,
            },
            PropDoc {
                name: "padding",
                prop_type: "Option<String>",
                default: Some("\"md\""),
                description: "Internal padding",
                required: false,
            },
            PropDoc {
                name: "radius",
                prop_type: "Option<String>",
                default: Some("\"sm\""),
                description: "Border radius",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Paper content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Paper with Shadow">
                    <Group spacing="lg">
                        <Paper shadow="xs" padding="md">"xs shadow"</Paper>
                        <Paper shadow="md" padding="md">"md shadow"</Paper>
                        <Paper shadow="xl" padding="md">"xl shadow"</Paper>
                    </Group>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

// ============================================================================
// Navigation Components
// ============================================================================

fn breadcrumbs_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Breadcrumbs",
        import_name: "Breadcrumbs, BreadcrumbItem",
        description: "Navigation breadcrumbs showing the current page location.",
        props: vec![
            PropDoc {
                name: "items",
                prop_type: "Vec<BreadcrumbItem>",
                default: None,
                description: "Vector of breadcrumb items",
                required: true,
            },
            PropDoc {
                name: "separator",
                prop_type: "Option<String>",
                default: Some("\"/\""),
                description: "Character or string used as separator",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Breadcrumbs" code=r#"<Breadcrumbs items=vec![
    BreadcrumbItem::new("Home").href("/"),
    BreadcrumbItem::new("Docs").href("/docs"),
    BreadcrumbItem::new("Current Page"),
] />"#>
                    <Breadcrumbs items=vec![
                        BreadcrumbItem::new("Home").href("/"),
                        BreadcrumbItem::new("Docs").href("/docs"),
                        BreadcrumbItem::new("Current Page"),
                    ] />
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn burger_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Burger",
        import_name: "Burger, BurgerSize",
        description: "A hamburger menu button with animated open/close state.",
        props: vec![
            PropDoc {
                name: "opened",
                prop_type: "Signal<bool>",
                default: None,
                description: "Whether the burger is in open state",
                required: true,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<BurgerSize>",
                default: Some("Md"),
                description: "Size: Xs, Sm, Md, Lg, Xl",
                required: false,
            },
            PropDoc {
                name: "on_click",
                prop_type: "Option<Callback<MouseEvent>>",
                default: None,
                description: "Click handler",
                required: false,
            },
        ],
        demo: || {
            let opened = RwSignal::new(false);
            view! {
                <DemoBlock title="Burger Menu">
                    <Group spacing="lg">
                        <Burger
                            opened=opened
                            on_click=Callback::new(move |_| opened.update(|o| *o = !*o))
                        />
                        <Text>{move || if opened.get() { "Open" } else { "Closed" }}</Text>
                    </Group>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn navbar_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Navbar",
        import_name: "Navbar, NavbarLink, NavbarBrand, NavbarOrientation, NavbarVariant",
        description: "A navigation component with horizontal or vertical orientation.",
        props: vec![
            PropDoc {
                name: "orientation",
                prop_type: "Option<NavbarOrientation>",
                default: Some("Horizontal"),
                description: "Layout direction: Horizontal or Vertical",
                required: false,
            },
            PropDoc {
                name: "spacing",
                prop_type: "Option<String>",
                default: None,
                description: "Gap between items",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Navbar content (NavbarLink, NavbarBrand)",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Navbar" code=r#"<Navbar orientation=NavbarOrientation::Vertical>
    <NavbarBrand>"Brand"</NavbarBrand>
    <NavbarLink href="/" active=true>"Home"</NavbarLink>
    <NavbarLink href="/about">"About"</NavbarLink>
</Navbar>"#>
                    <div style="border: 1px solid #dee2e6; border-radius: 0.25rem; padding: 1rem;">
                        <Navbar orientation=NavbarOrientation::Vertical>
                            <NavbarBrand>"Navigation"</NavbarBrand>
                            <NavbarLink href="#" active=true>"Dashboard"</NavbarLink>
                            <NavbarLink href="#">"Settings"</NavbarLink>
                            <NavbarLink href="#">"Profile"</NavbarLink>
                        </Navbar>
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn tabs_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Tabs",
        import_name: "Tabs, TabsList, TabsTab, TabsPanel",
        description: "Tabbed interface for organizing content into sections.",
        props: vec![
            PropDoc {
                name: "active",
                prop_type: "RwSignal<String>",
                default: None,
                description: "Signal controlling the active tab value",
                required: true,
            },
            PropDoc {
                name: "variant",
                prop_type: "Option<TabsVariant>",
                default: Some("Default"),
                description: "Visual style: Default, Outline, Pills",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "TabsList and TabsPanel components",
                required: true,
            },
        ],
        demo: || {
            let active = RwSignal::new("first".to_string());
            view! {
                <DemoBlock title="Tabs" code=r#"let active = RwSignal::new("first".to_string());
<Tabs active=active>
    <TabsList>
        <TabsTab value="first">"First"</TabsTab>
        <TabsTab value="second">"Second"</TabsTab>
    </TabsList>
    <TabsPanel value="first">"First content"</TabsPanel>
    <TabsPanel value="second">"Second content"</TabsPanel>
</Tabs>"#>
                    <Tabs active=active>
                        <TabsList>
                            <TabsTab value="first">"First"</TabsTab>
                            <TabsTab value="second">"Second"</TabsTab>
                            <TabsTab value="third">"Third"</TabsTab>
                        </TabsList>
                        <TabsPanel value="first">
                            <Text>"First tab content"</Text>
                        </TabsPanel>
                        <TabsPanel value="second">
                            <Text>"Second tab content"</Text>
                        </TabsPanel>
                        <TabsPanel value="third">
                            <Text>"Third tab content"</Text>
                        </TabsPanel>
                    </Tabs>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

// ============================================================================
// Form Components
// ============================================================================

fn checkbox_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Checkbox",
        import_name: "Checkbox",
        description: "A checkbox input component with label support.",
        props: vec![
            PropDoc {
                name: "checked",
                prop_type: "Signal<bool>",
                default: None,
                description: "Whether the checkbox is checked",
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
                name: "on_change",
                prop_type: "Option<Callback<bool>>",
                default: None,
                description: "Change handler",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "Signal<bool>",
                default: Some("false"),
                description: "Whether the checkbox is disabled",
                required: false,
            },
        ],
        demo: || {
            let checked = RwSignal::new(false);
            view! {
                <DemoBlock title="Checkbox">
                    <Stack spacing="md">
                        <Checkbox
                            checked=checked
                            label="Accept terms and conditions"
                            on_change=Callback::new(move |v| checked.set(v))
                        />
                        <Text size=TextSize::Sm color="dimmed">
                            {move || if checked.get() { "Checked" } else { "Unchecked" }}
                        </Text>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn password_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "PasswordInput",
        import_name: "PasswordInput",
        description: "A password input with visibility toggle.",
        props: vec![
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text",
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
                name: "on_input",
                prop_type: "Option<Callback<String>>",
                default: None,
                description: "Input handler",
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
                <DemoBlock title="Password Input">
                    <div style="max-width: 300px;">
                        <PasswordInput label="Password" placeholder="Enter your password" />
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn radio_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Radio",
        import_name: "Radio, RadioGroup",
        description: "Radio button inputs for selecting one option from a group.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "String",
                default: None,
                description: "Value of this radio option",
                required: true,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<String>>",
                default: None,
                description: "Callback when radio is selected",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether the radio is disabled",
                required: false,
            },
        ],
        demo: || {
            let selected = RwSignal::new("option1".to_string());
            view! {
                <DemoBlock title="Radio Group" code=r#"<RadioGroup label="Select an option">
    <Radio value="option1" label="Option 1" />
    <Radio value="option2" label="Option 2" />
</RadioGroup>"#>
                    <RadioGroup label="Select an option">
                        <Stack spacing="sm">
                            <Radio value="option1" label="Option 1" on_change=Callback::new(move |v| selected.set(v)) />
                            <Radio value="option2" label="Option 2" on_change=Callback::new(move |v| selected.set(v)) />
                            <Radio value="option3" label="Option 3" on_change=Callback::new(move |v| selected.set(v)) />
                        </Stack>
                    </RadioGroup>
                    <Text size=TextSize::Sm color="dimmed" style="margin-top: 0.5rem;">
                        "Selected: " {move || selected.get()}
                    </Text>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn select_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Select",
        import_name: "Select, SelectOption",
        description: "A dropdown select input component.",
        props: vec![
            PropDoc {
                name: "options",
                prop_type: "Vec<SelectOption>",
                default: None,
                description: "Vector of select options",
                required: true,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text",
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
                name: "value",
                prop_type: "Option<RwSignal<String>>",
                default: None,
                description: "Currently selected value",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<String>>",
                default: None,
                description: "Change handler",
                required: false,
            },
        ],
        demo: || {
            let value = RwSignal::new(String::new());
            view! {
                <DemoBlock title="Select" code=r#"<Select
    label="Choose a framework"
    placeholder="Select one"
    options=vec![
        SelectOption::new("leptos", "Leptos"),
        SelectOption::new("yew", "Yew"),
        SelectOption::new("dioxus", "Dioxus"),
    ]
/>"#>
                    <div style="max-width: 300px;">
                        <Select
                            label="Choose a framework"
                            placeholder="Select one"
                            value=value
                            options=vec![
                                SelectOption::new("leptos", "Leptos"),
                                SelectOption::new("yew", "Yew"),
                                SelectOption::new("dioxus", "Dioxus"),
                            ]
                            on_change=Callback::new(move |v| value.set(v))
                        />
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn switch_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Switch",
        import_name: "Switch",
        description: "A toggle switch component for boolean values.",
        props: vec![
            PropDoc {
                name: "checked",
                prop_type: "Signal<bool>",
                default: None,
                description: "Whether the switch is on",
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
                name: "on_change",
                prop_type: "Option<Callback<bool>>",
                default: None,
                description: "Change handler",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "Signal<bool>",
                default: Some("false"),
                description: "Whether the switch is disabled",
                required: false,
            },
        ],
        demo: || {
            let checked = RwSignal::new(false);
            view! {
                <DemoBlock title="Switch">
                    <Switch
                        checked=checked
                        label="Enable notifications"
                        on_change=Callback::new(move |v| checked.set(v))
                    />
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn textarea_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Textarea",
        import_name: "Textarea",
        description: "A multi-line text input component.",
        props: vec![
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text",
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
                name: "rows",
                prop_type: "Option<u32>",
                default: Some("4"),
                description: "Number of visible rows",
                required: false,
            },
            PropDoc {
                name: "on_input",
                prop_type: "Option<Callback<String>>",
                default: None,
                description: "Input handler",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Textarea">
                    <div style="max-width: 400px;">
                        <Textarea label="Message" placeholder="Enter your message..." rows=4 />
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn parameter_slider_doc() -> ComponentDoc {
    ComponentDoc {
        name: "ParameterSlider",
        import_name: "ParameterSlider, ParameterSliderSize, ParameterSliderScale",
        description: "A high-precision parameter slider with exact decimal values. Inspired by Mathematica's Manipulate controls.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Signal<String>",
                default: None,
                description: "Current value as a string for precision preservation",
                required: true,
            },
            PropDoc {
                name: "min",
                prop_type: "String",
                default: None,
                description: "Minimum value",
                required: true,
            },
            PropDoc {
                name: "max",
                prop_type: "String",
                default: None,
                description: "Maximum value",
                required: true,
            },
            PropDoc {
                name: "step",
                prop_type: "String",
                default: Some("\"1\""),
                description: "Step increment",
                required: false,
            },
            PropDoc {
                name: "scale",
                prop_type: "ParameterSliderScale",
                default: Some("Linear"),
                description: "Scale type (Linear or Logarithmic)",
                required: false,
            },
            PropDoc {
                name: "show_input",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to show an input field alongside the slider",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label displayed above the slider",
                required: false,
            },
        ],
        demo: || {
            let amplitude = RwSignal::new("5.0".to_string());
            let frequency = RwSignal::new("1.0".to_string());
            let log_value = RwSignal::new("100.0".to_string());

            view! {
                <DemoBlock title="Basic ParameterSlider">
                    <Stack spacing="lg">
                        <ParameterSlider
                            value=Signal::derive(move || amplitude.get())
                            min="0"
                            max="10"
                            step="0.1"
                            label="Amplitude"
                            show_input=true
                            on_change=Callback::new(move |v: String| amplitude.set(v))
                        />
                        <ParameterSlider
                            value=Signal::derive(move || frequency.get())
                            min="0.1"
                            max="10"
                            step="0.1"
                            label="Frequency"
                            show_input=true
                            on_change=Callback::new(move |v: String| frequency.set(v))
                        />
                        <ParameterSlider
                            value=Signal::derive(move || log_value.get())
                            min="1"
                            max="10000"
                            step="1"
                            scale=ParameterSliderScale::Logarithmic
                            label="Logarithmic Scale"
                            show_input=true
                            on_change=Callback::new(move |v: String| log_value.set(v))
                        />
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn parameter_grid_doc() -> ComponentDoc {
    ComponentDoc {
        name: "ParameterGrid",
        import_name: "ParameterGrid, ParameterDef, ParameterPreset",
        description: "A grid of parameter sliders for manipulating multiple values, inspired by Mathematica's Manipulate controls.",
        props: vec![
            PropDoc {
                name: "parameters",
                prop_type: "Signal<Vec<ParameterDef>>",
                default: None,
                description: "Parameter definitions",
                required: true,
            },
            PropDoc {
                name: "layout",
                prop_type: "ParameterGridLayout",
                default: Some("Vertical"),
                description: "Layout direction (Vertical, Horizontal, or Grid)",
                required: false,
            },
            PropDoc {
                name: "show_inputs",
                prop_type: "bool",
                default: Some("true"),
                description: "Whether to show input fields",
                required: false,
            },
            PropDoc {
                name: "show_reset",
                prop_type: "bool",
                default: Some("true"),
                description: "Whether to show reset button",
                required: false,
            },
            PropDoc {
                name: "presets",
                prop_type: "Option<Vec<ParameterPreset>>",
                default: None,
                description: "Available presets for quick configuration",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<HashMap<String, String>>>",
                default: None,
                description: "Callback when any parameter changes",
                required: false,
            },
        ],
        demo: || {
            let params = RwSignal::new(vec![
                ParameterDef::new("amplitude", "Amplitude")
                    .range("0", "10")
                    .step("0.1")
                    .default("5")
                    .display_precision(1),
                ParameterDef::new("frequency", "Frequency")
                    .range("0.1", "100")
                    .step("0.1")
                    .default("1")
                    .logarithmic()
                    .display_precision(2),
                ParameterDef::new("phase", "Phase")
                    .range("0", "6.28")
                    .step("0.01")
                    .default("0")
                    .display_precision(2),
            ]);

            view! {
                <DemoBlock title="ParameterGrid - Wave Parameters">
                    <ParameterGrid
                        parameters=Signal::derive(move || params.get())
                        show_inputs=true
                        show_reset=true
                    />
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn parameter_tree_doc() -> ComponentDoc {
    ComponentDoc {
        name: "ParameterTree",
        import_name: "ParameterTree, ParameterNode, ParameterValue",
        description: "A hierarchical parameter tree editor inspired by PyQtGraph's ParameterTree.",
        props: vec![
            PropDoc {
                name: "root",
                prop_type: "Signal<ParameterNode>",
                default: None,
                description: "Root parameter node",
                required: true,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<ParameterTreeSize>",
                default: Some("Md"),
                description: "Size variant",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<(String, String)>>",
                default: None,
                description: "Callback when a value changes (path, new_value)",
                required: false,
            },
            PropDoc {
                name: "on_action",
                prop_type: "Option<Callback<String>>",
                default: None,
                description: "Callback when an action button is clicked",
                required: false,
            },
        ],
        demo: || {
            let root = RwSignal::new(
                ParameterNode::group("root", "Configuration")
                    .with_child(
                        ParameterNode::group("display", "Display Settings")
                            .with_child(ParameterNode::bool("show_grid", "Show Grid", true))
                            .with_child(ParameterNode::color(
                                "background",
                                "Background Color",
                                "#1a1a2e",
                            ))
                            .with_child(ParameterNode::enumeration(
                                "theme",
                                "Theme",
                                "Dark",
                                vec!["Light".to_string(), "Dark".to_string(), "Auto".to_string()],
                            )),
                    )
                    .with_child(
                        ParameterNode::group("simulation", "Simulation")
                            .with_child(
                                ParameterNode::number("timestep", "Time Step", "0.001")
                                    .with_range("0.0001", "0.1")
                                    .with_step("0.0001"),
                            )
                            .with_child(
                                ParameterNode::number("iterations", "Iterations", "1000")
                                    .with_range("1", "10000")
                                    .with_step("1"),
                            )
                            .with_child(ParameterNode::bool("auto_run", "Auto Run", false)),
                    )
                    .with_child(
                        ParameterNode::group("actions", "Actions")
                            .with_child(ParameterNode::action("reset", "Reset", "Reset All"))
                            .with_child(ParameterNode::action("export", "Export", "Export Config")),
                    ),
            );

            view! {
                <DemoBlock title="ParameterTree - PyQtGraph Style">
                    <div style="max-width: 500px;">
                        <ParameterTree
                            root=Signal::derive(move || root.get())
                            on_change=Callback::new(move |(path, value): (String, String)| {
                                leptos::logging::log!("Changed: {} = {}", path, value);
                            })
                            on_action=Callback::new(move |path: String| {
                                leptos::logging::log!("Action: {}", path);
                            })
                        />
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

// ============================================================================
// Overlay Components
// ============================================================================

fn drawer_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Drawer",
        import_name: "Drawer, DrawerPosition",
        description: "A sliding panel that appears from the edge of the screen.",
        props: vec![
            PropDoc {
                name: "opened",
                prop_type: "Signal<bool>",
                default: None,
                description: "Whether the drawer is open",
                required: true,
            },
            PropDoc {
                name: "position",
                prop_type: "Option<DrawerPosition>",
                default: Some("Left"),
                description: "Position: Left, Right, Top, Bottom",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<String>",
                default: Some("\"300px\""),
                description: "Width (left/right) or height (top/bottom)",
                required: false,
            },
            PropDoc {
                name: "on_close",
                prop_type: "Option<Callback<()>>",
                default: None,
                description: "Called when drawer should close",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Drawer content",
                required: true,
            },
        ],
        demo: || {
            let opened = RwSignal::new(false);
            view! {
                <DemoBlock title="Drawer">
                    <Button on_click=Callback::new(move |_| opened.set(true))>
                        "Open Drawer"
                    </Button>
                    <Drawer opened=opened on_close=Callback::new(move |_| opened.set(false))>
                        <Stack spacing="md" style="padding: 1rem;">
                            <Text weight=TextWeight::Bold>"Drawer Content"</Text>
                            <Text>"This is the drawer panel."</Text>
                            <Button on_click=Callback::new(move |_| opened.set(false))>
                                "Close"
                            </Button>
                        </Stack>
                    </Drawer>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn loading_overlay_doc() -> ComponentDoc {
    ComponentDoc {
        name: "LoadingOverlay",
        import_name: "LoadingOverlay, LoaderVariant, LoaderSize",
        description: "An overlay with a loading indicator that covers its parent container. Parent should have position: relative.",
        props: vec![
            PropDoc {
                name: "visible",
                prop_type: "Signal<bool>",
                default: None,
                description: "Whether the overlay is visible",
                required: true,
            },
            PropDoc {
                name: "loader_variant",
                prop_type: "Option<LoaderVariant>",
                default: Some("Oval"),
                description: "Loader style: Oval, Dots, Bars",
                required: false,
            },
            PropDoc {
                name: "loader_size",
                prop_type: "Option<LoaderSize>",
                default: Some("Md"),
                description: "Loader size: Xs, Sm, Md, Lg, Xl",
                required: false,
            },
            PropDoc {
                name: "overlay_opacity",
                prop_type: "Option<f32>",
                default: Some("0.75"),
                description: "Opacity of the overlay background",
                required: false,
            },
        ],
        demo: || {
            let loading = RwSignal::new(false);
            view! {
                <DemoBlock title="Loading Overlay" code=r#"<div style="position: relative;">
    <LoadingOverlay visible=Signal::derive(move || loading.get()) />
    <p>"Content here"</p>
</div>"#>
                    <Button on_click=Callback::new(move |_| {
                        loading.set(true);
                        set_timeout(move || loading.set(false), std::time::Duration::from_secs(2));
                    })>
                        "Show Loading (2s)"
                    </Button>
                    <div style="position: relative; margin-top: 1rem; padding: 2rem; border: 1px solid #dee2e6; border-radius: 0.25rem; min-height: 100px;">
                        <LoadingOverlay visible=Signal::derive(move || loading.get()) />
                        <Text>"Content behind the overlay"</Text>
                    </div>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn modal_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Modal",
        import_name: "Modal",
        description: "A dialog overlay that appears above the page content.",
        props: vec![
            PropDoc {
                name: "opened",
                prop_type: "Signal<bool>",
                default: None,
                description: "Whether the modal is open",
                required: true,
            },
            PropDoc {
                name: "title",
                prop_type: "Option<String>",
                default: None,
                description: "Modal title",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<String>",
                default: Some("\"md\""),
                description: "Modal size: xs, sm, md, lg, xl",
                required: false,
            },
            PropDoc {
                name: "on_close",
                prop_type: "Option<Callback<()>>",
                default: None,
                description: "Called when modal should close",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Modal content",
                required: true,
            },
        ],
        demo: || {
            let opened = RwSignal::new(false);
            view! {
                <DemoBlock title="Modal">
                    <Button on_click=Callback::new(move |_| opened.set(true))>
                        "Open Modal"
                    </Button>
                    <Modal opened=opened title="Modal Title" on_close=Callback::new(move |_| opened.set(false))>
                        <Text>"This is the modal content."</Text>
                        <Group justify=GroupJustify::End style="margin-top: 1rem;">
                            <Button variant=ButtonVariant::Outline on_click=Callback::new(move |_| opened.set(false))>
                                "Cancel"
                            </Button>
                            <Button on_click=Callback::new(move |_| opened.set(false))>
                                "Confirm"
                            </Button>
                        </Group>
                    </Modal>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn popover_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Popover",
        import_name: "Popover, PopoverTarget, PopoverDropdown",
        description: "A floating panel that appears relative to a target element.",
        props: vec![
            PropDoc {
                name: "opened",
                prop_type: "Signal<bool>",
                default: None,
                description: "Whether the popover is open",
                required: false,
            },
            PropDoc {
                name: "position",
                prop_type: "Option<PopoverPosition>",
                default: Some("Bottom"),
                description: "Position relative to target",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "PopoverTarget and PopoverDropdown",
                required: true,
            },
        ],
        demo: || {
            let opened = RwSignal::new(false);
            view! {
                <DemoBlock title="Popover">
                    <Popover opened=opened>
                        <PopoverTarget>
                            <Button on_click=Callback::new(move |_| opened.update(|o| *o = !*o))>
                                "Toggle Popover"
                            </Button>
                        </PopoverTarget>
                        <PopoverDropdown>
                            <Text size=TextSize::Sm>"Popover content"</Text>
                        </PopoverDropdown>
                    </Popover>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn tooltip_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Tooltip",
        import_name: "Tooltip",
        description: "A small popup that appears on hover to provide additional information.",
        props: vec![
            PropDoc {
                name: "label",
                prop_type: "String",
                default: None,
                description: "Tooltip text content",
                required: true,
            },
            PropDoc {
                name: "position",
                prop_type: "Option<TooltipPosition>",
                default: Some("Top"),
                description: "Position: Top, Bottom, Left, Right",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Target element",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Tooltip">
                    <Group spacing="lg">
                        <Tooltip label="This is a tooltip">
                            <Button>"Hover me"</Button>
                        </Tooltip>
                    </Group>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

// ============================================================================
// Feedback Components
// ============================================================================

fn alert_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Alert",
        import_name: "Alert, AlertVariant, AlertColor",
        description: "A component to display important messages to users.",
        props: vec![
            PropDoc {
                name: "variant",
                prop_type: "Option<AlertVariant>",
                default: Some("Light"),
                description: "Visual style: Filled, Light, Outline",
                required: false,
            },
            PropDoc {
                name: "color",
                prop_type: "Option<AlertColor>",
                default: Some("Info"),
                description:
                    "Color theme: Info (blue), Success (green), Warning (yellow), Error (red)",
                required: false,
            },
            PropDoc {
                name: "title",
                prop_type: "Option<String>",
                default: None,
                description: "Alert title",
                required: false,
            },
            PropDoc {
                name: "with_close_button",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether alert can be dismissed",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Alert content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Alert Colors" code=r#"<Alert color=AlertColor::Info title="Information">
    "This is an informational message."
</Alert>
<Alert color=AlertColor::Success title="Success">
    "Operation completed successfully."
</Alert>"#>
                    <Stack spacing="md">
                        <Alert color=AlertColor::Info title="Information">
                            "This is an informational message."
                        </Alert>
                        <Alert color=AlertColor::Success title="Success">
                            "Operation completed successfully."
                        </Alert>
                        <Alert color=AlertColor::Warning title="Warning">
                            "Please review before proceeding."
                        </Alert>
                        <Alert color=AlertColor::Error title="Error">
                            "Something went wrong."
                        </Alert>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn loader_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Loader",
        import_name: "Loader, LoaderVariant, LoaderSize",
        description: "A loading spinner indicator.",
        props: vec![
            PropDoc {
                name: "variant",
                prop_type: "Option<LoaderVariant>",
                default: Some("Oval"),
                description: "Style: Oval, Dots, or Bars",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<LoaderSize>",
                default: Some("Md"),
                description: "Size: Xs, Sm, Md, Lg, Xl",
                required: false,
            },
            PropDoc {
                name: "color",
                prop_type: "Option<String>",
                default: Some("\"blue\""),
                description: "Theme color",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Loader Variants">
                    <Group spacing="xl">
                        <Loader variant=LoaderVariant::Oval />
                        <Loader variant=LoaderVariant::Dots />
                        <Loader variant=LoaderVariant::Bars />
                    </Group>
                </DemoBlock>
                <DemoBlock title="Loader Sizes">
                    <Group spacing="xl" align=GroupAlign::Center>
                        <Loader size=LoaderSize::Xs />
                        <Loader size=LoaderSize::Sm />
                        <Loader size=LoaderSize::Md />
                        <Loader size=LoaderSize::Lg />
                        <Loader size=LoaderSize::Xl />
                    </Group>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn notification_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Notification",
        import_name: "NotificationProvider, NotificationData, NotificationColor, use_notifications, show_notification",
        description: "Toast notifications system with provider pattern.",
        props: vec![
            PropDoc {
                name: "position",
                prop_type: "Option<NotificationPosition>",
                default: Some("TopRight"),
                description: "Position: TopLeft, TopRight, TopCenter, BottomLeft, BottomRight, BottomCenter",
                required: false,
            },
            PropDoc {
                name: "max_notifications",
                prop_type: "Option<usize>",
                default: Some("5"),
                description: "Maximum notifications shown at once",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Notifications" code=r#"// Wrap app with NotificationProvider
<NotificationProvider>
    <App />
</NotificationProvider>

// In components, use the hook
let show = use_notifications();
show(show_notification("Message", NotificationColor::Success, Some("Title".into())));"#>
                    <Stack spacing="md">
                        <Text size=TextSize::Sm color="dimmed">
                            "Notifications use a provider pattern. Wrap your app with NotificationProvider, then use the use_notifications() hook."
                        </Text>
                        <Card>
                            <Text weight=TextWeight::Bold>"Example Notification Structure"</Text>
                            <Stack spacing="xs" style="margin-top: 0.5rem;">
                                <Text size=TextSize::Sm>"• Info (blue) - Information messages"</Text>
                                <Text size=TextSize::Sm>"• Success (green) - Confirmations"</Text>
                                <Text size=TextSize::Sm>"• Warning (yellow) - Cautions"</Text>
                                <Text size=TextSize::Sm>"• Error (red) - Errors"</Text>
                            </Stack>
                        </Card>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn progress_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Progress",
        import_name: "Progress, ProgressSize",
        description: "A progress bar component.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Signal<f32>",
                default: None,
                description: "Current progress value (0-100)",
                required: true,
            },
            PropDoc {
                name: "color",
                prop_type: "Option<String>",
                default: Some("\"blue\""),
                description: "Theme color",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<ProgressSize>",
                default: Some("Md"),
                description: "Height: Xs, Sm, Md, Lg, Xl",
                required: false,
            },
            PropDoc {
                name: "striped",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to show stripes",
                required: false,
            },
            PropDoc {
                name: "animate",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to animate stripes",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Progress Bar" code=r#"<Progress value=Signal::derive(|| 50.0) />
<Progress value=Signal::derive(|| 75.0) striped=true animate=true />"#>
                    <Stack spacing="md" style="width: 100%;">
                        <Progress value=Signal::derive(|| 25.0) />
                        <Progress value=Signal::derive(|| 50.0) color="green" />
                        <Progress value=Signal::derive(|| 75.0) color="orange" striped=true />
                        <Progress value=Signal::derive(|| 100.0) color="red" striped=true animate=true />
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn skeleton_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Skeleton",
        import_name: "Skeleton, SkeletonText",
        description: "A placeholder loading component that mimics content layout.",
        props: vec![
            PropDoc {
                name: "height",
                prop_type: "Option<String>",
                default: Some("\"1rem\""),
                description: "Height of the skeleton",
                required: false,
            },
            PropDoc {
                name: "width",
                prop_type: "Option<String>",
                default: Some("\"100%\""),
                description: "Width of the skeleton",
                required: false,
            },
            PropDoc {
                name: "circle",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to render as a circle",
                required: false,
            },
            PropDoc {
                name: "animate",
                prop_type: "bool",
                default: Some("true"),
                description: "Whether to animate the shimmer effect",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Skeleton Loading">
                    <Stack spacing="md">
                        <Group spacing="md">
                            <Skeleton height="50px" width="50px" circle=true />
                            <Stack spacing="xs" style="flex: 1;">
                                <Skeleton height="1rem" width="40%" />
                                <Skeleton height="0.75rem" width="70%" />
                            </Stack>
                        </Group>
                        <SkeletonText lines=3 />
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

// ============================================================================
// Data Display Components
// ============================================================================

fn accordion_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Accordion",
        import_name: "Accordion, AccordionItem, AccordionVariant",
        description: "A collapsible content panel component.",
        props: vec![
            PropDoc {
                name: "variant",
                prop_type: "Option<AccordionVariant>",
                default: Some("Default"),
                description: "Visual style: Default, Contained, Separated",
                required: false,
            },
            PropDoc {
                name: "multiple",
                prop_type: "bool",
                default: Some("false"),
                description: "Allow multiple panels to be open",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "AccordionItem components",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Accordion" code=r#"<Accordion>
    <AccordionItem _value="first" label="First Section">
        <Text>"Content here"</Text>
    </AccordionItem>
</Accordion>"#>
                    <Accordion>
                        <AccordionItem _value="first" label="First Section">
                            <Text>"Content of the first section."</Text>
                        </AccordionItem>
                        <AccordionItem _value="second" label="Second Section">
                            <Text>"Content of the second section."</Text>
                        </AccordionItem>
                        <AccordionItem _value="third" label="Third Section">
                            <Text>"Content of the third section."</Text>
                        </AccordionItem>
                    </Accordion>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn avatar_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Avatar",
        import_name: "Avatar, AvatarSize, AvatarRadius, AvatarGroup",
        description: "A component for displaying user avatars with image or initials.",
        props: vec![
            PropDoc {
                name: "src",
                prop_type: "Option<String>",
                default: None,
                description: "Image URL",
                required: false,
            },
            PropDoc {
                name: "alt",
                prop_type: "Option<String>",
                default: None,
                description: "Alt text for image",
                required: false,
            },
            PropDoc {
                name: "initials",
                prop_type: "Option<String>",
                default: None,
                description: "Initials to display when no image",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<AvatarSize>",
                default: Some("Md"),
                description: "Size: Xs, Sm, Md, Lg, Xl",
                required: false,
            },
            PropDoc {
                name: "radius",
                prop_type: "Option<AvatarRadius>",
                default: Some("Full"),
                description: "Border radius: Xs, Sm, Md, Lg, Xl, Full",
                required: false,
            },
            PropDoc {
                name: "color",
                prop_type: "Option<String>",
                default: Some("\"blue\""),
                description: "Background color for placeholder",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Avatar" code=r#"<Avatar initials="JD" color="blue" />
            <Avatar src="/image.jpg" alt="User" />"#>
                    <Group spacing="md">
                        <Avatar initials="JD" color="blue" />
                        <Avatar initials="AB" color="red" />
                        <Avatar initials="CD" color="green" />
                        <Avatar size=AvatarSize::Lg initials="EF" color="orange" />
                    </Group>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn badge_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Badge",
        import_name: "Badge, BadgeVariant",
        description: "A small label component for displaying status or counts.",
        props: vec![
            PropDoc {
                name: "variant",
                prop_type: "Option<BadgeVariant>",
                default: Some("Filled"),
                description: "Visual style: Filled, Light, Outline, Dot",
                required: false,
            },
            PropDoc {
                name: "color",
                prop_type: "Option<String>",
                default: Some("\"blue\""),
                description: "Theme color",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<String>",
                default: Some("\"md\""),
                description: "Size: xs, sm, md, lg, xl",
                required: false,
            },
            PropDoc {
                name: "children",
                prop_type: "Children",
                default: None,
                description: "Badge content",
                required: true,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Badge Variants">
                    <Group spacing="md">
                        <Badge variant=BadgeVariant::Filled>"Filled"</Badge>
                        <Badge variant=BadgeVariant::Light>"Light"</Badge>
                        <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                        <Badge variant=BadgeVariant::Dot>"With Dot"</Badge>
                    </Group>
                </DemoBlock>
                <DemoBlock title="Badge Colors">
                    <Group spacing="md">
                        <Badge color="blue">"Blue"</Badge>
                        <Badge color="red">"Red"</Badge>
                        <Badge color="green">"Green"</Badge>
                        <Badge color="orange">"Orange"</Badge>
                    </Group>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn ring_progress_doc() -> ComponentDoc {
    ComponentDoc {
        name: "RingProgress",
        import_name: "RingProgress, RingProgressSection",
        description: "A circular progress indicator with support for multiple sections.",
        props: vec![
            PropDoc {
                name: "sections",
                prop_type: "Vec<RingProgressSection>",
                default: None,
                description: "Progress sections with value and color",
                required: true,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<RingProgressSize>",
                default: Some("Md"),
                description: "Size: Xs, Sm, Md, Lg, Xl",
                required: false,
            },
            PropDoc {
                name: "thickness",
                prop_type: "Option<f64>",
                default: Some("12"),
                description: "Thickness of the ring in pixels",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<Children>",
                default: None,
                description: "Content displayed in the center",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Ring Progress">
                    <Group spacing="xl">
                        <RingProgress
                            sections=vec![
                                RingProgressSection::new(40.0, "blue"),
                            ]
                        />
                        <RingProgress
                            sections=vec![
                                RingProgressSection::new(30.0, "blue"),
                                RingProgressSection::new(25.0, "green"),
                                RingProgressSection::new(20.0, "orange"),
                            ]
                        />
                    </Group>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn stats_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Stats",
        import_name: "Stats, StatsGroup",
        description: "A component for displaying statistics with labels and values.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "String",
                default: None,
                description: "The statistic value to display",
                required: true,
            },
            PropDoc {
                name: "label",
                prop_type: "String",
                default: None,
                description: "Label/title for the statistic",
                required: true,
            },
            PropDoc {
                name: "icon",
                prop_type: "Option<String>",
                default: None,
                description: "Icon to display",
                required: false,
            },
            PropDoc {
                name: "description",
                prop_type: "Option<String>",
                default: None,
                description: "Additional description",
                required: false,
            },
            PropDoc {
                name: "diff",
                prop_type: "Option<f32>",
                default: None,
                description: "Percentage difference (positive=green, negative=red)",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Stats" code=r#"<StatsGroup cols=3>
    <Stats value="1,234" label="Total Users" />
    <Stats value="$45,678" label="Revenue" diff=12.5 />
</StatsGroup>"#>
                    <StatsGroup cols=3>
                        <Stats value="1,234" label="Total Users" />
                        <Stats value="$45,678" label="Revenue" diff=12.5 />
                        <Stats value="89%" label="Conversion" diff=-2.3 />
                    </StatsGroup>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn table_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Table",
        import_name: "Table, TableColumn",
        description: "A data table component with sorting and styling options.",
        props: vec![
            PropDoc {
                name: "columns",
                prop_type: "Vec<TableColumn>",
                default: None,
                description: "Column definitions",
                required: true,
            },
            PropDoc {
                name: "data",
                prop_type: "Vec<T>",
                default: None,
                description: "Row data",
                required: true,
            },
            PropDoc {
                name: "striped",
                prop_type: "bool",
                default: Some("false"),
                description: "Alternate row colors",
                required: false,
            },
            PropDoc {
                name: "hoverable",
                prop_type: "bool",
                default: Some("false"),
                description: "Highlight rows on hover",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Table">
                    <table style="width: 100%; border-collapse: collapse;">
                        <thead>
                            <tr style="border-bottom: 1px solid #dee2e6;">
                                <th style="padding: 0.75rem; text-align: left;">"Name"</th>
                                <th style="padding: 0.75rem; text-align: left;">"Email"</th>
                                <th style="padding: 0.75rem; text-align: left;">"Role"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr style="border-bottom: 1px solid #dee2e6;">
                                <td style="padding: 0.75rem;">"John Doe"</td>
                                <td style="padding: 0.75rem;">"john@example.com"</td>
                                <td style="padding: 0.75rem;">"Admin"</td>
                            </tr>
                            <tr style="border-bottom: 1px solid #dee2e6;">
                                <td style="padding: 0.75rem;">"Jane Smith"</td>
                                <td style="padding: 0.75rem;">"jane@example.com"</td>
                                <td style="padding: 0.75rem;">"User"</td>
                            </tr>
                        </tbody>
                    </table>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

// ============================================================================
// Miscellaneous Components
// ============================================================================

fn error_page_doc() -> ComponentDoc {
    ComponentDoc {
        name: "ErrorPage",
        import_name: "ErrorPage, ErrorPageType",
        description: "A full-page error display component for common HTTP errors.",
        props: vec![
            PropDoc {
                name: "error_type",
                prop_type: "ErrorPageType",
                default: None,
                description: "Error type: NotFound, Unauthorized, Forbidden, InternalError, ServiceUnavailable, Custom",
                required: true,
            },
            PropDoc {
                name: "title",
                prop_type: "Option<String>",
                default: None,
                description: "Custom title (for Custom error type)",
                required: false,
            },
            PropDoc {
                name: "description",
                prop_type: "Option<String>",
                default: None,
                description: "Custom description",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Error Pages">
                    <Stack spacing="md">
                        <Card>
                            <Text weight=TextWeight::Bold>"404 - Not Found"</Text>
                            <Text size=TextSize::Sm color="dimmed">"The page you are looking for does not exist."</Text>
                        </Card>
                        <Card>
                            <Text weight=TextWeight::Bold>"500 - Internal Server Error"</Text>
                            <Text size=TextSize::Sm color="dimmed">"Something went wrong on our end."</Text>
                        </Card>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

// ============================================================================
// New Form Components (v0.5.0)
// ============================================================================

fn slider_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Slider",
        import_name: "Slider, SliderSize, SliderMark",
        description: "A slider component for selecting numeric values within a range.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Signal<f64>",
                default: None,
                description: "Current value of the slider",
                required: true,
            },
            PropDoc {
                name: "min",
                prop_type: "f64",
                default: Some("0.0"),
                description: "Minimum value",
                required: false,
            },
            PropDoc {
                name: "max",
                prop_type: "f64",
                default: Some("100.0"),
                description: "Maximum value",
                required: false,
            },
            PropDoc {
                name: "step",
                prop_type: "f64",
                default: Some("1.0"),
                description: "Step increment (0.0 for continuous)",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<SliderSize>",
                default: Some("Md"),
                description: "Size of the slider: Xs, Sm, Md, Lg, Xl",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label displayed above the slider",
                required: false,
            },
            PropDoc {
                name: "show_value",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to show the current value",
                required: false,
            },
            PropDoc {
                name: "marks",
                prop_type: "Option<Vec<SliderMark>>",
                default: None,
                description: "Marks to display on the track",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<f64>>",
                default: None,
                description: "Callback when value changes",
                required: false,
            },
        ],
        demo: || {
            let slider_value = RwSignal::new(50.0);
            let marks_value = RwSignal::new(25.0);

            view! {
                <DemoBlock title="Slider">
                    <Stack spacing="lg">
                        <div>
                            <Slider
                                value=slider_value
                                min=0.0
                                max=100.0
                                label="Volume"
                                show_value=true
                                on_change=Callback::new(move |v| slider_value.set(v))
                            />
                        </div>
                        <div>
                            <Slider
                                value=marks_value
                                min=0.0
                                max=100.0
                                label="With marks"
                                show_value=true
                                marks=vec![
                                    SliderMark::with_label(0.0, "0%"),
                                    SliderMark::with_label(50.0, "50%"),
                                    SliderMark::with_label(100.0, "100%"),
                                ]
                                on_change=Callback::new(move |v| marks_value.set(v))
                            />
                        </div>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn range_slider_doc() -> ComponentDoc {
    ComponentDoc {
        name: "RangeSlider",
        import_name: "RangeSlider, SliderSize",
        description: "A dual-handle slider for selecting a range of values.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Signal<(f64, f64)>",
                default: None,
                description: "Current range value (min, max)",
                required: true,
            },
            PropDoc {
                name: "min",
                prop_type: "f64",
                default: Some("0.0"),
                description: "Minimum allowed value",
                required: false,
            },
            PropDoc {
                name: "max",
                prop_type: "f64",
                default: Some("100.0"),
                description: "Maximum allowed value",
                required: false,
            },
            PropDoc {
                name: "min_range",
                prop_type: "f64",
                default: Some("0.0"),
                description: "Minimum gap between handles",
                required: false,
            },
            PropDoc {
                name: "step",
                prop_type: "f64",
                default: Some("1.0"),
                description: "Step increment",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label displayed above the slider",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<(f64, f64)>>",
                default: None,
                description: "Callback when range changes",
                required: false,
            },
        ],
        demo: || {
            let range_value = RwSignal::new((25.0, 75.0));

            view! {
                <DemoBlock title="Range Slider">
                    <Stack spacing="lg">
                        <div>
                            <RangeSlider
                                value=range_value
                                min=0.0
                                max=100.0
                                label="Price Range"
                                show_value=true
                                on_change=Callback::new(move |v| range_value.set(v))
                            />
                        </div>
                        <Text size=TextSize::Sm color="dimmed">
                            {move || format!("Selected: ${} - ${}", range_value.get().0 as i32, range_value.get().1 as i32)}
                        </Text>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn segmented_control_doc() -> ComponentDoc {
    ComponentDoc {
        name: "SegmentedControl",
        import_name: "SegmentedControl, SegmentedControlItem, SegmentedControlSize",
        description: "A toggle button group for single selection, like styled radio buttons.",
        props: vec![
            PropDoc {
                name: "data",
                prop_type: "Vec<SegmentedControlItem>",
                default: None,
                description: "Items to display (value, label, disabled)",
                required: true,
            },
            PropDoc {
                name: "value",
                prop_type: "Signal<String>",
                default: None,
                description: "Currently selected value",
                required: true,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<SegmentedControlSize>",
                default: Some("Md"),
                description: "Size: Xs, Sm, Md, Lg, Xl",
                required: false,
            },
            PropDoc {
                name: "full_width",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to take full container width",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<String>>",
                default: None,
                description: "Callback when selection changes",
                required: false,
            },
        ],
        demo: || {
            let selected = RwSignal::new("react".to_string());

            view! {
                <DemoBlock title="Segmented Control">
                    <Stack spacing="lg">
                        <SegmentedControl
                            data=vec![
                                SegmentedControlItem::new("react", "React"),
                                SegmentedControlItem::new("vue", "Vue"),
                                SegmentedControlItem::new("svelte", "Svelte"),
                                SegmentedControlItem::new("angular", "Angular"),
                            ]
                            value=selected
                            on_change=Callback::new(move |v| selected.set(v))
                        />
                        <Text size=TextSize::Sm color="dimmed">
                            {move || format!("Selected: {}", selected.get())}
                        </Text>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn file_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "FileInput",
        import_name: "FileInput, FileInfo",
        description: "A file input component for uploading files with a styled interface.",
        props: vec![
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label displayed above the input",
                required: false,
            },
            PropDoc {
                name: "placeholder",
                prop_type: "Option<String>",
                default: Some("\"Choose file...\""),
                description: "Placeholder text when no file is selected",
                required: false,
            },
            PropDoc {
                name: "accept",
                prop_type: "Option<String>",
                default: None,
                description: "Accepted file types (e.g., \".pdf,.doc\" or \"image/*\")",
                required: false,
            },
            PropDoc {
                name: "multiple",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether multiple files can be selected",
                required: false,
            },
            PropDoc {
                name: "clearable",
                prop_type: "bool",
                default: Some("true"),
                description: "Whether to show a clear button",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<Vec<FileInfo>>>",
                default: None,
                description: "Callback when files are selected",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="File Input">
                    <Stack spacing="lg">
                        <FileInput
                            label="Upload document"
                            accept=".pdf,.doc,.docx"
                            placeholder="Choose a document..."
                        />
                        <FileInput
                            label="Upload images"
                            accept="image/*"
                            multiple=true
                            placeholder="Choose images..."
                        />
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn pin_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "PinInput",
        import_name: "PinInput, PinInputType",
        description: "A PIN input component for entering verification codes, OTPs, etc.",
        props: vec![
            PropDoc {
                name: "length",
                prop_type: "usize",
                default: Some("4"),
                description: "Number of input fields",
                required: false,
            },
            PropDoc {
                name: "value",
                prop_type: "Signal<String>",
                default: None,
                description: "Current value (concatenated)",
                required: true,
            },
            PropDoc {
                name: "input_type",
                prop_type: "Option<PinInputType>",
                default: Some("Number"),
                description: "Type of allowed characters: Number, Alphanumeric, Text",
                required: false,
            },
            PropDoc {
                name: "mask",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to mask input like a password",
                required: false,
            },
            PropDoc {
                name: "placeholder",
                prop_type: "Option<String>",
                default: Some("\"○\""),
                description: "Placeholder character for empty fields",
                required: false,
            },
            PropDoc {
                name: "on_complete",
                prop_type: "Option<Callback<String>>",
                default: None,
                description: "Callback when all fields are filled",
                required: false,
            },
        ],
        demo: || {
            let pin_value = RwSignal::new(String::new());

            view! {
                <DemoBlock title="PIN Input">
                    <Stack spacing="lg">
                        <div>
                            <Text size=TextSize::Sm weight=TextWeight::Medium>"Enter verification code"</Text>
                            <PinInput
                                length=6
                                value=pin_value
                                on_complete=Callback::new(move |code: String| {
                                    web_sys::console::log_1(&format!("Code entered: {}", code).into());
                                })
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm weight=TextWeight::Medium>"Masked PIN"</Text>
                            <PinInput
                                length=4
                                value=Signal::derive(|| String::new())
                                mask=true
                            />
                        </div>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn pagination_doc() -> ComponentDoc {
    ComponentDoc {
        name: "Pagination",
        import_name: "Pagination, PaginationSize",
        description: "A pagination component for navigating through pages of content.",
        props: vec![
            PropDoc {
                name: "total",
                prop_type: "usize",
                default: None,
                description: "Total number of pages",
                required: true,
            },
            PropDoc {
                name: "value",
                prop_type: "Signal<usize>",
                default: None,
                description: "Current page (1-indexed)",
                required: true,
            },
            PropDoc {
                name: "siblings",
                prop_type: "usize",
                default: Some("1"),
                description: "Number of siblings on each side of current page",
                required: false,
            },
            PropDoc {
                name: "boundaries",
                prop_type: "usize",
                default: Some("1"),
                description: "Number of elements at the start and end",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "Option<PaginationSize>",
                default: Some("Md"),
                description: "Size of the pagination buttons: Xs, Sm, Md, Lg, Xl",
                required: false,
            },
            PropDoc {
                name: "with_edges",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to show first/last page buttons",
                required: false,
            },
            PropDoc {
                name: "with_controls",
                prop_type: "bool",
                default: Some("true"),
                description: "Whether to show previous/next buttons",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<usize>>",
                default: None,
                description: "Callback when page changes",
                required: false,
            },
        ],
        demo: || {
            let current_page = RwSignal::new(1_usize);
            let edge_page = RwSignal::new(10_usize);

            view! {
                <DemoBlock title="Pagination">
                    <Stack spacing="lg">
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Basic pagination"</Text>
                            <Pagination
                                total=10
                                value=current_page
                                on_change=Callback::new(move |p| current_page.set(p))
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"With edge buttons"</Text>
                            <Pagination
                                total=20
                                value=edge_page
                                with_edges=true
                                siblings=2
                                on_change=Callback::new(move |p| edge_page.set(p))
                            />
                        </div>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn angle_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "AngleInput",
        import_name: "AngleInput, AngleUnit, AngleNormalization, DMS",
        description: "A specialized input for angle values with support for degrees, radians, gradians, turns, and DMS (degrees-minutes-seconds) formats.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<Signal<f64>>",
                default: None,
                description: "Current angle value in degrees (controlled)",
                required: false,
            },
            PropDoc {
                name: "default_value",
                prop_type: "f64",
                default: Some("0.0"),
                description: "Default angle value in degrees",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<f64>>",
                default: None,
                description: "Callback when angle value changes",
                required: false,
            },
            PropDoc {
                name: "unit",
                prop_type: "AngleUnit",
                default: Some("Degrees"),
                description: "Display unit: Degrees, Radians, Gradians, Turns, or DMS",
                required: false,
            },
            PropDoc {
                name: "normalization",
                prop_type: "AngleNormalization",
                default: Some("None"),
                description: "Angle normalization: None, ZeroTo360, or NegativeTo180",
                required: false,
            },
            PropDoc {
                name: "show_unit_selector",
                prop_type: "bool",
                default: Some("true"),
                description: "Show dropdown to switch between units",
                required: false,
            },
            PropDoc {
                name: "precision",
                prop_type: "u32",
                default: Some("4"),
                description: "Number of decimal places to display",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text above the input",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether the input is disabled",
                required: false,
            },
        ],
        demo: || {
            let angle1 = RwSignal::new(45.0_f64);
            let angle2 = RwSignal::new(std::f64::consts::PI);
            let angle3 = RwSignal::new(45.5_f64);
            let angle4 = RwSignal::new(450.0_f64);
            view! {
                <DemoBlock title="Angle Input">
                    <Stack spacing="lg">
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Degrees (default)"</Text>
                            <AngleInput
                                value=angle1
                                on_change=Callback::new(move |v| angle1.set(v))
                                label="Rotation Angle".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Radians display"</Text>
                            <AngleInput
                                value=angle2
                                unit=AngleUnit::Radians
                                label="Phase Angle".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"DMS format"</Text>
                            <AngleInput
                                value=angle3
                                unit=AngleUnit::DMS
                                label="Geographic Coordinate".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"With normalization (0-360)"</Text>
                            <AngleInput
                                value=angle4
                                normalization=AngleNormalization::ZeroTo360
                                label="Compass Heading".to_string()
                            />
                        </div>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn fraction_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "FractionInput",
        import_name: "FractionInput, Fraction, FractionDisplayFormat",
        description: "A specialized input for exact rational numbers with automatic simplification and multiple display formats.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<Signal<Fraction>>",
                default: None,
                description: "Current fraction value (controlled)",
                required: false,
            },
            PropDoc {
                name: "default_value",
                prop_type: "Fraction",
                default: Some("Fraction::new(0, 1)"),
                description: "Default fraction value",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<Fraction>>",
                default: None,
                description: "Callback when fraction value changes",
                required: false,
            },
            PropDoc {
                name: "format",
                prop_type: "FractionDisplayFormat",
                default: Some("Fraction"),
                description: "Display format: Fraction (3/4), MixedNumber (1 1/2), or Decimal (0.75)",
                required: false,
            },
            PropDoc {
                name: "allow_format_switch",
                prop_type: "bool",
                default: Some("true"),
                description: "Show buttons to switch between display formats",
                required: false,
            },
            PropDoc {
                name: "auto_simplify",
                prop_type: "bool",
                default: Some("true"),
                description: "Automatically simplify fractions (e.g., 2/4 → 1/2)",
                required: false,
            },
            PropDoc {
                name: "decimal_places",
                prop_type: "u32",
                default: Some("4"),
                description: "Decimal precision for decimal format display",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text above the input",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether the input is disabled",
                required: false,
            },
        ],
        demo: || {
            let frac1 = RwSignal::new(Fraction::new(3, 4));
            let frac2 = RwSignal::new(Fraction::new(7, 4));
            let frac3 = RwSignal::new(Fraction::new(6, 8));
            view! {
                <DemoBlock title="Fraction Input">
                    <Stack spacing="lg">
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Simple fraction"</Text>
                            <FractionInput
                                value=frac1
                                label="Fraction".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Mixed number display"</Text>
                            <FractionInput
                                value=frac2
                                display_format=FractionDisplayFormat::MixedNumber
                                label="Mixed Number".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Auto-simplification"</Text>
                            <FractionInput
                                value=frac3
                                label="Enter 6/8 to see simplification".to_string()
                            />
                        </div>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn unit_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "UnitInput",
        import_name: "UnitInput, Unit, UnitValue, UnitCategory, length, mass, time, temperature, data",
        description: "A specialized input for values with physical units, supporting automatic unit conversion.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<RwSignal<UnitValue>>",
                default: None,
                description: "Current value with unit (controlled)",
                required: false,
            },
            PropDoc {
                name: "units",
                prop_type: "Vec<Unit>",
                default: None,
                description: "Available units for this input",
                required: true,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<UnitValue>>",
                default: None,
                description: "Callback when value or unit changes",
                required: false,
            },
            PropDoc {
                name: "allow_unit_switch",
                prop_type: "bool",
                default: Some("true"),
                description: "Allow switching between units via dropdown",
                required: false,
            },
            PropDoc {
                name: "precision",
                prop_type: "u32",
                default: Some("4"),
                description: "Number of decimal places to display",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text above the input",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether the input is disabled",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Unit Input">
                    <Stack spacing="lg">
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Length units"</Text>
                            <UnitInput
                                units=length::all()
                                label="Distance".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Mass units"</Text>
                            <UnitInput
                                units=mass::all()
                                label="Weight".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Temperature (with offset conversion)"</Text>
                            <UnitInput
                                units=temperature::all()
                                label="Temperature".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Data size"</Text>
                            <UnitInput
                                units=data::all()
                                label="File Size".to_string()
                            />
                        </div>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn complex_number_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "ComplexNumberInput",
        import_name: "ComplexNumberInput, ComplexNumber, ComplexFormat, PolarAngleUnit",
        description: "A specialized input for complex numbers with support for rectangular (a+bi), polar (r∠θ), and exponential forms.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<Signal<ComplexNumber>>",
                default: None,
                description: "Current complex number value (controlled)",
                required: false,
            },
            PropDoc {
                name: "default_value",
                prop_type: "ComplexNumber",
                default: Some("ComplexNumber::default()"),
                description: "Default complex number value",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<ComplexNumber>>",
                default: None,
                description: "Callback when complex number changes",
                required: false,
            },
            PropDoc {
                name: "format",
                prop_type: "ComplexFormat",
                default: Some("Rectangular"),
                description: "Display format: Rectangular (a+bi), Polar (r∠θ), or Exponential",
                required: false,
            },
            PropDoc {
                name: "angle_unit",
                prop_type: "PolarAngleUnit",
                default: Some("Degrees"),
                description: "Angle unit for polar form: Degrees or Radians",
                required: false,
            },
            PropDoc {
                name: "allow_format_switch",
                prop_type: "bool",
                default: Some("true"),
                description: "Show buttons to switch between formats",
                required: false,
            },
            PropDoc {
                name: "decimal_places",
                prop_type: "u32",
                default: Some("4"),
                description: "Number of decimal places for display",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text above the input",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether the input is disabled",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Complex Number Input">
                    <Stack spacing="lg">
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Rectangular form (a + bi)"</Text>
                            <ComplexNumberInput
                                default_value=ComplexNumber::new(3.0, 4.0)
                                label="Complex Z".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Polar form (r∠θ)"</Text>
                            <ComplexNumberInput
                                default_value=ComplexNumber::from_polar(5.0, std::f64::consts::FRAC_PI_4)
                                format=ComplexFormat::Polar
                                label="Phasor".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Pure imaginary"</Text>
                            <ComplexNumberInput
                                default_value=ComplexNumber::new(0.0, 1.0)
                                label="Unit Imaginary".to_string()
                            />
                        </div>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn uncertainty_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "UncertaintyInput",
        import_name: "UncertaintyInput, UncertainValue, UncertaintyFormat, UncertaintyType",
        description: "A specialized input for values with measurement uncertainty, supporting symmetric (±) and asymmetric (+/-) uncertainty bounds.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<Signal<UncertainValue>>",
                default: None,
                description: "Current uncertain value (controlled)",
                required: false,
            },
            PropDoc {
                name: "default_value",
                prop_type: "UncertainValue",
                default: Some("UncertainValue::default()"),
                description: "Default uncertain value",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<UncertainValue>>",
                default: None,
                description: "Callback when value changes",
                required: false,
            },
            PropDoc {
                name: "format",
                prop_type: "UncertaintyFormat",
                default: Some("Absolute"),
                description: "Display format: Absolute (value ± x), Relative (± x%), or Scientific",
                required: false,
            },
            PropDoc {
                name: "uncertainty_type",
                prop_type: "UncertaintyType",
                default: Some("Symmetric"),
                description: "Uncertainty type: Symmetric (±) or Asymmetric (+/-)",
                required: false,
            },
            PropDoc {
                name: "allow_type_switch",
                prop_type: "bool",
                default: Some("true"),
                description: "Allow switching between symmetric and asymmetric",
                required: false,
            },
            PropDoc {
                name: "decimal_places",
                prop_type: "u32",
                default: Some("4"),
                description: "Number of decimal places for display",
                required: false,
            },
            PropDoc {
                name: "show_info",
                prop_type: "bool",
                default: Some("true"),
                description: "Show additional info (bounds, relative uncertainty)",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text above the input",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether the input is disabled",
                required: false,
            },
        ],
        demo: || {
            view! {
                <DemoBlock title="Uncertainty Input">
                    <Stack spacing="lg">
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Symmetric uncertainty (±)"</Text>
                            <UncertaintyInput
                                default_value=UncertainValue::symmetric(100.0, 5.0)
                                label="Measurement".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Asymmetric uncertainty (+/-)"</Text>
                            <UncertaintyInput
                                default_value=UncertainValue::asymmetric(50.0, 3.0, 2.0)
                                uncertainty_type=UncertaintyType::Asymmetric
                                label="Asymmetric Error".to_string()
                            />
                        </div>
                        <div>
                            <Text size=TextSize::Sm color="dimmed">"Percentage display"</Text>
                            <UncertaintyInput
                                default_value=UncertainValue::from_percentage(1000.0, 2.5)
                                format=UncertaintyFormat::Relative
                                label="With Relative Error".to_string()
                            />
                        </div>
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn interval_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "IntervalInput",
        description:
            "Mathematical interval input with support for open, closed, and half-open intervals.",
        import_name: "IntervalInput",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<RwSignal<Interval>>",
                default: None,
                description: "The current interval value",
                required: false,
            },
            PropDoc {
                name: "bounds",
                prop_type: "IntervalBounds",
                default: Some("Closed"),
                description: "Default bounds type (Closed, Open, HalfOpenLeft, HalfOpenRight)",
                required: false,
            },
            PropDoc {
                name: "allow_infinity",
                prop_type: "bool",
                default: Some("true"),
                description: "Whether to allow infinite bounds",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text above the input",
                required: false,
            },
        ],
        demo: || {
            use mingot::prelude::*;

            view! {
                <DemoBlock title="IntervalInput" code=r#"<IntervalInput label="Select Range" />"#>
                    <Stack spacing="md">
                        <IntervalInput label="Range".to_string() />
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn coordinate_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "CoordinateInput",
        description: "Multi-dimensional coordinate input supporting Cartesian, Polar, Cylindrical, and Spherical systems.",
        import_name: "CoordinateInput",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<RwSignal<Coordinates>>",
                default: None,
                description: "The current coordinate value",
                required: false,
            },
            PropDoc {
                name: "system",
                prop_type: "CoordinateSystem",
                default: Some("Cartesian3D"),
                description: "Coordinate system (Cartesian2D, Cartesian3D, Polar, Cylindrical, Spherical)",
                required: false,
            },
            PropDoc {
                name: "allow_conversion",
                prop_type: "bool",
                default: Some("true"),
                description: "Allow switching between coordinate systems",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text above the input",
                required: false,
            },
        ],
        demo: || {
            use mingot::prelude::*;

            view! {
                <DemoBlock title="CoordinateInput" code=r#"<CoordinateInput system=CoordinateSystem::Cartesian3D />"#>
                    <Stack spacing="md">
                        <CoordinateInput
                            system=CoordinateSystem::Cartesian3D
                            label="3D Point".to_string()
                        />
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn point_locator_doc() -> ComponentDoc {
    ComponentDoc {
        name: "PointLocator",
        description: "Visual point positioning with drag-and-drop interface, grid snapping, and coordinate display.",
        import_name: "PointLocator",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<RwSignal<Point2D>>",
                default: None,
                description: "The current point value",
                required: false,
            },
            PropDoc {
                name: "bounds",
                prop_type: "Bounds",
                default: Some("(-10, 10, -10, 10)"),
                description: "Coordinate bounds",
                required: false,
            },
            PropDoc {
                name: "snap_to_grid",
                prop_type: "Option<f64>",
                default: None,
                description: "Grid snap size",
                required: false,
            },
            PropDoc {
                name: "width",
                prop_type: "u32",
                default: Some("300"),
                description: "Canvas width in pixels",
                required: false,
            },
            PropDoc {
                name: "height",
                prop_type: "u32",
                default: Some("300"),
                description: "Canvas height in pixels",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text above the canvas",
                required: false,
            },
        ],
        demo: || {
            use mingot::prelude::*;

            view! {
                <DemoBlock title="PointLocator" code=r#"<PointLocator bounds=Bounds::symmetric(5.0) snap_to_grid=1.0 />"#>
                    <Stack spacing="md">
                        <PointLocator
                            bounds=Bounds::symmetric(5.0)
                            snap_to_grid=1.0
                            label="Point".to_string()
                        />
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn matrix_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "MatrixInput",
        description: "Spreadsheet-style matrix entry with built-in operations like determinant, trace, and transpose.",
        import_name: "MatrixInput",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<RwSignal<Matrix>>",
                default: None,
                description: "Controlled matrix value",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<Matrix>>",
                default: None,
                description: "Called when matrix values change",
                required: false,
            },
            PropDoc {
                name: "rows",
                prop_type: "usize",
                default: Some("3"),
                description: "Initial number of rows",
                required: false,
            },
            PropDoc {
                name: "cols",
                prop_type: "usize",
                default: Some("3"),
                description: "Initial number of columns",
                required: false,
            },
            PropDoc {
                name: "allow_resize",
                prop_type: "bool",
                default: Some("true"),
                description: "Whether to show row/column manipulation buttons",
                required: false,
            },
            PropDoc {
                name: "show_operations",
                prop_type: "bool",
                default: Some("true"),
                description: "Show determinant/trace/norm preview",
                required: false,
            },
            PropDoc {
                name: "notation",
                prop_type: "MatrixNotation",
                default: Some("MatrixNotation::Brackets"),
                description: "Display notation style (Brackets, Parentheses, Bars, DoubleBars)",
                required: false,
            },
            PropDoc {
                name: "precision",
                prop_type: "usize",
                default: Some("4"),
                description: "Number of decimal places for display",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text",
                required: false,
            },
        ],
        demo: || {
            use mingot::prelude::*;

            let matrix = RwSignal::new(Matrix::identity(3));

            view! {
                <DemoBlock title="Matrix Input" code=r#"let matrix = RwSignal::new(Matrix::identity(3));

<MatrixInput
    value=matrix
    show_operations=true
    allow_resize=true
/>"#>
                    <Stack spacing="md">
                        <MatrixInput
                            value=matrix
                            show_operations=true
                            allow_resize=true
                        />
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn vector_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "VectorInput",
        description: "Mathematical vector entry with multiple notations, magnitude display, and vector operations.",
        import_name: "VectorInput",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<RwSignal<Vector>>",
                default: None,
                description: "Controlled vector value",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<Vector>>",
                default: None,
                description: "Called when vector values change",
                required: false,
            },
            PropDoc {
                name: "dimensions",
                prop_type: "usize",
                default: Some("3"),
                description: "Number of vector components",
                required: false,
            },
            PropDoc {
                name: "allow_resize",
                prop_type: "bool",
                default: Some("false"),
                description: "Whether to allow dimension changes",
                required: false,
            },
            PropDoc {
                name: "notation",
                prop_type: "VectorNotation",
                default: Some("VectorNotation::Column"),
                description: "Display notation (Row, Column, AngleBrackets, Parentheses, UnitVector)",
                required: false,
            },
            PropDoc {
                name: "show_magnitude",
                prop_type: "bool",
                default: Some("true"),
                description: "Show magnitude and direction display",
                required: false,
            },
            PropDoc {
                name: "precision",
                prop_type: "usize",
                default: Some("4"),
                description: "Number of decimal places for display",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text",
                required: false,
            },
        ],
        demo: || {
            use mingot::prelude::*;

            let vector = RwSignal::new(Vector::new(vec![1.0, 2.0, 3.0]));

            view! {
                <DemoBlock title="Vector Input" code=r#"let vector = RwSignal::new(Vector::new(vec![1.0, 2.0, 3.0]));

<VectorInput
    value=vector
    notation=VectorNotation::Column
    show_magnitude=true
/>"#>
                    <Stack spacing="md">
                        <VectorInput
                            value=vector
                            notation=VectorNotation::Column
                            show_magnitude=true
                        />
                    </Stack>
                </DemoBlock>
            }
            .into_any()
        },
    }
}

fn tensor_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "TensorInput",
        description: "Multi-dimensional tensor entry with slice navigation, reshape operations, and statistics display.",
        import_name: "TensorInput",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<RwSignal<Tensor>>",
                default: None,
                description: "Controlled tensor value",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<Tensor>>",
                default: None,
                description: "Called when tensor values change",
                required: false,
            },
            PropDoc {
                name: "shape",
                prop_type: "Option<Vec<usize>>",
                default: None,
                description: "Initial tensor shape",
                required: false,
            },
            PropDoc {
                name: "show_stats",
                prop_type: "bool",
                default: Some("true"),
                description: "Show min/max/mean statistics",
                required: false,
            },
            PropDoc {
                name: "precision",
                prop_type: "usize",
                default: Some("4"),
                description: "Number of decimal places for display",
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
                name: "disabled",
                prop_type: "Signal<bool>",
                default: Some("false"),
                description: "Whether the input is disabled",
                required: false,
            },
        ],
        demo: || {
            use mingot::prelude::*;

            // Rank 2: Simple 2D tensor (matrix-like, no slice navigation)
            let matrix_data: Vec<f64> = (1..=12).map(|x| x as f64).collect();
            let tensor_2d = RwSignal::new(Tensor::from_data(matrix_data, vec![3, 4]).unwrap());

            // Rank 3: 3D tensor with slice navigation (e.g., RGB image channels or time series)
            let tensor_3d_data: Vec<f64> = (0..24).map(|x| x as f64).collect();
            let tensor_3d = RwSignal::new(Tensor::from_data(tensor_3d_data, vec![2, 3, 4]).unwrap());

            // Rank 4: 4D tensor with two slice dimensions (e.g., batch of images, video frames)
            let tensor_4d_data: Vec<f64> = (0..36).map(|x| x as f64).collect();
            let tensor_4d = RwSignal::new(Tensor::from_data(tensor_4d_data, vec![2, 2, 3, 3]).unwrap());

            // Rank 5: Higher-dimensional tensor (e.g., batch of video clips)
            let tensor_5d_data: Vec<f64> = (0..48).map(|x| x as f64 * 0.1).collect();
            let tensor_5d = RwSignal::new(Tensor::from_data(tensor_5d_data, vec![2, 2, 2, 2, 3]).unwrap());

            view! {
                <Stack spacing="xl">
                    <DemoBlock title="Rank 2: Matrix-like Tensor" code=r#"// Simple 3×4 tensor - displays directly without slice navigation
let data: Vec<f64> = (1..=12).map(|x| x as f64).collect();
let tensor = RwSignal::new(Tensor::from_data(data, vec![3, 4]).unwrap());

<TensorInput
    value=tensor
    label="2D Tensor (3 × 4)"
    show_stats=true
/>"#>
                        <TensorInput
                            value=tensor_2d
                            label="2D Tensor (3 × 4)"
                            show_stats=true
                        />
                    </DemoBlock>

                    <DemoBlock title="Rank 3: Sliceable 3D Tensor" code=r#"// 2×3×4 tensor - navigate through 2 slices using dim[0]
let data: Vec<f64> = (0..24).map(|x| x as f64).collect();
let tensor = RwSignal::new(Tensor::from_data(data, vec![2, 3, 4]).unwrap());

<TensorInput
    value=tensor
    label="3D Tensor (2 × 3 × 4) - use dim[0] to switch slices"
    show_stats=true
/>"#>
                        <TensorInput
                            value=tensor_3d
                            label="3D Tensor (2 × 3 × 4) - use dim[0] to switch slices"
                            show_stats=true
                        />
                    </DemoBlock>

                    <DemoBlock title="Rank 4: Multi-dimensional Navigation" code=r#"// 2×2×3×3 tensor - two slice dimensions to navigate
// Could represent: batch × channels × height × width
let data: Vec<f64> = (0..36).map(|x| x as f64).collect();
let tensor = RwSignal::new(Tensor::from_data(data, vec![2, 2, 3, 3]).unwrap());

<TensorInput
    value=tensor
    label="4D Tensor (2 × 2 × 3 × 3) - navigate dim[0] and dim[1]"
    show_stats=true
/>"#>
                        <TensorInput
                            value=tensor_4d
                            label="4D Tensor (2 × 2 × 3 × 3) - navigate dim[0] and dim[1]"
                            show_stats=true
                        />
                    </DemoBlock>

                    <DemoBlock title="Rank 5: High-dimensional Tensor" code=r#"// 2×2×2×2×3 tensor - three slice dimensions
// Could represent: batch × time × channels × height × width
let data: Vec<f64> = (0..48).map(|x| x as f64 * 0.1).collect();
let tensor = RwSignal::new(Tensor::from_data(data, vec![2, 2, 2, 2, 3]).unwrap());

<TensorInput
    value=tensor
    label="5D Tensor (2 × 2 × 2 × 2 × 3)"
    precision=2
    show_stats=true
/>"#>
                        <TensorInput
                            value=tensor_5d
                            label="5D Tensor (2 × 2 × 2 × 2 × 3)"
                            precision=2
                            show_stats=true
                        />
                    </DemoBlock>

                    <DemoBlock title="Without Statistics" code=r#"<TensorInput
    shape=vec![2, 3]
    show_stats=false
    label="Minimal display (no stats)"
/>"#>
                        <TensorInput
                            shape=vec![2, 3]
                            show_stats=false
                            label="Minimal display (no stats)"
                        />
                    </DemoBlock>
                </Stack>
            }
            .into_any()
        },
    }
}

fn symbol_palette_doc() -> ComponentDoc {
    ComponentDoc {
        name: "SymbolPalette",
        description: "A searchable, categorized picker for mathematical symbols including Greek letters, operators, set theory, logic, arrows, and relations.",
        import_name: "SymbolPalette",
        props: vec![
            PropDoc {
                name: "categories",
                prop_type: "Option<Vec<SymbolCategory>>",
                default: Some("all categories"),
                description: "Categories of symbols to display",
                required: false,
            },
            PropDoc {
                name: "on_select",
                prop_type: "Option<Callback<Symbol>>",
                default: None,
                description: "Callback when a symbol is selected",
                required: false,
            },
            PropDoc {
                name: "searchable",
                prop_type: "bool",
                default: Some("true"),
                description: "Whether to show the search box",
                required: false,
            },
            PropDoc {
                name: "show_tabs",
                prop_type: "bool",
                default: Some("true"),
                description: "Whether to show category tabs",
                required: false,
            },
            PropDoc {
                name: "columns",
                prop_type: "usize",
                default: Some("8"),
                description: "Number of columns in the symbol grid",
                required: false,
            },
            PropDoc {
                name: "show_tooltip",
                prop_type: "bool",
                default: Some("true"),
                description: "Show symbol name on hover",
                required: false,
            },
            PropDoc {
                name: "show_latex",
                prop_type: "bool",
                default: Some("true"),
                description: "Include LaTeX code in tooltip",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label for the palette",
                required: false,
            },
        ],
        demo: || {
            use mingot::prelude::*;

            let selected_symbol = RwSignal::new(None::<Symbol>);

            view! {
                <Stack spacing="xl">
                    // Display selected symbol from any palette
                    {move || selected_symbol.get().map(|s| view! {
                        <Paper style="padding: 1rem; background: var(--surface);">
                            <Text>
                                {format!("Selected: {} ({}) - LaTeX: {}",
                                    s.char, s.name, s.latex.unwrap_or("N/A"))}
                            </Text>
                        </Paper>
                    })}

                    <DemoBlock title="Full Symbol Palette" code=r#"let selected = RwSignal::new(None::<Symbol>);

<SymbolPalette
    on_select=Callback::new(move |sym: Symbol| {
        selected.set(Some(sym));
    })
    label="Mathematical Symbols"
/>"#>
                        <SymbolPalette
                            on_select=Callback::new(move |sym: Symbol| {
                                selected_symbol.set(Some(sym));
                            })
                            label="Mathematical Symbols"
                        />
                    </DemoBlock>

                    <DemoBlock title="Greek Letters Only" code=r#"<SymbolPalette
    categories=vec![SymbolCategory::Greek]
    show_tabs=false
    label="Greek Alphabet"
    columns=12
    on_select=Callback::new(move |sym: Symbol| {
        selected.set(Some(sym));
    })
/>"#>
                        <SymbolPalette
                            categories=vec![SymbolCategory::Greek]
                            show_tabs=false
                            label="Greek Alphabet"
                            columns=12
                            on_select=Callback::new(move |sym: Symbol| {
                                selected_symbol.set(Some(sym));
                            })
                        />
                    </DemoBlock>

                    <DemoBlock title="Operators and Relations" code=r#"<SymbolPalette
    categories=vec![SymbolCategory::Operators, SymbolCategory::Relations]
    label="Operators & Relations"
    on_select=Callback::new(move |sym: Symbol| {
        selected.set(Some(sym));
    })
/>"#>
                        <SymbolPalette
                            categories=vec![SymbolCategory::Operators, SymbolCategory::Relations]
                            label="Operators & Relations"
                            on_select=Callback::new(move |sym: Symbol| {
                                selected_symbol.set(Some(sym));
                            })
                        />
                    </DemoBlock>

                    <DemoBlock title="Logic and Set Theory" code=r#"<SymbolPalette
    categories=vec![SymbolCategory::Logic, SymbolCategory::SetTheory]
    label="Logic & Set Theory"
    columns=6
    on_select=Callback::new(move |sym: Symbol| {
        selected.set(Some(sym));
    })
/>"#>
                        <SymbolPalette
                            categories=vec![SymbolCategory::Logic, SymbolCategory::SetTheory]
                            label="Logic & Set Theory"
                            columns=6
                            on_select=Callback::new(move |sym: Symbol| {
                                selected_symbol.set(Some(sym));
                            })
                        />
                    </DemoBlock>

                    <DemoBlock title="Compact (No Search)" code=r#"<SymbolPalette
    categories=vec![SymbolCategory::Arrows]
    searchable=false
    show_tabs=false
    label="Arrows"
    columns=10
    on_select=Callback::new(move |sym: Symbol| {
        selected.set(Some(sym));
    })
/>"#>
                        <SymbolPalette
                            categories=vec![SymbolCategory::Arrows]
                            searchable=false
                            show_tabs=false
                            label="Arrows"
                            columns=10
                            on_select=Callback::new(move |sym: Symbol| {
                                selected_symbol.set(Some(sym));
                            })
                        />
                    </DemoBlock>
                </Stack>
            }
            .into_any()
        },
    }
}

fn formula_input_doc() -> ComponentDoc {
    ComponentDoc {
        name: "FormulaInput",
        description: "Mathematical expression input with parsing, variable support, and function recognition. Supports standard math functions (sin, cos, exp, ln, sqrt, etc.) and evaluates expressions in real-time.",
        import_name: "FormulaInput",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "Option<RwSignal<String>>",
                default: None,
                description: "Controlled formula string value",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Option<Callback<FormulaResult>>",
                default: None,
                description: "Callback with parsed expression, variables, and evaluated result",
                required: false,
            },
            PropDoc {
                name: "variables",
                prop_type: "Option<Signal<HashMap<String, f64>>>",
                default: None,
                description: "Variable values for evaluation",
                required: false,
            },
            PropDoc {
                name: "show_parsed",
                prop_type: "bool",
                default: Some("false"),
                description: "Show the parsed expression tree",
                required: false,
            },
            PropDoc {
                name: "show_result",
                prop_type: "bool",
                default: Some("true"),
                description: "Show evaluation result",
                required: false,
            },
            PropDoc {
                name: "show_variables",
                prop_type: "bool",
                default: Some("false"),
                description: "Show detected variables",
                required: false,
            },
            PropDoc {
                name: "label",
                prop_type: "Option<String>",
                default: None,
                description: "Label text",
                required: false,
            },
        ],
        demo: || {
            use mingot::prelude::*;

            view! {
                <Stack spacing="xl">
                    <DemoBlock title="Basic Formula Input" code=r#"<FormulaInput
    label="Enter a formula"
    placeholder="e.g., sin(pi/2) + sqrt(16)"
    show_result=true
/>"#>
                        <FormulaInput
                            label="Enter a formula"
                            placeholder="e.g., sin(pi/2) + sqrt(16)"
                            show_result=true
                        />
                    </DemoBlock>

                    <DemoBlock title="Show Variables" code=r#"<FormulaInput
    label="Formula with variables"
    placeholder="e.g., x^2 + y"
    show_variables=true
    show_result=false
/>"#>
                        <FormulaInput
                            label="Formula with variables"
                            placeholder="e.g., x^2 + y"
                            show_variables=true
                            show_result=false
                        />
                    </DemoBlock>

                    <DemoBlock title="Show Parsed Expression" code=r#"<FormulaInput
    label="With parsed expression display"
    show_parsed=true
    show_result=false
/>"#>
                        <FormulaInput
                            label="With parsed expression display"
                            show_parsed=true
                            show_result=false
                        />
                    </DemoBlock>

                    <DemoBlock title="Supported Functions" code=r#"// Trigonometric: sin, cos, tan, asin, acos, atan
// Hyperbolic: sinh, cosh, tanh
// Exponential: exp, ln, log10, log2
// Power/Root: sqrt, cbrt, abs
// Rounding: floor, ceil, round
// Special: sign, factorial
// Constants: pi, e, tau

// Try: sin(pi/4), exp(1), sqrt(2)^2, factorial(5), ln(e)"#>
                        <FormulaInput
                            label="Try the examples above"
                            show_result=true
                        />
                    </DemoBlock>
                </Stack>
            }
            .into_any()
        },
    }
}

fn equation_editor_doc() -> ComponentDoc {
    ComponentDoc {
        name: "EquationEditor",
        import_name: "EquationEditor",
        description: "A WYSIWYG mathematical equation editor designed for geometric algebra expressions with support for Amari library operations.",
        props: vec![
            PropDoc {
                name: "value",
                prop_type: "RwSignal<EquationNode>",
                default: Some("Placeholder"),
                description: "Current equation value as an AST node",
                required: false,
            },
            PropDoc {
                name: "on_change",
                prop_type: "Callback<EquationNode>",
                default: None,
                description: "Callback when equation changes",
                required: false,
            },
            PropDoc {
                name: "show_toolbar",
                prop_type: "bool",
                default: Some("true"),
                description: "Show the operation toolbar",
                required: false,
            },
            PropDoc {
                name: "show_latex",
                prop_type: "bool",
                default: Some("false"),
                description: "Show LaTeX output below the equation",
                required: false,
            },
            PropDoc {
                name: "size",
                prop_type: "EquationEditorSize",
                default: Some("Md"),
                description: "Editor size (Sm, Md, Lg)",
                required: false,
            },
            PropDoc {
                name: "basis_type",
                prop_type: "BasisType",
                default: Some("Standard"),
                description: "Basis type for vector insertion (Standard, Conformal, Spacetime)",
                required: false,
            },
            PropDoc {
                name: "max_dimensions",
                prop_type: "usize",
                default: Some("3"),
                description: "Maximum dimensions for basis vectors",
                required: false,
            },
            PropDoc {
                name: "disabled",
                prop_type: "bool",
                default: Some("false"),
                description: "Disable the editor",
                required: false,
            },
            PropDoc {
                name: "read_only",
                prop_type: "bool",
                default: Some("false"),
                description: "Read-only display mode",
                required: false,
            },
            PropDoc {
                name: "placeholder",
                prop_type: "String",
                default: Some("\"Enter expression...\""),
                description: "Placeholder text when empty",
                required: false,
            },
        ],
        demo: || {
            view! {
                <Stack spacing="lg">
                    <DemoBlock title="Basic Equation Editor" code=r#"<EquationEditor
    show_latex=true
/>"#>
                        <EquationEditor
                            show_latex=true
                        />
                    </DemoBlock>

                    <DemoBlock title="Geometric Algebra Operations" code=r#"// The editor supports geometric algebra operations:
// - Geometric product (*)
// - Wedge product (^)
// - Inner product (.)
// - Left contraction
// - Right contraction
// - Scalar product

<EquationEditor show_toolbar=true />"#>
                        <Text size=TextSize::Sm color="dimmed">
                            "Use the Products toolbar to insert GA operations. Type a value and press Enter, then click an operation."
                        </Text>
                        <EquationEditor />
                    </DemoBlock>

                    <DemoBlock title="Unary Operations" code=r#"// Unary operations available:
// - Reverse (dagger)
// - Hodge dual (star)
// - Grade involution
// - Clifford conjugate
// - Normalize, Inverse, Magnitude
// - Exponential (for rotor generation)

<EquationEditor show_toolbar=true />"#>
                        <Text size=TextSize::Sm color="dimmed">
                            "Select the Unary tab to access operations like reverse, dual, and exponential."
                        </Text>
                        <EquationEditor />
                    </DemoBlock>

                    <DemoBlock title="Calculus Operations" code=r#"// Vector calculus operators:
// - Gradient (nabla)
// - Divergence (nabla dot)
// - Curl (nabla wedge)
// - Laplacian (nabla squared)
// - Partial derivative

<EquationEditor show_toolbar=true />"#>
                        <Text size=TextSize::Sm color="dimmed">
                            "Select the Calculus tab for differential operators from geometric calculus."
                        </Text>
                        <EquationEditor />
                    </DemoBlock>

                    <DemoBlock title="Basis Vectors" code=r#"// Supports different basis types:
// - Standard: e0, e1, e2, e3...
// - Conformal: e0, e1, e2, e3, e_inf
// - Spacetime: gamma0, gamma1, gamma2, gamma3

<EquationEditor basis_type=BasisType::Standard max_dimensions=4 />"#>
                        <Text size=TextSize::Sm color="dimmed">
                            "Select the Basis tab to insert basis vectors."
                        </Text>
                        <EquationEditor max_dimensions=4 />
                    </DemoBlock>

                    <DemoBlock title="Size Variants" code=r#"<EquationEditor size=EquationEditorSize::Sm />
<EquationEditor size=EquationEditorSize::Md />
<EquationEditor size=EquationEditorSize::Lg />"#>
                        <Stack spacing="md">
                            <div>
                                <Text size=TextSize::Xs color="dimmed">"Small"</Text>
                                <EquationEditor size=EquationEditorSize::Sm show_toolbar=false />
                            </div>
                            <div>
                                <Text size=TextSize::Xs color="dimmed">"Medium (default)"</Text>
                                <EquationEditor size=EquationEditorSize::Md show_toolbar=false />
                            </div>
                            <div>
                                <Text size=TextSize::Xs color="dimmed">"Large"</Text>
                                <EquationEditor size=EquationEditorSize::Lg show_toolbar=false />
                            </div>
                        </Stack>
                    </DemoBlock>

                    <DemoBlock title="Read-only Display" code=r#"// Create an equation programmatically
let equation = EquationNode::BinaryOp {
    op: GeometricOp::WedgeProduct,
    left: Box::new(EquationNode::Variable("a".to_string())),
    right: Box::new(EquationNode::Variable("b".to_string())),
};

<EquationEditor value=equation read_only=true show_latex=true />"#>
                        <EquationEditor
                            read_only=true
                            show_latex=true
                            show_toolbar=false
                        />
                    </DemoBlock>
                </Stack>
            }
            .into_any()
        },
    }
}
