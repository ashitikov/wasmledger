use crate::bindings::wasmledger::money::types::Money;
use crate::{MoneyCore, bindings};

fn assert_money_compatible(lhs: &Money, rhs: &Money) {
    if lhs.currency != rhs.currency {
        panic!("Currency mismatch: {} vs {}", lhs.currency, rhs.currency)
    }

    if lhs.precision != rhs.precision {
        panic!("Precision mismatch: {} vs {}", lhs.precision, rhs.precision)
    }
}

impl bindings::exports::wasmledger::money::core::Guest for MoneyCore {
    fn allocate(allocatable: Money, ratios: Vec<u8>) -> Vec<Money> {
        assert!(!ratios.is_empty(), "ratios must not be empty");

        let total_ratio: i128 = ratios.iter().map(|&r| r as i128).sum();

        assert!(total_ratio > 0, "sum of ratios must be > 0");

        let total_value: i128 = allocatable.value as i128;
        let mut result = Vec::with_capacity(ratios.len());
        let mut allocated_total: i64 = 0;

        for &ratio in ratios.iter() {
            let value = (total_value * (ratio as i128) / total_ratio) as i64;
            result.push(Money {
                value,
                precision: allocatable.precision,
                currency: allocatable.currency.clone(),
            });
            allocated_total += value;
        }

        let mut remainder = total_value - (allocated_total as i128);
        let mut i = 0;
        while remainder > 0 {
            result[i].value += 1;
            remainder -= 1;
            i += 1;

            if i >= result.len() {
                i = 0;
            }
        }

        result
    }

    fn add(lhs: Money, rhs: Money) -> Money {
        assert_money_compatible(&lhs, &rhs);

        Money {
            value: lhs
                .value
                .checked_add(rhs.value)
                .expect("Overflow in Money::add"),
            precision: lhs.precision,
            currency: lhs.currency,
        }
    }

    fn subtract(lhs: Money, rhs: Money) -> Money {
        assert_money_compatible(&lhs, &rhs);

        Money {
            value: lhs
                .value
                .checked_sub(rhs.value)
                .expect("Overflow in Money::subtract"),
            precision: lhs.precision,
            currency: lhs.currency,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn m(v: i64) -> Money {
        Money {
            value: v,
            precision: 2,
            currency: "USD".into(),
        }
    }

    #[inline]
    fn allocate(a: Money, ratios: Vec<u8>) -> Vec<Money> {
        <MoneyCore as bindings::exports::wasmledger::money::core::Guest>::allocate(a, ratios)
    }

    #[test]
    fn test_allocate_equal_ratios() {
        let money = m(10000); // 100.00
        let parts = allocate(money, vec![1, 1, 1]);

        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].value, 3334);
        assert_eq!(parts[1].value, 3333);
        assert_eq!(parts[2].value, 3333);
        assert_eq!(parts.iter().map(|m| m.value).sum::<i64>(), 10000);
    }

    #[test]
    fn test_allocate_uneven_ratios() {
        let money = m(10000); // 100.00
        let parts = allocate(money, vec![3, 2, 1]); // 3/6, 2/6, 1/6

        assert_eq!(parts.len(), 3);
        // approx:
        // 10000 * 3/6 = 5000
        // 10000 * 2/6 = 3333.33 → 3333
        // 10000 * 1/6 = 1666.66 → 1666
        // remainder = 1 → goes to first
        assert_eq!(parts[0].value, 5001);
        assert_eq!(parts[1].value, 3333);
        assert_eq!(parts[2].value, 1666);
        assert_eq!(parts.iter().map(|m| m.value).sum::<i64>(), 10000);
    }

    #[test]
    fn test_large_numbers() {
        let money = m(9_000_000_000); // 90 million units
        let parts = allocate(money, vec![1, 1, 1, 1]);

        assert_eq!(parts.len(), 4);
        // Should divide evenly: 2250000000 each
        assert_eq!(parts.iter().map(|m| m.value).sum::<i64>(), 9_000_000_000);
    }

    #[test]
    fn test_single_ratio() {
        let money = m(12345);
        let parts = allocate(money, vec![10]);

        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].value, 12345);
    }

    #[test]
    fn test_remainder_distribution_order() {
        let money = m(5);
        let parts = allocate(money, vec![1, 1, 1]);

        // floor:
        // 5 * 1/3 = 1
        // 5 * 1/3 = 1
        // 5 * 1/3 = 1
        // remainder = 2 → goes to [0], [1]
        assert_eq!(parts[0].value, 2);
        assert_eq!(parts[1].value, 2);
        assert_eq!(parts[2].value, 1);
        assert_eq!(parts.iter().map(|m| m.value).sum::<i64>(), 5);
    }

    #[test]
    #[should_panic(expected = "ratios must not be empty")]
    fn test_empty_ratios_panics() {
        allocate(m(100), vec![]);
    }

    #[test]
    #[should_panic(expected = "sum of ratios must be > 0")]
    fn test_zero_sum_ratios_panics() {
        allocate(m(100), vec![0, 0, 0]);
    }
}
