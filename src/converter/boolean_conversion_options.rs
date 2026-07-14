// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Boolean Conversion Options
//!
//! Defines options that control string-to-boolean conversion.

use serde::de::Error as DeError;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
};

use super::boolean_literal_conflict_error::BooleanLiteralConflictError;
use super::boolean_numeric_policy::BooleanNumericPolicy;

/// Options that control string-to-boolean conversion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BooleanConversionOptions {
    /// String literals accepted as `true`.
    true_literals: Vec<String>,
    /// String literals accepted as `false`.
    false_literals: Vec<String>,
    /// Whether literal matching is case-sensitive.
    case_sensitive: bool,
    /// Policy used when converting integer sources.
    numeric_policy: BooleanNumericPolicy,
}

impl BooleanConversionOptions {
    /// Creates validated boolean conversion options.
    ///
    /// # Errors
    ///
    /// Returns [`BooleanLiteralConflictError`] when a literal belongs to both
    /// sets under the selected case-sensitivity rule.
    pub fn try_new(
        true_literals: Vec<String>,
        false_literals: Vec<String>,
        case_sensitive: bool,
        numeric_policy: BooleanNumericPolicy,
    ) -> Result<Self, BooleanLiteralConflictError> {
        let options = Self {
            true_literals,
            false_literals,
            case_sensitive,
            numeric_policy,
        };
        options.validate()?;
        Ok(options)
    }

    /// Creates strict boolean conversion options.
    ///
    /// # Returns
    ///
    /// Options accepting the textual literals `true` and `false`.
    #[inline]
    pub fn strict() -> Self {
        Self {
            true_literals: default_true_literals(),
            false_literals: default_false_literals(),
            case_sensitive: false,
            numeric_policy: BooleanNumericPolicy::default(),
        }
    }

    /// Creates options suitable for environment variable values.
    ///
    /// # Returns
    ///
    /// Options accepting `true/false`, `yes/no`, and `on/off`.
    #[inline]
    pub fn env_friendly() -> Self {
        Self {
            true_literals: vec![
                "true".to_string(),
                "yes".to_string(),
                "on".to_string(),
            ],
            false_literals: vec![
                "false".to_string(),
                "no".to_string(),
                "off".to_string(),
            ],
            case_sensitive: false,
            numeric_policy: BooleanNumericPolicy::default(),
        }
    }

    /// Gets the accepted true literals.
    ///
    /// # Returns
    ///
    /// A slice of accepted true literals.
    #[inline(always)]
    pub fn true_literals(&self) -> &[String] {
        &self.true_literals
    }

    /// Gets the accepted false literals.
    ///
    /// # Returns
    ///
    /// A slice of accepted false literals.
    #[inline(always)]
    pub fn false_literals(&self) -> &[String] {
        &self.false_literals
    }

    /// Returns whether literal matching is case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive(&self) -> bool {
        self.case_sensitive
    }

    /// Returns the integer-to-boolean policy.
    #[inline(always)]
    pub const fn numeric_policy(&self) -> BooleanNumericPolicy {
        self.numeric_policy
    }

    /// Returns a copy with boolean literal matching case sensitivity changed.
    ///
    /// # Parameters
    ///
    /// * `case_sensitive` - Whether matching is case-sensitive.
    ///
    /// # Returns
    ///
    /// Updated options when the new matching rule keeps literal sets disjoint.
    ///
    /// # Errors
    ///
    /// Returns [`BooleanLiteralConflictError`] when changing the matching rule
    /// makes a true literal equal to a false literal.
    #[inline]
    pub fn with_case_sensitive(
        mut self,
        case_sensitive: bool,
    ) -> Result<Self, BooleanLiteralConflictError> {
        self.case_sensitive = case_sensitive;
        self.validate()?;
        Ok(self)
    }

    /// Returns a copy with a different integer-to-boolean policy.
    #[inline]
    #[must_use]
    pub fn with_numeric_policy(
        mut self,
        numeric_policy: BooleanNumericPolicy,
    ) -> Self {
        self.numeric_policy = numeric_policy;
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
    pub fn with_true_literal(
        mut self,
        literal: &str,
    ) -> Result<Self, BooleanLiteralConflictError> {
        self.true_literals.push(literal.to_string());
        self.validate()?;
        Ok(self)
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
    #[inline]
    pub fn with_false_literal(
        mut self,
        literal: &str,
    ) -> Result<Self, BooleanLiteralConflictError> {
        self.false_literals.push(literal.to_string());
        self.validate()?;
        Ok(self)
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
        if self.case_sensitive {
            if self.true_literals.iter().any(|literal| literal == value) {
                Some(true)
            } else if self.false_literals.iter().any(|literal| literal == value)
            {
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

    /// Checks whether the true and false literal sets are disjoint.
    ///
    /// # Errors
    ///
    /// Returns [`BooleanLiteralConflictError`] when the sets overlap according
    /// to the configured case-sensitivity rule.
    pub fn validate(&self) -> Result<(), BooleanLiteralConflictError> {
        let overlaps = self.true_literals.iter().any(|true_literal| {
            self.false_literals.iter().any(|false_literal| {
                if self.case_sensitive {
                    true_literal == false_literal
                } else {
                    true_literal.eq_ignore_ascii_case(false_literal)
                }
            })
        });
        if overlaps {
            Err(BooleanLiteralConflictError)
        } else {
            Ok(())
        }
    }
}

impl Default for BooleanConversionOptions {
    /// Creates default boolean conversion options.
    #[inline(always)]
    fn default() -> Self {
        Self::strict()
    }
}

/// Deserialization representation with field defaults.
#[derive(Deserialize)]
#[serde(default)]
struct BooleanConversionOptionsDef {
    /// String literals accepted as true.
    true_literals: Vec<String>,
    /// String literals accepted as false.
    false_literals: Vec<String>,
    /// Whether matching is case-sensitive.
    case_sensitive: bool,
    /// Numeric boolean policy.
    numeric_policy: BooleanNumericPolicy,
}

impl Default for BooleanConversionOptionsDef {
    /// Creates the wire defaults used by [`BooleanConversionOptions`].
    fn default() -> Self {
        let options = BooleanConversionOptions::default();
        Self {
            true_literals: options.true_literals,
            false_literals: options.false_literals,
            case_sensitive: options.case_sensitive,
            numeric_policy: options.numeric_policy,
        }
    }
}

impl<'de> Deserialize<'de> for BooleanConversionOptions {
    /// Deserializes and validates boolean conversion options.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition =
            BooleanConversionOptionsDef::deserialize(deserializer)?;
        Self::try_new(
            definition.true_literals,
            definition.false_literals,
            definition.case_sensitive,
            definition.numeric_policy,
        )
        .map_err(D::Error::custom)
    }
}

/// Creates the default true literal list.
fn default_true_literals() -> Vec<String> {
    vec!["true".to_string()]
}

/// Creates the default false literal list.
fn default_false_literals() -> Vec<String> {
    vec!["false".to_string()]
}
