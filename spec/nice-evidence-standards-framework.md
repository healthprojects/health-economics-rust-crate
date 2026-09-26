# Spec: NICE Evidence Standards Framework (ESF)

- **Module**: [`src/nice_evidence_standards_framework.rs`](../src/nice_evidence_standards_framework.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/nice-evidence-standards-framework.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `EsfTier`

Enum (`A`, `B`, `C`), with a derived `Ord` reflecting the escalation
(`A < B < C`): standards are cumulative, each tier adding to the ones below.

### `ClinicalFunction`

Enum (`SystemService`, `InformOrMonitor`, `TreatDiagnoseOrGuide`): the
clinical function a digital health technology performs, the axis on which
the ESF classifies it.

### `classify_tier(function: ClinicalFunction) -> EsfTier`

- Formula: direct mapping — `SystemService → A`, `InformOrMonitor → B`,
  `TreatDiagnoseOrGuide → C`.
- Total function: never returns `None`.
- Worked example: `classify_tier(ClinicalFunction::InformOrMonitor) == EsfTier::B`;
  `classify_tier(ClinicalFunction::TreatDiagnoseOrGuide) == EsfTier::C`

### `evidence_investment_range(tier: EsfTier) -> (f64, f64)`

- Formula: fixed lookup table — `A → (10_000.0, 50_000.0)`,
  `B → (50_000.0, 250_000.0)`, `C → (250_000.0, 2_000_000.0)` (Tier C's real
  upper bound is open-ended, "£2M+"; the returned high is its £2M anchor).
- Total function: never returns `None`.
- Worked example: `evidence_investment_range(EsfTier::C) == (250_000.0, 2_000_000.0)`,
  and a £600k RCT sits inside that range.

### `years_to_recoup_evidence_cost(evidence_cost: f64, incremental_annual_revenue: f64) -> Option<f64>`

- Formula: `evidence_cost / incremental_annual_revenue`
- Returns `None` iff `incremental_annual_revenue == 0.0`
- Worked example: `years_to_recoup_evidence_cost(600_000.0, 200_000.0) == Some(3.0)`
