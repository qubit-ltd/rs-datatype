/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Boolean Conversion Options
//!
//! Defines options that control string-to-boolean conversion.
//!
//! # Author
//!
//! Haixing Hu

/// Options that control string-to-boolean conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanConversionOptions {
    /// String literals accepted as `true`.
    true_literals: Vec<String>,
    /// String literals accepted as `false`.
    false_literals: Vec<String>,
    /// Whether literal matching is case-sensitive.
    pub case_sensitive: bool,
}

impl BooleanConversionOptions {
    /// Creates strict boolean conversion options.
    ///
    /// # Returns
    ///
    /// Options accepting `true`, `false`, `1`, and `0`.
    pub fn strict() -> Self {
        Self {
            true_literals: vec!["true".to_string(), "1".to_string()],
            false_literals: vec!["false".to_string(), "0".to_string()],
            case_sensitive: false,
        }
    }

    /// Creates options suitable for environment variable values.
    ///
    /// # Returns
    ///
    /// Options accepting `true/false`, `1/0`, `yes/no`, and `on/off`.
    pub fn env_friendly() -> Self {
        Self {
            true_literals: vec![
                "true".to_string(),
                "1".to_string(),
                "yes".to_string(),
                "on".to_string(),
            ],
            false_literals: vec![
                "false".to_string(),
                "0".to_string(),
                "no".to_string(),
                "off".to_string(),
            ],
            case_sensitive: false,
        }
    }

    /// Gets the accepted true literals.
    ///
    /// # Returns
    ///
    /// A slice of accepted true literals.
    #[inline]
    pub fn true_literals(&self) -> &[String] {
        &self.true_literals
    }

    /// Gets the accepted false literals.
    ///
    /// # Returns
    ///
    /// A slice of accepted false literals.
    #[inline]
    pub fn false_literals(&self) -> &[String] {
        &self.false_literals
    }

    /// Returns a copy with boolean literal matching case sensitivity changed.
    ///
    /// # Parameters
    ///
    /// * `case_sensitive` - Whether matching is case-sensitive.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    /// Returns a copy that accepts an additional true literal.
    ///
    /// # Parameters
    ///
    /// * `literal` - Literal to parse as `true`.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_true_literal(mut self, literal: &str) -> Self {
        self.true_literals.push(literal.to_string());
        self
    }

    /// Returns a copy that accepts an additional false literal.
    ///
    /// # Parameters
    ///
    /// * `literal` - Literal to parse as `false`.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_false_literal(mut self, literal: &str) -> Self {
        self.false_literals.push(literal.to_string());
        self
    }

    /// Parses a boolean literal using these options.
    ///
    /// # Parameters
    ///
    /// * `value` - Candidate boolean literal.
    ///
    /// # Returns
    ///
    /// Returns `Some(bool)` when the literal is recognized, or `None`
    /// otherwise.
    pub fn parse(&self, value: &str) -> Option<bool> {
        let value = value.trim();
        if self.case_sensitive {
            if self.true_literals.iter().any(|literal| literal == value) {
                Some(true)
            } else if self.false_literals.iter().any(|literal| literal == value) {
                Some(false)
            } else {
                None
            }
        } else if self
            .true_literals
            .iter()
            .any(|literal| literal.eq_ignore_ascii_case(value))
        {
            Some(true)
        } else if self
            .false_literals
            .iter()
            .any(|literal| literal.eq_ignore_ascii_case(value))
        {
            Some(false)
        } else {
            None
        }
    }
}

impl Default for BooleanConversionOptions {
    /// Creates default boolean conversion options.
    fn default() -> Self {
        Self::strict()
    }
}
