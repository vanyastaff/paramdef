//! Matrix container for table-based data entry.
//!
//! Matrix represents a table where rows are questions/items and columns are
//! possible values. Each row produces a single value selected from columns.
//! Inspired by `SurveyJS` matrix question type.

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, FxHashSet, Key, Metadata, SmartStr};
use crate::types::kind::NodeKind;
use crate::types::traits::{Container, Node};

/// A row in a Matrix container.
///
/// Each row represents an item (e.g., a question) that needs a value
/// selected from the matrix columns.
#[derive(Debug, Clone)]
pub struct MatrixRow {
    /// Unique key for this row.
    pub key: Key,
    /// Display label for this row.
    pub label: SmartStr,
    /// Optional description or help text.
    pub description: Option<SmartStr>,
}

impl MatrixRow {
    /// Creates a new row with key and label.
    #[must_use]
    pub fn new(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            description: None,
        }
    }

    /// Creates a new row with description.
    #[must_use]
    pub fn with_description(
        key: impl Into<Key>,
        label: impl Into<SmartStr>,
        description: impl Into<SmartStr>,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            description: Some(description.into()),
        }
    }
}

/// A column in a Matrix container.
///
/// Each column represents a possible value that can be selected for any row.
#[derive(Debug, Clone)]
pub struct MatrixColumn {
    /// Value stored when this column is selected.
    pub value: SmartStr,
    /// Display label for this column.
    pub label: SmartStr,
    /// Optional weight or score for this column (useful for scoring).
    pub weight: Option<i32>,
    /// If true, selecting this column excludes all other columns in the row.
    ///
    /// Useful for "Not Applicable", "N/A", or "Don't Know" options.
    pub exclusive: bool,
}

impl MatrixColumn {
    /// Creates a new column with value and label.
    #[must_use]
    pub fn new(value: impl Into<SmartStr>, label: impl Into<SmartStr>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            weight: None,
            exclusive: false,
        }
    }

    /// Creates a new column with a weight for scoring.
    #[must_use]
    pub fn with_weight(
        value: impl Into<SmartStr>,
        label: impl Into<SmartStr>,
        weight: i32,
    ) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            weight: Some(weight),
            exclusive: false,
        }
    }

    /// Creates an exclusive column that deselects other columns when selected.
    ///
    /// Useful for "Not Applicable", "N/A", or "Don't Know" options.
    #[must_use]
    pub fn exclusive(value: impl Into<SmartStr>, label: impl Into<SmartStr>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            weight: None,
            exclusive: true,
        }
    }

    /// Creates an exclusive column with a weight.
    #[must_use]
    pub fn exclusive_with_weight(
        value: impl Into<SmartStr>,
        label: impl Into<SmartStr>,
        weight: i32,
    ) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            weight: Some(weight),
            exclusive: true,
        }
    }

    /// Returns true if this column is exclusive.
    #[must_use]
    pub fn is_exclusive(&self) -> bool {
        self.exclusive
    }

    /// Creates columns from simple string labels.
    ///
    /// The value and label will be the same for each column.
    #[must_use]
    pub fn from_labels<S: Into<SmartStr> + Clone>(labels: &[S]) -> Vec<Self> {
        labels
            .iter()
            .cloned()
            .map(|label| {
                let s = label.into();
                Self {
                    value: s.clone(),
                    label: s,
                    weight: None,
                    exclusive: false,
                }
            })
            .collect()
    }
}

/// Selection mode for matrix cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatrixCellType {
    /// Radio buttons - single selection per row (default).
    #[default]
    Radio,
    /// Checkboxes - multiple selections per row.
    Checkbox,
    /// Dropdown - single selection via dropdown menu.
    Dropdown,
    /// Text input - free text entry per cell.
    Text,
    /// Rating - star/number rating per row.
    Rating,
}

impl MatrixCellType {
    /// Returns the name of this cell type.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Radio => "radio",
            Self::Checkbox => "checkbox",
            Self::Dropdown => "dropdown",
            Self::Text => "text",
            Self::Rating => "rating",
        }
    }

    /// Returns true if this cell type allows multiple selections.
    #[must_use]
    pub fn is_multi_select(&self) -> bool {
        matches!(self, Self::Checkbox)
    }
}

/// A container for table-based data entry.
///
/// Matrix is a specialized container that displays data in a table format
/// where rows represent items/questions and columns represent possible values.
///
/// # Value Format
///
/// For single-select (Radio/Dropdown):
/// ```json
/// {
///   "row1_key": "column_value",
///   "row2_key": "column_value"
/// }
/// ```
///
/// For multi-select (Checkbox):
/// ```json
/// {
///   "row1_key": ["value1", "value2"],
///   "row2_key": ["value1"]
/// }
/// ```
///
/// # Example
///
/// ```ignore
/// use paramdef::container::Matrix;
///
/// // Satisfaction survey matrix
/// let satisfaction = Matrix::builder("satisfaction")
///     .label("Rate your satisfaction")
///     .rows([
///         ("price", "Price"),
///         ("quality", "Quality"),
///         ("support", "Customer Support"),
///     ])
///     .columns([
///         ("1", "Very Poor"),
///         ("2", "Poor"),
///         ("3", "Fair"),
///         ("4", "Good"),
///         ("5", "Excellent"),
///     ])
///     .required()
///     .build()
///     .unwrap();
///
/// // Value: { "price": "4", "quality": "5", "support": "3" }
/// ```
#[derive(Clone)]
pub struct Matrix {
    metadata: Metadata,
    flags: Flags,
    rows: Vec<MatrixRow>,
    columns: Vec<MatrixColumn>,
    cell_type: MatrixCellType,
    /// If true, all rows must have a value.
    all_rows_required: bool,
    /// Show row numbers.
    show_row_numbers: bool,
    /// Alternate row styling.
    alternate_rows: bool,
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Matrix")
            .field("metadata", &self.metadata)
            .field("flags", &self.flags)
            .field("row_count", &self.rows.len())
            .field("column_count", &self.columns.len())
            .field("cell_type", &self.cell_type)
            .finish_non_exhaustive()
    }
}

impl Matrix {
    /// Creates a new builder for a Matrix.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> MatrixBuilder {
        MatrixBuilder::new(key)
    }

    /// Returns the flags for this matrix.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns all rows.
    #[must_use]
    pub fn rows(&self) -> &[MatrixRow] {
        &self.rows
    }

    /// Returns the number of rows.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns all columns.
    #[must_use]
    pub fn columns(&self) -> &[MatrixColumn] {
        &self.columns
    }

    /// Returns the number of columns.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Returns the cell type.
    #[must_use]
    pub fn cell_type(&self) -> MatrixCellType {
        self.cell_type
    }

    /// Returns true if all rows are required to have a value.
    #[must_use]
    pub fn all_rows_required(&self) -> bool {
        self.all_rows_required
    }

    /// Returns true if row numbers should be shown.
    #[must_use]
    pub fn show_row_numbers(&self) -> bool {
        self.show_row_numbers
    }

    /// Returns true if alternate row styling is enabled.
    #[must_use]
    pub fn alternate_rows(&self) -> bool {
        self.alternate_rows
    }

    /// Gets a row by key.
    #[must_use]
    pub fn get_row(&self, key: &str) -> Option<&MatrixRow> {
        self.rows.iter().find(|r| r.key == key)
    }

    /// Gets a column by value.
    #[must_use]
    pub fn get_column(&self, value: &str) -> Option<&MatrixColumn> {
        self.columns.iter().find(|c| c.value == value)
    }

    /// Returns exclusive columns (columns that deselect others when selected).
    #[must_use]
    pub fn exclusive_columns(&self) -> impl Iterator<Item = &MatrixColumn> {
        self.columns.iter().filter(|c| c.exclusive)
    }

    /// Returns true if this matrix has any exclusive columns.
    #[must_use]
    pub fn has_exclusive_columns(&self) -> bool {
        self.columns.iter().any(|c| c.exclusive)
    }

    /// Returns an iterator over row keys.
    pub fn row_keys(&self) -> impl Iterator<Item = &Key> {
        self.rows.iter().map(|r| &r.key)
    }

    /// Returns an iterator over column values.
    pub fn column_values(&self) -> impl Iterator<Item = &SmartStr> {
        self.columns.iter().map(|c| &c.value)
    }
}

impl Node for Matrix {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Container
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Container for Matrix {
    fn children(&self) -> &[Arc<dyn Node>] {
        // Matrix doesn't have child nodes in the traditional sense.
        // Rows and columns are metadata, not nodes.
        &[]
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Matrix`].
#[derive(Debug)]
pub struct MatrixBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    rows: Vec<MatrixRow>,
    columns: Vec<MatrixColumn>,
    cell_type: MatrixCellType,
    all_rows_required: bool,
    show_row_numbers: bool,
    alternate_rows: bool,
}

impl MatrixBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            rows: Vec::new(),
            columns: Vec::new(),
            cell_type: MatrixCellType::default(),
            all_rows_required: false,
            show_row_numbers: false,
            alternate_rows: true,
        }
    }

    /// Sets the label for this matrix.
    #[must_use]
    pub fn label(mut self, label: impl Into<SmartStr>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description for this matrix.
    #[must_use]
    pub fn description(mut self, description: impl Into<SmartStr>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the flags for this matrix.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Marks this matrix as required.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.flags |= Flags::REQUIRED;
        self
    }

    /// Adds a single row.
    #[must_use]
    pub fn row(mut self, key: impl Into<Key>, label: impl Into<SmartStr>) -> Self {
        self.rows.push(MatrixRow::new(key, label));
        self
    }

    /// Adds a row with description.
    #[must_use]
    pub fn row_with_description(
        mut self,
        key: impl Into<Key>,
        label: impl Into<SmartStr>,
        description: impl Into<SmartStr>,
    ) -> Self {
        self.rows
            .push(MatrixRow::with_description(key, label, description));
        self
    }

    /// Adds multiple rows from (key, label) tuples.
    #[must_use]
    pub fn rows<K, L, I>(mut self, rows: I) -> Self
    where
        K: Into<Key>,
        L: Into<SmartStr>,
        I: IntoIterator<Item = (K, L)>,
    {
        for (key, label) in rows {
            self.rows.push(MatrixRow::new(key, label));
        }
        self
    }

    /// Adds multiple rows from simple labels (key = label).
    #[must_use]
    pub fn rows_from_labels<S, I>(mut self, labels: I) -> Self
    where
        S: Into<SmartStr> + Clone,
        I: IntoIterator<Item = S>,
    {
        for label in labels {
            let s = label.into();
            self.rows.push(MatrixRow {
                key: Key::from(s.as_str()),
                label: s,
                description: None,
            });
        }
        self
    }

    /// Adds a single column.
    #[must_use]
    pub fn column(mut self, value: impl Into<SmartStr>, label: impl Into<SmartStr>) -> Self {
        self.columns.push(MatrixColumn::new(value, label));
        self
    }

    /// Adds a column with weight for scoring.
    #[must_use]
    pub fn column_with_weight(
        mut self,
        value: impl Into<SmartStr>,
        label: impl Into<SmartStr>,
        weight: i32,
    ) -> Self {
        self.columns
            .push(MatrixColumn::with_weight(value, label, weight));
        self
    }

    /// Adds an exclusive column that deselects others when selected.
    ///
    /// Useful for "Not Applicable", "N/A", or "Don't Know" options.
    #[must_use]
    pub fn exclusive_column(mut self, value: impl Into<SmartStr>, label: impl Into<SmartStr>) -> Self {
        self.columns.push(MatrixColumn::exclusive(value, label));
        self
    }

    /// Adds multiple columns from (value, label) tuples.
    #[must_use]
    pub fn columns<V, L, I>(mut self, columns: I) -> Self
    where
        V: Into<SmartStr>,
        L: Into<SmartStr>,
        I: IntoIterator<Item = (V, L)>,
    {
        for (value, label) in columns {
            self.columns.push(MatrixColumn::new(value, label));
        }
        self
    }

    /// Adds multiple columns from simple labels (value = label).
    #[must_use]
    pub fn columns_from_labels<S, I>(mut self, labels: I) -> Self
    where
        S: Into<SmartStr> + Clone,
        I: IntoIterator<Item = S>,
    {
        for label in labels {
            let s = label.into();
            self.columns.push(MatrixColumn {
                value: s.clone(),
                label: s,
                weight: None,
                exclusive: false,
            });
        }
        self
    }

    /// Sets the cell type for this matrix.
    #[must_use]
    pub fn cell_type(mut self, cell_type: MatrixCellType) -> Self {
        self.cell_type = cell_type;
        self
    }

    /// Sets the cell type to radio buttons (single select).
    #[must_use]
    pub fn radio(mut self) -> Self {
        self.cell_type = MatrixCellType::Radio;
        self
    }

    /// Sets the cell type to checkboxes (multi select).
    #[must_use]
    pub fn checkbox(mut self) -> Self {
        self.cell_type = MatrixCellType::Checkbox;
        self
    }

    /// Sets the cell type to dropdown.
    #[must_use]
    pub fn dropdown(mut self) -> Self {
        self.cell_type = MatrixCellType::Dropdown;
        self
    }

    /// Requires all rows to have a value.
    #[must_use]
    pub fn all_rows_required(mut self, required: bool) -> Self {
        self.all_rows_required = required;
        self
    }

    /// Shows row numbers.
    #[must_use]
    pub fn show_row_numbers(mut self, show: bool) -> Self {
        self.show_row_numbers = show;
        self
    }

    /// Enables alternate row styling.
    #[must_use]
    pub fn alternate_rows(mut self, alternate: bool) -> Self {
        self.alternate_rows = alternate;
        self
    }

    /// Builds the Matrix.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No rows were added
    /// - No columns were added
    /// - Duplicate row keys exist
    /// - Duplicate column values exist
    pub fn build(self) -> crate::core::Result<Matrix> {
        if self.rows.is_empty() {
            return Err(crate::core::Error::missing_required("rows"));
        }

        if self.columns.is_empty() {
            return Err(crate::core::Error::missing_required("columns"));
        }

        // Check for duplicate row keys
        let mut seen_row_keys = FxHashSet::default();
        for row in &self.rows {
            if !seen_row_keys.insert(&row.key) {
                return Err(crate::core::Error::validation(
                    "duplicate_key",
                    format!("duplicate row key: {}", row.key),
                ));
            }
        }

        // Check for duplicate column values
        let mut seen_column_values = FxHashSet::default();
        for column in &self.columns {
            if !seen_column_values.insert(&column.value) {
                return Err(crate::core::Error::validation(
                    "duplicate_value",
                    format!("duplicate column value: {}", column.value),
                ));
            }
        }

        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        Ok(Matrix {
            metadata,
            flags: self.flags,
            rows: self.rows,
            columns: self.columns,
            cell_type: self.cell_type,
            all_rows_required: self.all_rows_required,
            show_row_numbers: self.show_row_numbers,
            alternate_rows: self.alternate_rows,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_basic() {
        let matrix = Matrix::builder("satisfaction")
            .label("Rate your satisfaction")
            .row("price", "Price")
            .row("quality", "Quality")
            .column("1", "Poor")
            .column("2", "Fair")
            .column("3", "Good")
            .build()
            .unwrap();

        assert_eq!(matrix.key().as_str(), "satisfaction");
        assert_eq!(matrix.metadata().label(), Some("Rate your satisfaction"));
        assert_eq!(matrix.kind(), NodeKind::Container);
        assert_eq!(matrix.row_count(), 2);
        assert_eq!(matrix.column_count(), 3);
        assert_eq!(matrix.cell_type(), MatrixCellType::Radio);
    }

    #[test]
    fn test_matrix_with_tuples() {
        let matrix = Matrix::builder("survey")
            .rows([("price", "Price"), ("quality", "Quality"), ("speed", "Speed")])
            .columns([
                ("1", "Very Poor"),
                ("2", "Poor"),
                ("3", "Fair"),
                ("4", "Good"),
                ("5", "Excellent"),
            ])
            .build()
            .unwrap();

        assert_eq!(matrix.row_count(), 3);
        assert_eq!(matrix.column_count(), 5);
    }

    #[test]
    fn test_matrix_from_labels() {
        let matrix = Matrix::builder("features")
            .rows_from_labels(["Feature A", "Feature B", "Feature C"])
            .columns_from_labels(["Yes", "No", "Maybe"])
            .build()
            .unwrap();

        assert_eq!(matrix.row_count(), 3);
        assert_eq!(matrix.column_count(), 3);

        // Keys should be same as labels
        let row = matrix.get_row("Feature A");
        assert!(row.is_some());
        assert_eq!(row.unwrap().label, "Feature A");
    }

    #[test]
    fn test_matrix_cell_types() {
        let radio = Matrix::builder("m")
            .row("r", "R")
            .column("c", "C")
            .radio()
            .build()
            .unwrap();
        assert_eq!(radio.cell_type(), MatrixCellType::Radio);
        assert!(!radio.cell_type().is_multi_select());

        let checkbox = Matrix::builder("m")
            .row("r", "R")
            .column("c", "C")
            .checkbox()
            .build()
            .unwrap();
        assert_eq!(checkbox.cell_type(), MatrixCellType::Checkbox);
        assert!(checkbox.cell_type().is_multi_select());

        let dropdown = Matrix::builder("m")
            .row("r", "R")
            .column("c", "C")
            .dropdown()
            .build()
            .unwrap();
        assert_eq!(dropdown.cell_type(), MatrixCellType::Dropdown);
    }

    #[test]
    fn test_matrix_column_weights() {
        let matrix = Matrix::builder("rating")
            .row("item", "Item")
            .column_with_weight("1", "Poor", 1)
            .column_with_weight("2", "Fair", 2)
            .column_with_weight("3", "Good", 3)
            .column_with_weight("4", "Excellent", 4)
            .build()
            .unwrap();

        let col = matrix.get_column("3").unwrap();
        assert_eq!(col.weight, Some(3));
    }

    #[test]
    fn test_matrix_options() {
        let matrix = Matrix::builder("m")
            .row("r", "R")
            .column("c", "C")
            .all_rows_required(true)
            .show_row_numbers(true)
            .alternate_rows(false)
            .required()
            .build()
            .unwrap();

        assert!(matrix.all_rows_required());
        assert!(matrix.show_row_numbers());
        assert!(!matrix.alternate_rows());
        assert!(matrix.flags().contains(Flags::REQUIRED));
    }

    #[test]
    fn test_matrix_row_with_description() {
        let matrix = Matrix::builder("m")
            .row_with_description("price", "Price", "How satisfied are you with the price?")
            .column("1", "Bad")
            .build()
            .unwrap();

        let row = matrix.get_row("price").unwrap();
        assert_eq!(
            row.description.as_deref(),
            Some("How satisfied are you with the price?")
        );
    }

    #[test]
    fn test_matrix_requires_rows() {
        let result = Matrix::builder("m").column("c", "C").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_matrix_requires_columns() {
        let result = Matrix::builder("m").row("r", "R").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_matrix_duplicate_row_keys() {
        let result = Matrix::builder("m")
            .row("same", "First")
            .row("same", "Second")
            .column("c", "C")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_matrix_duplicate_column_values() {
        let result = Matrix::builder("m")
            .row("r", "R")
            .column("same", "First")
            .column("same", "Second")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_matrix_iterators() {
        let matrix = Matrix::builder("m")
            .rows([("a", "A"), ("b", "B")])
            .columns([("1", "One"), ("2", "Two")])
            .build()
            .unwrap();

        let row_keys: Vec<&str> = matrix.row_keys().map(|k| k.as_str()).collect();
        assert_eq!(row_keys, vec!["a", "b"]);

        let col_values: Vec<&str> = matrix.column_values().map(|v| v.as_str()).collect();
        assert_eq!(col_values, vec!["1", "2"]);
    }

    #[test]
    fn test_matrix_children_empty() {
        let matrix = Matrix::builder("m")
            .row("r", "R")
            .column("c", "C")
            .build()
            .unwrap();

        // Matrix has no child nodes - rows/columns are metadata
        assert!(matrix.children().is_empty());
    }

    #[test]
    fn test_cell_type_names() {
        assert_eq!(MatrixCellType::Radio.name(), "radio");
        assert_eq!(MatrixCellType::Checkbox.name(), "checkbox");
        assert_eq!(MatrixCellType::Dropdown.name(), "dropdown");
        assert_eq!(MatrixCellType::Text.name(), "text");
        assert_eq!(MatrixCellType::Rating.name(), "rating");
    }

    #[test]
    fn test_matrix_exclusive_columns() {
        let matrix = Matrix::builder("satisfaction")
            .row("price", "Price")
            .row("quality", "Quality")
            .column("1", "Poor")
            .column("2", "Fair")
            .column("3", "Good")
            .exclusive_column("na", "Not Applicable")
            .build()
            .unwrap();

        assert!(matrix.has_exclusive_columns());
        assert_eq!(matrix.exclusive_columns().count(), 1);

        let na_col = matrix.get_column("na").unwrap();
        assert!(na_col.is_exclusive());

        let good_col = matrix.get_column("3").unwrap();
        assert!(!good_col.is_exclusive());
    }

    #[test]
    fn test_matrix_column_exclusive_constructors() {
        let col = MatrixColumn::exclusive("na", "N/A");
        assert!(col.is_exclusive());
        assert!(col.weight.is_none());

        let col_weighted = MatrixColumn::exclusive_with_weight("na", "N/A", 0);
        assert!(col_weighted.is_exclusive());
        assert_eq!(col_weighted.weight, Some(0));
    }
}
