# Spec: Health Technology Assessment (HTA)

- **Module**: [`src/health_technology_assessment.rs`](../src/health_technology_assessment.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/health-technology-assessment.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `icer(delta_cost: f64, delta_effect: f64) -> Option<f64>`

- Formula: `delta_cost / delta_effect`
- Returns `None` iff `delta_effect == 0.0` (undefined at zero incremental
  effect)
- Worked example: `icer(450.0, 0.03) == Some(15_000.0)`

### `meets_threshold(icer_value: f64, threshold: f64) -> bool`

- Formula: `icer_value < threshold` (strict comparison; an ICER exactly
  equal to the threshold does not clear it)
- Total function: never returns `None`.
- Worked example: `meets_threshold(15_000.0, 20_000.0) == true`;
  `meets_threshold(36_000.0, 20_000.0) == false`

### `probability_cost_effective(psa_draws: &[(f64, f64)], lambda: f64) -> Option<f64>`

- Formula: fraction of `psa_draws` (each a `(ΔC, ΔE)` pair) for which
  `ΔE × lambda − ΔC > 0` (net monetary benefit strictly positive; equivalent
  to `ICER < lambda` but avoids division so `ΔE` near zero cannot blow up
  the statistic)
- Returns `None` iff `psa_draws` is empty
- Worked example: 71 draws of `(450.0, 0.03)` (NMB > 0) and 29 draws of
  `(450.0, 0.01)` (NMB < 0) at `lambda = 20_000.0` →
  `probability_cost_effective(&draws, 20_000.0) == Some(0.71)`

### `ReferenceCaseChecklist`

Reference-case conformance checklist for a NICE-style submission. Each
field is one of the mandated method choices the reference case removes from
the sponsor's discretion.

- `utilities_from_mandated_instrument: bool` — utilities measured with the
  mandated instrument (e.g. EQ-5D-5L with the UK value set), not a
  sponsor-chosen alternative.
- `comparator_is_current_care_pathway: bool` — comparator is the current
  care pathway, not "no treatment" or a strawman baseline.
- `psa_reported: bool` — probabilistic sensitivity analysis performed and
  reported.

### `ReferenceCaseChecklist::passes(&self) -> bool`

- Formula: `utilities_from_mandated_instrument && comparator_is_current_care_pathway && psa_reported`
- Total function: never returns `None`. Any single `false` fails the whole
  checklist.
- Worked example: a checklist with all three fields `true` →
  `.passes() == true`; with `comparator_is_current_care_pathway: false` →
  `.passes() == false`

### `recommend_routine_commissioning(checklist: &ReferenceCaseChecklist, icer_value: f64, threshold: f64) -> bool`

- Formula: `checklist.passes() && meets_threshold(icer_value, threshold)`
- Total function: never returns `None`. A flattering ICER computed off the
  reference case is rejected regardless of its value.
- Worked example: conformant checklist, `icer_value = 15_000.0`,
  `threshold = 20_000.0` → `true`; a strawman-comparator checklist with
  `icer_value = 9_000.0` → `false`

## Invariants

- `recommend_routine_commissioning` conjoins `ReferenceCaseChecklist::passes`
  with `meets_threshold`'s strict `<` comparison — a favorable ICER alone
  never overrides a failed reference-case check, per the module's "mandated
  method" framing.
