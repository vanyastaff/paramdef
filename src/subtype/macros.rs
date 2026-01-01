//! Macros for defining subtypes.

/// Defines a number subtype with type constraints.
///
/// # Variants
///
/// - `int_only`: Only works with integer types
/// - `float_only`: Only works with floating-point types
/// - `any`: Works with any numeric type
///
/// # Example
///
/// ```ignore
/// use paramdef::define_number_subtype;
///
/// // Integer-only subtype
/// define_number_subtype!(Port, int_only, i32, "port", range: (1, 65535));
///
/// // Float-only subtype
/// define_number_subtype!(Percentage, float_only, f64, "percentage", range: (0.0, 100.0));
///
/// // Universal subtype
/// define_number_subtype!(Distance, any, f64, "distance");
/// ```
#[macro_export]
macro_rules! define_number_subtype {
    // Integer-only with range
    ($name:ident, int_only, $value:ty, $str_name:literal, range: ($min:expr, $max:expr)) => {
        /// Number subtype (integer-only).
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::NumberSubtype for $name {
            type Value = $value;

            fn name() -> &'static str {
                $str_name
            }

            fn default_range() -> Option<(Self::Value, Self::Value)> {
                Some(($min, $max))
            }
        }
    };

    // Integer-only without range
    ($name:ident, int_only, $value:ty, $str_name:literal) => {
        /// Number subtype (integer-only).
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::NumberSubtype for $name {
            type Value = $value;

            fn name() -> &'static str {
                $str_name
            }
        }
    };

    // Float-only with range
    ($name:ident, float_only, $value:ty, $str_name:literal, range: ($min:expr, $max:expr)) => {
        /// Number subtype (float-only).
        #[derive(Debug, Clone, Copy, Default, PartialEq)]
        pub struct $name;

        impl $crate::subtype::NumberSubtype for $name {
            type Value = $value;

            fn name() -> &'static str {
                $str_name
            }

            fn default_range() -> Option<(Self::Value, Self::Value)> {
                Some(($min, $max))
            }
        }
    };

    // Float-only without range
    ($name:ident, float_only, $value:ty, $str_name:literal) => {
        /// Number subtype (float-only).
        #[derive(Debug, Clone, Copy, Default, PartialEq)]
        pub struct $name;

        impl $crate::subtype::NumberSubtype for $name {
            type Value = $value;

            fn name() -> &'static str {
                $str_name
            }
        }
    };

    // Universal with range
    ($name:ident, any, $value:ty, $str_name:literal, range: ($min:expr, $max:expr)) => {
        /// Number subtype (universal).
        #[derive(Debug, Clone, Copy, Default, PartialEq)]
        pub struct $name;

        impl $crate::subtype::NumberSubtype for $name {
            type Value = $value;

            fn name() -> &'static str {
                $str_name
            }

            fn default_range() -> Option<(Self::Value, Self::Value)> {
                Some(($min, $max))
            }
        }
    };

    // Universal without range
    ($name:ident, any, $value:ty, $str_name:literal) => {
        /// Number subtype (universal).
        #[derive(Debug, Clone, Copy, Default, PartialEq)]
        pub struct $name;

        impl $crate::subtype::NumberSubtype for $name {
            type Value = $value;

            fn name() -> &'static str {
                $str_name
            }
        }
    };
}

/// Defines a vector subtype with size constraint.
///
/// # Example
///
/// ```ignore
/// use paramdef::define_vector_subtype;
///
/// define_vector_subtype!(Position3D, 3, "position_3d", labels: ["X", "Y", "Z"]);
/// define_vector_subtype!(Quaternion, 4, "quaternion", labels: ["X", "Y", "Z", "W"], normalized: true);
/// ```
#[macro_export]
macro_rules! define_vector_subtype {
    // With labels
    ($name:ident, $size:expr, $str_name:literal, labels: [$($label:literal),+]) => {
        /// Vector subtype with size constraint.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::VectorSubtype<$size> for $name {
            fn name() -> &'static str {
                $str_name
            }

            fn component_labels() -> [&'static str; $size] {
                [$($label),+]
            }
        }
    };

    // With labels and normalized flag
    ($name:ident, $size:expr, $str_name:literal, labels: [$($label:literal),+], normalized: $normalized:expr) => {
        /// Vector subtype with size constraint.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::VectorSubtype<$size> for $name {
            fn name() -> &'static str {
                $str_name
            }

            fn component_labels() -> [&'static str; $size] {
                [$($label),+]
            }

            fn is_normalized() -> bool {
                $normalized
            }
        }
    };

    // With labels and range
    ($name:ident, $size:expr, $str_name:literal, labels: [$($label:literal),+], range: ($min:expr, $max:expr)) => {
        /// Vector subtype with size constraint.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::VectorSubtype<$size> for $name {
            fn name() -> &'static str {
                $str_name
            }

            fn component_labels() -> [&'static str; $size] {
                [$($label),+]
            }

            fn default_range() -> Option<(f64, f64)> {
                Some(($min, $max))
            }
        }
    };
}

/// Defines a text subtype with semantic meaning.
///
/// # Example
///
/// ```ignore
/// use paramdef::define_text_subtype;
///
/// define_text_subtype!(Email, "email", pattern: r"^[^@]+@[^@]+\.[^@]+$", placeholder: "user@example.com");
/// define_text_subtype!(Password, "password", sensitive: true);
/// define_text_subtype!(Json, "json", multiline: true);
/// ```
#[macro_export]
macro_rules! define_text_subtype {
    // Basic
    ($name:ident, $str_name:literal) => {
        /// Text subtype.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::TextSubtype for $name {
            fn name() -> &'static str {
                $str_name
            }
        }
    };

    // With pattern
    ($name:ident, $str_name:literal, pattern: $pattern:literal) => {
        /// Text subtype with pattern.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::TextSubtype for $name {
            fn name() -> &'static str {
                $str_name
            }

            fn pattern() -> Option<&'static str> {
                Some($pattern)
            }
        }
    };

    // With pattern and placeholder
    ($name:ident, $str_name:literal, pattern: $pattern:literal, placeholder: $placeholder:literal) => {
        /// Text subtype with pattern and placeholder.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::TextSubtype for $name {
            fn name() -> &'static str {
                $str_name
            }

            fn pattern() -> Option<&'static str> {
                Some($pattern)
            }

            fn placeholder() -> Option<&'static str> {
                Some($placeholder)
            }
        }
    };

    // Sensitive
    ($name:ident, $str_name:literal, sensitive: true) => {
        /// Text subtype (sensitive).
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::TextSubtype for $name {
            fn name() -> &'static str {
                $str_name
            }

            fn is_sensitive() -> bool {
                true
            }
        }
    };

    // Multiline
    ($name:ident, $str_name:literal, multiline: true) => {
        /// Text subtype (multiline).
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::TextSubtype for $name {
            fn name() -> &'static str {
                $str_name
            }

            fn is_multiline() -> bool {
                true
            }
        }
    };

    // Code with language
    ($name:ident, $str_name:literal, code: $lang:literal) => {
        /// Text subtype (code).
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::TextSubtype for $name {
            fn name() -> &'static str {
                $str_name
            }

            fn is_multiline() -> bool {
                true
            }

            fn code_language() -> Option<&'static str> {
                Some($lang)
            }
        }
    };

    // Placeholder only
    ($name:ident, $str_name:literal, placeholder: $placeholder:literal) => {
        /// Text subtype with placeholder.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::subtype::TextSubtype for $name {
            fn name() -> &'static str {
                $str_name
            }

            fn placeholder() -> Option<&'static str> {
                Some($placeholder)
            }
        }
    };
}

// Re-export macros at crate level
pub use define_number_subtype;
pub use define_text_subtype;
pub use define_vector_subtype;

#[cfg(test)]
mod tests {
    use crate::subtype::{NumberSubtype, TextSubtype, VectorSubtype};

    define_number_subtype!(TestPort, int_only, i32, "test_port", range: (1, 65535));
    define_number_subtype!(TestFactor, float_only, f64, "test_factor", range: (0.0, 1.0));
    define_number_subtype!(TestGeneric, any, f64, "test_generic");

    #[test]
    fn test_define_number_subtype_int_only() {
        assert_eq!(TestPort::name(), "test_port");
        assert_eq!(TestPort::default_range(), Some((1, 65535)));
    }

    #[test]
    fn test_define_number_subtype_float_only() {
        assert_eq!(TestFactor::name(), "test_factor");
        assert_eq!(TestFactor::default_range(), Some((0.0, 1.0)));
    }

    #[test]
    fn test_define_number_subtype_any() {
        assert_eq!(TestGeneric::name(), "test_generic");
        assert_eq!(TestGeneric::default_range(), None);
    }

    define_vector_subtype!(TestPos3D, 3, "test_pos3d", labels: ["X", "Y", "Z"]);
    define_vector_subtype!(TestNormal, 3, "test_normal", labels: ["X", "Y", "Z"], normalized: true);

    #[test]
    fn test_define_vector_subtype() {
        assert_eq!(TestPos3D::name(), "test_pos3d");
        assert_eq!(TestPos3D::SIZE, 3);
        assert_eq!(TestPos3D::component_labels(), ["X", "Y", "Z"]);
        assert!(!TestPos3D::is_normalized());
    }

    #[test]
    fn test_define_vector_subtype_normalized() {
        assert_eq!(TestNormal::name(), "test_normal");
        assert!(TestNormal::is_normalized());
    }

    define_text_subtype!(TestPlain, "test_plain");
    define_text_subtype!(TestEmail, "test_email", pattern: r"^[^@]+@[^@]+$", placeholder: "user@example.com");
    define_text_subtype!(TestSecret, "test_secret", sensitive: true);
    define_text_subtype!(TestMulti, "test_multi", multiline: true);
    define_text_subtype!(TestRust, "test_rust", code: "rust");

    #[test]
    fn test_define_text_subtype() {
        assert_eq!(TestPlain::name(), "test_plain");
        assert_eq!(TestPlain::pattern(), None);
        assert!(!TestPlain::is_sensitive());
    }

    #[test]
    fn test_define_text_subtype_with_pattern() {
        assert_eq!(TestEmail::name(), "test_email");
        assert_eq!(TestEmail::pattern(), Some(r"^[^@]+@[^@]+$"));
        assert_eq!(TestEmail::placeholder(), Some("user@example.com"));
    }

    #[test]
    fn test_define_text_subtype_sensitive() {
        assert!(TestSecret::is_sensitive());
    }

    #[test]
    fn test_define_text_subtype_multiline() {
        assert!(TestMulti::is_multiline());
    }

    #[test]
    fn test_define_text_subtype_code() {
        assert!(TestRust::is_multiline());
        assert_eq!(TestRust::code_language(), Some("rust"));
    }
}
