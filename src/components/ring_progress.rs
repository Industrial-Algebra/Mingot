use crate::theme::use_theme;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RingProgressSize {
    Xs,  // 44px
    Sm,  // 60px
    Md,  // 80px
    Lg,  // 120px
    Xl,  // 160px
}

impl RingProgressSize {
    fn to_px(&self) -> u32 {
        match self {
            RingProgressSize::Xs => 44,
            RingProgressSize::Sm => 60,
            RingProgressSize::Md => 80,
            RingProgressSize::Lg => 120,
            RingProgressSize::Xl => 160,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RingProgressSection {
    pub value: f32,
    pub color: String,
    pub tooltip: Option<String>,
}

impl RingProgressSection {
    pub fn new(value: f32, color: impl Into<String>) -> Self {
        Self {
            value,
            color: color.into(),
            tooltip: None,
        }
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

#[component]
pub fn RingProgress(
    #[prop(optional)] sections: Vec<RingProgressSection>,
    #[prop(optional)] size: Option<RingProgressSize>,
    #[prop(optional)] thickness: Option<u32>,
    #[prop(optional)] label: Option<Children>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or(RingProgressSize::Md);
    let size_px = size.to_px();
    let thickness = thickness.unwrap_or(8);

    let container_styles = move || {
        format!(
            "position: relative; \
             width: {}px; \
             height: {}px; \
             display: inline-flex; \
             align-items: center; \
             justify-content: center;",
            size_px, size_px
        )
    };

    let svg_styles = move || {
        "transform: rotate(-90deg); \
         position: absolute; \
         top: 0; \
         left: 0;"
            .to_string()
    };

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "position: absolute; \
             display: flex; \
             align-items: center; \
             justify-content: center; \
             font-weight: {}; \
             color: {};",
            theme_val.typography.font_weights.bold, scheme_colors.text
        )
    };

    // Calculate SVG properties
    let radius = (size_px / 2) - (thickness / 2);
    let circumference = 2.0 * std::f32::consts::PI * radius as f32;

    // Build sections with cumulative offsets
    let mut current_offset = 0.0;
    let ring_sections: Vec<_> = sections
        .iter()
        .map(|section| {
            let stroke_dasharray = format!("{} {}", (circumference * section.value / 100.0), circumference);
            let stroke_dashoffset = -(current_offset * circumference / 100.0);
            current_offset += section.value;

            (section.color.clone(), stroke_dasharray, stroke_dashoffset)
        })
        .collect();

    let class_str = format!("mingot-ring-progress {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", container_styles(), s)
                } else {
                    container_styles()
                }
            }
        >

            <svg width=size_px height=size_px style=svg_styles>
                // Background circle
                <circle
                    cx=size_px / 2
                    cy=size_px / 2
                    r=radius
                    fill="none"
                    stroke={move || {
                        let theme_val = theme.get();
                        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                        scheme_colors.border.clone()
                    }}

                    stroke-width=thickness
                />
                // Progress sections
                {ring_sections
                    .into_iter()
                    .map(|(color, dasharray, offset)| {
                        view! {
                            <circle
                                cx=size_px / 2
                                cy=size_px / 2
                                r=radius
                                fill="none"
                                stroke=color
                                stroke-width=thickness
                                stroke-dasharray=dasharray
                                stroke-dashoffset=offset
                                stroke-linecap="round"
                            />
                        }
                    })
                    .collect::<Vec<_>>()}

            </svg>

            {label.map(|l| view! { <div style=label_styles>{l()}</div> })}
        </div>
    }
}
