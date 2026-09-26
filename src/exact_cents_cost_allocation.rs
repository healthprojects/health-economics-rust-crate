//! # Exact-Cents Cost Allocation
//!
//! Splitting a total amount of money — a shared grant, an infrastructure
//! bill, a budget-impact total — across several recipients by naive
//! percentage arithmetic routinely produces parts that don't sum back to
//! the original total. This is the "Money" bug class from Martin Fowler's
//! *Patterns of Enterprise Application Architecture* (2002): rounding each
//! recipient's share independently loses (or gains) cents, and the error
//! surfaces only when someone reconciles the parts against the whole.
//!
//! The fix is the **largest remainder method**: give each recipient the
//! whole number of minor units (cents) their share entitles them to,
//! then distribute whatever cents are left over — one each — to the
//! recipients with the largest fractional remainder, so the parts always
//! sum to *exactly* the original total. `rusty-money`'s [`Money::allocate`]
//! implements this directly.
//!
//! [`Money::allocate`]: rusty_money::Money::allocate
//!
//! ## Formula
//!
//! ```text
//! Naive (broken) method:
//!   part_i = round(total * share_i / sum(shares))   -- rounds each part alone
//!
//! Exact method (largest remainder):
//!   1. base_i    = floor(total_minor_units * share_i / sum(shares))
//!   2. remainder = total_minor_units - sum(base_i)
//!   3. give one extra minor unit each to the `remainder` recipients with
//!      the largest fractional remainder from step 1
//!
//! Guarantee: sum(part_i) == total, always, regardless of how unevenly
//! total divides among shares.
//! ```
//!
//! ## Why it matters
//!
//! This is a named, foundational pattern in enterprise software
//! engineering, not a one-off trick — real financial-reconciliation
//! failures have shipped from exactly this class of `f64`-percentage
//! rounding bug. It is also directly relevant to this crate's
//! [`crate::budget_impact_analysis`] and [`crate::total_cost_of_ownership`]
//! modules, both of which currently sum plain `f64` costs: whenever a
//! budget-impact total or a TCO figure has to be split across sites,
//! cohorts, or financial years, the split must reconcile exactly back to
//! the published total, or the whole model loses the finance director's
//! trust.
//!
//! ## Example
//!
//! Split $100.00 three equal ways. Naively, $100.00 ÷ 3 = $33.333…, and
//! rounding each part independently to the nearest cent ($33.33, $33.33,
//! $33.33) sums to only $99.99 — one cent short. `allocate_by_shares`
//! never rounds a part in isolation, so the three parts it returns always
//! sum to exactly $100.00 (which specific recipient gets the extra cent
//! is an internal tie-break detail of [`Money::allocate`], not something a
//! caller should depend on):
//!
//! ```rust
//! use health_economics::exact_cents_cost_allocation::allocate_by_shares;
//! use rusty_money::{iso, Money};
//!
//! let total = Money::from_major(100, iso::USD);
//! let parts = allocate_by_shares(total, &[1, 1, 1]).unwrap();
//!
//! assert_eq!(parts.len(), 3);
//!
//! // The parts always sum to exactly the original total -- the actual
//! // guarantee being demonstrated, regardless of which part got the
//! // leftover cent.
//! let sum = parts[0].add(parts[1]).unwrap().add(parts[2]).unwrap();
//! assert_eq!(sum, total);
//! ```
//!
//! ## Software engineering connection
//!
//! This is literally "the `Money` pattern" — a foundational, named pattern
//! in enterprise software engineering for exactly this bug class. It ties
//! directly into this crate's existing [`crate::budget_impact_analysis`]
//! and [`crate::total_cost_of_ownership`] modules, both of which currently
//! sum plain `f64` costs: whenever one of those totals must be split
//! across recipients rather than merely summed, this is the correct
//! primitive to reach for instead of a hand-rolled percentage split.
//!
//! ## Pitfalls
//!
//! - **Percentage-then-round instead of largest-remainder**: allocating
//!   with `f64` percentages and rounding each recipient independently,
//!   which compounds rounding error and rarely sums back to the total,
//!   especially across many recipients.
//! - **Ignoring currency minor-unit exponents**: assuming every currency
//!   has 2 decimal places — Japanese Yen has 0 — a hand-rolled percentage
//!   split usually hard-codes 2 decimal places and silently breaks for
//!   other currencies; [`Money::allocate`] reads the exponent from the
//!   currency itself (ISO 4217), so it handles this automatically.
//! - **Re-running an allocation on an already-allocated remainder**
//!   without idempotency checks, which can double-credit the same cent to
//!   the same recipient.
//!
//! ## Sources
//!
//! - Fowler M. "Patterns of Enterprise Application Architecture."
//!   Addison-Wesley, 2002 — the `Money` and `Allocate` patterns.
//! - ISO 4217 — currency and funds code standard, which defines each
//!   currency's minor-unit exponent.
//!
//! Topic doc: health-economics-metrics/topics/exact-cents-cost-allocation.md

use rusty_money::{iso, Money, MoneyError};

/// Splits `total` across `shares` so the parts always sum to exactly `total`.
///
/// Uses the largest remainder method (via [`Money::allocate`]): each
/// recipient first receives `floor(total * share / sum(shares))` in minor
/// units, then any leftover minor units are distributed one at a time to
/// the recipients with the largest fractional remainder. The result is a
/// `Vec` of amounts, one per entry in `shares` and in the same order,
/// whose sum equals `total` exactly — regardless of how unevenly `total`
/// divides among `shares`.
///
/// # Arguments
///
/// * `total` — the amount to split.
/// * `shares` — the relative share of each recipient (e.g. `&[1, 1, 1]`
///   for three equal shares, or `&[40, 35, 25]` for a 40/35/25 split).
///
/// # Errors
///
/// Returns the [`MoneyError`] produced by `rusty-money`'s internal
/// allocation if it fails (for example, on arithmetic overflow). It does
/// not return an error merely because `total` doesn't divide evenly among
/// `shares` — that case is exactly what the largest remainder method
/// handles correctly.
///
/// # Examples
///
/// ```rust
/// use health_economics::exact_cents_cost_allocation::allocate_by_shares;
/// use rusty_money::{iso, Money};
///
/// let total = Money::from_major(100, iso::USD);
/// let parts = allocate_by_shares(total, &[1, 1, 1]).unwrap();
///
/// let sum = parts[0].add(parts[1]).unwrap().add(parts[2]).unwrap();
/// assert_eq!(sum, total);
/// ```
pub fn allocate_by_shares(
    total: Money<'static, iso::Currency>,
    shares: &[u32],
) -> Result<Vec<Money<'static, iso::Currency>>, MoneyError> {
    total.allocate(shares.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Worked example: $100.00 split three equal ways sums to exactly
    /// $100.00, even though $100.00 ÷ 3 doesn't divide evenly (the naive
    /// per-part rounding of $33.33 × 3 = $99.99 is one cent short).
    #[test]
    fn splits_100_usd_three_ways_summing_exactly() {
        let total = Money::from_major(100, iso::USD);
        let parts = allocate_by_shares(total, &[1, 1, 1]).unwrap();

        assert_eq!(parts.len(), 3);
        let sum = parts[0].add(parts[1]).unwrap().add(parts[2]).unwrap();
        assert_eq!(sum, total);
    }

    /// An uneven share split (40/35/25) still sums to exactly the total.
    #[test]
    fn splits_by_uneven_shares_summing_exactly() {
        let total = Money::from_major(100, iso::USD);
        let parts = allocate_by_shares(total, &[40, 35, 25]).unwrap();

        assert_eq!(parts.len(), 3);
        let sum = parts[0].add(parts[1]).unwrap().add(parts[2]).unwrap();
        assert_eq!(sum, total);
    }
}
