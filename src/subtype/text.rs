//! Standard text subtypes.
//!
//! Text subtypes provide semantic meaning for string values:
//!
//! ## Basic
//! - [`Plain`] - Plain text
//! - [`MultiLine`] - Multi-line text
//!
//! ## Network
//! - [`Email`] - Email address
//! - [`Url`] - URL
//! - [`Domain`] - Domain name
//! - [`IpAddressV4`] - IPv4 address
//! - [`IpAddressV6`] - IPv6 address
//! - [`Hostname`] - Hostname
//! - [`MacAddress`] - MAC address
//!
//! ## Paths
//! - [`FilePath`] - File path
//! - [`DirPath`] - Directory path
//! - [`FileName`] - File name
//!
//! ## Security
//! - [`Secret`] - Generic secret
//! - [`Password`] - Password
//! - [`ApiKey`] - API key
//! - [`BearerToken`] - Bearer token
//!
//! ## Identifiers
//! - [`Uuid`] - UUID
//! - [`Slug`] - URL slug
//!
//! ## Date/Time
//! - [`DateTime`] - ISO 8601 datetime
//! - [`Date`] - ISO 8601 date
//! - [`Time`] - ISO 8601 time
//! - [`Iso8601Duration`] - ISO 8601 duration (PT1H30M)
//! - [`Cron`] - Cron expression
//! - [`Timezone`] - Timezone identifier
//!
//! ## Structured Data
//! - [`Json`] - JSON
//! - [`Yaml`] - YAML
//! - [`Toml`] - TOML
//! - [`Xml`] - XML
//! - [`Markdown`] - Markdown text
//! - [`Html`] - HTML markup
//!
//! ## Code
//! - [`Sql`] - SQL query
//! - [`Regex`] - Regular expression
//! - [`Expression`] - Expression/formula
//! - [`JavaScript`] - JavaScript code
//! - [`Python`] - Python code
//! - [`Rust`] - Rust code
//!
//! ## Contact
//! - [`PhoneNumber`] - Phone number (E.164)
//!
//! ## Color
//! - [`HexColor`] - Hex color (#RRGGBB)
//!
//! ## Localization
//! - [`Country`] - ISO 3166 country code
//! - [`Language`] - ISO 639 language code
//! - [`CurrencyCode`] - ISO 4217 currency code
//!
//! ## Versioning
//! - [`Semver`] - Semantic version

use crate::define_text_subtype;

// === Basic ===

define_text_subtype!(Plain, "plain");
define_text_subtype!(MultiLine, "multiline", multiline: true);

// === Network ===

define_text_subtype!(Email, "email", pattern: r"^[^@\s]+@[^@\s]+\.[^@\s]+$", placeholder: "user@example.com");
define_text_subtype!(Url, "url", pattern: r"^https?://", placeholder: "https://example.com");
define_text_subtype!(Domain, "domain", placeholder: "example.com");
define_text_subtype!(IpAddressV4, "ip_v4", pattern: r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$", placeholder: "192.168.1.1");
define_text_subtype!(IpAddressV6, "ip_v6", placeholder: "::1");
define_text_subtype!(Hostname, "hostname", placeholder: "localhost");
define_text_subtype!(MacAddress, "mac_address", pattern: r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$", placeholder: "00:1A:2B:3C:4D:5E");

// === Paths ===

define_text_subtype!(FilePath, "file_path", placeholder: "/path/to/file");
define_text_subtype!(DirPath, "dir_path", placeholder: "/path/to/directory");
define_text_subtype!(FileName, "file_name", placeholder: "filename.ext");

// === Security ===

define_text_subtype!(Secret, "secret", sensitive: true);
define_text_subtype!(Password, "password", sensitive: true);
define_text_subtype!(ApiKey, "api_key", sensitive: true);
define_text_subtype!(BearerToken, "bearer_token", sensitive: true);

// === Identifiers ===

define_text_subtype!(Uuid, "uuid", pattern: r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$", placeholder: "00000000-0000-0000-0000-000000000000");
define_text_subtype!(Slug, "slug", pattern: r"^[a-z0-9]+(?:-[a-z0-9]+)*$", placeholder: "my-slug");

// === Date/Time ===

define_text_subtype!(DateTime, "datetime", placeholder: "2024-01-01T00:00:00Z");
define_text_subtype!(Date, "date", pattern: r"^\d{4}-\d{2}-\d{2}$", placeholder: "2024-01-01");
define_text_subtype!(Time, "time", pattern: r"^\d{2}:\d{2}(:\d{2})?$", placeholder: "12:00:00");
define_text_subtype!(Iso8601Duration, "iso8601_duration", pattern: r"^P", placeholder: "PT1H30M");
define_text_subtype!(Cron, "cron", placeholder: "*/5 * * * *");
define_text_subtype!(Timezone, "timezone", placeholder: "America/New_York");

// === Structured Data ===

define_text_subtype!(Json, "json", multiline: true);
define_text_subtype!(Yaml, "yaml", multiline: true);
define_text_subtype!(Toml, "toml", multiline: true);
define_text_subtype!(Xml, "xml", multiline: true);
define_text_subtype!(Markdown, "markdown", multiline: true);
define_text_subtype!(Html, "html", multiline: true);

// === Code ===

define_text_subtype!(Sql, "sql", code: "sql");
define_text_subtype!(Regex, "regex", placeholder: "^pattern$");
define_text_subtype!(Expression, "expression", placeholder: "{{ value }}");
define_text_subtype!(JavaScript, "javascript", code: "javascript");
define_text_subtype!(Python, "python", code: "python");
define_text_subtype!(Rust, "rust", code: "rust");

// === Contact ===

define_text_subtype!(PhoneNumber, "phone_number", placeholder: "+1 555 123 4567");

// === Color ===

define_text_subtype!(HexColor, "hex_color", pattern: r"^#([0-9A-Fa-f]{3}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$", placeholder: "#FF5733");

// === Localization ===

define_text_subtype!(Country, "country", pattern: r"^[A-Z]{2}$", placeholder: "US");
define_text_subtype!(Language, "language", pattern: r"^[a-z]{2}(-[A-Z]{2})?$", placeholder: "en");
define_text_subtype!(CurrencyCode, "currency_code", pattern: r"^[A-Z]{3}$", placeholder: "USD");

// === Versioning ===

define_text_subtype!(Semver, "semver", pattern: r"^\d+\.\d+\.\d+", placeholder: "1.0.0");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subtype::TextSubtype;

    // === Basic Tests ===

    #[test]
    fn test_plain() {
        assert_eq!(Plain::name(), "plain");
        assert!(!Plain::is_multiline());
        assert!(!Plain::is_sensitive());
    }

    #[test]
    fn test_multiline() {
        assert_eq!(MultiLine::name(), "multiline");
        assert!(MultiLine::is_multiline());
    }

    // === Network Tests ===

    #[test]
    fn test_email() {
        assert_eq!(Email::name(), "email");
        assert!(Email::pattern().is_some());
        assert_eq!(Email::placeholder(), Some("user@example.com"));
    }

    #[test]
    fn test_url() {
        assert_eq!(Url::name(), "url");
        assert!(Url::pattern().is_some());
        assert_eq!(Url::placeholder(), Some("https://example.com"));
    }

    #[test]
    fn test_domain() {
        assert_eq!(Domain::name(), "domain");
        assert_eq!(Domain::placeholder(), Some("example.com"));
    }

    #[test]
    fn test_ip_v4() {
        assert_eq!(IpAddressV4::name(), "ip_v4");
        assert!(IpAddressV4::pattern().is_some());
    }

    #[test]
    fn test_ip_v6() {
        assert_eq!(IpAddressV6::name(), "ip_v6");
    }

    #[test]
    fn test_hostname() {
        assert_eq!(Hostname::name(), "hostname");
        assert_eq!(Hostname::placeholder(), Some("localhost"));
    }

    // === Path Tests ===

    #[test]
    fn test_file_path() {
        assert_eq!(FilePath::name(), "file_path");
    }

    #[test]
    fn test_dir_path() {
        assert_eq!(DirPath::name(), "dir_path");
    }

    #[test]
    fn test_file_name() {
        assert_eq!(FileName::name(), "file_name");
    }

    // === Security Tests ===

    #[test]
    fn test_secret() {
        assert_eq!(Secret::name(), "secret");
        assert!(Secret::is_sensitive());
    }

    #[test]
    fn test_password() {
        assert_eq!(Password::name(), "password");
        assert!(Password::is_sensitive());
    }

    #[test]
    fn test_api_key() {
        assert_eq!(ApiKey::name(), "api_key");
        assert!(ApiKey::is_sensitive());
    }

    #[test]
    fn test_bearer_token() {
        assert_eq!(BearerToken::name(), "bearer_token");
        assert!(BearerToken::is_sensitive());
    }

    // === Identifier Tests ===

    #[test]
    fn test_uuid() {
        assert_eq!(Uuid::name(), "uuid");
        assert!(Uuid::pattern().is_some());
    }

    #[test]
    fn test_slug() {
        assert_eq!(Slug::name(), "slug");
        assert!(Slug::pattern().is_some());
    }

    // === Date/Time Tests ===

    #[test]
    fn test_datetime() {
        assert_eq!(DateTime::name(), "datetime");
    }

    #[test]
    fn test_date() {
        assert_eq!(Date::name(), "date");
        assert!(Date::pattern().is_some());
    }

    #[test]
    fn test_time() {
        assert_eq!(Time::name(), "time");
        assert!(Time::pattern().is_some());
    }

    // === Structured Data Tests ===

    #[test]
    fn test_json() {
        assert_eq!(Json::name(), "json");
        assert!(Json::is_multiline());
    }

    #[test]
    fn test_yaml() {
        assert_eq!(Yaml::name(), "yaml");
        assert!(Yaml::is_multiline());
    }

    #[test]
    fn test_toml() {
        assert_eq!(Toml::name(), "toml");
        assert!(Toml::is_multiline());
    }

    #[test]
    fn test_xml() {
        assert_eq!(Xml::name(), "xml");
        assert!(Xml::is_multiline());
    }

    // === Code Tests ===

    #[test]
    fn test_sql() {
        assert_eq!(Sql::name(), "sql");
        assert!(Sql::is_multiline());
        assert_eq!(Sql::code_language(), Some("sql"));
    }

    #[test]
    fn test_regex() {
        assert_eq!(Regex::name(), "regex");
        assert_eq!(Regex::placeholder(), Some("^pattern$"));
    }

    #[test]
    fn test_expression() {
        assert_eq!(Expression::name(), "expression");
    }

    #[test]
    fn test_javascript() {
        assert_eq!(JavaScript::name(), "javascript");
        assert_eq!(JavaScript::code_language(), Some("javascript"));
    }

    #[test]
    fn test_python() {
        assert_eq!(Python::name(), "python");
        assert_eq!(Python::code_language(), Some("python"));
    }

    #[test]
    fn test_rust() {
        assert_eq!(Rust::name(), "rust");
        assert_eq!(Rust::code_language(), Some("rust"));
    }

    // === Network (new) Tests ===

    #[test]
    fn test_mac_address() {
        assert_eq!(MacAddress::name(), "mac_address");
        assert!(MacAddress::pattern().is_some());
        assert_eq!(MacAddress::placeholder(), Some("00:1A:2B:3C:4D:5E"));
    }

    // === Date/Time (new) Tests ===

    #[test]
    fn test_iso8601_duration() {
        assert_eq!(Iso8601Duration::name(), "iso8601_duration");
        assert!(Iso8601Duration::pattern().is_some());
        assert_eq!(Iso8601Duration::placeholder(), Some("PT1H30M"));
    }

    #[test]
    fn test_cron() {
        assert_eq!(Cron::name(), "cron");
        assert_eq!(Cron::placeholder(), Some("*/5 * * * *"));
    }

    #[test]
    fn test_timezone() {
        assert_eq!(Timezone::name(), "timezone");
        assert_eq!(Timezone::placeholder(), Some("America/New_York"));
    }

    // === Structured Data (new) Tests ===

    #[test]
    fn test_markdown() {
        assert_eq!(Markdown::name(), "markdown");
        assert!(Markdown::is_multiline());
    }

    #[test]
    fn test_html() {
        assert_eq!(Html::name(), "html");
        assert!(Html::is_multiline());
    }

    // === Contact Tests ===

    #[test]
    fn test_phone_number() {
        assert_eq!(PhoneNumber::name(), "phone_number");
        assert_eq!(PhoneNumber::placeholder(), Some("+1 555 123 4567"));
    }

    // === Color Tests ===

    #[test]
    fn test_hex_color() {
        assert_eq!(HexColor::name(), "hex_color");
        assert!(HexColor::pattern().is_some());
        assert_eq!(HexColor::placeholder(), Some("#FF5733"));
    }

    // === Localization Tests ===

    #[test]
    fn test_country() {
        assert_eq!(Country::name(), "country");
        assert!(Country::pattern().is_some());
        assert_eq!(Country::placeholder(), Some("US"));
    }

    #[test]
    fn test_language() {
        assert_eq!(Language::name(), "language");
        assert!(Language::pattern().is_some());
        assert_eq!(Language::placeholder(), Some("en"));
    }

    #[test]
    fn test_currency_code() {
        assert_eq!(CurrencyCode::name(), "currency_code");
        assert!(CurrencyCode::pattern().is_some());
        assert_eq!(CurrencyCode::placeholder(), Some("USD"));
    }

    // === Versioning Tests ===

    #[test]
    fn test_semver() {
        assert_eq!(Semver::name(), "semver");
        assert!(Semver::pattern().is_some());
        assert_eq!(Semver::placeholder(), Some("1.0.0"));
    }
}
