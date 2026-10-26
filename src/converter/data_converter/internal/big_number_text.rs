// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conservative output-size preflight for arbitrary-precision number text.

use crate::converter::ConversionContext;
use crate::converter::DataConversionError;
use crate::converter::DataConverter;

/// Returns a conservative lower bound for the decimal digits of a binary value.
///
/// The estimate never formats or allocates from the value. It is used only to
/// reject values that cannot fit the remaining output budget; the bounded
/// writer still checks the complete rendered output.
///
/// # Parameters
///
/// * `bits` - Number of significant magnitude bits reported by the big number.
///
/// # Returns
///
/// A lower bound of at least one decimal digit, including for zero.
#[inline]
fn decimal_digit_lower_bound(bits: u64) -> u64 {
    if bits == 0 { 1 } else { (bits - 1) / 4 + 1 }
}

/// Rejects a big-number String conversion whose decimal digits cannot fit.
///
/// This is a read-only preflight. It neither clones the source nor consumes
/// output capacity; successful checks continue to the existing bounded writer.
/// Non-big-number sources are left unchanged.
///
/// # Parameters
///
/// * `source` - Admitted value whose direct Display output is being prepared.
/// * `context` - Conversion context binding the real source and String target.
///
/// # Returns
///
/// `Ok(())` when the lower bound fits or the source is not an arbitrary-
/// precision number.
///
/// # Errors
///
/// Returns the existing output-budget error when even the lower bound exceeds
/// the remaining output capacity.
#[inline]
pub(in crate::converter::data_converter) fn check_big_number_output(
    source: &DataConverter<'_>,
    context: &ConversionContext<'_, '_>,
) -> Result<(), DataConversionError> {
    let minimum = match source {
        #[cfg(feature = "big-integer")]
        DataConverter::BigInteger(value) => decimal_digit_lower_bound(value.bits()),
        #[cfg(feature = "big-decimal")]
        DataConverter::BigDecimal(value) => {
            let (coefficient, _) = value.as_bigint_and_scale();
            decimal_digit_lower_bound(coefficient.bits())
        }
        _ => return Ok(()),
    };
    context.check_output_bytes(minimum)
}

#[cfg(test)]
mod tests {
    use super::decimal_digit_lower_bound;

    #[test]
    fn test_decimal_digit_lower_bound_is_conservative() {
        assert_eq!(decimal_digit_lower_bound(0), 1);
        assert_eq!(decimal_digit_lower_bound(1), 1);
        assert_eq!(decimal_digit_lower_bound(4), 1);
        assert_eq!(decimal_digit_lower_bound(5), 2);
        assert_eq!(decimal_digit_lower_bound(8), 2);
        assert_eq!(decimal_digit_lower_bound(9), 3);
    }
}
