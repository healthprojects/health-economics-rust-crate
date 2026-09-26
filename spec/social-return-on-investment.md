# Spec: Social Return on Investment (SROI)

- **Module**: [`src/social_return_on_investment.rs`](../src/social_return_on_investment.rs)
- **Status**: implemented
- **Upstream topic**: health-economics-metrics/topics/social-return-on-investment.md

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `SocialOutcome`

One monetized social outcome with its SROI adjustment factors. All factor
fields are fractions in 0..1; a payer-real outcome with no adjustments sets
`attribution` to 1.0 and `deadweight`/`displacement` to 0.0.

- `quantity: f64` — number of people (or units) experiencing the outcome.
- `financial_proxy: f64` — monetary value per unit of outcome; source should
  be stated, not shopped.
- `attribution: f64` — share of the outcome caused by this intervention, 0..1.
- `deadweight: f64` — fraction of the outcome that would have happened
  anyway, 0..1.
- `displacement: f64` — fraction merely moved from elsewhere rather than
  created, 0..1.

### `SocialOutcome::value(&self) -> f64`

- Formula: `quantity * financial_proxy * attribution * (1 - deadweight) * (1 - displacement)`
- Total function: never returns `None`.
- Worked example: `SocialOutcome { quantity: 1_500.0, financial_proxy: 1_800.0, attribution: 0.80, deadweight: 0.25, displacement: 0.0 }.value() == 1_620_000.0`

### `total_outcome_value(outcomes: &[SocialOutcome]) -> f64`

- Formula: `Σ SocialOutcome::value() over outcomes`
- Total function: never returns `None`.
- Worked example: for the loneliness outcome (value £1,620,000) and a GP
  outcome (quantity 1,800.0, proxy 42.0, attribution 1.0, no deadweight/
  displacement, value £75,600), `total_outcome_value(&[loneliness, gp]) == 1_695_600.0`

### `sroi_ratio(pv_outcomes: f64, pv_investment: f64) -> Option<f64>`

- Formula: `pv_outcomes / pv_investment`
- Returns `None` iff `pv_investment == 0.0`
- Worked example: `sroi_ratio(1_620_000.0 + 75_600.0, 200_000.0) ≈ Some(8.5)` (tolerance 0.05)

### `value_after_drop_off(initial_value: f64, drop_off_rate: f64, years: u32) -> f64`

- Formula: `initial_value * (1 - drop_off_rate)^years` (geometric decay; year 0
  returns the value unchanged)
- Total function: never returns `None`.
- Worked example: `value_after_drop_off(1_000.0, 0.10, 2) == 810.0`

### `proxy_valued_share(proxy_valued: f64, payer_real: f64) -> Option<f64>`

- Formula: `proxy_valued / (proxy_valued + payer_real)`
- Returns `None` iff `proxy_valued + payer_real == 0.0`
- Worked example: `proxy_valued_share(1_620_000.0, 75_600.0) ≈ Some(0.96)` (tolerance 0.005)

## Invariants

- The SROI ratio is quoted as "N : 1" of monetized social value per unit
  invested; the module's rustdoc stresses that the 8.5:1 worked-example ratio
  is 96% proxy-valued (soft) and only 4% payer-real cash — `proxy_valued_share`
  is the honesty check that must accompany any SROI headline, never
  presenting the ratio as bankable cash.
