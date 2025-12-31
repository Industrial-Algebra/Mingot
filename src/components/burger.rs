use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

/// Burger size determines the dimensions
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BurgerSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl BurgerSize {
    fn to_size(self) -> &'static str {
        match self {
            BurgerSize::Xs => "12px",
            BurgerSize::Sm => "18px",
            BurgerSize::Md => "24px",
            BurgerSize::Lg => "32px",
            BurgerSize::Xl => "42px",
        }
    }

    fn to_bar_height(self) -> &'static str {
        match self {
            BurgerSize::Xs => "1px",
            BurgerSize::Sm => "2px",
            BurgerSize::Md => "2px",
            BurgerSize::Lg => "3px",
            BurgerSize::Xl => "4px",
        }
    }
}

/// A hamburger menu button component
///
/// Displays a hamburger icon that transforms into an X when opened.
/// Commonly used for mobile navigation toggles.
///
/// # Example
/// ```rust,ignore
/// let (opened, set_opened) = signal(false);
///
/// <Burger
///     opened=opened
///     on_click=Callback::new(move |_| set_opened.update(|v| *v = !*v))
///     aria_label="Toggle navigation"
/// />
/// ```
#[component]
pub fn Burger(
    /// Whether the burger is in opened (X) state
    #[prop(into)]
    opened: Signal<bool>,
    /// Size of the burger
    #[prop(optional)]
    size: Option<BurgerSize>,
    /// Color of the burger lines
    #[prop(optional, into)]
    color: Option<String>,
    /// Transition duration in milliseconds
    #[prop(optional)]
    transition_duration: Option<u32>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// Accessible label
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or_default();
    let transition_duration = transition_duration.unwrap_or(300);

    let burger_color = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        if let Some(ref c) = color {
            scheme_colors.get_color(c, 6).unwrap_or_else(|| c.clone())
        } else {
            scheme_colors.text.clone()
        }
    };

    let button_styles = move || {
        let mut builder = StyleBuilder::new();
        builder
            .add("width", size.to_size())
            .add("height", size.to_size())
            .add("padding", "0")
            .add("background", "transparent")
            .add("border", "none")
            .add("cursor", "pointer")
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("justify-content", "center")
            .add("align-items", "center")
            .add("position", "relative");

        if let Some(s) = style.as_ref() {
            format!("{}; {}", builder.build(), s)
        } else {
            builder.build()
        }
    };

    let bar_base_styles = {
        let burger_color = burger_color.clone();
        move || {
            format!(
                "display: block; \
                 width: 100%; \
                 height: {}; \
                 background-color: {}; \
                 border-radius: 9999px; \
                 position: absolute; \
                 left: 0; \
                 transition: transform {}ms ease, top {}ms ease, opacity {}ms ease;",
                size.to_bar_height(),
                burger_color(),
                transition_duration,
                transition_duration,
                transition_duration
            )
        }
    };

    let bar_base_1 = bar_base_styles.clone();
    let bar_base_2 = bar_base_styles.clone();
    let bar_base_3 = bar_base_styles;

    let top_bar_styles = move || {
        let is_opened = opened.get();
        let base = bar_base_1();
        if is_opened {
            format!(
                "{} top: 50%; transform: translateY(-50%) rotate(45deg);",
                base
            )
        } else {
            format!("{} top: 20%; transform: none;", base)
        }
    };

    let middle_bar_styles = move || {
        let is_opened = opened.get();
        let base = bar_base_2();
        if is_opened {
            format!(
                "{} top: 50%; transform: translateY(-50%); opacity: 0;",
                base
            )
        } else {
            format!(
                "{} top: 50%; transform: translateY(-50%); opacity: 1;",
                base
            )
        }
    };

    let bottom_bar_styles = move || {
        let is_opened = opened.get();
        let base = bar_base_3();
        if is_opened {
            format!(
                "{} top: 50%; transform: translateY(-50%) rotate(-45deg);",
                base
            )
        } else {
            format!("{} top: 80%; transform: none;", base)
        }
    };

    let handle_click = move |ev: ev::MouseEvent| {
        if let Some(callback) = on_click {
            callback.run(ev);
        }
    };

    let class_str = format!("mingot-burger {}", class.unwrap_or_default());

    view! {
        <button
            type="button"
            class=class_str
            style=button_styles
            on:click=handle_click
            aria-label=aria_label.unwrap_or_else(|| {
                if opened.get() {
                    "Close navigation".to_string()
                } else {
                    "Open navigation".to_string()
                }
            })

            aria-expanded=move || opened.get().to_string()
        >
            <span style=top_bar_styles></span>
            <span style=middle_bar_styles></span>
            <span style=bottom_bar_styles></span>
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burger_size_values() {
        assert_eq!(BurgerSize::Xs.to_size(), "12px");
        assert_eq!(BurgerSize::Sm.to_size(), "18px");
        assert_eq!(BurgerSize::Md.to_size(), "24px");
        assert_eq!(BurgerSize::Lg.to_size(), "32px");
        assert_eq!(BurgerSize::Xl.to_size(), "42px");
    }

    #[test]
    fn test_burger_bar_height_values() {
        assert_eq!(BurgerSize::Xs.to_bar_height(), "1px");
        assert_eq!(BurgerSize::Md.to_bar_height(), "2px");
        assert_eq!(BurgerSize::Xl.to_bar_height(), "4px");
    }

    #[test]
    fn test_burger_size_default() {
        assert_eq!(BurgerSize::default(), BurgerSize::Md);
    }
}
