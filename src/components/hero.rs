use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeroHeight {
    Sm,   // 300px
    Md,   // 400px
    Lg,   // 500px
    Xl,   // 600px
    Full, // 100vh
}

impl HeroHeight {
    fn value(&self) -> &'static str {
        match self {
            HeroHeight::Sm => "300px",
            HeroHeight::Md => "400px",
            HeroHeight::Lg => "500px",
            HeroHeight::Xl => "600px",
            HeroHeight::Full => "100vh",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeroAlign {
    Left,
    Center,
    Right,
}

impl HeroAlign {
    fn as_str(&self) -> &'static str {
        match self {
            HeroAlign::Left => "flex-start",
            HeroAlign::Center => "center",
            HeroAlign::Right => "flex-end",
        }
    }
}

#[component]
pub fn Hero(
    #[prop(optional)] height: Option<HeroHeight>,
    #[prop(optional)] align: Option<HeroAlign>,
    #[prop(optional)] background_color: Option<String>,
    #[prop(optional)] background_image: Option<String>,
    #[prop(optional)] overlay: bool,
    #[prop(optional)] overlay_opacity: Option<f32>,
    #[prop(optional)] padding: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let height = height.unwrap_or(HeroHeight::Md);
    let align = align.unwrap_or(HeroAlign::Center);
    let overlay_opacity = overlay_opacity.unwrap_or(0.6);

    let hero_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("position", "relative")
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("align-items", align.as_str())
            .add("justify-content", "center")
            .add("min-height", height.value())
            .add("overflow", "hidden");

        if let Some(p) = padding.as_ref() {
            builder.add("padding", p);
        } else {
            builder.add("padding", format!("{} {}", theme_val.spacing.xl, theme_val.spacing.md));
        }

        if let Some(bg_img) = background_image.as_ref() {
            builder
                .add("background-image", format!("url('{}')", bg_img))
                .add("background-size", "cover")
                .add("background-position", "center");
        } else if let Some(bg_color) = background_color.as_ref() {
            builder.add("background-color", bg_color);
        } else {
            let default_bg = scheme_colors
                .get_color("gray", 0)
                .unwrap_or_else(|| "#f8f9fa".to_string());
            builder.add("background-color", default_bg);
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let overlay_styles = move || {
        if !overlay {
            return "display: none;".to_string();
        }

        format!(
            "position: absolute; \
             top: 0; \
             left: 0; \
             right: 0; \
             bottom: 0; \
             background-color: rgba(0, 0, 0, {}); \
             z-index: 1;",
            overlay_opacity
        )
    };

    let content_styles = move || {
        "position: relative; z-index: 2; max-width: 100%; text-align: inherit;".to_string()
    };

    let class_str = format!("mingot-hero {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=hero_styles>
            <div class="mingot-hero-overlay" style=overlay_styles></div>
            <div class="mingot-hero-content" style=content_styles>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn HeroTitle(
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();

    let title_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("font-size", "3rem")
            .add("font-weight", theme_val.typography.font_weights.bold.to_string())
            .add("line-height", "1.2")
            .add("margin", "0")
            .add("margin-bottom", theme_val.spacing.md);

        if let Some(c) = color.as_ref() {
            builder.add("color", c);
        } else {
            builder.add("color", scheme_colors.text.clone());
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-hero-title {}", class.unwrap_or_default());

    view! {
        <h1 class=class_str style=title_styles>
            {children()}
        </h1>
    }
}

#[component]
pub fn HeroSubtitle(
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();

    let subtitle_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("font-size", theme_val.typography.font_sizes.xl)
            .add("font-weight", theme_val.typography.font_weights.normal.to_string())
            .add("line-height", "1.5")
            .add("margin", "0")
            .add("margin-bottom", theme_val.spacing.lg)
            .add("opacity", "0.9");

        if let Some(c) = color.as_ref() {
            builder.add("color", c);
        } else {
            builder.add("color", scheme_colors.text.clone());
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-hero-subtitle {}", class.unwrap_or_default());

    view! {
        <p class=class_str style=subtitle_styles>
            {children()}
        </p>
    }
}

#[component]
pub fn HeroActions(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();

    let actions_styles = move || {
        let theme_val = theme.get();
        format!(
            "display: flex; \
             gap: {}; \
             flex-wrap: wrap; \
             align-items: center; \
             margin-top: {};",
            theme_val.spacing.md, theme_val.spacing.md
        )
    };

    let class_str = format!("mingot-hero-actions {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=move || {
            if let Some(s) = style.as_ref() {
                format!("{}; {}", actions_styles(), s)
            } else {
                actions_styles()
            }
        }>
            {children()}
        </div>
    }
}
