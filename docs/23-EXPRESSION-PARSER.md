# Expression Parser Design

**String-based query language for expressions**

Status: 📋 Design Document (Not Implemented)  
Complexity: Medium (2-3 days)  
Version: 0.3.0 (Future)

---

## Overview

A string parser for expressions would allow users to define validation and visibility rules in configuration files, JSON, or UI without writing Rust code.

```rust
// Instead of
let rule = Rule::local(Expr::and(vec![
    Expr::email(),
    Expr::min_length(5),
]));

// Users could write
let rule = Rule::parse("email AND length >= 5")?;
```

---

## Complexity Analysis

### Easy Parts (1 day)

**1. Simple Expressions**
```
age >= 18
name == "admin"
enabled == true
email MATCHES ".*@.*"
length BETWEEN 5 AND 20
```

Lexer: Simple tokenizer (whitespace-separated)
Parser: Recursive descent for basic comparisons
Effort: ~4-6 hours

**2. Basic Logical Operators**
```
age >= 18 AND premium == true
role == "admin" OR role == "moderator"
NOT archived
```

Parser: Operator precedence (NOT > AND > OR)
Effort: ~2-4 hours

### Medium Parts (1 day)

**3. Function-like Syntax**
```
email()                    // Expr::Email
length(name) >= 5          // For field access
startsWith(name, "admin")
between(age, 18, 65)
```

Parser: Function call syntax with arguments
Effort: ~4-6 hours

**4. Field References**
```
password == confirm_password    // Cross-field
age > parent.age               // Nested access?
```

Parser: Distinguish field refs from values
Context: Need schema to resolve field types
Effort: ~4-6 hours

### Hard Parts (0.5-1 day)

**5. Parentheses & Precedence**
```
(age >= 18 OR guardian) AND premium
age >= 18 AND (premium OR trial)
```

Parser: Proper precedence handling, recursive descent
Effort: ~2-4 hours

**6. Type Coercion**
```
age >= "18"        // String "18" -> f64
premium == "true"  // String -> bool
```

Parser: Type inference from context
Schema integration needed
Effort: ~2-3 hours

**7. Array Operations**
```
tags CONTAINS "vip"
roles IN ["admin", "moderator"]
items.length >= 3
```

Parser: Array/collection syntax
Effort: ~2-3 hours

---

## Proposed Syntax

### Grammar (EBNF-like)

```ebnf
rule       := expr

expr       := or_expr

or_expr    := and_expr ( "OR" and_expr )*

and_expr   := not_expr ( "AND" not_expr )*

not_expr   := "NOT" primary_expr
            | primary_expr

primary_expr := comparison
              | function_call
              | "(" expr ")"

comparison := field compare_op value
            | field

compare_op := "==" | "!=" | ">=" | "<=" | ">" | "<"
            | "MATCHES" | "CONTAINS" | "STARTS_WITH" | "ENDS_WITH"
            | "BETWEEN"

function_call := identifier "(" args? ")"

args       := expr ( "," expr )*

field      := identifier ( "." identifier )*

value      := string | number | boolean | array

string     := '"' [^"]* '"'
number     := [0-9]+ ( "." [0-9]+ )?
boolean    := "true" | "false"
array      := "[" ( value ( "," value )* )? "]"
```

### Examples

```
// Comparisons
age >= 18
name == "admin"
score > 100

// Functions
email()
uuid()
between(age, 18, 65)

// Logical
age >= 18 AND premium == true
role == "admin" OR role == "moderator"
NOT archived

// Complex
(age >= 18 OR guardian) AND (premium OR trial)

// Field operations
password == confirm_password
length(name) >= 5
tags CONTAINS "vip"
```

---

## Implementation Plan

### Phase 1: Lexer (2-3 hours)

```rust
#[derive(Debug, PartialEq)]
enum Token {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    
    // Identifiers
    Ident(String),
    
    // Operators
    And,           // AND
    Or,            // OR
    Not,           // NOT
    Eq,            // ==
    Ne,            // !=
    Lt,            // <
    Gt,            // >
    Lte,           // <=
    Gte,           // >=
    
    // Special operators
    Matches,       // MATCHES
    Contains,      // CONTAINS
    StartsWith,    // STARTS_WITH
    EndsWith,      // ENDS_WITH
    Between,       // BETWEEN
    
    // Punctuation
    LParen,        // (
    RParen,        // )
    LBracket,      // [
    RBracket,      // ]
    Comma,         // ,
    Dot,           // .
    
    // End
    Eof,
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();
        
        match self.peek() {
            '"' => self.read_string(),
            '0'..='9' => self.read_number(),
            'a'..='z' | 'A'..='Z' => self.read_keyword_or_ident(),
            '(' => { self.advance(); Ok(Token::LParen) }
            ')' => { self.advance(); Ok(Token::RParen) }
            // ... etc
        }
    }
}
```

**Effort:** 2-3 hours

### Phase 2: Parser (4-6 hours)

```rust
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn parse(input: &str) -> Result<Expr, ParseError> {
        let mut parser = Parser::new(input);
        parser.parse_expr()
    }
    
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        
        while self.current == Token::Or {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::or(vec![left, right]);
        }
        
        Ok(left)
    }
    
    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_not()?;
        
        while self.current == Token::And {
            self.advance();
            let right = self.parse_not()?;
            left = Expr::and(vec![left, right]);
        }
        
        Ok(left)
    }
    
    fn parse_not(&mut self) -> Result<Expr, ParseError> {
        if self.current == Token::Not {
            self.advance();
            let expr = self.parse_primary()?;
            return Ok(Expr::not(expr));
        }
        
        self.parse_primary()
    }
    
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match &self.current {
            Token::LParen => {
                self.advance(); // consume '('
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                
                // Check if it's a function call
                if self.current == Token::LParen {
                    self.parse_function_call(name)
                } else {
                    self.parse_comparison(name)
                }
            }
            _ => Err(ParseError::UnexpectedToken(self.current.clone())),
        }
    }
    
    fn parse_comparison(&mut self, field: String) -> Result<Expr, ParseError> {
        match &self.current {
            Token::Eq => {
                self.advance();
                let value = self.parse_value()?;
                Ok(Expr::eq(value))
            }
            Token::Gte => {
                self.advance();
                let num = self.parse_number()?;
                Ok(Expr::gte(num))
            }
            // ... other operators
            _ => Err(ParseError::ExpectedComparison),
        }
    }
    
    fn parse_function_call(&mut self, name: String) -> Result<Expr, ParseError> {
        self.expect(Token::LParen)?;
        
        let expr = match name.as_str() {
            "email" => {
                self.expect(Token::RParen)?;
                Expr::email()
            }
            "uuid" => {
                self.expect(Token::RParen)?;
                Expr::uuid()
            }
            "between" => {
                let min = self.parse_number()?;
                self.expect(Token::Comma)?;
                let max = self.parse_number()?;
                self.expect(Token::RParen)?;
                Expr::between(min, max)
            }
            _ => return Err(ParseError::UnknownFunction(name)),
        };
        
        Ok(expr)
    }
}
```

**Effort:** 4-6 hours

### Phase 3: Rule Construction (2-3 hours)

```rust
impl Rule {
    /// Parse a rule from string.
    ///
    /// # Syntax
    ///
    /// - Local rules (validation): `email() AND length >= 5`
    /// - Field rules (visibility): `field: mode == "advanced"`
    ///
    /// # Examples
    ///
    /// ```
    /// // Validation
    /// let rule = Rule::parse("email() AND length >= 5")?;
    /// assert!(rule.is_local());
    ///
    /// // Visibility
    /// let rule = Rule::parse("mode == \"advanced\"")?;
    /// // Default to field reference for identifiers
    ///
    /// // Explicit field
    /// let rule = Rule::parse_field("mode", "== \"advanced\"")?;
    /// ```
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let expr = Parser::parse(input)?;
        Ok(Self::local(expr))
    }
    
    /// Parse a rule for a specific field (visibility).
    pub fn parse_field(field: impl Into<Key>, input: &str) -> Result<Self, ParseError> {
        let expr = Parser::parse(input)?;
        Ok(Self::field(field, expr))
    }
}
```

**Effort:** 2-3 hours

---

## Challenges & Solutions

### 1. Field vs Value Ambiguity

**Problem:**
```
name == "admin"     // "admin" is a value
name == admin       // Is 'admin' a field or value?
```

**Solution:**
- Require quotes for string values
- Bare identifiers are field references
- Or: provide schema context to resolve

### 2. Type Inference

**Problem:**
```
age >= "18"    // Need to convert "18" to number
```

**Solution:**
- Parser attempts smart coercion
- Or: require correct types (strict mode)
- Or: use schema to infer expected type

### 3. Cross-field References

**Problem:**
```
password == confirm_password    // Which is the target field?
```

**Solution:**
```
// Option A: Explicit syntax
FIELD(password) == FIELD(confirm_password)

// Option B: Infer from context
// When parsing for "password" field, RHS is reference
Rule::parse_for_field("password", "== confirm_password")

// Option C: Special prefix
$password == $confirm_password
```

### 4. Array Syntax

**Problem:**
```
roles IN ["admin", "moderator"]
tags CONTAINS "vip"
```

**Solution:**
- Add array literal support: `["a", "b"]`
- Special operators: `IN`, `CONTAINS`
- Map to `Expr::OneOf`, `Expr::Contains`

---

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_comparison() {
        let rule = Rule::parse("age >= 18").unwrap();
        assert!(matches!(rule.expr, Expr::Gte(18.0)));
    }
    
    #[test]
    fn test_logical_and() {
        let rule = Rule::parse("age >= 18 AND premium == true").unwrap();
        assert!(matches!(rule.expr, Expr::And(_)));
    }
    
    #[test]
    fn test_function_call() {
        let rule = Rule::parse("email()").unwrap();
        assert!(matches!(rule.expr, Expr::Email));
    }
    
    #[test]
    fn test_complex_expr() {
        let rule = Rule::parse("(age >= 18 OR guardian) AND premium").unwrap();
        // Verify AST structure
    }
    
    #[test]
    fn test_parse_error() {
        let err = Rule::parse("age >= ").unwrap_err();
        assert!(matches!(err, ParseError::UnexpectedEof));
    }
}
```

---

## Alternative: Existing Parser Crates

Instead of writing from scratch, consider existing solutions:

### Option 1: nom (Parser Combinator)

```rust
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    sequence::delimited,
    IResult,
};

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_function,
        parse_comparison,
        delimited(tag("("), parse_expr, tag(")")),
    ))(input)
}
```

**Pros:** Battle-tested, zero-copy, fast
**Cons:** Learning curve, complex error messages
**Effort:** 3-4 hours with nom experience

### Option 2: pest (PEG Parser)

```pest
// grammar.pest
rule = { expr }

expr = { or_expr }

or_expr = { and_expr ~ (OR ~ and_expr)* }

and_expr = { not_expr ~ (AND ~ not_expr)* }

comparison = { ident ~ compare_op ~ value }

compare_op = { "==" | "!=" | ">=" | "<=" | ">" | "<" }

ident = { ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }

value = { string | number | boolean }
```

**Pros:** Declarative grammar, good errors
**Cons:** Compile-time codegen, larger binary
**Effort:** 2-3 hours

### Option 3: Simple Recursive Descent (Recommended)

**Pros:** Full control, easy to debug, small footprint
**Cons:** More code to write
**Effort:** 6-8 hours total

---

## Recommendation

**For v0.3.0: Classic Lexer + Parser Architecture**

### Why This Approach?

1. **Separation of Concerns**
   - Lexer: String → Tokens (simple, stateless)
   - Parser: Tokens → AST (logic, precedence)
   - Clean error handling at each layer

2. **Industry Standard**
   - Used by all major languages (Rust, Python, JavaScript)
   - Well-documented patterns
   - Easy to test each layer independently

3. **Performance**
   - Lexer scans string once
   - Parser works with tokens (not strings)
   - Zero allocations in hot path (with proper design)

### Implementation Strategy

**Phase 1: Lexer (2-3 hours)**
```rust
struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

// Clean iterator pattern
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Peek next char, return appropriate token
    }
}
```

**Phase 2: Parser (4-6 hours)**
```rust
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

// Recursive descent with clear precedence
impl Parser {
    fn parse_or(&mut self) -> Result<Expr>;
    fn parse_and(&mut self) -> Result<Expr>;
    fn parse_not(&mut self) -> Result<Expr>;
    fn parse_primary(&mut self) -> Result<Expr>;
}
```

**Phase 3: Integration (2-3 hours)**
```rust
impl Rule {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let tokens = Lexer::new(input).collect::<Result<Vec<_>, _>>()?;
        let expr = Parser::new(tokens).parse()?;
        Ok(Self::local(expr))
    }
}
```

### Deferred Features

- Cross-field refs: v0.4.0
- Array operations: v0.4.0  
- Type inference: v0.4.0
- Schema validation: v0.4.0

**Total Effort:** 2-3 days (16-24 hours)

**Priority:** Medium (valuable for config files & UI builders)

---

## Example Usage

```rust
// In config file (TOML)
[parameters.age]
type = "number"
label = "Age"
validation = "between(18, 65)"

[parameters.email]
type = "text"
label = "Email"
validation = "email() AND length >= 5"

[parameters.advanced_option]
type = "text"
label = "Advanced Option"
visibility = "mode == \"advanced\""

// Load and parse
let schema = Schema::from_toml(config)?;
```

---

## Conclusion

**Complexity: Medium (2-3 days)**

The parser is **feasible and valuable**, especially for:
- Configuration files (TOML, JSON, YAML)
- UI builders (visual rule editors)
- Non-Rust API consumers

**Recommended Approach:**
1. Phase 1: Basic parser (comparisons, functions)
2. Phase 2: Logical operators (AND, OR, NOT)
3. Phase 3: Advanced features (cross-field, arrays)

**Dependencies:**
- None (pure Rust, no external crates needed)
- Optional: `nom` or `pest` for faster implementation

**Would be a great v0.3.0 feature!**
