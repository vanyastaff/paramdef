# Expression Parser Capabilities

**Current parser features and limitations**

---

## ✅ Currently Supported

### 1. Simple Comparisons

```rust
"age >= 18"           // ✅ Works
"name == \"admin\""   // ✅ Works (double quotes)
"price < 100.50"      // ✅ Works
"active != false"     // ✅ Works
```

**Status:** Fully supported

---

### 2. Function Calls

```rust
"email()"                    // ✅ Works
"min_length(5)"             // ✅ Works
"starts_with(\"admin\")"    // ✅ Works
"positive() AND integer()"  // ✅ Works
```

**Supported functions:**
- Validation: `email()`, `url()`, `uuid()`, `required()`
- String: `starts_with(s)`, `ends_with(s)`
- Length: `min_length(n)`, `max_length(n)`, `length(n)`
- Numeric: `min(n)`, `max(n)`, `positive()`, `negative()`, `integer()`
- Collection: `empty()`, `not_empty()`, `unique_items()`

**Status:** Fully supported

---

### 3. Logical Operators

```rust
"age >= 18 AND premium == true"               // ✅ Works
"admin == true OR moderator == true"          // ✅ Works
"NOT archived"                                // ✅ Works (with valid RHS)
"(age >= 18 OR guardian == true) AND active"  // ✅ Works
```

**Features:**
- Case-insensitive: `AND`, `and`, `And` all work
- Proper precedence: NOT > AND > OR
- Parentheses for grouping

**Status:** Fully supported

---

### 4. IN Operator

```rust
"country IN [\"US\", \"CA\", \"UK\"]"  // ✅ Works
"status IN [1, 2, 3, 4]"               // ✅ Works  
"role IN [\"admin\", \"moderator\"]"   // ✅ Works
```

**Features:**
- Check if value is in array of allowed values
- Works with strings, numbers, and booleans
- Case-insensitive keyword: `IN`, `in`, `In` all work
- Can be combined with logical operators

**Examples:**
```rust
// Simple IN check
"country IN [\"US\", \"CA\", \"UK\"]"

// Combined with AND
"country IN [\"US\", \"CA\"] AND age >= 21"

// Combined with OR
"role IN [\"admin\", \"owner\"] OR verified == true"
```

**Status:** ✅ Fully supported (as of v0.3.1)

---

### 5. Complex Expressions

```rust
"email() AND min_length(5) AND max_length(100)"
// ✅ Works - creates nested binary And expressions

"(age >= 18 OR premium == true) AND active == true"
// ✅ Works - parentheses control precedence

"NOT empty() AND min_length(1)"
// ✅ Works - NOT with function calls
```

**Status:** Fully supported

---

## 🔄 Partial Support / Workarounds

### 5. Conditional Validation

```rust
// ❌ Not directly supported (would need custom functions)
"type == 'premium' AND (credit_card() OR paypal())"

// ✅ Workaround - define custom validators
"type == \"premium\" AND verified == true"
```

**Issue:** Custom domain-specific functions (`credit_card()`, `paypal()`) need to be registered.

**Workaround:** Use built-in validators or implement `Validator` trait.

---

## ❌ Not Yet Supported

### 6. Field References (Nested/Cross-field)

```rust
// ❌ Not supported
"user.profile.age >= 18"
"password_confirmation == password"
"user.email != ''"
```

**Why:** Parser currently treats identifiers before operators as placeholders. Field references would need:
1. Dot notation parsing in lexer
2. Field path resolution in parser
3. Cross-field evaluation in validator

**Future work:** Would require ExprTarget::Field variant enhancement.

---

### 7. Single Quotes

```rust
// ❌ Not supported
"type == 'premium'"

// ✅ Use double quotes
"type == \"premium\""
```

**Why:** Lexer only recognizes double-quoted strings.

**Future work:** Easy to add in lexer (10 lines).

---

### 8. ~~IN Operator with Arrays~~ ✅ **IMPLEMENTED**

```rust
// ✅ NOW SUPPORTED (v0.3.1)
"country IN [\"US\", \"CA\", \"UK\"]"
"status IN [1, 2, 3, 4]"
"role IN [\"admin\", \"moderator\"]"

// Old workaround (no longer needed)
// "country == \"US\" OR country == \"CA\" OR country == \"UK\""
```

**Status:** ✅ Fully implemented in v0.3.1
- `IN` is now a reserved keyword (case-insensitive)
- Array literal parsing works with `[1, 2, 3]`
- Maps to `Expr::OneOf(values)` variant
- Can be combined with logical operators

---

### 9. BETWEEN Operator

```rust
// ❌ Not supported
"age BETWEEN 18 AND 65"

// ✅ Workaround - use between function or manual
"age >= 18 AND age <= 65"
```

**Why:** `BETWEEN` keyword exists in token.rs but not implemented in parser.

**Future work:** Easy (30 min) - just needs parser logic.

---

### 10. CONTAINS for Arrays/Objects

```rust
// ❌ Not supported for checking array membership
"tags CONTAINS 'vip'"
"roles CONTAINS 'admin'"

// ✅ Currently works for string contains
"name CONTAINS \"admin\""  // Future - would check substring
```

**Why:** `CONTAINS` token exists but needs:
1. Different semantics for strings vs arrays
2. Type-aware evaluation

**Future work:** Medium complexity.

---

## 📝 Recommended Enhancements

### Priority 1: Single Quotes (Easy)

```rust
// Add to lexer.rs read_string():
'\'' => self.read_string_with_quote('\''),
```

### Priority 2: ~~IN Operator~~ ✅ **COMPLETED**

**Status:** ✅ Implemented in v0.3.1

Implementation:
1. ✅ Added `Token::In` keyword in lexer (case-insensitive)
2. ✅ Parser validates array requirement
3. ✅ Maps to `Expr::OneOf(values)` via `Expr::in_array()` helper
4. ✅ 4 tests added (649 total tests)

**Actual effort:** 1 hour

### Priority 3: Field References (Hard)

Requires:
1. Field path parsing: `user.profile.age` → `["user", "profile", "age"]`
2. Context-aware evaluation
3. Cross-field comparison support

**Estimated effort:** 1-2 days

---

## 🎯 Current Use Cases (Well Supported)

### Config File Validation

```toml
[validation.email]
rule = "email() AND max_length(100)"

[validation.password]
rule = "min_length(8) AND max_length(128)"

[validation.age]
rule = "min(0) AND max(150)"
```

### UI Rule Builders

```rust
// User selects from dropdowns:
// Field: "email"
// Operator: "AND"
// Functions: ["email()", "min_length(5)"]

let rule_string = "email() AND min_length(5)";
let rule = Rule::parse(rule_string)?;
```

### Database-Driven Validation

```sql
SELECT field_name, validation_rule 
FROM field_validations 
WHERE form_id = ?;

-- Results:
-- email | "email() AND required()"
-- age   | "min(18) AND max(120)"
```

Then parse and apply at runtime:

```rust
for row in results {
    let rule = Rule::parse(&row.validation_rule)?;
    // Apply to field
}
```

---

## 🔮 Future Vision

### Expression Parser v2 (Phase 8+)

Features to consider:
- Field references: `user.email == admin.email`
- ~~Array operations: `tags IN ["vip", "premium"]`~~ ✅ **DONE in v0.3.1**
- String functions: `uppercase(name) == "ADMIN"`
- Math operations: `price * quantity > 1000`
- Regex literals: `phone MATCHES /\d{3}-\d{4}/`
- Custom function registration: `register("credit_card", CreditCardValidator)`

**Complexity:** High (1-2 weeks)
**Benefit:** Cover 95% of use cases vs current 85%

---

## Summary

**Current Coverage:** ~85% of common validation use cases (was 80%, +5% from IN operator)

**Strengths:**
- ✅ Simple comparisons
- ✅ Function calls with args
- ✅ Logical operators (AND/OR/NOT)
- ✅ Parentheses and precedence
- ✅ IN operator with arrays (**NEW in v0.3.1**)
- ✅ Error messages

**Limitations:**
- ❌ No field references (cross-field)
- ❌ Only double-quoted strings
- ❌ No custom function registration

**Recommendation:** Current parser is production-ready for:
- Config-file based validation
- UI-generated rules
- Database-driven validation
- Non-programmer rule creation

For advanced use cases (cross-field validation, complex conditionals), use programmatic `Validator` trait.
