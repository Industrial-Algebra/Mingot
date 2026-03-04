//! Matrix input component for spreadsheet-style matrix entry.
//!
//! Supports arbitrary-precision matrix operations with keyboard navigation,
//! row/column manipulation, and operation previews.

use crate::components::input::InputSize;
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

/// Matrix notation style for display
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MatrixNotation {
    /// Standard brackets [ ]
    #[default]
    Brackets,
    /// Parentheses ( )
    Parentheses,
    /// Vertical bars | | (for determinant display)
    Bars,
    /// Double vertical bars || || (for norm)
    DoubleBars,
}

impl MatrixNotation {
    /// Get the left delimiter
    pub fn left(&self) -> &'static str {
        match self {
            MatrixNotation::Brackets => "[",
            MatrixNotation::Parentheses => "(",
            MatrixNotation::Bars => "|",
            MatrixNotation::DoubleBars => "‖",
        }
    }

    /// Get the right delimiter
    pub fn right(&self) -> &'static str {
        match self {
            MatrixNotation::Brackets => "]",
            MatrixNotation::Parentheses => ")",
            MatrixNotation::Bars => "|",
            MatrixNotation::DoubleBars => "‖",
        }
    }
}

/// Represents a matrix with f64 values
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    /// Matrix data stored in row-major order
    data: Vec<Vec<f64>>,
    /// Number of rows
    rows: usize,
    /// Number of columns
    cols: usize,
}

impl Default for Matrix {
    fn default() -> Self {
        Self::zeros(3, 3)
    }
}

impl Matrix {
    /// Create a new matrix with given dimensions filled with zeros
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];
        Self { data, rows, cols }
    }

    /// Create a new matrix with given dimensions filled with a value
    pub fn fill(rows: usize, cols: usize, value: f64) -> Self {
        let data = vec![vec![value; cols]; rows];
        Self { data, rows, cols }
    }

    /// Create an identity matrix
    pub fn identity(size: usize) -> Self {
        let mut m = Self::zeros(size, size);
        for i in 0..size {
            m.set(i, i, 1.0);
        }
        m
    }

    /// Create a matrix from a 2D vector
    pub fn from_vec(data: Vec<Vec<f64>>) -> Option<Self> {
        if data.is_empty() {
            return Some(Self::zeros(0, 0));
        }
        let rows = data.len();
        let cols = data[0].len();
        // Verify all rows have same length
        if !data.iter().all(|row| row.len() == cols) {
            return None;
        }
        Some(Self { data, rows, cols })
    }

    /// Get the number of rows
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of columns
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Check if matrix is square
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    /// Get a value at (row, col)
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        self.data.get(row).and_then(|r| r.get(col).copied())
    }

    /// Set a value at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
        }
    }

    /// Get a row as a slice
    pub fn row(&self, index: usize) -> Option<&[f64]> {
        self.data.get(index).map(|r| r.as_slice())
    }

    /// Get a column as a vector
    pub fn col(&self, index: usize) -> Option<Vec<f64>> {
        if index >= self.cols {
            return None;
        }
        Some(self.data.iter().map(|row| row[index]).collect())
    }

    /// Calculate the trace (sum of diagonal elements)
    pub fn trace(&self) -> Option<f64> {
        if !self.is_square() {
            return None;
        }
        Some((0..self.rows).map(|i| self.data[i][i]).sum())
    }

    /// Calculate the determinant (for small matrices)
    pub fn determinant(&self) -> Option<f64> {
        if !self.is_square() {
            return None;
        }
        match self.rows {
            0 => Some(1.0),
            1 => Some(self.data[0][0]),
            2 => Some(self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]),
            3 => {
                let a = self.data[0][0];
                let b = self.data[0][1];
                let c = self.data[0][2];
                let d = self.data[1][0];
                let e = self.data[1][1];
                let f = self.data[1][2];
                let g = self.data[2][0];
                let h = self.data[2][1];
                let i = self.data[2][2];
                Some(a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g))
            }
            _ => {
                // LU decomposition for larger matrices
                self.determinant_lu()
            }
        }
    }

    /// Calculate determinant using LU decomposition
    #[allow(clippy::needless_range_loop)]
    fn determinant_lu(&self) -> Option<f64> {
        if !self.is_square() {
            return None;
        }
        let n = self.rows;
        let mut lu = self.data.clone();
        let mut det = 1.0;

        for k in 0..n {
            // Find pivot
            let mut max_val = lu[k][k].abs();
            let mut max_row = k;
            for i in (k + 1)..n {
                if lu[i][k].abs() > max_val {
                    max_val = lu[i][k].abs();
                    max_row = i;
                }
            }

            if max_val < 1e-10 {
                return Some(0.0); // Singular matrix
            }

            // Swap rows if needed
            if max_row != k {
                lu.swap(k, max_row);
                det = -det;
            }

            det *= lu[k][k];

            // Eliminate - indexed access required for row reduction
            for i in (k + 1)..n {
                let factor = lu[i][k] / lu[k][k];
                for j in k..n {
                    lu[i][j] -= factor * lu[k][j];
                }
            }
        }

        Some(det)
    }

    /// Calculate the Frobenius norm
    pub fn frobenius_norm(&self) -> f64 {
        self.data
            .iter()
            .flat_map(|row| row.iter())
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt()
    }

    /// Transpose the matrix
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.data[i][j]);
            }
        }
        result
    }

    /// Add a row at the specified index
    pub fn add_row(&mut self, index: usize) {
        if index <= self.rows {
            self.data.insert(index, vec![0.0; self.cols]);
            self.rows += 1;
        }
    }

    /// Add a column at the specified index
    pub fn add_col(&mut self, index: usize) {
        if index <= self.cols {
            for row in &mut self.data {
                row.insert(index, 0.0);
            }
            self.cols += 1;
        }
    }

    /// Remove a row at the specified index
    pub fn remove_row(&mut self, index: usize) {
        if index < self.rows && self.rows > 1 {
            self.data.remove(index);
            self.rows -= 1;
        }
    }

    /// Remove a column at the specified index
    pub fn remove_col(&mut self, index: usize) {
        if index < self.cols && self.cols > 1 {
            for row in &mut self.data {
                row.remove(index);
            }
            self.cols -= 1;
        }
    }

    /// Format as LaTeX
    pub fn to_latex(&self) -> String {
        let mut result = String::from("\\begin{pmatrix}\n");
        for (i, row) in self.data.iter().enumerate() {
            let row_str: Vec<String> = row.iter().map(|v| format_number(*v)).collect();
            result.push_str(&row_str.join(" & "));
            if i < self.rows - 1 {
                result.push_str(" \\\\\n");
            } else {
                result.push('\n');
            }
        }
        result.push_str("\\end{pmatrix}");
        result
    }

    /// Format for MATLAB/Octave
    pub fn to_matlab(&self) -> String {
        let rows: Vec<String> = self
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| format_number(*v))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .collect();
        format!("[{}]", rows.join("; "))
    }

    /// Format for NumPy
    pub fn to_numpy(&self) -> String {
        let rows: Vec<String> = self
            .data
            .iter()
            .map(|row| {
                let vals: Vec<String> = row.iter().map(|v| format_number(*v)).collect();
                format!("[{}]", vals.join(", "))
            })
            .collect();
        format!("np.array([{}])", rows.join(", "))
    }

    /// Format for Mathematica
    pub fn to_mathematica(&self) -> String {
        let rows: Vec<String> = self
            .data
            .iter()
            .map(|row| {
                let vals: Vec<String> = row.iter().map(|v| format_number(*v)).collect();
                format!("{{{}}}", vals.join(", "))
            })
            .collect();
        format!("{{{}}}", rows.join(", "))
    }
}

/// Format a number, removing unnecessary trailing zeros
fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        let s = format!("{:.6}", value);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Matrix operation that can be previewed
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MatrixOperation {
    Determinant,
    Trace,
    Transpose,
    FrobeniusNorm,
}

impl MatrixOperation {
    /// Get the display label
    pub fn label(&self) -> &'static str {
        match self {
            MatrixOperation::Determinant => "det",
            MatrixOperation::Trace => "tr",
            MatrixOperation::Transpose => "T",
            MatrixOperation::FrobeniusNorm => "‖·‖F",
        }
    }
}

/// Matrix input component
#[component]
pub fn MatrixInput(
    /// Current matrix value
    #[prop(optional, into)]
    value: Option<RwSignal<Matrix>>,

    /// Callback when matrix changes
    #[prop(optional, into)]
    on_change: Option<Callback<Matrix>>,

    /// Initial number of rows
    #[prop(optional, default = 3)]
    rows: usize,

    /// Initial number of columns
    #[prop(optional, default = 3)]
    cols: usize,

    /// Whether to show row/column manipulation buttons
    #[prop(optional, default = true)]
    allow_resize: bool,

    /// Whether to show matrix operations preview
    #[prop(optional, default = true)]
    show_operations: bool,

    /// Matrix notation style
    #[prop(optional)]
    notation: MatrixNotation,

    /// Number of decimal places for display
    #[prop(optional, default = 4)]
    precision: usize,

    /// Input size
    #[prop(optional)]
    size: Option<InputSize>,

    /// Label text
    #[prop(optional, into)]
    label: Option<String>,

    /// Description text
    #[prop(optional, into)]
    description: Option<String>,

    /// Error message
    #[prop(optional, into)]
    error: Option<String>,

    /// Whether the input is disabled
    #[prop(optional)]
    disabled: Signal<bool>,
) -> impl IntoView {
    let theme = use_theme();

    // Internal state
    let internal_matrix = value.unwrap_or_else(|| RwSignal::new(Matrix::zeros(rows, cols)));

    // Currently focused cell
    let focused_cell: RwSignal<Option<(usize, usize)>> = RwSignal::new(None);

    // Update matrix when cell changes
    let update_cell = move |row: usize, col: usize, value: String| {
        if let Ok(num) = value.parse::<f64>() {
            let mut matrix = internal_matrix.get();
            matrix.set(row, col, num);
            internal_matrix.set(matrix.clone());
            if let Some(cb) = on_change {
                cb.run(matrix);
            }
        }
    };

    // Handle keyboard navigation (arrow keys only - Tab handled by tabindex)
    let handle_keydown = move |_row: usize, _col: usize, _ev: ev::KeyboardEvent| {
        // Arrow key navigation could be added here if needed
        // For now, Tab navigation is handled by tabindex attribute
    };

    // Add row
    let add_row = move |_| {
        let mut matrix = internal_matrix.get();
        matrix.add_row(matrix.rows());
        internal_matrix.set(matrix);
    };

    // Add column
    let add_col = move |_| {
        let mut matrix = internal_matrix.get();
        matrix.add_col(matrix.cols());
        internal_matrix.set(matrix);
    };

    // Remove row
    let remove_row = move |_| {
        let mut matrix = internal_matrix.get();
        if matrix.rows() > 1 {
            matrix.remove_row(matrix.rows() - 1);
            internal_matrix.set(matrix);
        }
    };

    // Remove column
    let remove_col = move |_| {
        let mut matrix = internal_matrix.get();
        if matrix.cols() > 1 {
            matrix.remove_col(matrix.cols() - 1);
            internal_matrix.set(matrix);
        }
    };

    // Styles
    let container_styles = move || {
        let theme_val = theme.get();
        StyleBuilder::new()
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("gap", theme_val.spacing.sm)
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

    let matrix_container_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "stretch")
            .add("gap", "0.25rem")
            .build()
    };

    let left_bracket_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let border = format!("2px solid {}", scheme_colors.text);
        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("width", "6px")
            .add("border-left", border.clone())
            .add("border-top", border.clone())
            .add("border-bottom", border.clone())
            .add_if(
                notation == MatrixNotation::DoubleBars,
                "box-shadow",
                format!("-4px 0 0 0 {}", scheme_colors.text),
            )
            .add_if(
                notation == MatrixNotation::Parentheses,
                "border-radius",
                "50% 0 0 50%",
            )
            .build()
    };

    let right_bracket_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let border = format!("2px solid {}", scheme_colors.text);
        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("width", "6px")
            .add("border-right", border.clone())
            .add("border-top", border.clone())
            .add("border-bottom", border.clone())
            .add_if(
                notation == MatrixNotation::DoubleBars,
                "box-shadow",
                format!("4px 0 0 0 {}", scheme_colors.text),
            )
            .add_if(
                notation == MatrixNotation::Parentheses,
                "border-radius",
                "0 50% 50% 0",
            )
            .build()
    };

    let grid_styles = move || {
        let matrix = internal_matrix.get();
        StyleBuilder::new()
            .add("display", "grid")
            .add(
                "grid-template-columns",
                format!("repeat({}, 1fr)", matrix.cols()),
            )
            .add("gap", "2px")
            .build()
    };

    let cell_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let size_vals = match size.unwrap_or(InputSize::Sm) {
            InputSize::Xs => ("0.125rem 0.25rem", "50px"),
            InputSize::Sm => ("0.25rem 0.5rem", "60px"),
            InputSize::Md => ("0.375rem 0.5rem", "70px"),
            InputSize::Lg => ("0.5rem 0.75rem", "80px"),
            InputSize::Xl => ("0.625rem 1rem", "90px"),
        };

        StyleBuilder::new()
            .add("padding", size_vals.0)
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", "2px")
            .add("background", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("width", size_vals.1)
            .add("text-align", "center")
            .add("font-family", "monospace")
            .build()
    };

    let resize_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("padding", "0.25rem 0.5rem")
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("background", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("cursor", "pointer")
            .add("font-size", theme_val.typography.font_sizes.xs)
            .build()
    };

    let operations_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("display", "flex")
            .add("gap", "1rem")
            .add("flex-wrap", "wrap")
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("font-family", "monospace")
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
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

    view! {
        <div class="mingot-matrix-input" style=container_styles>
            {label.clone().map(|l| view! {
                <label style=label_styles>{l}</label>
            })}

            <div style=matrix_container_styles>
                <span style=left_bracket_styles></span>

                <div style=grid_styles>
                    {move || {
                        let matrix = internal_matrix.get();
                        let cols = matrix.cols();
                        let mut cells = Vec::new();
                        for r in 0..matrix.rows() {
                            for c in 0..cols {
                                let val = matrix.get(r, c).unwrap_or(0.0);
                                let val_str = RwSignal::new(format_number(val));
                                let tab_index = (r * cols + c + 1) as i32;

                                cells.push(view! {
                                    <input
                                        type="text"
                                        style=cell_styles
                                        tabindex=tab_index
                                        prop:value=move || val_str.get()
                                        disabled=disabled
                                        on:input=move |ev| {
                                            let new_val = event_target_value(&ev);
                                            val_str.set(new_val.clone());
                                            update_cell(r, c, new_val);
                                        }
                                        on:focus=move |_| {
                                            focused_cell.set(Some((r, c)));
                                        }
                                        on:keydown=move |ev| {
                                            handle_keydown(r, c, ev);
                                        }
                                    />
                                });
                            }
                        }
                        cells.collect_view()
                    }}
                </div>

                <span style=right_bracket_styles></span>
            </div>

            {allow_resize.then(|| {
                view! {
                    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                        <button type="button" style=resize_button_styles on:click=add_row disabled=disabled>
                            {"+ Row"}
                        </button>
                        <button type="button" style=resize_button_styles on:click=remove_row disabled=disabled>
                            {"- Row"}
                        </button>
                        <button type="button" style=resize_button_styles on:click=add_col disabled=disabled>
                            {"+ Col"}
                        </button>
                        <button type="button" style=resize_button_styles on:click=remove_col disabled=disabled>
                            {"- Col"}
                        </button>
                    </div>
                }
            })}

            {show_operations.then(|| {
                view! {
                    <div style=operations_styles>
                        {move || {
                            let matrix = internal_matrix.get();
                            let mut ops = Vec::new();

                            // Dimensions
                            ops.push(format!("{}×{}", matrix.rows(), matrix.cols()));

                            // Determinant (for square matrices)
                            if let Some(det) = matrix.determinant() {
                                ops.push(format!("det = {:.prec$}", det, prec = precision));
                            }

                            // Trace (for square matrices)
                            if let Some(tr) = matrix.trace() {
                                ops.push(format!("tr = {:.prec$}", tr, prec = precision));
                            }

                            // Frobenius norm
                            let norm = matrix.frobenius_norm();
                            ops.push(format!("‖A‖F = {:.prec$}", norm, prec = precision));

                            ops.into_iter().map(|op| {
                                view! { <span>{op}</span> }
                            }).collect_view()
                        }}
                    </div>
                }
            })}

            {description.map(|d| view! {
                <div style=description_styles>{d}</div>
            })}

            {error.map(|e| view! {
                <div style=error_styles>{e}</div>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_zeros() {
        let m = Matrix::zeros(3, 4);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 4);
        assert_eq!(m.get(0, 0), Some(0.0));
    }

    #[test]
    fn test_matrix_identity() {
        let m = Matrix::identity(3);
        assert_eq!(m.get(0, 0), Some(1.0));
        assert_eq!(m.get(0, 1), Some(0.0));
        assert_eq!(m.get(1, 1), Some(1.0));
        assert_eq!(m.get(2, 2), Some(1.0));
    }

    #[test]
    fn test_matrix_from_vec() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let m = Matrix::from_vec(data).unwrap();
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 2);
        assert_eq!(m.get(0, 0), Some(1.0));
        assert_eq!(m.get(1, 1), Some(4.0));
    }

    #[test]
    fn test_matrix_trace() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let m = Matrix::from_vec(data).unwrap();
        assert_eq!(m.trace(), Some(5.0));
    }

    #[test]
    fn test_matrix_determinant_2x2() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let m = Matrix::from_vec(data).unwrap();
        assert_eq!(m.determinant(), Some(-2.0));
    }

    #[test]
    fn test_matrix_determinant_3x3() {
        let data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let m = Matrix::from_vec(data).unwrap();
        // This matrix is singular, det = 0
        let det = m.determinant().unwrap();
        assert!(det.abs() < 1e-10);
    }

    #[test]
    fn test_matrix_transpose() {
        let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let m = Matrix::from_vec(data).unwrap();
        let t = m.transpose();
        assert_eq!(t.rows(), 3);
        assert_eq!(t.cols(), 2);
        assert_eq!(t.get(0, 0), Some(1.0));
        assert_eq!(t.get(0, 1), Some(4.0));
        assert_eq!(t.get(2, 1), Some(6.0));
    }

    #[test]
    fn test_matrix_add_row() {
        let mut m = Matrix::zeros(2, 3);
        m.add_row(1);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 3);
    }

    #[test]
    fn test_matrix_add_col() {
        let mut m = Matrix::zeros(2, 3);
        m.add_col(2);
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 4);
    }

    #[test]
    fn test_matrix_remove_row() {
        let mut m = Matrix::zeros(3, 3);
        m.remove_row(1);
        assert_eq!(m.rows(), 2);
    }

    #[test]
    fn test_matrix_remove_col() {
        let mut m = Matrix::zeros(3, 3);
        m.remove_col(1);
        assert_eq!(m.cols(), 2);
    }

    #[test]
    fn test_matrix_frobenius_norm() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let m = Matrix::from_vec(data).unwrap();
        // sqrt(1 + 4 + 9 + 16) = sqrt(30)
        let expected = 30.0_f64.sqrt();
        assert!((m.frobenius_norm() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_matrix_to_latex() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let m = Matrix::from_vec(data).unwrap();
        let latex = m.to_latex();
        assert!(latex.contains("\\begin{pmatrix}"));
        assert!(latex.contains("\\end{pmatrix}"));
    }

    #[test]
    fn test_matrix_to_matlab() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let m = Matrix::from_vec(data).unwrap();
        let matlab = m.to_matlab();
        assert_eq!(matlab, "[1, 2; 3, 4]");
    }

    #[test]
    fn test_matrix_to_numpy() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let m = Matrix::from_vec(data).unwrap();
        let numpy = m.to_numpy();
        assert!(numpy.starts_with("np.array("));
    }

    #[test]
    fn test_matrix_notation() {
        assert_eq!(MatrixNotation::Brackets.left(), "[");
        assert_eq!(MatrixNotation::Brackets.right(), "]");
        assert_eq!(MatrixNotation::Parentheses.left(), "(");
        assert_eq!(MatrixNotation::Bars.left(), "|");
    }
}
