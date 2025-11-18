use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

/// Responsive column span configuration
#[derive(Clone, Debug)]
pub struct ColSpan {
    pub xs: Option<u32>,
    pub sm: Option<u32>,
    pub md: Option<u32>,
    pub lg: Option<u32>,
    pub xl: Option<u32>,
}

impl ColSpan {
    pub fn new(default_span: u32) -> Self {
        Self {
            xs: Some(default_span),
            sm: None,
            md: None,
            lg: None,
            xl: None,
        }
    }

    pub fn xs(mut self, span: u32) -> Self {
        self.xs = Some(span);
        self
    }

    pub fn sm(mut self, span: u32) -> Self {
        self.sm = Some(span);
        self
    }

    pub fn md(mut self, span: u32) -> Self {
        self.md = Some(span);
        self
    }

    pub fn lg(mut self, span: u32) -> Self {
        self.lg = Some(span);
        self
    }

    pub fn xl(mut self, span: u32) -> Self {
        self.xl = Some(span);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridAlign {
    Start,
    Center,
    End,
    Stretch,
}

impl GridAlign {
    fn as_str(&self) -> &'static str {
        match self {
            GridAlign::Start => "start",
            GridAlign::Center => "center",
            GridAlign::End => "end",
            GridAlign::Stretch => "stretch",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

impl GridJustify {
    fn as_str(&self) -> &'static str {
        match self {
            GridJustify::Start => "start",
            GridJustify::Center => "center",
            GridJustify::End => "end",
            GridJustify::SpaceBetween => "space-between",
            GridJustify::SpaceAround => "space-around",
        }
    }
}

#[component]
pub fn Grid(
    #[prop(optional)] columns: Option<u32>,
    #[prop(optional)] gutter: Option<String>,
    #[prop(optional)] align: Option<GridAlign>,
    #[prop(optional)] justify: Option<GridJustify>,
    #[prop(optional)] _grow: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let columns = columns.unwrap_or(12);

    let grid_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        builder.add("display", "grid").add(
            "grid-template-columns",
            format!("repeat({}, minmax(0, 1fr))", columns),
        );

        if let Some(g) = gutter.as_ref() {
            builder.add("gap", g);
        } else {
            builder.add("gap", theme_val.spacing.md);
        }

        if let Some(a) = align {
            builder.add("align-items", a.as_str());
        }

        if let Some(j) = justify {
            builder.add("justify-content", j.as_str());
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    // Provide grid columns context so GridCol can access it
    provide_context::<Signal<u32>>(Signal::derive(move || columns));

    let class_str = format!("mingot-grid {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=grid_styles>
            {children()}
        </div>
    }
}

#[component]
pub fn GridCol(
    #[prop(optional)] span: Option<u32>,
    #[prop(optional)] offset: Option<u32>,
    #[prop(optional)] xs: Option<u32>,
    #[prop(optional)] sm: Option<u32>,
    #[prop(optional)] md: Option<u32>,
    #[prop(optional)] lg: Option<u32>,
    #[prop(optional)] xl: Option<u32>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let grid_columns = use_context::<Signal<u32>>().unwrap_or(Signal::derive(move || 12));

    let default_span = span.unwrap_or(12);

    let col_styles = move || {
        let theme_val = theme.get();
        let total_cols = grid_columns.get();
        let mut builder = StyleBuilder::new();

        // Base span
        let base_span = default_span.min(total_cols);
        builder.add("grid-column", format!("span {}", base_span));

        // Offset
        if let Some(off) = offset {
            let current_style = builder.build();
            let parts: Vec<&str> = current_style.split(": ").collect();
            if parts.len() == 2 {
                let span_part = parts[1].trim_end_matches(';');
                builder = StyleBuilder::new();
                builder.add("grid-column", format!("{} / {}", off + 1, span_part));
            }
        }

        // Build responsive styles using CSS custom properties and media queries
        let mut responsive_styles = String::new();

        if let Some(span_xs) = xs {
            responsive_styles.push_str(&format!(
                "@media (min-width: {}) {{ .mingot-grid-col {{ grid-column: span {}; }} }} ",
                theme_val.breakpoints.xs,
                span_xs.min(total_cols)
            ));
        }

        if let Some(span_sm) = sm {
            responsive_styles.push_str(&format!(
                "@media (min-width: {}) {{ .mingot-grid-col {{ grid-column: span {}; }} }} ",
                theme_val.breakpoints.sm,
                span_sm.min(total_cols)
            ));
        }

        if let Some(span_md) = md {
            responsive_styles.push_str(&format!(
                "@media (min-width: {}) {{ .mingot-grid-col {{ grid-column: span {}; }} }} ",
                theme_val.breakpoints.md,
                span_md.min(total_cols)
            ));
        }

        if let Some(span_lg) = lg {
            responsive_styles.push_str(&format!(
                "@media (min-width: {}) {{ .mingot-grid-col {{ grid-column: span {}; }} }} ",
                theme_val.breakpoints.lg,
                span_lg.min(total_cols)
            ));
        }

        if let Some(span_xl) = xl {
            responsive_styles.push_str(&format!(
                "@media (min-width: {}) {{ .mingot-grid-col {{ grid-column: span {}; }} }} ",
                theme_val.breakpoints.xl,
                span_xl.min(total_cols)
            ));
        }

        if let Some(s) = style.as_ref() {
            format!("{}; {}", builder.build(), s)
        } else {
            builder.build()
        }
    };

    let class_str = format!("mingot-grid-col {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=col_styles>
            {children()}
        </div>
    }
}

/// SimpleGrid - A simpler grid with auto-fit columns
#[component]
pub fn SimpleGrid(
    #[prop(optional)] cols: Option<u32>,
    #[prop(optional)] spacing: Option<String>,
    #[prop(optional)] min_child_width: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();

    let grid_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        builder.add("display", "grid");

        if let Some(min_width) = min_child_width.as_ref() {
            // Auto-fit based on minimum child width
            builder.add(
                "grid-template-columns",
                format!("repeat(auto-fit, minmax({}, 1fr))", min_width),
            );
        } else {
            // Fixed number of columns
            let columns = cols.unwrap_or(1);
            builder.add(
                "grid-template-columns",
                format!("repeat({}, minmax(0, 1fr))", columns),
            );
        }

        if let Some(s) = spacing.as_ref() {
            builder.add("gap", s);
        } else {
            builder.add("gap", theme_val.spacing.md);
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-simple-grid {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=grid_styles>
            {children()}
        </div>
    }
}
