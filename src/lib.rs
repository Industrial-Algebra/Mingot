//! Mingot - The Leptos UI library for applications that demand mathematical precision
//!
//! Mingot provides ultra-precision UI components for scientific computing, financial applications,
//! and mathematical software. Built with first-class support for u64+ precision integers and
//! arbitrary-precision arithmetic via Amari integration.
//!
//! ## Why Mingot?
//!
//! Most web UI libraries stop at JavaScript's Number type (safe integers up to 2^53 - 1).
//! Mingot goes beyond with:
//!
//! - **NumberInput** supporting u64, u128, i64, i128, and arbitrary precision
//! - **Zero precision loss** throughout the component lifecycle
//! - **Type-safe validation** with detailed error types
//! - **Amari integration** for advanced mathematical computing (optional)
//!
//! ## Precision Without Compromise
//!
//! ```rust,no_run
//! use mingot::prelude::*;
//!
//! // High-precision number input
//! <NumberInput
//!     precision=NumberInputPrecision::U64
//!     label="Large Integer"
//!     on_valid_change=Callback::new(move |result| {
//!         // Supports values up to 18,446,744,073,709,551,615
//!     })
//! />
//! ```
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use leptos::prelude::*;
//! use mingot::prelude::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <MingotProvider>
//!             <Container>
//!                 <Stack spacing="md">
//!                     <Text size=TextSize::Xl weight=TextWeight::Bold>
//!                         "Welcome to Mingot"
//!                     </Text>
//!                     <Button variant=ButtonVariant::Filled on_click=Callback::new(move |_| {})>
//!                         "Click me"
//!                     </Button>
//!                 </Stack>
//!             </Container>
//!         </MingotProvider>
//!     }
//! }
//! ```

pub mod components;
pub mod theme;
pub mod utils;
pub mod validation;

// Re-export commonly used components
pub use components::*;
pub use theme::{
    use_color_scheme, use_color_scheme_toggle, use_set_color_scheme, use_theme, ActiveColorScheme,
    ColorSchemeMode, MingotProvider, Theme, ThemeContext,
};
pub use validation::{ValidationError, ValidationResult, Validator};

// Re-export leptos for convenience
pub use leptos;

/// Prelude module with commonly used imports
pub mod prelude {
    pub use crate::components::*;
    pub use crate::theme::{
        use_color_scheme, use_color_scheme_toggle, use_set_color_scheme, ActiveColorScheme,
        ColorSchemeMode, MingotProvider, Theme,
    };
    pub use crate::validation::{self, ValidationError, ValidationResult, Validator};
}
