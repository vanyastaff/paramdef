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
            Self::Email => {
                // HTML5 email regex (simplified)
                const EMAIL_REGEX: &str = r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$";

                value.as_text().is_some_and(|s| {
                    regex::Regex::new(EMAIL_REGEX)
                        .ok()
                        .is_some_and(|re| re.is_match(s))
                })
            }

            Self::Url => value.as_text().is_some_and(|s| {
                s.starts_with("http://") || s.starts_with("https://") || s.starts_with("ftp://")
            }),

            Self::Uuid => {
                const UUID_REGEX: &str = r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$";

                value.as_text().is_some_and(|s| {
                    regex::Regex::new(UUID_REGEX)
                        .ok()
                        .is_some_and(|re| re.is_match(s))
                })
            }

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
}
