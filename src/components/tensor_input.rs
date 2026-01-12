//! Tensor input component for multi-dimensional array entry.
//!
//! Supports arbitrary-rank tensors with slice navigation and
//! shape manipulation.

use crate::components::input::InputSize;
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

/// Represents a multi-dimensional tensor
#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    /// Flattened data storage
    data: Vec<f64>,
    /// Shape of the tensor (dimensions)
    shape: Vec<usize>,
    /// Total number of elements
    size: usize,
}

impl Default for Tensor {
    fn default() -> Self {
        Self::zeros(vec![2, 2, 2])
    }
}

impl Tensor {
    /// Create a tensor filled with zeros
    pub fn zeros(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        Self {
            data: vec![0.0; size],
            shape,
            size,
        }
    }

    /// Create a tensor filled with a value
    pub fn fill(shape: Vec<usize>, value: f64) -> Self {
        let size: usize = shape.iter().product();
        Self {
            data: vec![value; size],
            shape,
            size,
        }
    }

    /// Create a tensor from data with given shape
    pub fn from_data(data: Vec<f64>, shape: Vec<usize>) -> Option<Self> {
        let size: usize = shape.iter().product();
        if data.len() != size {
            return None;
        }
        Some(Self { data, shape, size })
    }

    /// Get the shape
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /// Get the rank (number of dimensions)
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    /// Get total number of elements
    pub fn size(&self) -> usize {
        self.size
    }

    /// Convert multi-dimensional index to flat index
    fn flat_index(&self, indices: &[usize]) -> Option<usize> {
        if indices.len() != self.shape.len() {
            return None;
        }
        let mut flat = 0;
        let mut stride = 1;
        for (i, &idx) in indices.iter().rev().enumerate() {
            let dim = self.shape[self.shape.len() - 1 - i];
            if idx >= dim {
                return None;
            }
            flat += idx * stride;
            stride *= dim;
        }
        Some(flat)
    }

    /// Convert flat index to multi-dimensional indices
    fn multi_index(&self, flat: usize) -> Option<Vec<usize>> {
        if flat >= self.size {
            return None;
        }
        let mut indices = vec![0; self.shape.len()];
        let mut remaining = flat;
        for i in (0..self.shape.len()).rev() {
            indices[i] = remaining % self.shape[i];
            remaining /= self.shape[i];
        }
        Some(indices)
    }

    /// Get a value at the given indices
    pub fn get(&self, indices: &[usize]) -> Option<f64> {
        self.flat_index(indices)
            .and_then(|i| self.data.get(i).copied())
    }

    /// Set a value at the given indices
    pub fn set(&mut self, indices: &[usize], value: f64) -> bool {
        if let Some(i) = self.flat_index(indices) {
            if i < self.data.len() {
                self.data[i] = value;
                return true;
            }
        }
        false
    }

    /// Get a 2D slice at fixed indices for other dimensions
    /// Returns (rows, cols, data) for the slice
    pub fn slice_2d(&self, fixed_indices: &[(usize, usize)]) -> Option<(usize, usize, Vec<f64>)> {
        if self.rank() < 2 {
            return None;
        }

        // Determine which dimensions are free (the last two by default)
        let fixed_dims: Vec<(usize, usize)> = fixed_indices.to_vec();

        // Collect free dimensions
        let mut free_dims: Vec<usize> = (0..self.rank())
            .filter(|d| !fixed_dims.iter().any(|(dim, _)| dim == d))
            .collect();

        if free_dims.len() < 2 {
            return None;
        }

        // Take last two free dimensions as row and col
        let col_dim = free_dims.pop()?;
        let row_dim = free_dims.pop()?;

        let rows = self.shape[row_dim];
        let cols = self.shape[col_dim];

        let mut slice_data = Vec::with_capacity(rows * cols);
        let mut indices = vec![0; self.rank()];

        // Set fixed dimensions
        for (dim, val) in &fixed_dims {
            indices[*dim] = *val;
        }

        for r in 0..rows {
            indices[row_dim] = r;
            for c in 0..cols {
                indices[col_dim] = c;
                slice_data.push(self.get(&indices).unwrap_or(0.0));
            }
        }

        Some((rows, cols, slice_data))
    }

    /// Reshape the tensor (must have same total size)
    pub fn reshape(&mut self, new_shape: Vec<usize>) -> bool {
        let new_size: usize = new_shape.iter().product();
        if new_size != self.size {
            return false;
        }
        self.shape = new_shape;
        true
    }

    /// Transpose (swap last two dimensions)
    pub fn transpose(&self) -> Option<Tensor> {
        if self.rank() < 2 {
            return None;
        }

        let mut new_shape = self.shape.clone();
        let n = new_shape.len();
        new_shape.swap(n - 2, n - 1);

        let mut new_data = vec![0.0; self.size];

        for flat in 0..self.size {
            if let Some(idx) = self.multi_index(flat) {
                // Swap last two indices
                let mut new_idx = idx.clone();
                new_idx.swap(n - 2, n - 1);

                // Calculate new flat index
                let mut new_flat = 0;
                let mut stride = 1;
                for i in (0..n).rev() {
                    new_flat += new_idx[i] * stride;
                    stride *= new_shape[i];
                }

                new_data[new_flat] = self.data[flat];
            }
        }

        Tensor::from_data(new_data, new_shape)
    }

    /// Calculate Frobenius norm
    pub fn frobenius_norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// Get sum of all elements
    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    /// Get mean of all elements
    pub fn mean(&self) -> f64 {
        if self.size == 0 {
            return 0.0;
        }
        self.sum() / self.size as f64
    }

    /// Get min value
    pub fn min(&self) -> Option<f64> {
        self.data.iter().copied().reduce(f64::min)
    }

    /// Get max value
    pub fn max(&self) -> Option<f64> {
        self.data.iter().copied().reduce(f64::max)
    }

    /// Format shape as string
    pub fn shape_string(&self) -> String {
        format!(
            "({})",
            self.shape
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(" × ")
        )
    }
}

/// Format a number
fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        let s = format!("{:.4}", value);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Tensor input component
#[component]
pub fn TensorInput(
    /// Current tensor value
    #[prop(optional, into)]
    value: Option<RwSignal<Tensor>>,

    /// Callback when tensor changes
    #[prop(optional, into)]
    on_change: Option<Callback<Tensor>>,

    /// Initial shape
    #[prop(optional, into)]
    shape: Option<Vec<usize>>,

    /// Whether to show statistics
    #[prop(optional, default = true)]
    show_stats: bool,

    /// Number of decimal places
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
    let initial_shape = shape.unwrap_or_else(|| vec![2, 3, 4]);
    let internal_tensor = value.unwrap_or_else(|| RwSignal::new(Tensor::zeros(initial_shape)));

    // Current slice indices (for dimensions beyond 2)
    let slice_indices: RwSignal<Vec<usize>> = RwSignal::new(Vec::new());

    // Initialize slice indices
    Effect::new(move |_| {
        let tensor = internal_tensor.get();
        if tensor.rank() > 2 && slice_indices.get().len() != tensor.rank() - 2 {
            slice_indices.set(vec![0; tensor.rank() - 2]);
        }
    });

    // Get current 2D slice to display
    let current_slice = move || {
        let tensor = internal_tensor.get();
        let indices = slice_indices.get();

        if tensor.rank() <= 2 {
            // Direct 2D (or less) view
            let rows = tensor.shape().first().copied().unwrap_or(1);
            let cols = tensor.shape().get(1).copied().unwrap_or(1);
            (rows, cols, tensor.data.clone())
        } else {
            // Get 2D slice at current indices
            let fixed: Vec<(usize, usize)> =
                indices.iter().enumerate().map(|(i, &v)| (i, v)).collect();
            tensor.slice_2d(&fixed).unwrap_or((1, 1, vec![0.0]))
        }
    };

    // Update tensor value
    let update_value = move |row: usize, col: usize, new_val: f64| {
        let mut tensor = internal_tensor.get();
        let indices = slice_indices.get();

        let full_indices: Vec<usize> = if tensor.rank() <= 2 {
            if tensor.rank() == 1 {
                vec![col]
            } else {
                vec![row, col]
            }
        } else {
            let mut idx = indices.clone();
            idx.push(row);
            idx.push(col);
            idx
        };

        tensor.set(&full_indices, new_val);
        internal_tensor.set(tensor.clone());

        if let Some(cb) = on_change {
            cb.run(tensor);
        }
    };

    // Navigate slice
    let change_slice_index = move |dim: usize, delta: i32| {
        let tensor = internal_tensor.get();
        let mut indices = slice_indices.get();

        if dim < indices.len() {
            let max_val = tensor.shape()[dim];
            let current = indices[dim] as i32;
            let new_val = (current + delta).rem_euclid(max_val as i32) as usize;
            indices[dim] = new_val;
            slice_indices.set(indices);
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

    let shape_info_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
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

    let slice_nav_styles = move || {
        let theme_val = theme.get();
        StyleBuilder::new()
            .add("display", "flex")
            .add("gap", "1rem")
            .add("flex-wrap", "wrap")
            .add("align-items", "center")
            .add("margin-bottom", theme_val.spacing.xs)
            .build()
    };

    let slice_control_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("gap", "0.25rem")
            .add("font-size", theme_val.typography.font_sizes.xs)
            .add("color", scheme_colors.text.clone())
            .build()
    };

    let nav_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("padding", "0.125rem 0.375rem")
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", "2px")
            .add("background", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("cursor", "pointer")
            .add("font-size", theme_val.typography.font_sizes.xs)
            .build()
    };

    let grid_styles = move || {
        let (_, cols, _) = current_slice();
        StyleBuilder::new()
            .add("display", "grid")
            .add("grid-template-columns", format!("repeat({}, 1fr)", cols))
            .add("gap", "2px")
            .build()
    };

    let cell_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let size_vals = match size.unwrap_or(InputSize::Sm) {
            InputSize::Xs => ("0.125rem 0.25rem", "45px"),
            InputSize::Sm => ("0.25rem 0.5rem", "55px"),
            InputSize::Md => ("0.375rem 0.5rem", "65px"),
            InputSize::Lg => ("0.5rem 0.75rem", "75px"),
            InputSize::Xl => ("0.625rem 1rem", "85px"),
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
            .add("font-size", theme_val.typography.font_sizes.xs)
            .add("width", size_vals.1)
            .add("text-align", "center")
            .add("font-family", "monospace")
            .build()
    };

    let stats_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("display", "flex")
            .add("gap", "1rem")
            .add("flex-wrap", "wrap")
            .add("font-size", theme_val.typography.font_sizes.xs)
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
        <div class="mingot-tensor-input" style=container_styles>
            {label.clone().map(|l| view! {
                <label style=label_styles>{l}</label>
            })}

            <div style=shape_info_styles>
                {move || {
                    let tensor = internal_tensor.get();
                    format!("Shape: {} | Rank: {} | Size: {}",
                        tensor.shape_string(),
                        tensor.rank(),
                        tensor.size()
                    )
                }}
            </div>

            // Slice navigation (for rank > 2)
            {move || {
                let tensor = internal_tensor.get();
                let indices = slice_indices.get();

                if tensor.rank() <= 2 {
                    return view! { <div></div> }.into_any();
                }

                view! {
                    <div style=slice_nav_styles>
                        {(0..tensor.rank() - 2).map(|dim| {
                            let max_val = tensor.shape()[dim];
                            let current = indices.get(dim).copied().unwrap_or(0);
                            view! {
                                <div style=slice_control_styles>
                                    <span>{format!("dim[{}]:", dim)}</span>
                                    <button
                                        type="button"
                                        style=nav_button_styles
                                        on:click=move |_| change_slice_index(dim, -1)
                                        disabled=disabled
                                    >
                                        {"<"}
                                    </button>
                                    <span>{format!("{}/{}", current, max_val)}</span>
                                    <button
                                        type="button"
                                        style=nav_button_styles
                                        on:click=move |_| change_slice_index(dim, 1)
                                        disabled=disabled
                                    >
                                        {">"}
                                    </button>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            }}

            // 2D slice grid
            <div style=grid_styles>
                {move || {
                    let (rows, cols, data) = current_slice();
                    let mut cells = Vec::with_capacity(rows * cols);
                    for r in 0..rows {
                        for c in 0..cols {
                            let idx = r * cols + c;
                            let val = data.get(idx).copied().unwrap_or(0.0);
                            let val_str = RwSignal::new(format_number(val));
                            let tab_index = (idx + 1) as i32;

                            cells.push(view! {
                                <input
                                    type="text"
                                    style=cell_styles
                                    tabindex=tab_index
                                    prop:value=move || val_str.get()
                                    disabled=disabled
                                    on:input=move |ev| {
                                        let new_val_str = event_target_value(&ev);
                                        val_str.set(new_val_str.clone());
                                        if let Ok(num) = new_val_str.parse::<f64>() {
                                            update_value(r, c, num);
                                        }
                                    }
                                />
                            });
                        }
                    }
                    cells.collect_view()
                }}
            </div>

            {show_stats.then(|| {
                view! {
                    <div style=stats_styles>
                        {move || {
                            let tensor = internal_tensor.get();
                            let mut stats = Vec::new();

                            stats.push(format!("‖T‖F = {:.prec$}", tensor.frobenius_norm(), prec = precision));
                            stats.push(format!("Σ = {:.prec$}", tensor.sum(), prec = precision));
                            stats.push(format!("μ = {:.prec$}", tensor.mean(), prec = precision));

                            if let (Some(min), Some(max)) = (tensor.min(), tensor.max()) {
                                stats.push(format!("min = {:.prec$}", min, prec = precision));
                                stats.push(format!("max = {:.prec$}", max, prec = precision));
                            }

                            stats.into_iter().map(|s| view! { <span>{s}</span> }).collect_view()
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
    fn test_tensor_zeros() {
        let t = Tensor::zeros(vec![2, 3, 4]);
        assert_eq!(t.rank(), 3);
        assert_eq!(t.size(), 24);
        assert_eq!(t.shape(), &[2, 3, 4]);
    }

    #[test]
    fn test_tensor_get_set() {
        let mut t = Tensor::zeros(vec![2, 3]);
        assert!(t.set(&[1, 2], 5.0));
        assert_eq!(t.get(&[1, 2]), Some(5.0));
        assert_eq!(t.get(&[0, 0]), Some(0.0));
    }

    #[test]
    fn test_tensor_flat_index() {
        let t = Tensor::zeros(vec![2, 3, 4]);
        // For shape [2, 3, 4], index [1, 2, 3] should be:
        // 1*12 + 2*4 + 3 = 23
        assert_eq!(t.flat_index(&[1, 2, 3]), Some(23));
    }

    #[test]
    fn test_tensor_multi_index() {
        let t = Tensor::zeros(vec![2, 3, 4]);
        assert_eq!(t.multi_index(23), Some(vec![1, 2, 3]));
        assert_eq!(t.multi_index(0), Some(vec![0, 0, 0]));
    }

    #[test]
    fn test_tensor_reshape() {
        let mut t = Tensor::zeros(vec![2, 3, 4]);
        assert!(t.reshape(vec![4, 6]));
        assert_eq!(t.shape(), &[4, 6]);
        assert_eq!(t.rank(), 2);

        // Invalid reshape
        assert!(!t.reshape(vec![5, 5]));
    }

    #[test]
    fn test_tensor_frobenius_norm() {
        let t = Tensor::from_data(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]).unwrap();
        // sqrt(1 + 4 + 9 + 16) = sqrt(30)
        let expected = 30.0_f64.sqrt();
        assert!((t.frobenius_norm() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_tensor_statistics() {
        let t = Tensor::from_data(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]).unwrap();
        assert_eq!(t.sum(), 10.0);
        assert_eq!(t.mean(), 2.5);
        assert_eq!(t.min(), Some(1.0));
        assert_eq!(t.max(), Some(4.0));
    }

    #[test]
    fn test_tensor_transpose() {
        let t = Tensor::from_data(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![2, 3]).unwrap();
        let transposed = t.transpose().unwrap();
        assert_eq!(transposed.shape(), &[3, 2]);
        // Original: [[1, 2, 3], [4, 5, 6]]
        // Transposed: [[1, 4], [2, 5], [3, 6]]
        assert_eq!(transposed.get(&[0, 0]), Some(1.0));
        assert_eq!(transposed.get(&[0, 1]), Some(4.0));
        assert_eq!(transposed.get(&[1, 0]), Some(2.0));
        assert_eq!(transposed.get(&[2, 1]), Some(6.0));
    }

    #[test]
    fn test_tensor_slice_2d() {
        let t = Tensor::from_data((0..24).map(|x| x as f64).collect(), vec![2, 3, 4]).unwrap();

        // Get slice at dim[0] = 0
        let (rows, cols, data) = t.slice_2d(&[(0, 0)]).unwrap();
        assert_eq!(rows, 3);
        assert_eq!(cols, 4);
        assert_eq!(data.len(), 12);
        assert_eq!(data[0], 0.0);
        assert_eq!(data[11], 11.0);

        // Get slice at dim[0] = 1
        let (rows, cols, data) = t.slice_2d(&[(0, 1)]).unwrap();
        assert_eq!(rows, 3);
        assert_eq!(cols, 4);
        assert_eq!(data[0], 12.0);
    }

    #[test]
    fn test_tensor_shape_string() {
        let t = Tensor::zeros(vec![2, 3, 4]);
        assert_eq!(t.shape_string(), "(2 × 3 × 4)");
    }
}
