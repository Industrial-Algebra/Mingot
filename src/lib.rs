//! Mingot - A Leptos Component Library inspired by Mantine UI
//!
//! Mingot provides a comprehensive set of UI components for Leptos applications,
//! following the design principles and component API of Mantine UI.
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
//!                     <Button variant=ButtonVariant::Filled>
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
