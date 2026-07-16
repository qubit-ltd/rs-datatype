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

use super::super::error::BooleanLiteralConflictError;
use super::boolean_numeric_policy::BooleanNumericPolicy;
use super::internal::UncheckedBooleanConversionOptions;

/// Validated rules for textual and numeric boolean conversion.
///
/// True and false literal sets must remain disjoint under the selected
/// case-sensitivity rule. Constructors and fallible builder methods validate
/// that invariant, and deserialization validates it before returning a value.
/// Numeric handling is independent from textual literal matching.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{BooleanConversionOptions, BooleanNumericPolicy};
///
/// let options = BooleanConversionOptions::strict()
///     .with_true_literal("enabled")
///     .expect("literal sets are disjoint")
///     .with_numeric_policy(BooleanNumericPolicy::Reject);
/// assert_eq!(options.parse("ENABLED"), Some(true));
/// assert_eq!(options.parse("unknown"), None);
/// ```
#[must_use]
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
    /// Text literals accepted as `true` by the strict/default profile.
    pub const DEFAULT_TRUE_LITERALS: &'static [&'static str] = &["true"];

    /// Text literals accepted as `false` by the strict/default profile.
    pub const DEFAULT_FALSE_LITERALS: &'static [&'static str] = &["false"];

    /// Creates validated boolean conversion options.
    ///
    /// # Parameters
    ///
    /// * `true_literals` - Text values recognized as `true`.
    /// * `false_literals` - Text values recognized as `false`.
    /// * `case_sensitive` - Whether literal comparison observes ASCII case.
    /// * `numeric_policy` - Rule for integer-to-boolean conversion.
    ///
    /// # Returns
    ///
    /// Returns validated options containing the supplied values.
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
            true_literals: Self::DEFAULT_TRUE_LITERALS
                .iter()
                .map(|literal| (*literal).to_string())
                .collect(),
            false_literals: Self::DEFAULT_FALSE_LITERALS
                .iter()
                .map(|literal| (*literal).to_string())
                .collect(),
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
    ///
    /// # Parameters
    ///
    /// * `numeric_policy` - New policy for integer sources.
    ///
    /// # Returns
    ///
    /// Returns the updated options value.
    #[inline]
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
    ///
    /// # Errors
    ///
    /// Returns [`BooleanLiteralConflictError`] if the new literal overlaps a
    /// false literal under the configured case-sensitivity rule.
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
    ///
    /// # Errors
    ///
    /// Returns [`BooleanLiteralConflictError`] if the new literal overlaps a
    /// true literal under the configured case-sensitivity rule.
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

impl<'de> Deserialize<'de> for BooleanConversionOptions {
    /// Deserializes and validates boolean conversion options.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition =
            UncheckedBooleanConversionOptions::deserialize(deserializer)?;
        Self::try_new(
            definition.true_literals,
            definition.false_literals,
            definition.case_sensitive,
            definition.numeric_policy,
        )
        .map_err(D::Error::custom)
    }
}
