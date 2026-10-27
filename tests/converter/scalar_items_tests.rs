// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for lazy scalar collection item iteration.

use proptest::prelude::Strategy;
use proptest::prelude::any;
use proptest::proptest;
use qubit_datatype::ScalarItemError;
use qubit_datatype::converter::CollectionConversionLimits;
use qubit_datatype::converter::CollectionConversionPolicy;
use qubit_datatype::converter::EmptyItemPolicy;
use qubit_datatype::converter::ScalarItems;

fn reference_scalar_items(
    text: &str,
    delimiters: &[char],
    trim: bool,
    empty_policy: EmptyItemPolicy,
    maximum: u64,
) -> Vec<Result<(usize, String), ScalarItemError>> {
    let mut retained = 0_u64;
    let mut expected = Vec::new();
    for (source_index, raw) in text.split(|ch| delimiters.contains(&ch)).enumerate() {
        let value = if trim { raw.trim() } else { raw };
        if value.is_empty() {
            match empty_policy {
                EmptyItemPolicy::Skip => continue,
                EmptyItemPolicy::Reject => {
                    expected.push(Err(ScalarItemError::BlankRejected { source_index }));
                    continue;
                }
                EmptyItemPolicy::Keep => {}
            }
        }
        if retained == maximum {
            expected.push(Err(ScalarItemError::ItemLimitExceeded { source_index, maximum }));
            break;
        }
        retained += 1;
        expected.push(Ok((source_index, value.to_owned())));
    }
    expected
}

fn assert_scalar_items_match_reference(
    text: &str,
    delimiters: &[char],
    trim: bool,
    empty_policy: EmptyItemPolicy,
    maximum: u64,
) {
    let expected = reference_scalar_items(text, delimiters, trim, empty_policy, maximum);
    let policy = CollectionConversionPolicy::builder()
        .split_scalar_strings(true)
        .delimiters(delimiters.iter().copied())
        .trim_items(trim)
        .empty_item_policy(empty_policy)
        .build();
    let limits = CollectionConversionLimits::builder().max_items(maximum).build();
    let mut iterator = policy.scalar_items(&limits, text);
    let actual = iterator
        .by_ref()
        .map(|item| item.map(|item| (item.source_index, item.value.to_owned())))
        .collect::<Vec<_>>();
    assert_eq!(
        actual, expected,
        "split results and source indices must match the reference"
    );
    assert!(iterator.next().is_none(), "exhaustion must remain stable");
}

proptest! {
    #[test]
    fn test_scalar_items_matches_unicode_reference(
        delimiters in proptest::collection::vec(any::<char>(), 0..65),
        pieces in proptest::collection::vec(
            proptest::collection::vec(any::<char>(), 0..25)
                .prop_map(|chars| chars.into_iter().collect::<String>()),
            0..25,
        ),
        trim in any::<bool>(),
        empty_control in 0_u8..3,
        maximum in 0_u64..33,
    ) {
        let mut text = String::new();
        for (index, piece) in pieces.iter().enumerate() {
            if index != 0 && !delimiters.is_empty() {
                text.push(delimiters[(index - 1) % delimiters.len()]);
            }
            text.push_str(piece);
        }
        let empty_policy = match empty_control {
            0 => EmptyItemPolicy::Keep,
            1 => EmptyItemPolicy::Skip,
            _ => EmptyItemPolicy::Reject,
        };
        assert_scalar_items_match_reference(&text, &delimiters, trim, empty_policy, maximum);
    }
}

#[test]
fn test_scalar_items_reference_at_delimiter_threshold() {
    let delimiters = ['|', ',', ';', ':', '/', '、', '😀', '💠', '\u{10000}'];
    for count in [0, 8, 9] {
        for trim in [false, true] {
            for empty_policy in [EmptyItemPolicy::Keep, EmptyItemPolicy::Skip, EmptyItemPolicy::Reject] {
                for maximum in [0, 1, 32] {
                    for text in ["", " a| |b、c😀d💠e𐀀 ", "||"] {
                        assert_scalar_items_match_reference(text, &delimiters[..count], trim, empty_policy, maximum);
                    }
                }
            }
        }
    }
}

/// Test lazy splitting, trimming, skipping, and original source indices.
#[test]
fn test_scalar_items_lazily_retains_original_indices() {
    let options = CollectionConversionPolicy::builder()
        .split_scalar_strings(true)
        .delimiters([',', '、'])
        .trim_items(true)
        .empty_item_policy(EmptyItemPolicy::Skip)
        .build();
    let mut items: ScalarItems<'_> = options.scalar_items(&CollectionConversionLimits::default(), " alpha,  、beta ");

    let first = items
        .next()
        .expect("first item should exist")
        .expect("first item should be valid");
    assert_eq!((first.source_index, first.value), (0, "alpha"));

    let second = items
        .next()
        .expect("second retained item should exist")
        .expect("second retained item should be valid");
    assert_eq!((second.source_index, second.value), (2, "beta"));
    assert!(items.next().is_none());
}

/// Test rejection is discovered lazily and iteration can continue afterward.
#[test]
fn test_scalar_items_reports_rejection_when_reached() {
    let options = CollectionConversionPolicy::builder()
        .split_scalar_strings(true)
        .trim_items(true)
        .empty_item_policy(EmptyItemPolicy::Reject)
        .build();
    let mut items = options.scalar_items(&CollectionConversionLimits::default(), "first, ,third");

    assert_eq!(
        items
            .next()
            .expect("first item should exist")
            .expect("first item should be valid")
            .value,
        "first",
    );
    assert_eq!(
        items
            .next()
            .expect("blank item should be visited")
            .expect_err("blank item should be rejected")
            .source_index(),
        1,
    );
    assert_eq!(
        items
            .next()
            .expect("iteration should continue")
            .expect("third item should be valid")
            .value,
        "third",
    );
    assert!(items.next().is_none());
}

/// Test retained-item limits after empty-item filtering.
#[test]
fn test_scalar_items_enforces_retained_item_limit() {
    let options = CollectionConversionPolicy::builder()
        .split_scalar_strings(true)
        .trim_items(true)
        .empty_item_policy(EmptyItemPolicy::Skip)
        .build();
    let limits = CollectionConversionLimits::builder().max_items(2).build();
    let mut items = options.scalar_items(&limits, "a,  ,b,c,d");

    assert_eq!(items.next().expect("first item").expect("valid").value, "a");
    assert_eq!(items.next().expect("second item").expect("valid").value, "b");
    let error = items
        .next()
        .expect("limit error")
        .expect_err("third retained item must exceed the limit");
    assert_eq!(error.source_index(), 3);
    assert_eq!(error.maximum_items(), Some(2));
    assert!(items.next().is_none());
}

/// Test rejected empty items take precedence and do not consume quota.
#[test]
fn test_scalar_items_rejection_precedes_item_limit() {
    let options = CollectionConversionPolicy::builder()
        .split_scalar_strings(true)
        .empty_item_policy(EmptyItemPolicy::Reject)
        .build();
    let limits = CollectionConversionLimits::builder().max_items(1).build();
    let mut items = options.scalar_items(&limits, "a,,b");

    assert_eq!(items.next().expect("first item").expect("valid").value, "a");
    let blank = items
        .next()
        .expect("blank rejection")
        .expect_err("blank item must be rejected before limit checking");
    assert_eq!(blank.source_index(), 1);
    assert_eq!(blank.maximum_items(), None);
    let limit = items
        .next()
        .expect("limit error")
        .expect_err("next retained item must exceed the limit");
    assert_eq!(limit.source_index(), 2);
    assert_eq!(limit.maximum_items(), Some(1));
    assert!(items.next().is_none());
}

/// Test zero permits only an empty retained result.
#[test]
fn test_scalar_items_zero_limit_allows_only_empty_result() {
    let retained = CollectionConversionPolicy::builder().split_scalar_strings(true).build();
    let zero = CollectionConversionLimits::builder().max_items(0).build();
    let error = retained
        .scalar_items(&zero, "a")
        .next()
        .expect("limit error")
        .expect_err("zero limit must reject the first retained item");
    assert_eq!(error.source_index(), 0);
    assert_eq!(error.maximum_items(), Some(0));

    let skipped = CollectionConversionPolicy::builder()
        .split_scalar_strings(true)
        .empty_item_policy(EmptyItemPolicy::Skip)
        .build();
    assert!(skipped.scalar_items(&zero, ",,").next().is_none());
}

/// Test a large delimiter set preserves Unicode splitting semantics.
#[test]
fn test_scalar_items_supports_large_delimiter_sets() {
    let delimiters = std::iter::once(',').chain((0x100..0x140).filter_map(char::from_u32));
    let options = CollectionConversionPolicy::builder()
        .split_scalar_strings(true)
        .delimiters(delimiters)
        .build();
    let values = options
        .scalar_items(&CollectionConversionLimits::default(), "alpha,bravoĀcharlie")
        .map(|item| item.expect("all split items should be valid").value)
        .collect::<Vec<_>>();

    assert_eq!(values, ["alpha", "bravo", "charlie"]);
}

#[test]
fn test_scalar_items_can_be_cloned_before_iteration() {
    let options = CollectionConversionPolicy::builder().split_scalar_strings(true).build();
    let items = options.scalar_items(&CollectionConversionLimits::default(), "alpha,beta");
    let mut cloned = items.clone();

    assert_eq!(
        cloned
            .next()
            .expect("cloned iterator should yield an item")
            .expect("item should be valid")
            .value,
        "alpha"
    );
}

#[test]
fn test_scalar_items_clone_after_partial_consumption_is_independent() {
    let options = CollectionConversionPolicy::builder()
        .split_scalar_strings(true)
        .empty_item_policy(EmptyItemPolicy::Skip)
        .build();
    let limits = CollectionConversionLimits::builder().max_items(2).build();
    let mut original = options.scalar_items(&limits, "a,,b,c");

    let first = original
        .next()
        .expect("first item should exist")
        .expect("first item should be valid");
    assert_eq!((first.source_index, first.value), (0, "a"));

    let mut cloned = original.clone();

    let original_second = original
        .next()
        .expect("second original item should exist")
        .expect("second original item should be valid");
    assert_eq!((original_second.source_index, original_second.value), (2, "b"));
    let cloned_second = cloned
        .next()
        .expect("second cloned item should exist")
        .expect("second cloned item should be valid");
    assert_eq!((cloned_second.source_index, cloned_second.value), (2, "b"));

    let original_limit = original
        .next()
        .expect("original limit error should exist")
        .expect_err("third retained original item should exceed the limit");
    assert_eq!(original_limit.source_index(), 3);
    assert_eq!(original_limit.maximum_items(), Some(2));
    let cloned_limit = cloned
        .next()
        .expect("cloned limit error should exist")
        .expect_err("third retained cloned item should exceed the limit");
    assert_eq!(cloned_limit.source_index(), 3);
    assert_eq!(cloned_limit.maximum_items(), Some(2));
    assert!(original.next().is_none());
    assert!(original.next().is_none());
    assert!(cloned.next().is_none());

    let mut exhausted_clone = original.clone();
    assert!(exhausted_clone.next().is_none());
    assert!(exhausted_clone.next().is_none());
}

#[test]
fn test_scalar_items_clone_after_normal_exhaustion_stays_exhausted() {
    let options = CollectionConversionPolicy::builder().split_scalar_strings(true).build();
    let limits = CollectionConversionLimits::default();
    let mut original = options.scalar_items(&limits, "a");

    assert_eq!(
        original
            .next()
            .expect("only item should exist")
            .expect("only item should be valid")
            .value,
        "a"
    );
    assert!(original.next().is_none());

    let mut cloned = original.clone();
    assert!(cloned.next().is_none());
    assert!(cloned.next().is_none());
    assert!(original.next().is_none());
}
