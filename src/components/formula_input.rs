//! Formula input component for mathematical expression entry.
//!
//! Provides a text input that parses and validates mathematical expressions,
//! supports variables, and recognizes common functions.

use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};
use std::f64::consts::{E, PI, TAU};

/// Recognized mathematical functions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MathFunction {
    // Trigonometric
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    // Exponential/Logarithmic
    Exp,
    Ln,
    Log10,
    Log2,
    // Power/Root
    Sqrt,
    Cbrt,
    Abs,
    // Rounding
    Floor,
    Ceil,
    Round,
    // Special
    Sign,
    Factorial,
}

impl MathFunction {
    /// Get function name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Asin => "asin",
            Self::Acos => "acos",
            Self::Atan => "atan",
            Self::Sinh => "sinh",
            Self::Cosh => "cosh",
            Self::Tanh => "tanh",
            Self::Exp => "exp",
            Self::Ln => "ln",
            Self::Log10 => "log10",
            Self::Log2 => "log2",
            Self::Sqrt => "sqrt",
            Self::Cbrt => "cbrt",
            Self::Abs => "abs",
            Self::Floor => "floor",
            Self::Ceil => "ceil",
            Self::Round => "round",
            Self::Sign => "sign",
            Self::Factorial => "factorial",
        }
    }

    /// Get all functions
    pub fn all() -> Vec<Self> {
        vec![
            Self::Sin,
            Self::Cos,
            Self::Tan,
            Self::Asin,
            Self::Acos,
            Self::Atan,
            Self::Sinh,
            Self::Cosh,
            Self::Tanh,
            Self::Exp,
            Self::Ln,
            Self::Log10,
            Self::Log2,
            Self::Sqrt,
            Self::Cbrt,
            Self::Abs,
            Self::Floor,
            Self::Ceil,
            Self::Round,
            Self::Sign,
            Self::Factorial,
        ]
    }

    /// Try to parse function from string
    pub fn try_from_name(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sin" => Some(Self::Sin),
            "cos" => Some(Self::Cos),
            "tan" => Some(Self::Tan),
            "asin" | "arcsin" => Some(Self::Asin),
            "acos" | "arccos" => Some(Self::Acos),
            "atan" | "arctan" => Some(Self::Atan),
            "sinh" => Some(Self::Sinh),
            "cosh" => Some(Self::Cosh),
            "tanh" => Some(Self::Tanh),
            "exp" => Some(Self::Exp),
            "ln" | "log" => Some(Self::Ln),
            "log10" => Some(Self::Log10),
            "log2" => Some(Self::Log2),
            "sqrt" => Some(Self::Sqrt),
            "cbrt" => Some(Self::Cbrt),
            "abs" => Some(Self::Abs),
            "floor" => Some(Self::Floor),
            "ceil" => Some(Self::Ceil),
            "round" => Some(Self::Round),
            "sign" | "sgn" => Some(Self::Sign),
            "factorial" | "fact" => Some(Self::Factorial),
            _ => None,
        }
    }

    /// Evaluate the function
    pub fn evaluate(&self, arg: f64) -> f64 {
        match self {
            Self::Sin => arg.sin(),
            Self::Cos => arg.cos(),
            Self::Tan => arg.tan(),
            Self::Asin => arg.asin(),
            Self::Acos => arg.acos(),
            Self::Atan => arg.atan(),
            Self::Sinh => arg.sinh(),
            Self::Cosh => arg.cosh(),
            Self::Tanh => arg.tanh(),
            Self::Exp => arg.exp(),
            Self::Ln => arg.ln(),
            Self::Log10 => arg.log10(),
            Self::Log2 => arg.log2(),
            Self::Sqrt => arg.sqrt(),
            Self::Cbrt => arg.cbrt(),
            Self::Abs => arg.abs(),
            Self::Floor => arg.floor(),
            Self::Ceil => arg.ceil(),
            Self::Round => arg.round(),
            Self::Sign => {
                if arg > 0.0 {
                    1.0
                } else if arg < 0.0 {
                    -1.0
                } else {
                    0.0
                }
            }
            Self::Factorial => {
                if arg < 0.0 || arg.fract() != 0.0 {
                    f64::NAN
                } else {
                    let n = arg as u64;
                    if n > 170 {
                        f64::INFINITY
                    } else {
                        (1..=n).product::<u64>() as f64
                    }
                }
            }
        }
    }
}

/// Token types for the expression parser
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Variable(String),
    Function(MathFunction),
    Operator(char),
    LeftParen,
    RightParen,
    Comma,
}

/// Expression AST node
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Variable(String),
    BinaryOp {
        op: char,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: char,
        operand: Box<Expression>,
    },
    FunctionCall {
        function: MathFunction,
        args: Vec<Expression>,
    },
}

impl Expression {
    /// Get all variables in the expression
    pub fn variables(&self) -> HashSet<String> {
        let mut vars = HashSet::new();
        self.collect_variables(&mut vars);
        vars
    }

    fn collect_variables(&self, vars: &mut HashSet<String>) {
        match self {
            Expression::Variable(name) => {
                vars.insert(name.clone());
            }
            Expression::BinaryOp { left, right, .. } => {
                left.collect_variables(vars);
                right.collect_variables(vars);
            }
            Expression::UnaryOp { operand, .. } => {
                operand.collect_variables(vars);
            }
            Expression::FunctionCall { args, .. } => {
                for arg in args {
                    arg.collect_variables(vars);
                }
            }
            _ => {}
        }
    }

    /// Evaluate the expression with given variable values
    pub fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        match self {
            Expression::Number(n) => Ok(*n),
            Expression::Variable(name) => {
                // Check for constants first
                match name.as_str() {
                    "pi" | "PI" | "π" => Ok(PI),
                    "e" | "E" => Ok(E),
                    "tau" | "TAU" | "τ" => Ok(TAU),
                    _ => variables
                        .get(name)
                        .copied()
                        .ok_or_else(|| format!("Undefined variable: {}", name)),
                }
            }
            Expression::BinaryOp { op, left, right } => {
                let l = left.evaluate(variables)?;
                let r = right.evaluate(variables)?;
                Ok(match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    '^' => l.powf(r),
                    '%' => l % r,
                    _ => return Err(format!("Unknown operator: {}", op)),
                })
            }
            Expression::UnaryOp { op, operand } => {
                let val = operand.evaluate(variables)?;
                Ok(match op {
                    '-' => -val,
                    '+' => val,
                    _ => return Err(format!("Unknown unary operator: {}", op)),
                })
            }
            Expression::FunctionCall { function, args } => {
                if args.len() != 1 {
                    return Err(format!(
                        "Function {} expects 1 argument, got {}",
                        function.name(),
                        args.len()
                    ));
                }
                let arg = args[0].evaluate(variables)?;
                Ok(function.evaluate(arg))
            }
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.0}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::BinaryOp { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expression::UnaryOp { op, operand } => {
                write!(f, "({}{})", op, operand)
            }
            Expression::FunctionCall { function, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", function.name(), args_str.join(", "))
            }
        }
    }
}

/// Parse error types
#[derive(Clone, Debug, PartialEq)]
pub enum FormulaParseError {
    UnexpectedCharacter(char),
    UnexpectedToken(String),
    UnmatchedParenthesis,
    EmptyExpression,
    InvalidNumber(String),
    UnknownFunction(String),
    MissingOperand,
    TrailingInput(String),
}

impl std::fmt::Display for FormulaParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedCharacter(c) => write!(f, "Unexpected character: '{}'", c),
            Self::UnexpectedToken(t) => write!(f, "Unexpected token: {}", t),
            Self::UnmatchedParenthesis => write!(f, "Unmatched parenthesis"),
            Self::EmptyExpression => write!(f, "Empty expression"),
            Self::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            Self::UnknownFunction(s) => write!(f, "Unknown function: {}", s),
            Self::MissingOperand => write!(f, "Missing operand"),
            Self::TrailingInput(s) => write!(f, "Trailing input: {}", s),
        }
    }
}

/// Tokenizer for mathematical expressions
fn tokenize(input: &str) -> Result<Vec<Token>, FormulaParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' {
                        num_str.push(c);
                        chars.next();
                        // Handle negative exponent
                        if (c == 'e' || c == 'E') && chars.peek() == Some(&'-') {
                            num_str.push('-');
                            chars.next();
                        }
                    } else {
                        break;
                    }
                }
                let num: f64 = num_str
                    .parse()
                    .map_err(|_| FormulaParseError::InvalidNumber(num_str))?;
                tokens.push(Token::Number(num));
            }
            'a'..='z' | 'A'..='Z' | '_' | 'α'..='ω' | 'Α'..='Ω' => {
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric()
                        || c == '_'
                        || ('α'..='ω').contains(&c)
                        || ('Α'..='Ω').contains(&c)
                    {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Check if it's a function
                if let Some(func) = MathFunction::try_from_name(&name) {
                    tokens.push(Token::Function(func));
                } else {
                    tokens.push(Token::Variable(name));
                }
            }
            '+' | '-' | '*' | '/' | '^' | '%' => {
                tokens.push(Token::Operator(ch));
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            _ => {
                return Err(FormulaParseError::UnexpectedCharacter(ch));
            }
        }
    }

    Ok(tokens)
}

/// Parser for mathematical expressions
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: &Token) -> Result<(), FormulaParseError> {
        if self.peek() == Some(expected) {
            self.advance();
            Ok(())
        } else {
            Err(FormulaParseError::UnexpectedToken(format!(
                "Expected {:?}, got {:?}",
                expected,
                self.peek()
            )))
        }
    }

    pub fn parse(&mut self) -> Result<Expression, FormulaParseError> {
        if self.tokens.is_empty() {
            return Err(FormulaParseError::EmptyExpression);
        }
        let expr = self.parse_expression()?;
        if self.pos < self.tokens.len() {
            return Err(FormulaParseError::TrailingInput(format!(
                "{:?}",
                &self.tokens[self.pos..]
            )));
        }
        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Expression, FormulaParseError> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expression, FormulaParseError> {
        let mut left = self.parse_multiplicative()?;

        while let Some(Token::Operator(op)) = self.peek() {
            if *op == '+' || *op == '-' {
                let op = *op;
                self.advance();
                let right = self.parse_multiplicative()?;
                left = Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, FormulaParseError> {
        let mut left = self.parse_power()?;

        while let Some(Token::Operator(op)) = self.peek() {
            if *op == '*' || *op == '/' || *op == '%' {
                let op = *op;
                self.advance();
                let right = self.parse_power()?;
                left = Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expression, FormulaParseError> {
        let base = self.parse_unary()?;

        if let Some(Token::Operator('^')) = self.peek() {
            self.advance();
            let exponent = self.parse_power()?; // Right associative
            Ok(Expression::BinaryOp {
                op: '^',
                left: Box::new(base),
                right: Box::new(exponent),
            })
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<Expression, FormulaParseError> {
        if let Some(Token::Operator(op)) = self.peek() {
            if *op == '+' || *op == '-' {
                let op = *op;
                self.advance();
                let operand = self.parse_unary()?;
                return Ok(Expression::UnaryOp {
                    op,
                    operand: Box::new(operand),
                });
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, FormulaParseError> {
        match self.peek().cloned() {
            Some(Token::Number(n)) => {
                self.advance();
                Ok(Expression::Number(n))
            }
            Some(Token::Variable(name)) => {
                self.advance();
                Ok(Expression::Variable(name))
            }
            Some(Token::Function(func)) => {
                self.advance();
                self.expect(&Token::LeftParen)?;
                let arg = self.parse_expression()?;
                self.expect(&Token::RightParen)?;
                Ok(Expression::FunctionCall {
                    function: func,
                    args: vec![arg],
                })
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&Token::RightParen)?;
                Ok(expr)
            }
            Some(token) => Err(FormulaParseError::UnexpectedToken(format!("{:?}", token))),
            None => Err(FormulaParseError::MissingOperand),
        }
    }
}

/// Parse an expression string
pub fn parse_expression(input: &str) -> Result<Expression, FormulaParseError> {
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Result of parsing a formula
#[derive(Clone, Debug)]
pub struct FormulaResult {
    /// The parsed expression (if valid)
    pub expression: Option<Expression>,
    /// Parse error (if any)
    pub error: Option<FormulaParseError>,
    /// Variables found in the expression
    pub variables: HashSet<String>,
    /// Evaluated result (if no free variables)
    pub value: Option<f64>,
}

/// Formula input component
#[component]
pub fn FormulaInput(
    /// Current formula value
    #[prop(optional, into)]
    value: Option<RwSignal<String>>,

    /// Callback when formula changes
    #[prop(optional, into)]
    on_change: Option<Callback<FormulaResult>>,

    /// Variables to use for evaluation
    #[prop(optional, into)]
    variables: Option<Signal<HashMap<String, f64>>>,

    /// Allowed variables (empty = any) - reserved for future validation
    #[prop(optional, into)]
    _allowed_variables: Option<Vec<String>>,

    /// Whether to show the parsed expression
    #[prop(optional, default = false)]
    show_parsed: bool,

    /// Whether to show evaluation result
    #[prop(optional, default = true)]
    show_result: bool,

    /// Whether to show variables list
    #[prop(optional, default = false)]
    show_variables: bool,

    /// Placeholder text
    #[prop(optional, into)]
    placeholder: Option<String>,

    /// Label
    #[prop(optional, into)]
    label: Option<String>,

    /// Description
    #[prop(optional, into)]
    description: Option<String>,

    /// Whether disabled
    #[prop(optional)]
    disabled: Signal<bool>,
) -> impl IntoView {
    let theme = use_theme();

    // Internal state
    let internal_value = value.unwrap_or_else(|| RwSignal::new(String::new()));
    let parse_result: RwSignal<Option<FormulaResult>> = RwSignal::new(None);

    // Parse on input change
    let parse_formula = move |input: &str| {
        if input.is_empty() {
            parse_result.set(None);
            return;
        }

        let result = parse_expression(input);
        let vars_map = variables.map(|v| v.get()).unwrap_or_default();

        let formula_result = match result {
            Ok(expr) => {
                let vars = expr.variables();
                let value = if vars.iter().all(|v| {
                    vars_map.contains_key(v)
                        || matches!(
                            v.as_str(),
                            "pi" | "PI" | "π" | "e" | "E" | "tau" | "TAU" | "τ"
                        )
                }) {
                    expr.evaluate(&vars_map).ok()
                } else {
                    None
                };
                FormulaResult {
                    expression: Some(expr),
                    error: None,
                    variables: vars,
                    value,
                }
            }
            Err(err) => FormulaResult {
                expression: None,
                error: Some(err),
                variables: HashSet::new(),
                value: None,
            },
        };

        if let Some(cb) = on_change {
            cb.run(formula_result.clone());
        }
        parse_result.set(Some(formula_result));
    };

    // Initial parse
    Effect::new(move |_| {
        parse_formula(&internal_value.get());
    });

    // Styles
    let container_styles = move || {
        let theme_val = theme.get();
        StyleBuilder::new()
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("gap", theme_val.spacing.xs)
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
            .build()
    };

    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let has_error = parse_result
            .get()
            .map(|r| r.error.is_some())
            .unwrap_or(false);

        StyleBuilder::new()
            .add("width", "100%")
            .add(
                "padding",
                format!("{} {}", theme_val.spacing.xs, theme_val.spacing.sm),
            )
            .add(
                "border",
                format!(
                    "1px solid {}",
                    if has_error {
                        scheme_colors
                            .get_color("red", 6)
                            .unwrap_or_else(|| "#fa5252".to_string())
                    } else {
                        scheme_colors.border.clone()
                    }
                ),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("background", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("font-family", "monospace")
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("outline", "none")
            .build()
    };

    let result_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-family", "monospace")
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("padding", theme_val.spacing.xs)
            .add(
                "background",
                scheme_colors
                    .get_color("gray", 1)
                    .unwrap_or_else(|| "#f8f9fa".to_string()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("color", scheme_colors.text.clone())
            .build()
    };

    let error_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", theme_val.typography.font_sizes.xs)
            .add(
                "color",
                scheme_colors
                    .get_color("red", 6)
                    .unwrap_or_else(|| "#fa5252".to_string()),
            )
            .build()
    };

    let description_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", theme_val.typography.font_sizes.xs)
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
            )
            .build()
    };

    let vars_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("display", "flex")
            .add("flex-wrap", "wrap")
            .add("gap", theme_val.spacing.xs)
            .add("font-size", theme_val.typography.font_sizes.xs)
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
            )
            .build()
    };

    let var_badge_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("padding", "0.125rem 0.375rem")
            .add(
                "background",
                scheme_colors
                    .get_color(&theme_val.colors.primary_color, 1)
                    .unwrap_or_else(|| "#e7f5ff".to_string()),
            )
            .add(
                "color",
                scheme_colors
                    .get_color(&theme_val.colors.primary_color, 7)
                    .unwrap_or_else(|| "#1971c2".to_string()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("font-family", "monospace")
            .build()
    };

    view! {
        <div class="mingot-formula-input" style=container_styles>
            {label.map(|l| view! {
                <label style=label_styles>{l}</label>
            })}

            <input
                type="text"
                style=input_styles
                placeholder=placeholder.unwrap_or_else(|| "Enter formula (e.g., sin(x) + 2*y)".to_string())
                prop:value=move || internal_value.get()
                disabled=disabled
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    internal_value.set(val.clone());
                    parse_formula(&val);
                }
            />

            {move || {
                let result = parse_result.get();
                match result {
                    Some(r) if r.error.is_some() => {
                        view! {
                            <div style=error_styles>
                                {r.error.map(|e| e.to_string()).unwrap_or_default()}
                            </div>
                        }.into_any()
                    }
                    Some(r) if show_result && r.value.is_some() => {
                        view! {
                            <div style=result_styles>
                                {"= "}{format!("{:.10}", r.value.unwrap()).trim_end_matches('0').trim_end_matches('.').to_string()}
                            </div>
                        }.into_any()
                    }
                    Some(r) if show_parsed && r.expression.is_some() => {
                        view! {
                            <div style=result_styles>
                                {"Parsed: "}{r.expression.map(|e| e.to_string()).unwrap_or_default()}
                            </div>
                        }.into_any()
                    }
                    _ => view! { <div></div> }.into_any()
                }
            }}

            {move || {
                let result = parse_result.get();
                if show_variables {
                    if let Some(r) = result {
                        if !r.variables.is_empty() {
                            let vars: Vec<_> = r.variables.into_iter().collect();
                            return view! {
                                <div style=vars_styles>
                                    <span>{"Variables: "}</span>
                                    {vars.into_iter().map(|v| view! {
                                        <span style=var_badge_styles>{v}</span>
                                    }).collect_view()}
                                </div>
                            }.into_any();
                        }
                    }
                }
                view! { <div></div> }.into_any()
            }}

            {description.map(|d| view! {
                <div style=description_styles>{d}</div>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("1 + 2").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Number(1.0));
        assert_eq!(tokens[1], Token::Operator('+'));
        assert_eq!(tokens[2], Token::Number(2.0));
    }

    #[test]
    fn test_tokenize_function() {
        let tokens = tokenize("sin(x)").unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Function(MathFunction::Sin));
        assert_eq!(tokens[1], Token::LeftParen);
        assert_eq!(tokens[2], Token::Variable("x".to_string()));
        assert_eq!(tokens[3], Token::RightParen);
    }

    #[test]
    fn test_parse_number() {
        let expr = parse_expression("42").unwrap();
        assert_eq!(expr, Expression::Number(42.0));
    }

    #[test]
    fn test_parse_variable() {
        let expr = parse_expression("x").unwrap();
        assert_eq!(expr, Expression::Variable("x".to_string()));
    }

    #[test]
    fn test_parse_addition() {
        let expr = parse_expression("1 + 2").unwrap();
        assert!(matches!(expr, Expression::BinaryOp { op: '+', .. }));
    }

    #[test]
    fn test_parse_multiplication() {
        let expr = parse_expression("3 * 4").unwrap();
        assert!(matches!(expr, Expression::BinaryOp { op: '*', .. }));
    }

    #[test]
    fn test_parse_function() {
        let expr = parse_expression("sin(x)").unwrap();
        assert!(matches!(expr, Expression::FunctionCall { .. }));
    }

    #[test]
    fn test_parse_complex() {
        let expr = parse_expression("sin(x) + 2 * cos(y)").unwrap();
        assert!(matches!(expr, Expression::BinaryOp { op: '+', .. }));
    }

    #[test]
    fn test_parse_power() {
        let expr = parse_expression("x^2").unwrap();
        assert!(matches!(expr, Expression::BinaryOp { op: '^', .. }));
    }

    #[test]
    fn test_parse_nested_parens() {
        let expr = parse_expression("((1 + 2) * 3)").unwrap();
        assert!(matches!(expr, Expression::BinaryOp { op: '*', .. }));
    }

    #[test]
    fn test_evaluate_simple() {
        let expr = parse_expression("1 + 2").unwrap();
        let result = expr.evaluate(&HashMap::new()).unwrap();
        assert!((result - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_with_variables() {
        let expr = parse_expression("x + y").unwrap();
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 3.0);
        vars.insert("y".to_string(), 4.0);
        let result = expr.evaluate(&vars).unwrap();
        assert!((result - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_function() {
        let expr = parse_expression("sqrt(16)").unwrap();
        let result = expr.evaluate(&HashMap::new()).unwrap();
        assert!((result - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_pi() {
        let expr = parse_expression("pi").unwrap();
        let result = expr.evaluate(&HashMap::new()).unwrap();
        assert!((result - PI).abs() < 1e-10);
    }

    #[test]
    fn test_get_variables() {
        let expr = parse_expression("x + y * z").unwrap();
        let vars = expr.variables();
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
        assert!(vars.contains("z"));
        assert_eq!(vars.len(), 3);
    }

    #[test]
    fn test_operator_precedence() {
        let expr = parse_expression("1 + 2 * 3").unwrap();
        let result = expr.evaluate(&HashMap::new()).unwrap();
        assert!((result - 7.0).abs() < 1e-10); // Should be 7, not 9
    }

    #[test]
    fn test_unary_minus() {
        let expr = parse_expression("-5").unwrap();
        let result = expr.evaluate(&HashMap::new()).unwrap();
        assert!((result - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn test_scientific_notation() {
        let expr = parse_expression("1e-3").unwrap();
        let result = expr.evaluate(&HashMap::new()).unwrap();
        assert!((result - 0.001).abs() < 1e-15);
    }

    #[test]
    fn test_math_function_evaluate() {
        assert!((MathFunction::Sin.evaluate(0.0)).abs() < 1e-10);
        assert!((MathFunction::Cos.evaluate(0.0) - 1.0).abs() < 1e-10);
        assert!((MathFunction::Exp.evaluate(0.0) - 1.0).abs() < 1e-10);
        assert!((MathFunction::Ln.evaluate(E) - 1.0).abs() < 1e-10);
        assert!((MathFunction::Sqrt.evaluate(4.0) - 2.0).abs() < 1e-10);
        assert!((MathFunction::Factorial.evaluate(5.0) - 120.0).abs() < 1e-10);
    }
}
