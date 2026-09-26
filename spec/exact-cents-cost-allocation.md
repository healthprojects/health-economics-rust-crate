# Spec: Exact-Cents Cost Allocation

- **Module**: [`src/exact_cents_cost_allocation.rs`](../src/exact_cents_cost_allocation.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/exact-cents-cost-allocation.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `allocate_by_shares(total: Money<'static, iso::Currency>, shares: &[u32]) -> Result<Vec<Money<'static, iso::Currency>>, MoneyError>`

- Formula (largest remainder method, delegated to `rusty-money`'s
  [`Money::allocate`]): each recipient first receives
  `floor(total_minor_units × share / sum(shares))` minor units, then any
  leftover minor units are distributed one at a time to the recipients with
  the largest fractional remainder. Guarantee: `sum(parts) == total`, always,
  regardless of how unevenly `total` divides among `shares`.
- Returns `Err(MoneyError)` iff `rusty-money`'s internal allocation fails
  (for example, on arithmetic overflow). It does **not** return an error
  merely because `total` doesn't divide evenly among `shares` — that case is
  exactly what the largest remainder method handles correctly.
- Worked example: `allocate_by_shares(Money::from_major(100, iso::USD), &[1, 1, 1])`
  returns 3 parts whose sum equals `Money::from_major(100, iso::USD)` exactly
  (even though $100.00 ÷ 3 does not divide evenly); which specific recipient
  receives the leftover cent is an internal tie-break detail of
  `Money::allocate`, not part of the contract.
