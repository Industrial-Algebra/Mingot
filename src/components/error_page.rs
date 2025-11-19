use crate::theme::use_theme;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrorPageType {
    NotFound,           // 404
    InternalError,      // 500
    Forbidden,          // 403
    Unauthorized,       // 401
    ServiceUnavailable, // 503
    Custom,
}

impl ErrorPageType {
    fn status_code(&self) -> &str {
        match self {
            ErrorPageType::NotFound => "404",
            ErrorPageType::InternalError => "500",
            ErrorPageType::Forbidden => "403",
            ErrorPageType::Unauthorized => "401",
            ErrorPageType::ServiceUnavailable => "503",
            ErrorPageType::Custom => "",
        }
    }

    fn default_title(&self) -> &str {
        match self {
            ErrorPageType::NotFound => "Page Not Found",
            ErrorPageType::InternalError => "Internal Server Error",
            ErrorPageType::Forbidden => "Access Forbidden",
            ErrorPageType::Unauthorized => "Unauthorized",
            ErrorPageType::ServiceUnavailable => "Service Unavailable",
            ErrorPageType::Custom => "Error",
        }
    }

    fn default_description(&self) -> &str {
        match self {
            ErrorPageType::NotFound => {
                "The page you are looking for doesn't exist or has been moved."
            }
            ErrorPageType::InternalError => {
                "Something went wrong on our end. Please try again later."
            }
            ErrorPageType::Forbidden => "You don't have permission to access this resource.",
            ErrorPageType::Unauthorized => "Please log in to access this page.",
            ErrorPageType::ServiceUnavailable => {
                "The service is temporarily unavailable. Please try again later."
            }
            ErrorPageType::Custom => "An error occurred.",
        }
    }

    fn emoji(&self) -> &str {
        match self {
            ErrorPageType::NotFound => "🔍",
            ErrorPageType::InternalError => "⚠️",
            ErrorPageType::Forbidden => "🚫",
            ErrorPageType::Unauthorized => "🔒",
            ErrorPageType::ServiceUnavailable => "🔧",
            ErrorPageType::Custom => "❌",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_defaults() {
        let error_type = ErrorPageType::NotFound;
        assert_eq!(error_type.status_code(), "404");
        assert_eq!(error_type.default_title(), "Page Not Found");
        assert_eq!(
            error_type.default_description(),
            "The page you are looking for doesn't exist or has been moved."
        );
        assert_eq!(error_type.emoji(), "🔍");
    }

    #[test]
    fn test_internal_error_defaults() {
        let error_type = ErrorPageType::InternalError;
        assert_eq!(error_type.status_code(), "500");
        assert_eq!(error_type.default_title(), "Internal Server Error");
        assert_eq!(
            error_type.default_description(),
            "Something went wrong on our end. Please try again later."
        );
        assert_eq!(error_type.emoji(), "⚠️");
    }

    #[test]
    fn test_forbidden_defaults() {
        let error_type = ErrorPageType::Forbidden;
        assert_eq!(error_type.status_code(), "403");
        assert_eq!(error_type.default_title(), "Access Forbidden");
        assert_eq!(
            error_type.default_description(),
            "You don't have permission to access this resource."
        );
        assert_eq!(error_type.emoji(), "🚫");
    }

    #[test]
    fn test_unauthorized_defaults() {
        let error_type = ErrorPageType::Unauthorized;
        assert_eq!(error_type.status_code(), "401");
        assert_eq!(error_type.default_title(), "Unauthorized");
        assert_eq!(
            error_type.default_description(),
            "Please log in to access this page."
        );
        assert_eq!(error_type.emoji(), "🔒");
    }

    #[test]
    fn test_service_unavailable_defaults() {
        let error_type = ErrorPageType::ServiceUnavailable;
        assert_eq!(error_type.status_code(), "503");
        assert_eq!(error_type.default_title(), "Service Unavailable");
        assert_eq!(
            error_type.default_description(),
            "The service is temporarily unavailable. Please try again later."
        );
        assert_eq!(error_type.emoji(), "🔧");
    }

    #[test]
    fn test_custom_error_defaults() {
        let error_type = ErrorPageType::Custom;
        assert_eq!(error_type.status_code(), "");
        assert_eq!(error_type.default_title(), "Error");
        assert_eq!(error_type.default_description(), "An error occurred.");
        assert_eq!(error_type.emoji(), "❌");
    }

    #[test]
    fn test_all_error_types_have_unique_codes() {
        let codes = vec![
            ErrorPageType::NotFound.status_code(),
            ErrorPageType::InternalError.status_code(),
            ErrorPageType::Forbidden.status_code(),
            ErrorPageType::Unauthorized.status_code(),
            ErrorPageType::ServiceUnavailable.status_code(),
        ];

        // Check that all non-empty codes are unique
        let mut seen = std::collections::HashSet::new();
        for code in codes {
            if !code.is_empty() {
                assert!(seen.insert(code), "Duplicate status code: {}", code);
            }
        }
    }
}

#[component]
pub fn ErrorPage(
    #[prop(optional)] error_type: Option<ErrorPageType>,
    #[prop(optional, into)] status_code: Option<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional)] actions: Option<Children>,
    #[prop(optional)] show_status_code: bool,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let error_type = error_type.unwrap_or(ErrorPageType::NotFound);

    let status_code = status_code.unwrap_or_else(|| error_type.status_code().to_string());
    let title = title.unwrap_or_else(|| error_type.default_title().to_string());
    let description = description.unwrap_or_else(|| error_type.default_description().to_string());
    let icon = icon.unwrap_or_else(|| error_type.emoji().to_string());

    let container_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: flex; \
             flex-direction: column; \
             align-items: center; \
             justify-content: center; \
             min-height: 100vh; \
             padding: {} {}; \
             background-color: {}; \
             text-align: center;",
            theme_val.spacing.xl, theme_val.spacing.md, scheme_colors.background
        )
    };

    let status_code_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "font-size: 120px; \
             font-weight: {}; \
             line-height: 1; \
             color: {}; \
             margin: 0; \
             opacity: 0.1;",
            theme_val.typography.font_weights.bold, scheme_colors.text
        )
    };

    let icon_styles = move || {
        "font-size: 80px; \
         margin-bottom: 1rem;"
            .to_string()
    };

    let title_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "font-size: 32px; \
             font-weight: {}; \
             color: {}; \
             margin: 0 0 1rem 0;",
            theme_val.typography.font_weights.bold, scheme_colors.text
        )
    };

    let description_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let text_secondary = scheme_colors
            .get_color("gray", 6)
            .unwrap_or_else(|| "#868e96".to_string());
        format!(
            "font-size: {}; \
             color: {}; \
             max-width: 500px; \
             margin: 0 0 2rem 0; \
             line-height: 1.6;",
            theme_val.typography.font_sizes.md, text_secondary
        )
    };

    let actions_styles = move || {
        "display: flex; \
         gap: 1rem; \
         flex-wrap: wrap; \
         justify-content: center;"
            .to_string()
    };

    let class_str = format!("mingot-error-page {}", class.unwrap_or_default());

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

            {if show_status_code && !status_code.is_empty() {
                view! { <div style=status_code_styles>{status_code.clone()}</div> }.into_any()
            } else {
                ().into_any()
            }}

            <div style=icon_styles>{icon}</div>

            <h1 style=title_styles>{title}</h1>

            <p style=description_styles>{description}</p>

            {actions.map(|a| view! { <div style=actions_styles>{a()}</div> })}
        </div>
    }
}
