use qubit_budget::{BudgetError, ResourceLimit};
use qubit_datatype::converter::{ConversionLimitExceeded, ConversionResource};

#[test]
fn test_conversion_limit_exceeded_exposes_point_facts() {
    let error = ConversionLimitExceeded::from_budget_error(BudgetError::LimitExceeded {
        resource: ConversionResource::NumericTextBytes,
        actual: 8,
        maximum: 3,
    });

    assert_eq!(error.resource(), ConversionResource::NumericTextBytes);
    assert_eq!(error.actual(), Some(8));
    assert_eq!(error.maximum(), Some(3));
    assert_eq!(error.limit(), None);
    assert_eq!(error.remaining(), None);
    assert_eq!(error.requested(), None);
    assert_eq!(error.used(), None);
}

#[test]
fn test_conversion_limit_exceeded_exposes_budget_facts() {
    let limit = ResourceLimit::new(ConversionResource::Items, 5);
    let error = ConversionLimitExceeded::from_budget_error(BudgetError::Insufficient {
        resource: ConversionResource::Items,
        limit: 5,
        remaining: 2,
        requested: 4,
    });

    assert_eq!(error.resource(), ConversionResource::Items);
    assert_eq!(error.actual(), None);
    assert_eq!(error.maximum(), None);
    assert_eq!(error.limit(), Some(5));
    assert_eq!(error.remaining(), Some(2));
    assert_eq!(error.requested(), Some(4));
    assert_eq!(error.used(), Some(3));
    assert_eq!(*limit.resource(), ConversionResource::Items);
}
