use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

#[derive(Clone)]
pub struct NavSection {
    pub title: &'static str,
    pub items: Vec<NavItem>,
}

#[derive(Clone)]
pub struct NavItem {
    pub label: &'static str,
    pub href: &'static str,
    pub badge: Option<&'static str>,
}

pub fn get_navigation() -> Vec<NavSection> {
    vec![
        NavSection {
            title: "Getting Started",
            items: vec![
                NavItem {
                    label: "Introduction",
                    href: "/",
                    badge: None,
                },
                NavItem {
                    label: "Installation",
                    href: "/getting-started",
                    badge: None,
                },
            ],
        },
        NavSection {
            title: "Core",
            items: vec![
                NavItem {
                    label: "ActionIcon",
                    href: "/core/action-icon",
                    badge: None,
                },
                NavItem {
                    label: "Button",
                    href: "/core/button",
                    badge: None,
                },
                NavItem {
                    label: "Container",
                    href: "/core/container",
                    badge: None,
                },
                NavItem {
                    label: "Divider",
                    href: "/core/divider",
                    badge: None,
                },
                NavItem {
                    label: "Group",
                    href: "/core/group",
                    badge: None,
                },
                NavItem {
                    label: "Stack",
                    href: "/core/stack",
                    badge: None,
                },
                NavItem {
                    label: "Text",
                    href: "/core/text",
                    badge: None,
                },
            ],
        },
        NavSection {
            title: "Layout",
            items: vec![
                NavItem {
                    label: "AppShell",
                    href: "/layout/app-shell",
                    badge: None,
                },
                NavItem {
                    label: "Card",
                    href: "/layout/card",
                    badge: None,
                },
                NavItem {
                    label: "Grid",
                    href: "/layout/grid",
                    badge: None,
                },
                NavItem {
                    label: "Header",
                    href: "/layout/header",
                    badge: None,
                },
                NavItem {
                    label: "Paper",
                    href: "/layout/paper",
                    badge: None,
                },
            ],
        },
        NavSection {
            title: "Navigation",
            items: vec![
                NavItem {
                    label: "Breadcrumbs",
                    href: "/navigation/breadcrumbs",
                    badge: None,
                },
                NavItem {
                    label: "Burger",
                    href: "/navigation/burger",
                    badge: None,
                },
                NavItem {
                    label: "Navbar",
                    href: "/navigation/navbar",
                    badge: None,
                },
                NavItem {
                    label: "Pagination",
                    href: "/navigation/pagination",
                    badge: Some("New"),
                },
                NavItem {
                    label: "Tabs",
                    href: "/navigation/tabs",
                    badge: None,
                },
            ],
        },
        NavSection {
            title: "Form",
            items: vec![
                NavItem {
                    label: "Checkbox",
                    href: "/form/checkbox",
                    badge: None,
                },
                NavItem {
                    label: "FileInput",
                    href: "/form/file-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "Input",
                    href: "/form/input",
                    badge: None,
                },
                NavItem {
                    label: "NumberInput",
                    href: "/form/number-input",
                    badge: Some("Precision"),
                },
                NavItem {
                    label: "AngleInput",
                    href: "/form/angle-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "FractionInput",
                    href: "/form/fraction-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "UnitInput",
                    href: "/form/unit-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "ComplexNumberInput",
                    href: "/form/complex-number-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "UncertaintyInput",
                    href: "/form/uncertainty-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "IntervalInput",
                    href: "/form/interval-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "CoordinateInput",
                    href: "/form/coordinate-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "PointLocator",
                    href: "/form/point-locator",
                    badge: Some("New"),
                },
                NavItem {
                    label: "MatrixInput",
                    href: "/form/matrix-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "VectorInput",
                    href: "/form/vector-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "TensorInput",
                    href: "/form/tensor-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "PasswordInput",
                    href: "/form/password-input",
                    badge: None,
                },
                NavItem {
                    label: "PinInput",
                    href: "/form/pin-input",
                    badge: Some("New"),
                },
                NavItem {
                    label: "Radio",
                    href: "/form/radio",
                    badge: None,
                },
                NavItem {
                    label: "RangeSlider",
                    href: "/form/range-slider",
                    badge: Some("New"),
                },
                NavItem {
                    label: "SegmentedControl",
                    href: "/form/segmented-control",
                    badge: Some("New"),
                },
                NavItem {
                    label: "Select",
                    href: "/form/select",
                    badge: None,
                },
                NavItem {
                    label: "Slider",
                    href: "/form/slider",
                    badge: Some("New"),
                },
                NavItem {
                    label: "Switch",
                    href: "/form/switch",
                    badge: None,
                },
                NavItem {
                    label: "Textarea",
                    href: "/form/textarea",
                    badge: None,
                },
            ],
        },
        NavSection {
            title: "Overlay",
            items: vec![
                NavItem {
                    label: "Drawer",
                    href: "/overlay/drawer",
                    badge: None,
                },
                NavItem {
                    label: "LoadingOverlay",
                    href: "/overlay/loading-overlay",
                    badge: None,
                },
                NavItem {
                    label: "Modal",
                    href: "/overlay/modal",
                    badge: None,
                },
                NavItem {
                    label: "Popover",
                    href: "/overlay/popover",
                    badge: None,
                },
                NavItem {
                    label: "Tooltip",
                    href: "/overlay/tooltip",
                    badge: None,
                },
            ],
        },
        NavSection {
            title: "Feedback",
            items: vec![
                NavItem {
                    label: "Alert",
                    href: "/feedback/alert",
                    badge: None,
                },
                NavItem {
                    label: "Loader",
                    href: "/feedback/loader",
                    badge: None,
                },
                NavItem {
                    label: "Notification",
                    href: "/feedback/notification",
                    badge: None,
                },
                NavItem {
                    label: "Progress",
                    href: "/feedback/progress",
                    badge: None,
                },
                NavItem {
                    label: "Skeleton",
                    href: "/feedback/skeleton",
                    badge: None,
                },
            ],
        },
        NavSection {
            title: "Data Display",
            items: vec![
                NavItem {
                    label: "Accordion",
                    href: "/data-display/accordion",
                    badge: None,
                },
                NavItem {
                    label: "Avatar",
                    href: "/data-display/avatar",
                    badge: None,
                },
                NavItem {
                    label: "Badge",
                    href: "/data-display/badge",
                    badge: None,
                },
                NavItem {
                    label: "RingProgress",
                    href: "/data-display/ring-progress",
                    badge: None,
                },
                NavItem {
                    label: "Stats",
                    href: "/data-display/stats",
                    badge: None,
                },
                NavItem {
                    label: "Table",
                    href: "/data-display/table",
                    badge: None,
                },
            ],
        },
        NavSection {
            title: "Miscellaneous",
            items: vec![NavItem {
                label: "ErrorPage",
                href: "/misc/error-page",
                badge: None,
            }],
        },
    ]
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    let sections = get_navigation();

    view! {
        <nav class="docs-sidebar">
            {sections.into_iter().map(|section| {
                view! {
                    <div class="nav-section">
                        <div class="nav-section-title">{section.title}</div>
                        <div class="nav-links">
                            {section.items.into_iter().map(|item| {
                                let href = item.href;
                                let is_active = move || location.pathname.get() == href;
                                let navigate = navigate.clone();

                                view! {
                                    <button
                                        type="button"
                                        class=move || {
                                            if is_active() {
                                                "nav-link active"
                                            } else {
                                                "nav-link"
                                            }
                                        }
                                        on:click=move |_| {
                                            navigate(href, Default::default());
                                        }
                                    >
                                        {item.label}
                                        {item.badge.map(|badge| view! {
                                            <span style="margin-left: 0.5rem; font-size: 0.625rem; padding: 0.125rem 0.375rem; background: #228be6; color: white; border-radius: 0.25rem;">
                                                {badge}
                                            </span>
                                        })}
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                }
            }).collect_view()}
        </nav>
    }
}
