# Health Economics Rust crate

Health economics models, structs, calculations, and examples — 85 modules
covering the metrics of health technology assessment, healthcare operations,
digital health products, clinical AI evaluation, and their software
engineering analogues. One module per topic.

Almost every function is `f64` in, `f64`/`Option<f64>` out (`None` wherever
a denominator can be zero). Three modules —
[`exact_cents_cost_allocation`](src/exact_cents_cost_allocation.rs),
[`currency_safe_cost_rollup`](src/currency_safe_cost_rollup.rs), and
[`cross_currency_icer_comparison`](src/cross_currency_icer_comparison.rs) —
instead use the [`rusty-money`](https://docs.rs/rusty-money) crate's `Money`
type for exact-decimal currency arithmetic, because summing or allocating
money in binary floating point is a real correctness bug class. That's this
crate's only reason to depend on anything outside `std`. Randomness (for
probabilistic sensitivity analysis) uses a seeded, deterministic generator
so results are reproducible.

## Install

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
health-economics = "0.1"
```

## Quickstart

The canonical chain — health gain, monetized, compared with cost, decided:

```rust
use health_economics::quality_adjusted_life_year as qaly;
use health_economics::incremental_cost_effectiveness_ratio as icer;
use health_economics::net_monetary_benefit as nmb;

// Software removes a 6-month wait (utility 0.6 → 0.85) for 400 patients.
let per_patient = qaly::qaly_loss_from_delay(0.5, 0.6, 0.85);
let total = qaly::population_qalys(per_patient, 400.0);
assert_eq!(total, 50.0);

// At £500k/year the software buys QALYs at £10k — under the £20k threshold.
let ratio = icer::icer(500_000.0, total).unwrap();
assert!(icer::adopt_at_threshold(ratio, 20_000.0));

// Net monetary benefit says the same thing in money: +£500k/year.
let benefit = nmb::net_monetary_benefit(total, 500_000.0, 20_000.0);
assert!(nmb::adopt(benefit));
```

## Learning path

Each module's rustdoc explains its topic — what the metric is, its formula
with a legend, why it matters, and a worked example that runs as a doctest.
Start with `cargo doc --open`.

For the calculation contract independent of the narrative — exact function
signatures, formulas, and `None`/`Err` conditions — see
[`spec/`](spec/README.md). For a machine-readable index of every module and
function, see [`llms.txt`](llms.txt) and [`llms.json`](llms.json). An AI
agent modifying this crate should start with [`AGENTS.md`](AGENTS.md); one
answering a metrics question should start with
[`health-economics-skill/SKILL.md`](health-economics-skill/SKILL.md).

Long-form tutorials live in the `tutorials` module
([`docs/tutorials/`](docs/tutorials/)), in reading order:

1. **From a waiting list to a business case** — the canonical chain: QALYs →
   willingness-to-pay threshold → ICER → net monetary benefit → price.
2. **Building the financial case** — discounted cost-benefit analysis,
   optimism bias, cash-releasing vs economic ROI, break-even horizons, and
   budget impact.
3. **Quantifying uncertainty** — tornado diagrams, probabilistic sensitivity
   analysis, acceptability curves, and the expected value of perfect
   information.
4. **The engineering mirror** — cost of delay, CD3/WSJF sequencing, DORA,
   Little's Law, and technical debt as principal-plus-interest.

Runnable programs cover the same ground in `examples/`:

```sh
cargo run --example qaly_to_decision            # QALYs → ICER → NMB → price
cargo run --example business_case               # Green Book-style financial case
cargo run --example uncertainty_and_information # tornado, PSA, EVPI
cargo run --example screening_and_diagnostics   # why prevalence rules everything
cargo run --example unit_economics              # cloud, inference, LTV/CAC
cargo run --example engineering_economics       # delivery metrics as economics
```

## Module index by theme

### Health outcome measures

- `quality_adjusted_life_year` — QALY: duration × utility, the common currency of HTA
- `eq_5d` — EQ-5D utility instrument
- `disability_adjusted_life_year` — DALY: years of life lost plus years lived with disability
- `life_years_gained` — survival gains without quality weighting
- `health_adjusted_life_expectancy` — HALE
- `patient_reported_outcomes` — PROMs and PREMs
- `number_needed_to_treat` — NNT and NNH from absolute risk differences
- `work_productivity_and_activity_impairment` — WPAI: absenteeism and presenteeism

### Economic evaluation frameworks

- `cost_effectiveness_analysis` — cost per natural unit of outcome
- `cost_utility_analysis` — cost per QALY
- `cost_benefit_analysis` — benefits and costs both in money; NPV and BCR
- `cost_minimization_analysis` — cheapest option given equivalent outcomes
- `cost_consequence_analysis` — disaggregated costs and outcomes
- `budget_impact_analysis` — affordability for the payer
- `social_return_on_investment` — SROI
- `health_technology_assessment` — the HTA process end-to-end
- `human_capital_and_friction_cost` — indirect/productivity cost valuation methods

### Decision rules and thresholds

- `incremental_cost_effectiveness_ratio` — ICER: Δcost / Δeffect
- `net_monetary_benefit` — NMB: effect × threshold − cost
- `willingness_to_pay_thresholds` — NICE and other threshold conventions
- `dominance_and_efficiency_frontier` — strict and extended dominance
- `qaly_shortfall_and_severity_modifiers` — severity weighting of QALYs
- `opportunity_cost` — what the money would otherwise buy
- `analysis_perspective` — payer, health system, societal
- `time_horizon` — how far consequences are counted
- `discounting_and_time_preference` — present values at reference-case rates
- `marginal_vs_average_cost` — which cost belongs in which decision
- `value_of_a_statistical_life` — VSL/VPF: the population revealed-preference alternative to QALYs
- `multi_criteria_decision_analysis` — MCDA weighted scoring beyond a single threshold
- `carbon_footprint_per_qaly` — carbon efficiency alongside cost efficiency
- `cross_currency_icer_comparison` — PPP vs market-FX conversion for international comparisons

### Uncertainty and evidence

- `sensitivity_analysis` — one-way analysis and tornado diagrams
- `probabilistic_sensitivity_analysis` — Monte Carlo over parameter distributions
- `expected_value_of_perfect_information` — EVPI: what resolving uncertainty is worth
- `expected_value_of_sample_information` — EVSI: pricing a specific proposed study
- `benefits_realization` — tracking promised benefits after go-live

### Healthcare operations

- `bed_days_saved`, `length_of_stay`, `readmission_rate`
- `referral_to_treatment`, `waiting_list_impact`, `did_not_attend_rate`
- `emergency_attendance_avoidance`, `practitioner_time`
- `national_tariff_and_unit_costs`, `avoidable_outsourcing_costs`
- `avoided_downstream_costs`, `downstream_resource_optimization`
- `earlier_intervention`, `prevention_economics`, `screening_economics`
- `workforce_retention`, `cash_releasing_vs_non_cash_releasing`
- `hard_cash_releasing_savings_deficit_defense`
- `value_generating_capacity_operational_turnaround`

### Digital health products

- `activation_and_uptake`, `adherence_and_persistence`
- `engagement_metrics`, `retention_and_churn`, `reach_and_equity`
- `health_app_unit_economics`, `remote_patient_monitoring_economics`
- `digital_endpoints_and_biomarkers`, `wearable_validation`
- `diga_fast_track` — Germany's DiGA reimbursement pathway
- `nice_evidence_standards_framework` — NICE ESF for digital health
- `gds_service_metrics` — UK Government Digital Service metrics

### AI evaluation and economics

- `clinical_ai_evaluation` — sensitivity, specificity, PPV, and prevalence effects
- `ai_quality_metrics`, `ai_regulatory_evaluation`
- `ai_return_on_investment`, `ai_developer_productivity`
- `inference_unit_economics` — cost per token, per request, per outcome

### Software engineering economics

- `dora_metrics`, `flow_metrics`, `space_and_devex`, `technical_debt`
- `cost_of_delay`, `wsjf_and_cd3` — sequencing by cost of delay
- `return_on_investment`, `total_cost_of_ownership`, `build_vs_buy`
- `cloud_unit_economics`
- `exact_cents_cost_allocation` — proportional splits that always sum exactly
- `currency_safe_cost_rollup` — exact-decimal summation instead of drifting `f64`

## Testing

Every module reproduces its topic's worked example in unit tests, every
doc example compiles and asserts under `cargo test --doc`, and the
`tests/` directory adds comprehensive integration coverage:

```sh
cargo test
```

## Citation

See [`CITATION.cff`](CITATION.cff) for citation metadata.

## License

Any of MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, or GPL-3.0-only, at
your option — or contact us for custom license options. See
[`LICENSE.md`](LICENSE.md).

## Tracking

- Package: [health-economics](https://crates.io/crates/health-economics)
- Repository: [github.com/healthprojects/health-economics-rust-crate](https://github.com/healthprojects/health-economics-rust-crate)
- Author: [Joel Parker Henderson](https://joelparkerhenderson.com) — joel@joelparkerhenderson.com
