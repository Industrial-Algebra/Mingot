use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

/// Loader variant determines the visual style of the loading indicator
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LoaderVariant {
    /// Spinning circle with a gap (default)
    #[default]
    Oval,
    /// Three bouncing dots
    Dots,
    /// Pulsing bars
    Bars,
}

/// Loader size determines the dimensions of the loader
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LoaderSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl LoaderSize {
    fn to_px(self) -> u32 {
        match self {
            LoaderSize::Xs => 18,
            LoaderSize::Sm => 22,
            LoaderSize::Md => 36,
            LoaderSize::Lg => 44,
            LoaderSize::Xl => 58,
        }
    }
}

/// A loading indicator component
///
/// # Example
/// ```rust,ignore
/// <Loader />
/// <Loader size=LoaderSize::Lg color="red" />
/// <Loader variant=LoaderVariant::Dots />
/// ```
#[component]
pub fn Loader(
    /// Visual variant of the loader
    #[prop(optional)]
    variant: Option<LoaderVariant>,
    /// Size of the loader
    #[prop(optional)]
    size: Option<LoaderSize>,
    /// Color of the loader (theme color name like "blue", "red", etc.)
    #[prop(optional, into)]
    color: Option<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let size_px = size.to_px();

    let loader_color = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        if let Some(ref c) = color {
            scheme_colors
                .get_color(c, 6)
                .unwrap_or_else(|| "#228be6".to_string())
        } else {
            scheme_colors
                .get_color("blue", 6)
                .unwrap_or_else(|| "#228be6".to_string())
        }
    };

    let class_str = format!("mingot-loader {}", class.unwrap_or_default());

    match variant {
        LoaderVariant::Oval => {
            let container_styles = move || {
                let mut builder = StyleBuilder::new();
                builder
                    .add("display", "inline-block")
                    .add("width", format!("{}px", size_px))
                    .add("height", format!("{}px", size_px));

                if let Some(s) = style.as_ref() {
                    format!("{}; {}", builder.build(), s)
                } else {
                    builder.build()
                }
            };

            let svg_style = format!(
                "width: {}px; height: {}px; animation: mingot-loader-spin 1s linear infinite;",
                size_px, size_px
            );

            let stroke_width = if size_px < 30 { 4 } else { 3 };
            let loader_color_1 = loader_color.clone();
            let loader_color_2 = loader_color;

            view! {
                <>
                    <style>
                        "@keyframes mingot-loader-spin {
                            0% { transform: rotate(0deg); }
                            100% { transform: rotate(360deg); }
                        }"
                    </style>
                    <div class=class_str style=container_styles>
                        <svg viewBox="0 0 38 38" style=svg_style>
                            <g fill="none" fill-rule="evenodd">
                                <g transform="translate(2.5 2.5)" stroke-width=stroke_width>
                                    <circle
                                        stroke-opacity=".2"
                                        cx="16"
                                        cy="16"
                                        r="16"
                                        stroke=loader_color_1
                                    />
                                    <path
                                        d="M32 16c0-8.837-7.163-16-16-16"
                                        stroke=loader_color_2
                                        stroke-linecap="round"
                                    />
                                </g>
                            </g>
                        </svg>
                    </div>
                </>
            }
            .into_any()
        }
        LoaderVariant::Dots => {
            let dot_size = size_px / 4;
            let container_styles = move || {
                let mut builder = StyleBuilder::new();
                builder
                    .add("display", "inline-flex")
                    .add("align-items", "center")
                    .add("gap", format!("{}px", dot_size / 2))
                    .add("height", format!("{}px", size_px));

                if let Some(s) = style.as_ref() {
                    format!("{}; {}", builder.build(), s)
                } else {
                    builder.build()
                }
            };

            let make_dot_style = move |delay: &'static str| {
                let loader_color = loader_color.clone();
                move || {
                    format!(
                        "width: {}px; height: {}px; border-radius: 50%; background-color: {}; animation: mingot-loader-bounce 1.4s ease-in-out infinite both; animation-delay: {};",
                        dot_size, dot_size, loader_color(), delay
                    )
                }
            };

            let dot1 = make_dot_style("-0.32s");
            let dot2 = make_dot_style("-0.16s");
            let dot3 = make_dot_style("0s");

            view! {
                <>
                    <style>
                        "@keyframes mingot-loader-bounce {
                            0%, 80%, 100% { transform: scale(0); }
                            40% { transform: scale(1); }
                        }"
                    </style>
                    <div class=class_str style=container_styles>
                        <div style=dot1></div>
                        <div style=dot2></div>
                        <div style=dot3></div>
                    </div>
                </>
            }
            .into_any()
        }
        LoaderVariant::Bars => {
            let bar_width = size_px / 6;
            let container_styles = move || {
                let mut builder = StyleBuilder::new();
                builder
                    .add("display", "inline-flex")
                    .add("align-items", "center")
                    .add("gap", format!("{}px", bar_width / 2))
                    .add("height", format!("{}px", size_px));

                if let Some(s) = style.as_ref() {
                    format!("{}; {}", builder.build(), s)
                } else {
                    builder.build()
                }
            };

            let make_bar_style = move |delay: &'static str| {
                let loader_color = loader_color.clone();
                move || {
                    format!(
                        "width: {}px; height: {}px; background-color: {}; animation: mingot-loader-bars 1.2s ease-in-out infinite; animation-delay: {};",
                        bar_width, size_px, loader_color(), delay
                    )
                }
            };

            let bar1 = make_bar_style("-0.32s");
            let bar2 = make_bar_style("-0.16s");
            let bar3 = make_bar_style("0s");
            let bar4 = make_bar_style("0.16s");

            view! {
                <>
                    <style>
                        "@keyframes mingot-loader-bars {
                            0%, 40%, 100% { transform: scaleY(0.4); }
                            20% { transform: scaleY(1); }
                        }"
                    </style>
                    <div class=class_str style=container_styles>
                        <div style=bar1></div>
                        <div style=bar2></div>
                        <div style=bar3></div>
                        <div style=bar4></div>
                    </div>
                </>
            }
            .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_size_values() {
        assert_eq!(LoaderSize::Xs.to_px(), 18);
        assert_eq!(LoaderSize::Sm.to_px(), 22);
        assert_eq!(LoaderSize::Md.to_px(), 36);
        assert_eq!(LoaderSize::Lg.to_px(), 44);
        assert_eq!(LoaderSize::Xl.to_px(), 58);
    }

    #[test]
    fn test_loader_variant_default() {
        assert_eq!(LoaderVariant::default(), LoaderVariant::Oval);
    }

    #[test]
    fn test_loader_size_default() {
        assert_eq!(LoaderSize::default(), LoaderSize::Md);
    }
}
