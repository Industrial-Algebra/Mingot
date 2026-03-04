//! EquationEditor component for WYSIWYG mathematical expression editing.
//!
//! A structured math editor designed for geometric algebra expressions,
//! with support for Amari library operations.

use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Geometric algebra operations supported by the editor
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GeometricOp {
    /// Geometric product (∗ or juxtaposition)
    GeometricProduct,
    /// Wedge/outer product (∧)
    WedgeProduct,
    /// Inner/dot product (·)
    InnerProduct,
    /// Left contraction (⌟)
    LeftContraction,
    /// Right contraction (⌞)
    RightContraction,
    /// Scalar product (returns scalar)
    ScalarProduct,
}

impl GeometricOp {
    /// Get the Unicode symbol for this operation
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::GeometricProduct => "∗",
            Self::WedgeProduct => "∧",
            Self::InnerProduct => "·",
            Self::LeftContraction => "⌟",
            Self::RightContraction => "⌞",
            Self::ScalarProduct => "⟨⟩",
        }
    }

    /// Get the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::GeometricProduct => "Geometric Product",
            Self::WedgeProduct => "Wedge Product",
            Self::InnerProduct => "Inner Product",
            Self::LeftContraction => "Left Contraction",
            Self::RightContraction => "Right Contraction",
            Self::ScalarProduct => "Scalar Product",
        }
    }

    /// Get the LaTeX representation
    pub fn latex(&self) -> &'static str {
        match self {
            Self::GeometricProduct => "\\ast",
            Self::WedgeProduct => "\\wedge",
            Self::InnerProduct => "\\cdot",
            Self::LeftContraction => "\\lrcorner",
            Self::RightContraction => "\\llcorner",
            Self::ScalarProduct => "\\langle \\rangle",
        }
    }

    /// Get all geometric operations
    pub fn all() -> Vec<Self> {
        vec![
            Self::GeometricProduct,
            Self::WedgeProduct,
            Self::InnerProduct,
            Self::LeftContraction,
            Self::RightContraction,
            Self::ScalarProduct,
        ]
    }
}

/// Unary operations (applied to single operand)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    /// Reverse (†)
    Reverse,
    /// Hodge dual (⋆)
    HodgeDual,
    /// Grade involution (ˆ)
    GradeInvolution,
    /// Clifford conjugate (‾)
    CliffordConjugate,
    /// Normalization
    Normalize,
    /// Inverse
    Inverse,
    /// Magnitude/norm
    Magnitude,
    /// Exponential (for rotor generation)
    Exp,
}

impl UnaryOp {
    /// Get the Unicode symbol/notation
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Reverse => "†",
            Self::HodgeDual => "⋆",
            Self::GradeInvolution => "ˆ",
            Self::CliffordConjugate => "‾",
            Self::Normalize => "normalize",
            Self::Inverse => "⁻¹",
            Self::Magnitude => "‖‖",
            Self::Exp => "exp",
        }
    }

    /// Get the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Reverse => "Reverse",
            Self::HodgeDual => "Hodge Dual",
            Self::GradeInvolution => "Grade Involution",
            Self::CliffordConjugate => "Clifford Conjugate",
            Self::Normalize => "Normalize",
            Self::Inverse => "Inverse",
            Self::Magnitude => "Magnitude",
            Self::Exp => "Exponential",
        }
    }

    /// Get all unary operations
    pub fn all() -> Vec<Self> {
        vec![
            Self::Reverse,
            Self::HodgeDual,
            Self::GradeInvolution,
            Self::CliffordConjugate,
            Self::Normalize,
            Self::Inverse,
            Self::Magnitude,
            Self::Exp,
        ]
    }
}

/// Calculus operators from Amari
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CalculusOp {
    /// Gradient (∇)
    Gradient,
    /// Divergence (∇·)
    Divergence,
    /// Curl (∇∧)
    Curl,
    /// Laplacian (∇²)
    Laplacian,
    /// Partial derivative (∂)
    Partial,
}

impl CalculusOp {
    /// Get the Unicode symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Gradient => "∇",
            Self::Divergence => "∇·",
            Self::Curl => "∇∧",
            Self::Laplacian => "∇²",
            Self::Partial => "∂",
        }
    }

    /// Get the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Gradient => "Gradient",
            Self::Divergence => "Divergence",
            Self::Curl => "Curl",
            Self::Laplacian => "Laplacian",
            Self::Partial => "Partial Derivative",
        }
    }

    /// Get all calculus operations
    pub fn all() -> Vec<Self> {
        vec![
            Self::Gradient,
            Self::Divergence,
            Self::Curl,
            Self::Laplacian,
            Self::Partial,
        ]
    }
}

/// Grade projection notation
#[derive(Clone, Debug, PartialEq)]
pub struct GradeProjection {
    /// The grade to project to (0 = scalar, 1 = vector, 2 = bivector, etc.)
    pub grade: u8,
}

impl GradeProjection {
    pub fn new(grade: u8) -> Self {
        Self { grade }
    }

    /// Get the subscript notation (⟨M⟩ₖ)
    pub fn symbol(&self) -> String {
        let subscript = match self.grade {
            0 => "₀",
            1 => "₁",
            2 => "₂",
            3 => "₃",
            4 => "₄",
            5 => "₅",
            6 => "₆",
            7 => "₇",
            8 => "₈",
            9 => "₉",
            _ => "ₙ",
        };
        format!("⟨⟩{}", subscript)
    }
}

/// Basis vector types for different algebras
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BasisType {
    /// Standard orthonormal basis (e₁, e₂, e₃, ...)
    Standard,
    /// Conformal basis (e₀, e₁, e₂, e₃, e∞)
    Conformal,
    /// Spacetime basis (γ₀, γ₁, γ₂, γ₃)
    Spacetime,
}

impl BasisType {
    /// Get basis vector symbol for given index
    pub fn basis_symbol(&self, index: usize) -> String {
        let subscript = match index {
            0 => "₀",
            1 => "₁",
            2 => "₂",
            3 => "₃",
            4 => "₄",
            5 => "₅",
            6 => "₆",
            7 => "₇",
            8 => "₈",
            9 => "₉",
            _ => "ₙ",
        };
        match self {
            Self::Standard => format!("e{}", subscript),
            Self::Conformal => {
                if index == 0 {
                    "e₀".to_string()
                } else if index == 4 {
                    "e∞".to_string()
                } else {
                    format!("e{}", subscript)
                }
            }
            Self::Spacetime => format!("γ{}", subscript),
        }
    }

    /// Get all basis types
    pub fn all() -> Vec<Self> {
        vec![Self::Standard, Self::Conformal, Self::Spacetime]
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::Conformal => "Conformal",
            Self::Spacetime => "Spacetime",
        }
    }
}

/// A node in the equation tree
#[derive(Clone, Debug, PartialEq)]
pub enum EquationNode {
    /// A number literal
    Number(f64),
    /// A variable or symbol
    Variable(String),
    /// A basis vector
    BasisVector { basis_type: BasisType, index: usize },
    /// Multivector literal (coefficients for each basis blade)
    Multivector(Vec<(String, f64)>),
    /// Binary geometric operation
    BinaryOp {
        op: GeometricOp,
        left: Box<EquationNode>,
        right: Box<EquationNode>,
    },
    /// Arithmetic binary operation
    ArithmeticOp {
        op: char, // +, -, *, /
        left: Box<EquationNode>,
        right: Box<EquationNode>,
    },
    /// Unary operation
    UnaryOp {
        op: UnaryOp,
        operand: Box<EquationNode>,
    },
    /// Calculus operation
    CalculusOp {
        op: CalculusOp,
        operand: Box<EquationNode>,
        /// Variable for partial derivative
        variable: Option<String>,
    },
    /// Grade projection
    GradeProjection {
        grade: u8,
        operand: Box<EquationNode>,
    },
    /// Rotor application (sandwich product R·x·R†)
    RotorApplication {
        rotor: Box<EquationNode>,
        operand: Box<EquationNode>,
    },
    /// Parenthesized expression
    Parenthesized(Box<EquationNode>),
    /// Fraction (for display)
    Fraction {
        numerator: Box<EquationNode>,
        denominator: Box<EquationNode>,
    },
    /// Subscript
    Subscript {
        base: Box<EquationNode>,
        subscript: Box<EquationNode>,
    },
    /// Superscript/power
    Superscript {
        base: Box<EquationNode>,
        superscript: Box<EquationNode>,
    },
    /// Empty placeholder (cursor position)
    Placeholder,
}

impl EquationNode {
    /// Convert to LaTeX string
    pub fn to_latex(&self) -> String {
        match self {
            Self::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    format!("{}", n)
                }
            }
            Self::Variable(name) => name.clone(),
            Self::BasisVector { basis_type, index } => {
                let base = match basis_type {
                    BasisType::Standard => "e",
                    BasisType::Conformal => "e",
                    BasisType::Spacetime => "\\gamma",
                };
                if *basis_type == BasisType::Conformal && *index == 4 {
                    "e_\\infty".to_string()
                } else {
                    format!("{}_{}", base, index)
                }
            }
            Self::Multivector(terms) => {
                let parts: Vec<String> = terms
                    .iter()
                    .map(|(blade, coef)| {
                        if blade.is_empty() || blade == "1" {
                            format!("{}", coef)
                        } else if *coef == 1.0 {
                            blade.clone()
                        } else if *coef == -1.0 {
                            format!("-{}", blade)
                        } else {
                            format!("{}{}", coef, blade)
                        }
                    })
                    .collect();
                parts.join(" + ")
            }
            Self::BinaryOp { op, left, right } => {
                format!("{} {} {}", left.to_latex(), op.latex(), right.to_latex())
            }
            Self::ArithmeticOp { op, left, right } => {
                format!("{} {} {}", left.to_latex(), op, right.to_latex())
            }
            Self::UnaryOp { op, operand } => match op {
                UnaryOp::Reverse => format!("{}^\\dagger", operand.to_latex()),
                UnaryOp::HodgeDual => format!("\\star {}", operand.to_latex()),
                UnaryOp::GradeInvolution => format!("\\hat{{{}}}", operand.to_latex()),
                UnaryOp::CliffordConjugate => format!("\\overline{{{}}}", operand.to_latex()),
                UnaryOp::Normalize => format!("\\text{{normalize}}({})", operand.to_latex()),
                UnaryOp::Inverse => format!("{}^{{-1}}", operand.to_latex()),
                UnaryOp::Magnitude => format!("\\|{}\\|", operand.to_latex()),
                UnaryOp::Exp => format!("\\exp({})", operand.to_latex()),
            },
            Self::CalculusOp {
                op,
                operand,
                variable,
            } => match op {
                CalculusOp::Gradient => format!("\\nabla {}", operand.to_latex()),
                CalculusOp::Divergence => format!("\\nabla \\cdot {}", operand.to_latex()),
                CalculusOp::Curl => format!("\\nabla \\wedge {}", operand.to_latex()),
                CalculusOp::Laplacian => format!("\\nabla^2 {}", operand.to_latex()),
                CalculusOp::Partial => {
                    if let Some(var) = variable {
                        format!(
                            "\\frac{{\\partial {}}}{{\\partial {}}}",
                            operand.to_latex(),
                            var
                        )
                    } else {
                        format!("\\partial {}", operand.to_latex())
                    }
                }
            },
            Self::GradeProjection { grade, operand } => {
                format!("\\langle {} \\rangle_{}", operand.to_latex(), grade)
            }
            Self::RotorApplication { rotor, operand } => {
                format!(
                    "{} {} {}^\\dagger",
                    rotor.to_latex(),
                    operand.to_latex(),
                    rotor.to_latex()
                )
            }
            Self::Parenthesized(inner) => format!("\\left( {} \\right)", inner.to_latex()),
            Self::Fraction {
                numerator,
                denominator,
            } => {
                format!(
                    "\\frac{{{}}}{{{}}}",
                    numerator.to_latex(),
                    denominator.to_latex()
                )
            }
            Self::Subscript { base, subscript } => {
                format!("{}_{{{}}}", base.to_latex(), subscript.to_latex())
            }
            Self::Superscript { base, superscript } => {
                format!("{}^{{{}}}", base.to_latex(), superscript.to_latex())
            }
            Self::Placeholder => "\\square".to_string(),
        }
    }

    /// Convert to Unicode display string
    pub fn to_unicode(&self) -> String {
        match self {
            Self::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    format!("{}", n)
                }
            }
            Self::Variable(name) => name.clone(),
            Self::BasisVector { basis_type, index } => basis_type.basis_symbol(*index),
            Self::Multivector(terms) => {
                let parts: Vec<String> = terms
                    .iter()
                    .map(|(blade, coef)| {
                        if blade.is_empty() || blade == "1" {
                            format!("{}", coef)
                        } else if *coef == 1.0 {
                            blade.clone()
                        } else if *coef == -1.0 {
                            format!("-{}", blade)
                        } else {
                            format!("{}{}", coef, blade)
                        }
                    })
                    .collect();
                parts.join(" + ")
            }
            Self::BinaryOp { op, left, right } => {
                format!(
                    "{} {} {}",
                    left.to_unicode(),
                    op.symbol(),
                    right.to_unicode()
                )
            }
            Self::ArithmeticOp { op, left, right } => {
                format!("{} {} {}", left.to_unicode(), op, right.to_unicode())
            }
            Self::UnaryOp { op, operand } => match op {
                UnaryOp::Reverse => format!("{}†", operand.to_unicode()),
                UnaryOp::HodgeDual => format!("⋆{}", operand.to_unicode()),
                UnaryOp::GradeInvolution => format!("{}ˆ", operand.to_unicode()),
                UnaryOp::CliffordConjugate => format!("{}̄", operand.to_unicode()),
                UnaryOp::Normalize => format!("normalize({})", operand.to_unicode()),
                UnaryOp::Inverse => format!("{}⁻¹", operand.to_unicode()),
                UnaryOp::Magnitude => format!("‖{}‖", operand.to_unicode()),
                UnaryOp::Exp => format!("exp({})", operand.to_unicode()),
            },
            Self::CalculusOp {
                op,
                operand,
                variable,
            } => match op {
                CalculusOp::Gradient => format!("∇{}", operand.to_unicode()),
                CalculusOp::Divergence => format!("∇·{}", operand.to_unicode()),
                CalculusOp::Curl => format!("∇∧{}", operand.to_unicode()),
                CalculusOp::Laplacian => format!("∇²{}", operand.to_unicode()),
                CalculusOp::Partial => {
                    if let Some(var) = variable {
                        format!("∂{}/∂{}", operand.to_unicode(), var)
                    } else {
                        format!("∂{}", operand.to_unicode())
                    }
                }
            },
            Self::GradeProjection { grade, operand } => {
                let subscript = match grade {
                    0 => "₀",
                    1 => "₁",
                    2 => "₂",
                    3 => "₃",
                    4 => "₄",
                    5 => "₅",
                    6 => "₆",
                    7 => "₇",
                    8 => "₈",
                    9 => "₉",
                    _ => "ₙ",
                };
                format!("⟨{}⟩{}", operand.to_unicode(), subscript)
            }
            Self::RotorApplication { rotor, operand } => {
                format!(
                    "{}{}{}†",
                    rotor.to_unicode(),
                    operand.to_unicode(),
                    rotor.to_unicode()
                )
            }
            Self::Parenthesized(inner) => format!("({})", inner.to_unicode()),
            Self::Fraction {
                numerator,
                denominator,
            } => {
                format!("{}/{}", numerator.to_unicode(), denominator.to_unicode())
            }
            Self::Subscript { base, subscript } => {
                format!("{}_{}", base.to_unicode(), subscript.to_unicode())
            }
            Self::Superscript { base, superscript } => {
                format!("{}^{}", base.to_unicode(), superscript.to_unicode())
            }
            Self::Placeholder => "□".to_string(),
        }
    }
}

impl std::fmt::Display for EquationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_unicode())
    }
}

/// Editor mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EditorMode {
    /// Standard editing mode
    #[default]
    Normal,
    /// Inserting a geometric operation
    GeometricOp,
    /// Inserting a unary operation
    UnaryOp,
    /// Inserting calculus operation
    CalculusOp,
    /// Inserting grade projection
    GradeProjection,
    /// Inserting basis vector
    BasisVector,
}

/// Toolbar category for organizing operations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToolbarCategory {
    #[default]
    Geometric,
    Unary,
    Calculus,
    Basis,
    Structure,
}

impl ToolbarCategory {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Geometric,
            Self::Unary,
            Self::Calculus,
            Self::Basis,
            Self::Structure,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Geometric => "Products",
            Self::Unary => "Unary",
            Self::Calculus => "Calculus",
            Self::Basis => "Basis",
            Self::Structure => "Structure",
        }
    }
}

/// Size variants for the editor
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EquationEditorSize {
    /// Compact editor
    Sm,
    /// Default size
    #[default]
    Md,
    /// Large editor
    Lg,
}

/// Props for the EquationEditor component
#[component]
pub fn EquationEditor(
    /// Current equation value
    #[prop(optional, into)]
    value: Option<RwSignal<EquationNode>>,
    /// Callback when equation changes
    #[prop(optional, into)]
    on_change: Option<Callback<EquationNode>>,
    /// Show the toolbar
    #[prop(default = true)]
    show_toolbar: bool,
    /// Show LaTeX output
    #[prop(default = false)]
    show_latex: bool,
    /// Editor size
    #[prop(default = EquationEditorSize::Md)]
    size: EquationEditorSize,
    /// Basis type for basis vector insertion
    #[prop(default = BasisType::Standard)]
    basis_type: BasisType,
    /// Maximum dimensions for basis vectors
    #[prop(default = 3)]
    max_dimensions: usize,
    /// Disabled state
    #[prop(default = false)]
    disabled: bool,
    /// Read-only mode (display only)
    #[prop(default = false)]
    read_only: bool,
    /// Placeholder text when empty
    #[prop(optional, into)]
    placeholder: Option<String>,
) -> impl IntoView {
    let theme = use_theme();

    // Internal state
    let equation = value.unwrap_or_else(|| RwSignal::new(EquationNode::Placeholder));
    let active_category = RwSignal::new(ToolbarCategory::Geometric);
    let input_text = RwSignal::new(String::new());
    let is_focused = RwSignal::new(false);

    // Parse simple text input into equation node
    let parse_input = move |text: &str| -> Option<EquationNode> {
        let text = text.trim();
        if text.is_empty() {
            return None;
        }

        // Try to parse as number
        if let Ok(n) = text.parse::<f64>() {
            return Some(EquationNode::Number(n));
        }

        // Otherwise treat as variable
        Some(EquationNode::Variable(text.to_string()))
    };

    // Handle text input
    let on_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        input_text.set(input.value());
    };

    // Handle key press
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            if let Some(node) = parse_input(&input_text.get()) {
                equation.set(node.clone());
                if let Some(cb) = on_change {
                    cb.run(node);
                }
                input_text.set(String::new());
            }
        }
    };

    // Insert geometric operation
    let insert_geometric_op = move |op: GeometricOp| {
        let current = equation.get();
        if matches!(current, EquationNode::Placeholder) {
            // If placeholder, wait for operands
            return;
        }
        let new_node = EquationNode::BinaryOp {
            op,
            left: Box::new(current),
            right: Box::new(EquationNode::Placeholder),
        };
        equation.set(new_node.clone());
        if let Some(cb) = on_change {
            cb.run(new_node);
        }
    };

    // Insert unary operation
    let insert_unary_op = move |op: UnaryOp| {
        let current = equation.get();
        if matches!(current, EquationNode::Placeholder) {
            return;
        }
        let new_node = EquationNode::UnaryOp {
            op,
            operand: Box::new(current),
        };
        equation.set(new_node.clone());
        if let Some(cb) = on_change {
            cb.run(new_node);
        }
    };

    // Insert calculus operation
    let insert_calculus_op = move |op: CalculusOp| {
        let current = equation.get();
        if matches!(current, EquationNode::Placeholder) {
            return;
        }
        let new_node = EquationNode::CalculusOp {
            op,
            operand: Box::new(current),
            variable: None,
        };
        equation.set(new_node.clone());
        if let Some(cb) = on_change {
            cb.run(new_node);
        }
    };

    // Insert basis vector
    let insert_basis = move |index: usize| {
        let new_node = EquationNode::BasisVector { basis_type, index };
        let current = equation.get();
        let final_node = if matches!(current, EquationNode::Placeholder) {
            new_node
        } else {
            // Append with geometric product by default
            EquationNode::BinaryOp {
                op: GeometricOp::GeometricProduct,
                left: Box::new(current),
                right: Box::new(new_node),
            }
        };
        equation.set(final_node.clone());
        if let Some(cb) = on_change {
            cb.run(final_node);
        }
    };

    // Insert grade projection
    let insert_grade_projection = move |grade: u8| {
        let current = equation.get();
        if matches!(current, EquationNode::Placeholder) {
            return;
        }
        let new_node = EquationNode::GradeProjection {
            grade,
            operand: Box::new(current),
        };
        equation.set(new_node.clone());
        if let Some(cb) = on_change {
            cb.run(new_node);
        }
    };

    // Insert parentheses
    let insert_parens = move |_| {
        let current = equation.get();
        if matches!(current, EquationNode::Placeholder) {
            return;
        }
        let new_node = EquationNode::Parenthesized(Box::new(current));
        equation.set(new_node.clone());
        if let Some(cb) = on_change {
            cb.run(new_node);
        }
    };

    // Insert fraction
    let insert_fraction = move |_| {
        let current = equation.get();
        let new_node = EquationNode::Fraction {
            numerator: Box::new(current),
            denominator: Box::new(EquationNode::Placeholder),
        };
        equation.set(new_node.clone());
        if let Some(cb) = on_change {
            cb.run(new_node);
        }
    };

    // Clear equation
    let clear = move |_| {
        equation.set(EquationNode::Placeholder);
        input_text.set(String::new());
        if let Some(cb) = on_change {
            cb.run(EquationNode::Placeholder);
        }
    };

    // Container styles
    let container_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let font_size = match size {
            EquationEditorSize::Sm => theme_val.typography.font_sizes.sm,
            EquationEditorSize::Md => theme_val.typography.font_sizes.md,
            EquationEditorSize::Lg => theme_val.typography.font_sizes.lg,
        };

        StyleBuilder::new()
            .add("width", "100%")
            .add("font-family", "'Cambria Math', 'Latin Modern Math', serif")
            .add("font-size", font_size)
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.md)
            .add("background", scheme_colors.background.clone())
            .add_if(disabled, "opacity", "0.6")
            .add_if(disabled, "pointer-events", "none")
            .build()
    };

    // Toolbar styles
    let toolbar_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        StyleBuilder::new()
            .add("display", "flex")
            .add("flex-wrap", "wrap")
            .add("gap", theme_val.spacing.xs)
            .add(
                "padding",
                format!("{} {}", theme_val.spacing.xs, theme_val.spacing.sm),
            )
            .add(
                "border-bottom",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add(
                "background",
                scheme_colors
                    .get_color("gray", 0)
                    .unwrap_or_else(|| "#f8f9fa".to_string()),
            )
            .build()
    };

    // Category tab styles
    let category_tab_styles = move |cat: ToolbarCategory| {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_active = active_category.get() == cat;

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
                    "transparent".to_string()
                },
            )
            .add(
                "color",
                if is_active {
                    "white".to_string()
                } else {
                    scheme_colors.text.clone()
                },
            )
            .build()
    };

    // Operation button styles
    let op_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        StyleBuilder::new()
            .add("padding", format!("{} {}", "4px", "8px"))
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("cursor", "pointer")
            .add("font-size", theme_val.typography.font_sizes.md)
            .add("font-family", "'Cambria Math', 'Latin Modern Math', serif")
            .add("background", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("min-width", "32px")
            .add("text-align", "center")
            .build()
    };

    // Display area styles
    let display_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let padding = match size {
            EquationEditorSize::Sm => theme_val.spacing.sm,
            EquationEditorSize::Md => theme_val.spacing.md,
            EquationEditorSize::Lg => theme_val.spacing.lg,
        };

        let min_height = match size {
            EquationEditorSize::Sm => "40px",
            EquationEditorSize::Md => "60px",
            EquationEditorSize::Lg => "80px",
        };

        let is_placeholder = matches!(equation.get(), EquationNode::Placeholder);
        let text_color = if is_placeholder {
            scheme_colors
                .get_color("gray", 5)
                .unwrap_or_else(|| "#adb5bd".to_string())
        } else {
            scheme_colors.text.clone()
        };

        StyleBuilder::new()
            .add("padding", padding)
            .add("min-height", min_height)
            .add("font-size", "1.25em")
            .add("display", "flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("color", text_color)
            .build()
    };

    // Input styles
    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        StyleBuilder::new()
            .add("flex", "1")
            .add("padding", theme_val.spacing.xs)
            .add("border", "none")
            .add("outline", "none")
            .add("font-family", "'Cambria Math', 'Latin Modern Math', serif")
            .add("font-size", theme_val.typography.font_sizes.md)
            .add("background", "transparent")
            .add("color", scheme_colors.text.clone())
            .build()
    };

    // LaTeX output styles
    let latex_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        StyleBuilder::new()
            .add(
                "padding",
                format!("{} {}", theme_val.spacing.xs, theme_val.spacing.sm),
            )
            .add(
                "border-top",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("font-family", "monospace")
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
            )
            .add(
                "background",
                scheme_colors
                    .get_color("gray", 0)
                    .unwrap_or_else(|| "#f8f9fa".to_string()),
            )
            .add("word-break", "break-all")
            .build()
    };

    // Input row styles
    let input_row_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("gap", theme_val.spacing.xs)
            .add(
                "padding",
                format!("{} {}", theme_val.spacing.xs, theme_val.spacing.sm),
            )
            .add(
                "border-top",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .build()
    };

    let placeholder_text = placeholder.unwrap_or_else(|| "Enter expression...".to_string());

    view! {
        <div style=container_styles>
            // Toolbar
            {move || show_toolbar.then(|| {
                view! {
                    <div style=toolbar_styles>
                        // Category tabs
                        <div style="display: flex; gap: 2px; margin-right: 8px;">
                            {ToolbarCategory::all().into_iter().map(|cat| {
                                view! {
                                    <button
                                        type="button"
                                        style=move || category_tab_styles(cat)
                                        on:click=move |_| active_category.set(cat)
                                        disabled=read_only
                                    >
                                        {cat.name()}
                                    </button>
                                }
                            }).collect_view()}
                        </div>

                        // Operations based on category
                        <div style="display: flex; flex-wrap: wrap; gap: 4px;">
                            {move || match active_category.get() {
                                ToolbarCategory::Geometric => {
                                    GeometricOp::all().into_iter().map(|op| {
                                        view! {
                                            <button
                                                type="button"
                                                style=op_button_styles
                                                on:click=move |_| insert_geometric_op(op)
                                                title=op.name()
                                                disabled=read_only
                                            >
                                                {op.symbol()}
                                            </button>
                                        }
                                    }).collect_view().into_any()
                                }
                                ToolbarCategory::Unary => {
                                    UnaryOp::all().into_iter().map(|op| {
                                        view! {
                                            <button
                                                type="button"
                                                style=op_button_styles
                                                on:click=move |_| insert_unary_op(op)
                                                title=op.name()
                                                disabled=read_only
                                            >
                                                {op.symbol()}
                                            </button>
                                        }
                                    }).collect_view().into_any()
                                }
                                ToolbarCategory::Calculus => {
                                    CalculusOp::all().into_iter().map(|op| {
                                        view! {
                                            <button
                                                type="button"
                                                style=op_button_styles
                                                on:click=move |_| insert_calculus_op(op)
                                                title=op.name()
                                                disabled=read_only
                                            >
                                                {op.symbol()}
                                            </button>
                                        }
                                    }).collect_view().into_any()
                                }
                                ToolbarCategory::Basis => {
                                    (0..max_dimensions).map(|i| {
                                        let symbol = basis_type.basis_symbol(i);
                                        view! {
                                            <button
                                                type="button"
                                                style=op_button_styles
                                                on:click=move |_| insert_basis(i)
                                                title=format!("Basis vector {}", i)
                                                disabled=read_only
                                            >
                                                {symbol}
                                            </button>
                                        }
                                    }).collect_view().into_any()
                                }
                                ToolbarCategory::Structure => {
                                    view! {
                                        <>
                                            <button
                                                type="button"
                                                style=op_button_styles
                                                on:click=insert_parens
                                                title="Parentheses"
                                                disabled=read_only
                                            >
                                                "()"
                                            </button>
                                            <button
                                                type="button"
                                                style=op_button_styles
                                                on:click=insert_fraction
                                                title="Fraction"
                                                disabled=read_only
                                            >
                                                "a/b"
                                            </button>
                                            // Grade projections
                                            {(0..=3u8).map(|grade| {
                                                let proj = GradeProjection::new(grade);
                                                view! {
                                                    <button
                                                        type="button"
                                                        style=op_button_styles
                                                        on:click=move |_| insert_grade_projection(grade)
                                                        title=format!("Grade {} projection", grade)
                                                        disabled=read_only
                                                    >
                                                        {proj.symbol()}
                                                    </button>
                                                }
                                            }).collect_view()}
                                            <button
                                                type="button"
                                                style=op_button_styles
                                                on:click=clear
                                                title="Clear"
                                                disabled=read_only
                                            >
                                                "C"
                                            </button>
                                        </>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                }
            })}

            // Display area
            <div style=display_styles>
                {move || {
                    let eq = equation.get();
                    if matches!(eq, EquationNode::Placeholder) {
                        placeholder_text.clone()
                    } else {
                        eq.to_unicode()
                    }
                }}
            </div>

            // Text input row (for entering numbers/variables)
            {move || (!read_only).then(|| {
                view! {
                    <div style=input_row_styles>
                        <input
                            type="text"
                            style=input_styles
                            placeholder="Type value or variable..."
                            prop:value=move || input_text.get()
                            on:input=on_input
                            on:keydown=on_keydown
                            on:focus=move |_| is_focused.set(true)
                            on:blur=move |_| is_focused.set(false)
                            disabled=disabled
                        />
                    </div>
                }
            })}

            // LaTeX output
            {move || show_latex.then(|| {
                view! {
                    <div style=latex_styles>
                        {move || equation.get().to_latex()}
                    </div>
                }
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometric_op_symbols() {
        assert_eq!(GeometricOp::GeometricProduct.symbol(), "∗");
        assert_eq!(GeometricOp::WedgeProduct.symbol(), "∧");
        assert_eq!(GeometricOp::InnerProduct.symbol(), "·");
        assert_eq!(GeometricOp::LeftContraction.symbol(), "⌟");
        assert_eq!(GeometricOp::RightContraction.symbol(), "⌞");
    }

    #[test]
    fn test_unary_op_symbols() {
        assert_eq!(UnaryOp::Reverse.symbol(), "†");
        assert_eq!(UnaryOp::HodgeDual.symbol(), "⋆");
        assert_eq!(UnaryOp::Inverse.symbol(), "⁻¹");
        assert_eq!(UnaryOp::Magnitude.symbol(), "‖‖");
    }

    #[test]
    fn test_calculus_op_symbols() {
        assert_eq!(CalculusOp::Gradient.symbol(), "∇");
        assert_eq!(CalculusOp::Divergence.symbol(), "∇·");
        assert_eq!(CalculusOp::Curl.symbol(), "∇∧");
        assert_eq!(CalculusOp::Laplacian.symbol(), "∇²");
    }

    #[test]
    fn test_basis_vector_symbols() {
        let standard = BasisType::Standard;
        assert_eq!(standard.basis_symbol(0), "e₀");
        assert_eq!(standard.basis_symbol(1), "e₁");
        assert_eq!(standard.basis_symbol(2), "e₂");

        let spacetime = BasisType::Spacetime;
        assert_eq!(spacetime.basis_symbol(0), "γ₀");
        assert_eq!(spacetime.basis_symbol(1), "γ₁");

        let conformal = BasisType::Conformal;
        assert_eq!(conformal.basis_symbol(0), "e₀");
        assert_eq!(conformal.basis_symbol(4), "e∞");
    }

    #[test]
    fn test_grade_projection_symbol() {
        assert_eq!(GradeProjection::new(0).symbol(), "⟨⟩₀");
        assert_eq!(GradeProjection::new(1).symbol(), "⟨⟩₁");
        assert_eq!(GradeProjection::new(2).symbol(), "⟨⟩₂");
    }

    #[test]
    fn test_equation_node_number() {
        let node = EquationNode::Number(42.0);
        assert_eq!(node.to_unicode(), "42");
        assert_eq!(node.to_latex(), "42");

        let decimal = EquationNode::Number(1.5);
        assert_eq!(decimal.to_unicode(), "1.5");
    }

    #[test]
    fn test_equation_node_variable() {
        let node = EquationNode::Variable("x".to_string());
        assert_eq!(node.to_unicode(), "x");
        assert_eq!(node.to_latex(), "x");
    }

    #[test]
    fn test_equation_node_basis_vector() {
        let node = EquationNode::BasisVector {
            basis_type: BasisType::Standard,
            index: 1,
        };
        assert_eq!(node.to_unicode(), "e₁");
        assert_eq!(node.to_latex(), "e_1");
    }

    #[test]
    fn test_equation_node_binary_op() {
        let node = EquationNode::BinaryOp {
            op: GeometricOp::WedgeProduct,
            left: Box::new(EquationNode::Variable("a".to_string())),
            right: Box::new(EquationNode::Variable("b".to_string())),
        };
        assert_eq!(node.to_unicode(), "a ∧ b");
        assert_eq!(node.to_latex(), "a \\wedge b");
    }

    #[test]
    fn test_equation_node_unary_op() {
        let node = EquationNode::UnaryOp {
            op: UnaryOp::Reverse,
            operand: Box::new(EquationNode::Variable("R".to_string())),
        };
        assert_eq!(node.to_unicode(), "R†");
        assert_eq!(node.to_latex(), "R^\\dagger");
    }

    #[test]
    fn test_equation_node_grade_projection() {
        let node = EquationNode::GradeProjection {
            grade: 2,
            operand: Box::new(EquationNode::Variable("M".to_string())),
        };
        assert_eq!(node.to_unicode(), "⟨M⟩₂");
        assert_eq!(node.to_latex(), "\\langle M \\rangle_2");
    }

    #[test]
    fn test_equation_node_calculus_gradient() {
        let node = EquationNode::CalculusOp {
            op: CalculusOp::Gradient,
            operand: Box::new(EquationNode::Variable("f".to_string())),
            variable: None,
        };
        assert_eq!(node.to_unicode(), "∇f");
        assert_eq!(node.to_latex(), "\\nabla f");
    }

    #[test]
    fn test_equation_node_partial_derivative() {
        let node = EquationNode::CalculusOp {
            op: CalculusOp::Partial,
            operand: Box::new(EquationNode::Variable("f".to_string())),
            variable: Some("x".to_string()),
        };
        assert_eq!(node.to_unicode(), "∂f/∂x");
        assert_eq!(node.to_latex(), "\\frac{\\partial f}{\\partial x}");
    }

    #[test]
    fn test_equation_node_fraction() {
        let node = EquationNode::Fraction {
            numerator: Box::new(EquationNode::Number(1.0)),
            denominator: Box::new(EquationNode::Number(2.0)),
        };
        assert_eq!(node.to_unicode(), "1/2");
        assert_eq!(node.to_latex(), "\\frac{1}{2}");
    }

    #[test]
    fn test_complex_expression() {
        // Build: (a ∧ b)†
        let wedge = EquationNode::BinaryOp {
            op: GeometricOp::WedgeProduct,
            left: Box::new(EquationNode::Variable("a".to_string())),
            right: Box::new(EquationNode::Variable("b".to_string())),
        };
        let parens = EquationNode::Parenthesized(Box::new(wedge));
        let reversed = EquationNode::UnaryOp {
            op: UnaryOp::Reverse,
            operand: Box::new(parens),
        };
        assert_eq!(reversed.to_unicode(), "(a ∧ b)†");
    }
}
