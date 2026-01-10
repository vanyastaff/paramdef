//! Expression evaluation logic.

use super::Expr;
use crate::core::Value;

impl Expr {
    /// Evaluate this expression against a value.
    ///
    /// Returns `true` if the expression passes, `false` otherwise.
    ///
    /// # Note
    ///
    /// This is a basic evaluation that works for most cases. For validation
    /// with detailed error messages, use the validation system's `validate` method.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::Expr;
    /// use paramdef::core::Value;
    ///
    /// let expr = Expr::MinLength(5);
    /// assert!(expr.eval(&Value::text("hello")));
    /// assert!(!expr.eval(&Value::text("hi")));
    /// ```
    #[must_use]
    #[allow(clippy::too_many_lines, clippy::excessive_nesting)]
    pub fn eval(&self, value: &Value) -> bool {
        match self {
            // === Value Comparisons ===
            Self::Eq(expected) => value == expected,
            Self::Ne(expected) => value != expected,

            Self::Lt(threshold) => value.as_f64().is_some_and(|n| n < *threshold),
            Self::Gt(threshold) => value.as_f64().is_some_and(|n| n > *threshold),
            Self::Lte(threshold) => value.as_f64().is_some_and(|n| n <= *threshold),
            Self::Gte(threshold) => value.as_f64().is_some_and(|n| n >= *threshold),

            Self::Between(min, max) => value.as_f64().is_some_and(|n| n >= *min && n <= *max),

            // === String Operations ===
            Self::StartsWith(prefix) => value
                .as_text()
                .is_some_and(|s| s.starts_with(prefix.as_str())),

            Self::EndsWith(suffix) => value
                .as_text()
                .is_some_and(|s| s.ends_with(suffix.as_str())),

            Self::Contains(search) => match value {
                Value::Text(s) => search.as_text().is_some_and(|needle| s.contains(needle)),
                Value::Array(arr) => arr.iter().any(|item| item == search),
                _ => false,
            },

            #[cfg(feature = "validation")]
            Self::Matches(pattern) => {
                use std::cell::RefCell;
                use std::collections::HashMap;

                thread_local! {
                    static REGEX_CACHE: RefCell<HashMap<String, Option<regex::Regex>>> =
                        RefCell::new(HashMap::new());
                }

                value.as_text().is_some_and(|s| {
                    REGEX_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        let re = cache
                            .entry(pattern.clone())
                            .or_insert_with(|| regex::Regex::new(pattern).ok());
                        re.as_ref().is_some_and(|r| r.is_match(s))
                    })
                })
            }

            // === String Validation ===
            #[cfg(feature = "validation")]
            Self::Email => {
                use std::cell::RefCell;

                thread_local! {
                    static EMAIL_CACHE: RefCell<Option<regex::Regex>> = const { RefCell::new(None) };
                }

                const EMAIL_REGEX: &str = r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$";

                value.as_text().is_some_and(|s| {
                    EMAIL_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.is_none() {
                            *cache = regex::Regex::new(EMAIL_REGEX).ok();
                        }
                        cache.as_ref().is_some_and(|re| re.is_match(s))
                    })
                })
            }

            #[cfg(not(feature = "validation"))]
            Self::Email => value.as_text().is_some_and(|s| s.contains('@')),

            Self::Url => value.as_text().is_some_and(|s| {
                s.starts_with("http://") || s.starts_with("https://") || s.starts_with("ftp://")
            }),

            #[cfg(feature = "validation")]
            Self::Uuid => {
                use std::cell::RefCell;

                thread_local! {
                    static UUID_CACHE: RefCell<Option<regex::Regex>> = const { RefCell::new(None) };
                }

                const UUID_REGEX: &str = r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$";

                value.as_text().is_some_and(|s| {
                    UUID_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.is_none() {
                            *cache = regex::Regex::new(UUID_REGEX).ok();
                        }
                        cache.as_ref().is_some_and(|re| re.is_match(s))
                    })
                })
            }

            #[cfg(not(feature = "validation"))]
            Self::Uuid => value.as_text().is_some_and(|s| {
                s.len() == 36
                    && s.chars().enumerate().all(|(i, c)| match i {
                        8 | 13 | 18 | 23 => c == '-',
                        _ => c.is_ascii_hexdigit(),
                    })
            }),

            // === Length Checks ===
            Self::MinLength(min) => match value {
                Value::Text(s) => s.len() >= *min,
                Value::Array(arr) => arr.len() >= *min,
                _ => false,
            },

            Self::MaxLength(max) => match value {
                Value::Text(s) => s.len() <= *max,
                Value::Array(arr) => arr.len() <= *max,
                _ => false,
            },

            Self::Length(len) => match value {
                Value::Text(s) => s.len() == *len,
                Value::Array(arr) => arr.len() == *len,
                _ => false,
            },

            Self::LengthBetween(min, max) => {
                let len = match value {
                    Value::Text(s) => s.len(),
                    Value::Array(arr) => arr.len(),
                    _ => return false,
                };
                len >= *min && len <= *max
            }

            // === Numeric Constraints ===
            Self::Min(min) => value.as_f64().is_some_and(|n| n >= *min),
            Self::Max(max) => value.as_f64().is_some_and(|n| n <= *max),
            Self::ExclusiveMin(min) => value.as_f64().is_some_and(|n| n > *min),
            Self::ExclusiveMax(max) => value.as_f64().is_some_and(|n| n < *max),

            Self::MultipleOf(divisor) => {
                if *divisor == 0.0 {
                    return false;
                }
                value
                    .as_f64()
                    .is_some_and(|n| (n % divisor).abs() < f64::EPSILON)
            }

            Self::Positive => value.as_f64().is_some_and(|n| n > 0.0),
            Self::Negative => value.as_f64().is_some_and(|n| n < 0.0),
            Self::NonNegative => value.as_f64().is_some_and(|n| n >= 0.0),

            Self::Integer => value
                .as_f64()
                .is_some_and(|n| n.fract().abs() < f64::EPSILON),

            // === Collection Constraints ===
            Self::MinItems(min) => {
                if let Value::Array(arr) = value {
                    arr.len() >= *min
                } else {
                    false
                }
            }

            Self::MaxItems(max) => {
                if let Value::Array(arr) = value {
                    arr.len() <= *max
                } else {
                    false
                }
            }

            Self::ItemCount(count) => {
                if let Value::Array(arr) = value {
                    arr.len() == *count
                } else {
                    false
                }
            }

            Self::UniqueItems => {
                if let Value::Array(arr) = value {
                    // Check for duplicates using nested iteration
                    // (Value doesn't implement Hash, so we can't use HashSet)
                    for i in 0..arr.len() {
                        for j in (i + 1)..arr.len() {
                            if arr[i] == arr[j] {
                                return false;
                            }
                        }
                    }
                    true
                } else {
                    false
                }
            }

            // === State Checks ===
            Self::Required => !value.is_null() && !value.is_empty(),
            Self::IsSet => !value.is_null(),
            Self::IsEmpty => value.is_empty(),
            Self::IsNull => value.is_null(),
            Self::IsNotEmpty => !value.is_empty(),

            Self::IsTrue => value.as_bool().unwrap_or(false),
            Self::IsFalse => value.as_bool().is_some_and(|b| !b),

            Self::IsValid => true, // Always true for basic eval

            // === Set Operations ===
            Self::OneOf(values) => values.iter().any(|v| v == value),
            Self::Const(expected) => value == expected,

            // === Logical Operators ===
            Self::And(exprs) => exprs.iter().all(|e| e.eval(value)),
            Self::Or(exprs) => exprs.iter().any(|e| e.eval(value)),
            Self::Not(expr) => !expr.eval(value),

            Self::If {
                condition,
                then,
                otherwise,
            } => {
                if condition.eval(value) {
                    then.eval(value)
                } else if let Some(else_expr) = otherwise {
                    else_expr.eval(value)
                } else {
                    true // No else branch means pass if condition is false
                }
            }

            // === Cross-Field References ===
            Self::FieldRef(_) | Self::FieldEq { .. } => {
                // Field references require ValidationContext, which eval() doesn't have.
                // This is a limitation of the basic eval() method.
                // For cross-field validation, use eval_with_context() or the validation system.
                false
            }
        }
    }

    /// Evaluate this expression against a value with access to other field values.
    ///
    /// Returns `true` if the expression passes, `false` otherwise.
    ///
    /// This method is similar to `eval()` but supports cross-field validation
    /// through the `ValidationContext`.
    ///
    /// # Note
    ///
    /// For detailed error messages, use the validation system's `validate` method.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::Expr;
    /// use paramdef::core::Value;
    /// use paramdef::validation::ValidationContext;
    ///
    /// let expr = Expr::field_eq("password", "password_confirm");
    /// let value = Value::text("secret123");
    ///
    /// // ctx provides access to password_confirm field
    /// assert!(expr.eval_with_context(&value, &ctx));
    /// ```
    #[must_use]
    #[allow(clippy::too_many_lines)]
    #[cfg(feature = "validation")]
    pub fn eval_with_context(
        &self,
        value: &Value,
        ctx: &crate::validation::ValidationContext<'_>,
    ) -> bool {
        match self {
            // === Cross-Field References ===
            Self::FieldRef(field_key) => {
                // Get the referenced field's value
                ctx.get(field_key.as_str()).is_some()
            }

            Self::FieldEq { field, other } => {
                // Get both field values and compare
                let field_value = ctx.get(field.as_str());
                let other_value = ctx.get(other.as_str());

                match (field_value, other_value) {
                    (Some(a), Some(b)) => a == b,
                    _ => false, // If either field doesn't exist, comparison fails
                }
            }

            // === Logical Operators (need recursive context support) ===
            Self::And(exprs) => exprs.iter().all(|e| e.eval_with_context(value, ctx)),
            Self::Or(exprs) => exprs.iter().any(|e| e.eval_with_context(value, ctx)),
            Self::Not(expr) => !expr.eval_with_context(value, ctx),

            Self::If {
                condition,
                then,
                otherwise,
            } => {
                if condition.eval_with_context(value, ctx) {
                    then.eval_with_context(value, ctx)
                } else if let Some(else_expr) = otherwise {
                    else_expr.eval_with_context(value, ctx)
                } else {
                    true
                }
            }

            // === All other variants delegate to basic eval() ===
            _ => self.eval(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let expr = Expr::eq(Value::Int(42));
        assert!(expr.eval(&Value::Int(42)));
        assert!(!expr.eval(&Value::Int(43)));
    }

    #[test]
    fn test_ne() {
        let expr = Expr::ne(Value::Int(42));
        assert!(!expr.eval(&Value::Int(42)));
        assert!(expr.eval(&Value::Int(43)));
    }

    #[test]
    fn test_numeric_comparisons() {
        assert!(Expr::lt(10.0).eval(&Value::Int(5)));
        assert!(!Expr::lt(10.0).eval(&Value::Int(15)));

        assert!(Expr::gt(10.0).eval(&Value::Int(15)));
        assert!(!Expr::gt(10.0).eval(&Value::Int(5)));

        assert!(Expr::between(10.0, 20.0).eval(&Value::Int(15)));
        assert!(!Expr::between(10.0, 20.0).eval(&Value::Int(25)));
    }

    #[test]
    fn test_string_operations() {
        let text = Value::text("hello world");

        assert!(Expr::starts_with("hello").eval(&text));
        assert!(!Expr::starts_with("world").eval(&text));

        assert!(Expr::ends_with("world").eval(&text));
        assert!(!Expr::ends_with("hello").eval(&text));

        assert!(Expr::contains(Value::text("o w")).eval(&text));
        assert!(!Expr::contains(Value::text("xyz")).eval(&text));
    }

    #[test]
    fn test_length_checks() {
        let text = Value::text("hello");

        assert!(Expr::min_length(3).eval(&text));
        assert!(!Expr::min_length(10).eval(&text));

        assert!(Expr::max_length(10).eval(&text));
        assert!(!Expr::max_length(3).eval(&text));

        assert!(Expr::length(5).eval(&text));
        assert!(!Expr::length(3).eval(&text));

        assert!(Expr::length_between(3, 10).eval(&text));
        assert!(!Expr::length_between(6, 10).eval(&text));
    }

    #[test]
    fn test_numeric_constraints() {
        assert!(Expr::positive().eval(&Value::Int(5)));
        assert!(!Expr::positive().eval(&Value::Int(-5)));

        assert!(Expr::negative().eval(&Value::Int(-5)));
        assert!(!Expr::negative().eval(&Value::Int(5)));

        assert!(Expr::non_negative().eval(&Value::Int(0)));
        assert!(Expr::non_negative().eval(&Value::Int(5)));
        assert!(!Expr::non_negative().eval(&Value::Int(-5)));

        assert!(Expr::integer().eval(&Value::Int(5)));
        assert!(Expr::integer().eval(&Value::Float(5.0)));
        assert!(!Expr::integer().eval(&Value::Float(5.5)));

        assert!(Expr::multiple_of(5.0).eval(&Value::Int(10)));
        assert!(!Expr::multiple_of(5.0).eval(&Value::Int(7)));
    }

    #[test]
    fn test_state_checks() {
        assert!(Expr::required().eval(&Value::text("hello")));
        assert!(!Expr::required().eval(&Value::Null));
        assert!(!Expr::required().eval(&Value::text("")));

        assert!(Expr::is_set().eval(&Value::text("")));
        assert!(!Expr::is_set().eval(&Value::Null));

        assert!(Expr::is_empty().eval(&Value::text("")));
        assert!(!Expr::is_empty().eval(&Value::text("hello")));

        assert!(Expr::is_null().eval(&Value::Null));
        assert!(!Expr::is_null().eval(&Value::text("")));

        assert!(Expr::is_true().eval(&Value::Bool(true)));
        assert!(!Expr::is_true().eval(&Value::Bool(false)));

        assert!(Expr::is_false().eval(&Value::Bool(false)));
        assert!(!Expr::is_false().eval(&Value::Bool(true)));
    }

    #[test]
    fn test_logical_operators() {
        let value = Value::Int(15);

        let and_expr = Expr::and(vec![Expr::gt(10.0), Expr::lt(20.0)]);
        assert!(and_expr.eval(&value));

        let or_expr = Expr::or(vec![Expr::lt(10.0), Expr::gt(20.0)]);
        assert!(!or_expr.eval(&value));

        let not_expr = Expr::not(Expr::lt(10.0));
        assert!(not_expr.eval(&value));
    }

    #[test]
    fn test_if_expression() {
        let value = Value::Int(15);

        // If-then: if value > 10 then value < 20
        let expr = Expr::if_then(Expr::gt(10.0), Expr::lt(20.0));
        assert!(expr.eval(&value));

        // If-then-else: if value > 20 then value < 25 else value > 10
        let expr = Expr::if_then_else(Expr::gt(20.0), Expr::lt(25.0), Some(Expr::gt(10.0)));
        assert!(expr.eval(&value)); // Condition false, else branch passes
    }

    #[test]
    fn test_collection_constraints() {
        let arr = Value::array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);

        assert!(Expr::min_items(2).eval(&arr));
        assert!(!Expr::min_items(5).eval(&arr));

        assert!(Expr::max_items(5).eval(&arr));
        assert!(!Expr::max_items(2).eval(&arr));

        assert!(Expr::item_count(3).eval(&arr));
        assert!(!Expr::item_count(2).eval(&arr));

        let unique = Value::array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert!(Expr::unique_items().eval(&unique));

        let duplicate = Value::array(vec![Value::Int(1), Value::Int(2), Value::Int(1)]);
        assert!(!Expr::unique_items().eval(&duplicate));
    }

    #[cfg(feature = "validation")]
    #[test]
    fn test_cross_field_validation() {
        use crate::schema::Schema;
        use crate::types::leaf::Text;
        use crate::validation::ValidationContext;
        use std::collections::HashMap;
        use std::sync::Arc;

        // Create a simple schema
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("password").build())
                .parameter(Text::builder("password_confirm").build())
                .parameter(Text::builder("email").build())
                .build(),
        );

        // Create test values
        let mut values: HashMap<crate::core::Key, Value> = HashMap::new();
        values.insert("password".into(), Value::text("secret123"));
        values.insert("password_confirm".into(), Value::text("secret123"));
        values.insert("email".into(), Value::text("user@example.com"));

        let key = "password".into();
        let ctx = ValidationContext::new(&key, &schema, &values);

        // Test FieldRef - checks if field exists
        let field_ref = Expr::field_ref("password_confirm");
        assert!(field_ref.eval_with_context(&Value::text("secret123"), &ctx));

        let missing_field = Expr::field_ref("nonexistent");
        assert!(!missing_field.eval_with_context(&Value::text("secret123"), &ctx));

        // Test FieldEq - compares two fields
        let field_eq = Expr::field_eq("password", "password_confirm");
        assert!(field_eq.eval_with_context(&Value::text("secret123"), &ctx));

        // Test with mismatched fields
        let mut values_mismatch = values.clone();
        values_mismatch.insert("password_confirm".into(), Value::text("different"));
        let ctx_mismatch = ValidationContext::new(&key, &schema, &values_mismatch);

        assert!(!field_eq.eval_with_context(&Value::text("secret123"), &ctx_mismatch));

        // Test with missing field
        let mut values_missing = values.clone();
        values_missing.remove("password_confirm");
        let ctx_missing = ValidationContext::new(&key, &schema, &values_missing);

        assert!(!field_eq.eval_with_context(&Value::text("secret123"), &ctx_missing));
    }

    #[cfg(feature = "validation")]
    #[test]
    fn test_cross_field_with_logical_operators() {
        use crate::schema::Schema;
        use crate::types::leaf::Text;
        use crate::validation::ValidationContext;
        use std::collections::HashMap;
        use std::sync::Arc;

        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("password").build())
                .parameter(Text::builder("password_confirm").build())
                .build(),
        );

        let mut values: HashMap<crate::core::Key, Value> = HashMap::new();
        values.insert("password".into(), Value::text("secret123"));
        values.insert("password_confirm".into(), Value::text("secret123"));

        let key = "password".into();
        let ctx = ValidationContext::new(&key, &schema, &values);

        // Test And with field reference
        let expr = Expr::and(vec![
            Expr::min_length(8),
            Expr::field_eq("password", "password_confirm"),
        ]);
        assert!(expr.eval_with_context(&Value::text("secret123"), &ctx));

        // Test Or with field reference
        let expr = Expr::or(vec![
            Expr::min_length(100),               // Fails
            Expr::field_ref("password_confirm"), // Passes
        ]);
        assert!(expr.eval_with_context(&Value::text("secret123"), &ctx));

        // Test Not with field reference
        let expr = Expr::not(Expr::field_ref("nonexistent"));
        assert!(expr.eval_with_context(&Value::text("secret123"), &ctx));
    }

    #[test]
    fn test_field_ref_without_context_returns_false() {
        // Field references should return false when using basic eval()
        let field_ref = Expr::field_ref("some_field");
        assert!(!field_ref.eval(&Value::text("test")));

        let field_eq = Expr::field_eq("field1", "field2");
        assert!(!field_eq.eval(&Value::text("test")));
    }
}
