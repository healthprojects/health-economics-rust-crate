# Spec: Return on Investment (ROI)

- **Module**: [`src/return_on_investment.rs`](../src/return_on_investment.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/return-on-investment.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `BenefitClass`

Classification of a benefit line: `CashReleasing` (cash the budget holder can
bank), `Capacity` (freed and valued in money, but not banked), or
`Qualitative` (real but not monetized).

### `BenefitLine`

- `class: BenefitClass` — the line's classification.
- `amount: f64` — monetized value of the benefit in currency units (0.0 if
  not monetized, e.g. a qualitative line kept visible without inflating any
  ratio).

### `roi(benefits: f64, costs: f64) -> Option<f64>`

- Formula: `(benefits - costs) / costs`
- Returns `None` iff `costs == 0.0`
- Worked example: `roi(3_000_000.0, 2_000_000.0) == Some(0.50)`

### `payback_period_years(costs: f64, annual_net_benefit: f64) -> Option<f64>`

- Formula: `costs / annual_net_benefit`
- Returns `None` iff `annual_net_benefit == 0.0` (a negative annual net
  benefit yields a negative, meaningless period rather than `None`)
- Worked example: `payback_period_years(500_000.0, 250_000.0) == Some(2.0)`

### `total_benefits(lines: &[BenefitLine], include: &[BenefitClass]) -> f64`

- Formula: `Σ amount over lines whose class is in include`
- Total function: never returns `None`.
- Worked example: `total_benefits(&[BenefitLine { class: BenefitClass::CashReleasing, amount: 450_000.0 }, BenefitLine { class: BenefitClass::Capacity, amount: 600_000.0 }], &[BenefitClass::CashReleasing]) == 450_000.0`

### `strict_financial_roi(lines: &[BenefitLine], costs: f64) -> Option<f64>`

- Formula: `roi(total_benefits(lines, &[BenefitClass::CashReleasing]), costs)`
- Returns `None` iff `costs == 0.0`
- Worked example: for lines `[CashReleasing 450_000.0, Capacity 600_000.0]`,
  `strict_financial_roi(&lines, 500_000.0) == Some(-0.10)`

### `economic_roi(lines: &[BenefitLine], costs: f64) -> Option<f64>`

- Formula: `roi(total_benefits(lines, &[BenefitClass::CashReleasing, BenefitClass::Capacity]), costs)`
- Returns `None` iff `costs == 0.0`
- Worked example: for lines `[CashReleasing 450_000.0, Capacity 600_000.0]`,
  `economic_roi(&lines, 500_000.0) == Some(1.10)`

## Invariants

- `strict_financial_roi(lines, costs) <= economic_roi(lines, costs)` whenever
  capacity-line amounts are non-negative, since `economic_roi` sums a
  superset of the benefit lines `strict_financial_roi` sums.
- Qualitative lines never enter either ROI: only `CashReleasing` feeds
  `strict_financial_roi`, and only `CashReleasing` plus `Capacity` feed
  `economic_roi`.
