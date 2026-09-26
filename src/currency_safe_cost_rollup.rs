//! # Currency-Safe Cost Rollup
//!
//! Summing many money line items — monthly invoices, per-site costs,
//! multi-year budget-impact figures — with ordinary binary floating-point
//! (`f64`) numbers accumulates small representation errors, because most
//! decimal fractions ($1,234.56, for instance) aren't exactly representable
//! in binary floating point (Goldberg, "What Every Computer Scientist
//! Should Know About Floating-Point Arithmetic," 1991). Each individual
//! error is tiny, but a model summing hundreds of line items over several
//! years can drift by fractions of a cent — and the drift depends on the
//! *order* the additions happen in, which makes it non-reproducible. A
//! rollup done in exact decimal (`Money` backed by [`rust_decimal::Decimal`])
//! sums exactly, matching how accounting systems and double-entry ledgers
//! must reconcile to the cent.
//!
//! ## Formula
//!
//! ```text
//! Naive rollup:          total = sum(f64(line_item_i))         -- order-dependent drift
//! Currency-safe rollup:  total = sum(Decimal(line_item_i))     -- exact, reproducible
//!
//! Applying a percentage adjustment (e.g. a contingency buffer):
//!   adjusted = total * multiplier   -- exact Decimal result, may carry MORE
//!                                       decimal places than the currency's
//!                                       minor-unit exponent
//!   rounded  = round(adjusted, currency_exponent, rounding_rule)
//!                                    -- the rounding rule (half-up vs
//!                                       half-even/banker's rounding) must
//!                                       be a stated, auditable convention
//! ```
//!
//! ## Why it matters
//!
//! This crate's [`crate::total_cost_of_ownership`],
//! [`crate::cloud_unit_economics`], and [`crate::budget_impact_analysis`]
//! modules all currently sum plain `f64` costs. This topic doesn't ask you
//! to migrate them — that is out of scope here — it states the correctness
//! argument for *when* a real system must reconcile to the cent: an audit
//! that recomputes the total by hand must get the identical figure, and
//! only exact decimal arithmetic guarantees that.
//!
//! ## Example
//!
//! Twelve identical monthly invoices of $1,234.56 each sum to exactly
//! $14,814.72 — contrast this with summing the `f64` literal `1234.56`
//! twelve times in IEEE-754 double precision, which can drift by fractions
//! of a cent depending on summation order, a real and well-documented
//! class of bug that exact `Decimal` arithmetic avoids entirely. Applying a
//! 5% contingency buffer gives an *exact* intermediate result of
//! $15,555.456 (three decimal places — `Money`'s stored precision isn't
//! automatically rounded to the currency's exponent), which must then be
//! rounded explicitly, with a stated rounding rule, to $15,555.46:
//!
//! ```rust
//! use health_economics::currency_safe_cost_rollup::{apply_multiplier, sum_line_items};
//! use rusty_money::{iso, Money, Round};
//! use rust_decimal::Decimal;
//!
//! let invoice = Money::from_minor(123_456, iso::USD); // $1,234.56
//! let items = vec![invoice; 12];
//!
//! let total = sum_line_items(&items).unwrap();
//! assert_eq!(total, Money::from_minor(1_481_472, iso::USD)); // $14,814.72 exactly
//!
//! // A standard 5% budget-impact contingency buffer.
//! let adjusted = apply_multiplier(total, Decimal::new(105, 2)).unwrap();
//!
//! // The exact Decimal product carries 3 decimal places ($15,555.456);
//! // only an explicit, stated rounding rule takes it to the currency's
//! // 2 decimal places.
//! let rounded = adjusted.round(2, Round::HalfEven);
//! assert_eq!(rounded, Money::from_minor(1_555_546, iso::USD)); // $15,555.46
//! ```
//!
//! ## Software engineering connection
//!
//! This is the direct, foundational "why financial software uses `Decimal`,
//! not `float`" lesson. It connects explicitly to this crate's existing
//! [`crate::total_cost_of_ownership`], [`crate::cloud_unit_economics`], and
//! [`crate::budget_impact_analysis`] modules, all of which currently sum
//! plain `f64` costs — the argument here is about correctness under
//! reconciliation, not a call to migrate those modules immediately.
//!
//! ## Pitfalls
//!
//! - **Converting a `Money` to `f64` partway through a calculation chain**
//!   — `rusty-money`'s own method for this is literally named
//!   `to_f64_lossy`, an explicit warning label in its own name — silently
//!   loses the exactness guarantee for the rest of the chain.
//! - **Assuming `Decimal` arithmetic is "too slow to bother with"** when
//!   correctness and auditability matter more than raw performance for
//!   financial reporting.
//! - **Applying a contingency percentage without stating the rounding
//!   rule used** (half-up vs half-even/banker's rounding) — the rounding
//!   rule itself must be a stated, auditable convention, not an implicit
//!   default.
//!
//! ## Sources
//!
//! - Fowler M. "Patterns of Enterprise Application Architecture."
//!   Addison-Wesley, 2002 — the `Money` pattern.
//! - Goldberg D. "What Every Computer Scientist Should Know About
//!   Floating-Point Arithmetic." ACM Computing Surveys, 1991.
//! - HM Treasury, The Green Book — optimism-bias/contingency guidance for
//!   budget-impact modelling.
//!
//! Topic doc: health-economics-metrics/topics/currency-safe-cost-rollup.md

use rust_decimal::Decimal;
use rusty_money::{iso, Money, MoneyError};

/// Sums a slice of money line items into a single exact total.
///
/// Folds `items` with [`Money::add`], so the result is exact regardless of
/// how many items there are or what order they're summed in — unlike
/// summing the equivalent `f64` values, whose result can vary with
/// summation order.
///
/// # Arguments
///
/// * `items` — the line items to sum, all in the same currency.
///
/// # Errors
///
/// Returns [`MoneyError::InvalidAmount`] if `items` is empty (there is no
/// well-defined total, and no currency to report one in). Also propagates
/// any [`MoneyError`] that `add` returns while folding — most notably
/// `CurrencyMismatch` if the items aren't all in the same currency, or
/// `Overflow` on arithmetic overflow.
///
/// # Examples
///
/// ```rust
/// use health_economics::currency_safe_cost_rollup::sum_line_items;
/// use rusty_money::{iso, Money};
///
/// let items = vec![Money::from_minor(123_456, iso::USD); 12]; // $1,234.56 x 12
/// let total = sum_line_items(&items).unwrap();
/// assert_eq!(total, Money::from_minor(1_481_472, iso::USD)); // $14,814.72 exactly
/// ```
pub fn sum_line_items(
    items: &[Money<'static, iso::Currency>],
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    let (first, rest) = items.split_first().ok_or(MoneyError::InvalidAmount)?;
    rest.iter().try_fold(*first, |running_total, item| running_total.add(*item))
}

/// Applies a multiplier (e.g. a contingency buffer) to a total, exactly.
///
/// The result is an exact [`rust_decimal::Decimal`] product and may carry
/// more decimal places than the currency's minor-unit exponent — call
/// [`Money::round`] explicitly, with a stated rounding rule, before
/// treating the result as a currency amount ready to pay or display.
///
/// # Arguments
///
/// * `total` — the amount to multiply.
/// * `multiplier` — the multiplier to apply (e.g. `Decimal::new(105, 2)`
///   for a 5% contingency buffer, i.e. ×1.05).
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows.
///
/// # Examples
///
/// ```rust
/// use health_economics::currency_safe_cost_rollup::apply_multiplier;
/// use rusty_money::{iso, Money, Round};
/// use rust_decimal::Decimal;
///
/// let total = Money::from_minor(1_481_472, iso::USD); // $14,814.72
/// let adjusted = apply_multiplier(total, Decimal::new(105, 2)).unwrap();
/// let rounded = adjusted.round(2, Round::HalfEven);
/// assert_eq!(rounded, Money::from_minor(1_555_546, iso::USD)); // $15,555.46
/// ```
pub fn apply_multiplier(
    total: Money<'static, iso::Currency>,
    multiplier: Decimal,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    total.mul(multiplier)
}

/// Exact variance between an actual amount and a budgeted amount.
///
/// Positive means over budget, negative means under budget — exactly, to
/// the minor unit, with none of the drift a plain `f64` subtraction of two
/// large rolled-up totals could introduce.
///
/// # Arguments
///
/// * `actual` — the amount actually spent.
/// * `budgeted` — the amount that was budgeted.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if `actual` and `budgeted` are
/// in different currencies.
///
/// # Examples
///
/// ```rust
/// use health_economics::currency_safe_cost_rollup::budget_variance;
/// use rusty_money::{iso, Money};
///
/// let actual = Money::from_minor(1_555_546, iso::USD); // $15,555.46
/// let budgeted = Money::from_minor(1_481_472, iso::USD); // $14,814.72
/// let variance = budget_variance(actual, budgeted).unwrap();
/// assert_eq!(variance, Money::from_minor(74_074, iso::USD)); // $740.74 over budget
/// ```
pub fn budget_variance(
    actual: Money<'static, iso::Currency>,
    budgeted: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    actual.sub(budgeted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_money::Round;

    /// Worked example: 12 invoices of $1,234.56 sum to exactly $14,814.72.
    #[test]
    fn sums_twelve_monthly_invoices_exactly() {
        let items = vec![Money::from_minor(123_456, iso::USD); 12];
        let total = sum_line_items(&items).unwrap();
        assert_eq!(total, Money::from_minor(1_481_472, iso::USD));
    }

    /// An empty slice has no well-defined total.
    #[test]
    fn empty_slice_is_an_error() {
        let items: Vec<Money<'static, iso::Currency>> = vec![];
        assert_eq!(sum_line_items(&items), Err(MoneyError::InvalidAmount));
    }

    /// Worked example: a 5% contingency buffer on $14,814.72 is an exact
    /// $15,555.456, which rounds (half-even) to $15,555.46.
    #[test]
    fn applies_five_percent_contingency_and_rounds_half_even() {
        let total = Money::from_minor(1_481_472, iso::USD);
        let adjusted = apply_multiplier(total, Decimal::new(105, 2)).unwrap();
        let rounded = adjusted.round(2, Round::HalfEven);
        assert_eq!(rounded, Money::from_minor(1_555_546, iso::USD));
    }

    /// Worked example: $15,555.46 actual against a $14,814.72 budget is
    /// exactly $740.74 over budget.
    #[test]
    fn variance_between_escalated_and_original_total() {
        let actual = Money::from_minor(1_555_546, iso::USD);
        let budgeted = Money::from_minor(1_481_472, iso::USD);
        let variance = budget_variance(actual, budgeted).unwrap();
        assert_eq!(variance, Money::from_minor(74_074, iso::USD));
    }

    /// Comparing amounts in different currencies is a caught error, not a
    /// silent miscalculation.
    #[test]
    fn mismatched_currencies_is_an_error() {
        let actual = Money::from_major(100, iso::USD);
        let budgeted = Money::from_major(100, iso::GBP);
        assert!(matches!(
            budget_variance(actual, budgeted),
            Err(MoneyError::CurrencyMismatch { .. })
        ));
    }
}
