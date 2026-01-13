//! Symbol palette component for selecting mathematical symbols.
//!
//! Provides a searchable, categorized picker for Greek letters, mathematical
//! operators, set theory symbols, logic symbols, and relations.

use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

/// Categories of mathematical symbols
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SymbolCategory {
    /// Greek lowercase and uppercase letters (α, β, Γ, Δ, etc.)
    #[default]
    Greek,
    /// Mathematical operators (∑, ∏, ∫, ∂, ∇, etc.)
    Operators,
    /// Set theory symbols (∈, ⊂, ∪, ∩, ∅, etc.)
    SetTheory,
    /// Logic symbols (∀, ∃, ∧, ∨, ¬, ⇒, etc.)
    Logic,
    /// Arrows (→, ←, ↔, ⇒, ⇐, etc.)
    Arrows,
    /// Relations (≤, ≥, ≠, ≈, ≡, ∝, etc.)
    Relations,
    /// Miscellaneous (∞, ℏ, ℵ, etc.)
    Miscellaneous,
}

impl SymbolCategory {
    /// Get display name for the category
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Greek => "Greek",
            Self::Operators => "Operators",
            Self::SetTheory => "Sets",
            Self::Logic => "Logic",
            Self::Arrows => "Arrows",
            Self::Relations => "Relations",
            Self::Miscellaneous => "Misc",
        }
    }

    /// Get all categories
    pub fn all() -> Vec<Self> {
        vec![
            Self::Greek,
            Self::Operators,
            Self::SetTheory,
            Self::Logic,
            Self::Arrows,
            Self::Relations,
            Self::Miscellaneous,
        ]
    }
}

/// A mathematical symbol with metadata
#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    /// The Unicode character
    pub char: &'static str,
    /// Human-readable name
    pub name: &'static str,
    /// LaTeX equivalent (if any)
    pub latex: Option<&'static str>,
    /// Category this symbol belongs to
    pub category: SymbolCategory,
}

impl Symbol {
    /// Create a new symbol
    pub const fn new(
        char: &'static str,
        name: &'static str,
        latex: Option<&'static str>,
        category: SymbolCategory,
    ) -> Self {
        Self {
            char,
            name,
            latex,
            category,
        }
    }
}

/// Get all available symbols
pub fn get_all_symbols() -> Vec<Symbol> {
    vec![
        // Greek lowercase
        Symbol::new("α", "alpha", Some("\\alpha"), SymbolCategory::Greek),
        Symbol::new("β", "beta", Some("\\beta"), SymbolCategory::Greek),
        Symbol::new("γ", "gamma", Some("\\gamma"), SymbolCategory::Greek),
        Symbol::new("δ", "delta", Some("\\delta"), SymbolCategory::Greek),
        Symbol::new("ε", "epsilon", Some("\\epsilon"), SymbolCategory::Greek),
        Symbol::new("ζ", "zeta", Some("\\zeta"), SymbolCategory::Greek),
        Symbol::new("η", "eta", Some("\\eta"), SymbolCategory::Greek),
        Symbol::new("θ", "theta", Some("\\theta"), SymbolCategory::Greek),
        Symbol::new("ι", "iota", Some("\\iota"), SymbolCategory::Greek),
        Symbol::new("κ", "kappa", Some("\\kappa"), SymbolCategory::Greek),
        Symbol::new("λ", "lambda", Some("\\lambda"), SymbolCategory::Greek),
        Symbol::new("μ", "mu", Some("\\mu"), SymbolCategory::Greek),
        Symbol::new("ν", "nu", Some("\\nu"), SymbolCategory::Greek),
        Symbol::new("ξ", "xi", Some("\\xi"), SymbolCategory::Greek),
        Symbol::new("π", "pi", Some("\\pi"), SymbolCategory::Greek),
        Symbol::new("ρ", "rho", Some("\\rho"), SymbolCategory::Greek),
        Symbol::new("σ", "sigma", Some("\\sigma"), SymbolCategory::Greek),
        Symbol::new("τ", "tau", Some("\\tau"), SymbolCategory::Greek),
        Symbol::new("υ", "upsilon", Some("\\upsilon"), SymbolCategory::Greek),
        Symbol::new("φ", "phi", Some("\\phi"), SymbolCategory::Greek),
        Symbol::new("χ", "chi", Some("\\chi"), SymbolCategory::Greek),
        Symbol::new("ψ", "psi", Some("\\psi"), SymbolCategory::Greek),
        Symbol::new("ω", "omega", Some("\\omega"), SymbolCategory::Greek),
        // Greek uppercase
        Symbol::new("Γ", "Gamma", Some("\\Gamma"), SymbolCategory::Greek),
        Symbol::new("Δ", "Delta", Some("\\Delta"), SymbolCategory::Greek),
        Symbol::new("Θ", "Theta", Some("\\Theta"), SymbolCategory::Greek),
        Symbol::new("Λ", "Lambda", Some("\\Lambda"), SymbolCategory::Greek),
        Symbol::new("Ξ", "Xi", Some("\\Xi"), SymbolCategory::Greek),
        Symbol::new("Π", "Pi", Some("\\Pi"), SymbolCategory::Greek),
        Symbol::new("Σ", "Sigma", Some("\\Sigma"), SymbolCategory::Greek),
        Symbol::new("Φ", "Phi", Some("\\Phi"), SymbolCategory::Greek),
        Symbol::new("Ψ", "Psi", Some("\\Psi"), SymbolCategory::Greek),
        Symbol::new("Ω", "Omega", Some("\\Omega"), SymbolCategory::Greek),
        // Variant Greek
        Symbol::new(
            "ϵ",
            "varepsilon",
            Some("\\varepsilon"),
            SymbolCategory::Greek,
        ),
        Symbol::new("ϑ", "vartheta", Some("\\vartheta"), SymbolCategory::Greek),
        Symbol::new("ϕ", "varphi", Some("\\varphi"), SymbolCategory::Greek),
        Symbol::new("ϱ", "varrho", Some("\\varrho"), SymbolCategory::Greek),
        Symbol::new("ς", "varsigma", Some("\\varsigma"), SymbolCategory::Greek),
        // Operators
        Symbol::new("∑", "summation", Some("\\sum"), SymbolCategory::Operators),
        Symbol::new("∏", "product", Some("\\prod"), SymbolCategory::Operators),
        Symbol::new("∫", "integral", Some("\\int"), SymbolCategory::Operators),
        Symbol::new(
            "∬",
            "double integral",
            Some("\\iint"),
            SymbolCategory::Operators,
        ),
        Symbol::new(
            "∭",
            "triple integral",
            Some("\\iiint"),
            SymbolCategory::Operators,
        ),
        Symbol::new(
            "∮",
            "contour integral",
            Some("\\oint"),
            SymbolCategory::Operators,
        ),
        Symbol::new("∂", "partial", Some("\\partial"), SymbolCategory::Operators),
        Symbol::new("∇", "nabla/del", Some("\\nabla"), SymbolCategory::Operators),
        Symbol::new(
            "√",
            "square root",
            Some("\\sqrt"),
            SymbolCategory::Operators,
        ),
        Symbol::new(
            "∛",
            "cube root",
            Some("\\sqrt[3]"),
            SymbolCategory::Operators,
        ),
        Symbol::new(
            "∜",
            "fourth root",
            Some("\\sqrt[4]"),
            SymbolCategory::Operators,
        ),
        Symbol::new("±", "plus-minus", Some("\\pm"), SymbolCategory::Operators),
        Symbol::new("∓", "minus-plus", Some("\\mp"), SymbolCategory::Operators),
        Symbol::new("×", "times", Some("\\times"), SymbolCategory::Operators),
        Symbol::new("÷", "division", Some("\\div"), SymbolCategory::Operators),
        Symbol::new("·", "center dot", Some("\\cdot"), SymbolCategory::Operators),
        Symbol::new(
            "∘",
            "composition",
            Some("\\circ"),
            SymbolCategory::Operators,
        ),
        Symbol::new(
            "⊗",
            "tensor product",
            Some("\\otimes"),
            SymbolCategory::Operators,
        ),
        Symbol::new(
            "⊕",
            "direct sum",
            Some("\\oplus"),
            SymbolCategory::Operators,
        ),
        Symbol::new(
            "⊙",
            "circled dot",
            Some("\\odot"),
            SymbolCategory::Operators,
        ),
        Symbol::new("†", "dagger", Some("\\dagger"), SymbolCategory::Operators),
        Symbol::new(
            "‡",
            "double dagger",
            Some("\\ddagger"),
            SymbolCategory::Operators,
        ),
        Symbol::new("∗", "asterisk", Some("\\ast"), SymbolCategory::Operators),
        Symbol::new("⋆", "star", Some("\\star"), SymbolCategory::Operators),
        // Set theory
        Symbol::new("∈", "element of", Some("\\in"), SymbolCategory::SetTheory),
        Symbol::new(
            "∉",
            "not element of",
            Some("\\notin"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new("∋", "contains", Some("\\ni"), SymbolCategory::SetTheory),
        Symbol::new("⊂", "subset", Some("\\subset"), SymbolCategory::SetTheory),
        Symbol::new("⊃", "superset", Some("\\supset"), SymbolCategory::SetTheory),
        Symbol::new(
            "⊆",
            "subset or equal",
            Some("\\subseteq"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "⊇",
            "superset or equal",
            Some("\\supseteq"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "⊄",
            "not subset",
            Some("\\not\\subset"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "⊈",
            "not subset or equal",
            Some("\\nsubseteq"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new("∪", "union", Some("\\cup"), SymbolCategory::SetTheory),
        Symbol::new(
            "∩",
            "intersection",
            Some("\\cap"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "∅",
            "empty set",
            Some("\\emptyset"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "∖",
            "set minus",
            Some("\\setminus"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "⊎",
            "multiset union",
            Some("\\uplus"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "⊔",
            "square union",
            Some("\\sqcup"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "⊓",
            "square intersection",
            Some("\\sqcap"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new("℘", "power set", Some("\\wp"), SymbolCategory::SetTheory),
        Symbol::new(
            "ℕ",
            "natural numbers",
            Some("\\mathbb{N}"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "ℤ",
            "integers",
            Some("\\mathbb{Z}"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new(
            "ℚ",
            "rationals",
            Some("\\mathbb{Q}"),
            SymbolCategory::SetTheory,
        ),
        Symbol::new("ℝ", "reals", Some("\\mathbb{R}"), SymbolCategory::SetTheory),
        Symbol::new(
            "ℂ",
            "complex",
            Some("\\mathbb{C}"),
            SymbolCategory::SetTheory,
        ),
        // Logic
        Symbol::new("∀", "for all", Some("\\forall"), SymbolCategory::Logic),
        Symbol::new("∃", "exists", Some("\\exists"), SymbolCategory::Logic),
        Symbol::new("∄", "not exists", Some("\\nexists"), SymbolCategory::Logic),
        Symbol::new("∧", "logical and", Some("\\land"), SymbolCategory::Logic),
        Symbol::new("∨", "logical or", Some("\\lor"), SymbolCategory::Logic),
        Symbol::new("¬", "logical not", Some("\\neg"), SymbolCategory::Logic),
        Symbol::new("⊻", "xor", Some("\\veebar"), SymbolCategory::Logic),
        Symbol::new("⊼", "nand", Some("\\barwedge"), SymbolCategory::Logic),
        Symbol::new("⊽", "nor", None, SymbolCategory::Logic),
        Symbol::new("⇒", "implies", Some("\\Rightarrow"), SymbolCategory::Logic),
        Symbol::new(
            "⇐",
            "implied by",
            Some("\\Leftarrow"),
            SymbolCategory::Logic,
        ),
        Symbol::new("⇔", "iff", Some("\\Leftrightarrow"), SymbolCategory::Logic),
        Symbol::new("⊢", "proves", Some("\\vdash"), SymbolCategory::Logic),
        Symbol::new("⊣", "proved by", Some("\\dashv"), SymbolCategory::Logic),
        Symbol::new("⊨", "models", Some("\\models"), SymbolCategory::Logic),
        Symbol::new("⊤", "tautology/top", Some("\\top"), SymbolCategory::Logic),
        Symbol::new(
            "⊥",
            "contradiction/bottom",
            Some("\\bot"),
            SymbolCategory::Logic,
        ),
        Symbol::new("□", "necessity", Some("\\Box"), SymbolCategory::Logic),
        Symbol::new("◇", "possibility", Some("\\Diamond"), SymbolCategory::Logic),
        // Arrows
        Symbol::new(
            "→",
            "right arrow",
            Some("\\rightarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "←",
            "left arrow",
            Some("\\leftarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "↔",
            "left-right arrow",
            Some("\\leftrightarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new("↑", "up arrow", Some("\\uparrow"), SymbolCategory::Arrows),
        Symbol::new(
            "↓",
            "down arrow",
            Some("\\downarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "↕",
            "up-down arrow",
            Some("\\updownarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "⇒",
            "double right",
            Some("\\Rightarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "⇐",
            "double left",
            Some("\\Leftarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "⇔",
            "double left-right",
            Some("\\Leftrightarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new("⇑", "double up", Some("\\Uparrow"), SymbolCategory::Arrows),
        Symbol::new(
            "⇓",
            "double down",
            Some("\\Downarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new("↦", "maps to", Some("\\mapsto"), SymbolCategory::Arrows),
        Symbol::new(
            "↪",
            "hook right",
            Some("\\hookrightarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "↩",
            "hook left",
            Some("\\hookleftarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "↠",
            "two-head right",
            Some("\\twoheadrightarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "↞",
            "two-head left",
            Some("\\twoheadleftarrow"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "⇀",
            "harpoon right",
            Some("\\rightharpoonup"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "↼",
            "harpoon left",
            Some("\\leftharpoonup"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "⇌",
            "equilibrium",
            Some("\\rightleftharpoons"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "↺",
            "anticlockwise",
            Some("\\circlearrowleft"),
            SymbolCategory::Arrows,
        ),
        Symbol::new(
            "↻",
            "clockwise",
            Some("\\circlearrowright"),
            SymbolCategory::Arrows,
        ),
        // Relations
        Symbol::new("=", "equals", Some("="), SymbolCategory::Relations),
        Symbol::new("≠", "not equal", Some("\\neq"), SymbolCategory::Relations),
        Symbol::new("<", "less than", Some("<"), SymbolCategory::Relations),
        Symbol::new(">", "greater than", Some(">"), SymbolCategory::Relations),
        Symbol::new(
            "≤",
            "less or equal",
            Some("\\leq"),
            SymbolCategory::Relations,
        ),
        Symbol::new(
            "≥",
            "greater or equal",
            Some("\\geq"),
            SymbolCategory::Relations,
        ),
        Symbol::new("≪", "much less", Some("\\ll"), SymbolCategory::Relations),
        Symbol::new("≫", "much greater", Some("\\gg"), SymbolCategory::Relations),
        Symbol::new(
            "≈",
            "approximately",
            Some("\\approx"),
            SymbolCategory::Relations,
        ),
        Symbol::new(
            "≃",
            "asymptotically equal",
            Some("\\simeq"),
            SymbolCategory::Relations,
        ),
        Symbol::new("≅", "congruent", Some("\\cong"), SymbolCategory::Relations),
        Symbol::new("∼", "similar", Some("\\sim"), SymbolCategory::Relations),
        Symbol::new(
            "≡",
            "identical/equivalent",
            Some("\\equiv"),
            SymbolCategory::Relations,
        ),
        Symbol::new(
            "≢",
            "not identical",
            Some("\\not\\equiv"),
            SymbolCategory::Relations,
        ),
        Symbol::new(
            "∝",
            "proportional",
            Some("\\propto"),
            SymbolCategory::Relations,
        ),
        Symbol::new("≺", "precedes", Some("\\prec"), SymbolCategory::Relations),
        Symbol::new("≻", "succeeds", Some("\\succ"), SymbolCategory::Relations),
        Symbol::new(
            "⊥",
            "perpendicular",
            Some("\\perp"),
            SymbolCategory::Relations,
        ),
        Symbol::new(
            "∥",
            "parallel",
            Some("\\parallel"),
            SymbolCategory::Relations,
        ),
        Symbol::new(
            "∦",
            "not parallel",
            Some("\\nparallel"),
            SymbolCategory::Relations,
        ),
        Symbol::new(
            "≍",
            "equivalent",
            Some("\\asymp"),
            SymbolCategory::Relations,
        ),
        Symbol::new(
            "≎",
            "geometrically equivalent",
            Some("\\Bumpeq"),
            SymbolCategory::Relations,
        ),
        // Miscellaneous
        Symbol::new(
            "∞",
            "infinity",
            Some("\\infty"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new("ℏ", "h-bar", Some("\\hbar"), SymbolCategory::Miscellaneous),
        Symbol::new("ℵ", "aleph", Some("\\aleph"), SymbolCategory::Miscellaneous),
        Symbol::new("ℶ", "beth", Some("\\beth"), SymbolCategory::Miscellaneous),
        Symbol::new("ℷ", "gimel", Some("\\gimel"), SymbolCategory::Miscellaneous),
        Symbol::new("℩", "turned iota", None, SymbolCategory::Miscellaneous),
        Symbol::new(
            "ℓ",
            "script l",
            Some("\\ell"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "℘",
            "Weierstrass p",
            Some("\\wp"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "ℜ",
            "real part",
            Some("\\Re"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "ℑ",
            "imaginary part",
            Some("\\Im"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new("∠", "angle", Some("\\angle"), SymbolCategory::Miscellaneous),
        Symbol::new(
            "∡",
            "measured angle",
            Some("\\measuredangle"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "∢",
            "spherical angle",
            Some("\\sphericalangle"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "°",
            "degree",
            Some("^\\circ"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new("′", "prime", Some("'"), SymbolCategory::Miscellaneous),
        Symbol::new(
            "″",
            "double prime",
            Some("''"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "‴",
            "triple prime",
            Some("'''"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "∴",
            "therefore",
            Some("\\therefore"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "∵",
            "because",
            Some("\\because"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "…",
            "ellipsis",
            Some("\\ldots"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "⋯",
            "center ellipsis",
            Some("\\cdots"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "⋮",
            "vertical ellipsis",
            Some("\\vdots"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "⋱",
            "diagonal ellipsis",
            Some("\\ddots"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "ℯ",
            "euler's number",
            Some("e"),
            SymbolCategory::Miscellaneous,
        ),
        Symbol::new(
            "ⅈ",
            "imaginary unit",
            Some("i"),
            SymbolCategory::Miscellaneous,
        ),
    ]
}

/// Get symbols filtered by categories
pub fn get_symbols_by_categories(categories: &[SymbolCategory]) -> Vec<Symbol> {
    get_all_symbols()
        .into_iter()
        .filter(|s| categories.contains(&s.category))
        .collect()
}

/// Symbol palette component for selecting mathematical symbols
#[component]
pub fn SymbolPalette(
    /// Categories to display (defaults to all)
    #[prop(optional, into)]
    categories: Option<Vec<SymbolCategory>>,

    /// Callback when a symbol is selected
    #[prop(optional, into)]
    on_select: Option<Callback<Symbol>>,

    /// Whether to show the search box
    #[prop(optional, default = true)]
    searchable: bool,

    /// Whether to show category tabs
    #[prop(optional, default = true)]
    show_tabs: bool,

    /// Number of columns in the grid
    #[prop(optional, default = 8)]
    columns: usize,

    /// Whether to show symbol names on hover
    #[prop(optional, default = true)]
    show_tooltip: bool,

    /// Whether to show LaTeX in tooltip
    #[prop(optional, default = true)]
    show_latex: bool,

    /// Label for the palette
    #[prop(optional, into)]
    label: Option<String>,
) -> impl IntoView {
    let theme = use_theme();

    // Available categories
    let available_categories = categories.unwrap_or_else(SymbolCategory::all);
    let categories_for_filter = available_categories.clone();
    let categories_for_tabs = available_categories.clone();

    // State
    let search_query = RwSignal::new(String::new());
    let active_category: RwSignal<Option<SymbolCategory>> = RwSignal::new(None);

    // Get filtered symbols (using Memo so it can be used in multiple closures)
    let filtered_symbols = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        let cat = active_category.get();
        let cats = categories_for_filter.clone();

        get_all_symbols()
            .into_iter()
            .filter(|s| {
                // Filter by category
                let in_category = if let Some(c) = cat {
                    s.category == c
                } else {
                    cats.contains(&s.category)
                };

                // Filter by search
                let matches_search = if query.is_empty() {
                    true
                } else {
                    s.name.to_lowercase().contains(&query)
                        || s.char.contains(&query)
                        || s.latex
                            .map(|l| l.to_lowercase().contains(&query))
                            .unwrap_or(false)
                };

                in_category && matches_search
            })
            .collect::<Vec<_>>()
    });

    // Styles
    let container_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("gap", theme_val.spacing.sm)
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.md)
            .add("padding", theme_val.spacing.sm)
            .add("background", scheme_colors.background.clone())
            .build()
    };

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add(
                "font-weight",
                theme_val.typography.font_weights.medium.to_string(),
            )
            .add("color", scheme_colors.text.clone())
            .add("margin-bottom", theme_val.spacing.xs)
            .build()
    };

    let search_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("width", "100%")
            .add(
                "padding",
                format!("{} {}", theme_val.spacing.xs, theme_val.spacing.sm),
            )
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("background", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("outline", "none")
            .build()
    };

    let tabs_styles = move || {
        let theme_val = theme.get();
        StyleBuilder::new()
            .add("display", "flex")
            .add("flex-wrap", "wrap")
            .add("gap", theme_val.spacing.xs)
            .add("margin-bottom", theme_val.spacing.xs)
            .build()
    };

    let grid_styles = move || {
        StyleBuilder::new()
            .add("display", "grid")
            .add("grid-template-columns", format!("repeat({}, 1fr)", columns))
            .add("gap", "2px")
            .add("max-height", "300px")
            .add("overflow-y", "auto")
            .build()
    };

    view! {
        <div class="mingot-symbol-palette" style=container_styles>
            {label.map(|l| view! {
                <div style=label_styles>{l}</div>
            })}

            {searchable.then(|| {
                view! {
                    <input
                        type="text"
                        placeholder="Search symbols..."
                        style=search_styles
                        prop:value=move || search_query.get()
                        on:input=move |ev| {
                            search_query.set(event_target_value(&ev));
                        }
                    />
                }
            })}

            {show_tabs.then(|| {
                let cats = categories_for_tabs.clone();
                view! {
                    <div style=tabs_styles>
                        <TabButton
                            label="All"
                            active=move || active_category.get().is_none()
                            on_click=move |_| active_category.set(None)
                        />
                        {cats.into_iter().map(|cat| {
                            view! {
                                <TabButton
                                    label=cat.display_name()
                                    active=move || active_category.get() == Some(cat)
                                    on_click=move |_| active_category.set(Some(cat))
                                />
                            }
                        }).collect_view()}
                    </div>
                }
            })}

            <div style=grid_styles>
                {move || {
                    filtered_symbols.get().into_iter().map(|symbol| {
                        let sym = symbol.clone();
                        let sym_for_click = symbol.clone();
                        view! {
                            <SymbolButton
                                symbol=sym
                                show_tooltip=show_tooltip
                                show_latex=show_latex
                                on_click=move |_| {
                                    if let Some(cb) = on_select {
                                        cb.run(sym_for_click.clone());
                                    }
                                }
                            />
                        }
                    }).collect_view()
                }}
            </div>

            <div style=move || {
                let theme_val = theme.get();
                let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                StyleBuilder::new()
                    .add("font-size", theme_val.typography.font_sizes.xs)
                    .add("color", scheme_colors.get_color("gray", 6).unwrap_or_else(|| "#868e96".to_string()))
                    .add("text-align", "center")
                    .build()
            }>
                {move || format!("{} symbols", filtered_symbols.get().len())}
            </div>
        </div>
    }
}

/// Internal tab button component
#[component]
fn TabButton(
    label: &'static str,
    active: impl Fn() -> bool + Send + Sync + 'static,
    on_click: impl Fn(ev::MouseEvent) + Send + Sync + 'static,
) -> impl IntoView {
    let theme = use_theme();

    let styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_active = active();

        StyleBuilder::new()
            .add(
                "padding",
                format!("{} {}", theme_val.spacing.xs, theme_val.spacing.sm),
            )
            .add("border", "none")
            .add("border-radius", theme_val.radius.sm)
            .add("cursor", "pointer")
            .add("font-size", theme_val.typography.font_sizes.xs)
            .add(
                "background",
                if is_active {
                    scheme_colors
                        .get_color(&theme_val.colors.primary_color, 6)
                        .unwrap_or_else(|| "#228be6".to_string())
                } else {
                    scheme_colors
                        .get_color("gray", 1)
                        .unwrap_or_else(|| "#f1f3f5".to_string())
                },
            )
            .add(
                "color",
                if is_active {
                    "#ffffff".to_string()
                } else {
                    scheme_colors.text.clone()
                },
            )
            .add(
                "font-weight",
                if is_active {
                    theme_val.typography.font_weights.semibold.to_string()
                } else {
                    theme_val.typography.font_weights.normal.to_string()
                },
            )
            .build()
    };

    view! {
        <button type="button" style=styles on:click=on_click>
            {label}
        </button>
    }
}

/// Internal symbol button component
#[component]
fn SymbolButton(
    symbol: Symbol,
    show_tooltip: bool,
    show_latex: bool,
    on_click: impl Fn(ev::MouseEvent) + Send + Sync + 'static,
) -> impl IntoView {
    let theme = use_theme();
    let is_hovered = RwSignal::new(false);

    let styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let hovered = is_hovered.get();

        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("width", "32px")
            .add("height", "32px")
            .add("border", "none")
            .add("border-radius", theme_val.radius.sm)
            .add("cursor", "pointer")
            .add("font-size", "1.25rem")
            .add(
                "background",
                if hovered {
                    scheme_colors
                        .get_color("gray", 2)
                        .unwrap_or_else(|| "#e9ecef".to_string())
                } else {
                    "transparent".to_string()
                },
            )
            .add("color", scheme_colors.text.clone())
            .add("position", "relative")
            .build()
    };

    let tooltip_text = if show_latex {
        symbol
            .latex
            .map(|l| format!("{} ({})", symbol.name, l))
            .unwrap_or_else(|| symbol.name.to_string())
    } else {
        symbol.name.to_string()
    };

    view! {
        <button
            type="button"
            style=styles
            title=if show_tooltip { Some(tooltip_text) } else { None }
            on:click=on_click
            on:mouseenter=move |_| is_hovered.set(true)
            on:mouseleave=move |_| is_hovered.set(false)
        >
            {symbol.char}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_category_display_names() {
        assert_eq!(SymbolCategory::Greek.display_name(), "Greek");
        assert_eq!(SymbolCategory::Operators.display_name(), "Operators");
        assert_eq!(SymbolCategory::SetTheory.display_name(), "Sets");
        assert_eq!(SymbolCategory::Logic.display_name(), "Logic");
    }

    #[test]
    fn test_symbol_category_all() {
        let all = SymbolCategory::all();
        assert_eq!(all.len(), 7);
        assert!(all.contains(&SymbolCategory::Greek));
        assert!(all.contains(&SymbolCategory::Miscellaneous));
    }

    #[test]
    fn test_get_all_symbols() {
        let symbols = get_all_symbols();
        assert!(symbols.len() > 100); // Should have many symbols

        // Check some specific symbols exist
        assert!(symbols.iter().any(|s| s.char == "α"));
        assert!(symbols.iter().any(|s| s.char == "∑"));
        assert!(symbols.iter().any(|s| s.char == "∞"));
    }

    #[test]
    fn test_get_symbols_by_categories() {
        let greek = get_symbols_by_categories(&[SymbolCategory::Greek]);
        assert!(greek.iter().all(|s| s.category == SymbolCategory::Greek));
        assert!(greek.iter().any(|s| s.char == "α"));
        assert!(greek.iter().any(|s| s.char == "Ω"));

        let logic = get_symbols_by_categories(&[SymbolCategory::Logic]);
        assert!(logic.iter().all(|s| s.category == SymbolCategory::Logic));
        assert!(logic.iter().any(|s| s.char == "∀"));
    }

    #[test]
    fn test_symbol_latex() {
        let symbols = get_all_symbols();
        let alpha = symbols.iter().find(|s| s.char == "α").unwrap();
        assert_eq!(alpha.latex, Some("\\alpha"));

        let infinity = symbols.iter().find(|s| s.char == "∞").unwrap();
        assert_eq!(infinity.latex, Some("\\infty"));
    }

    #[test]
    fn test_symbol_names() {
        let symbols = get_all_symbols();
        let sum = symbols.iter().find(|s| s.char == "∑").unwrap();
        assert_eq!(sum.name, "summation");

        let forall = symbols.iter().find(|s| s.char == "∀").unwrap();
        assert_eq!(forall.name, "for all");
    }
}
