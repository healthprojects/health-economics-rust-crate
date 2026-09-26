# Specifications

This directory is the single source of truth for each module's **calculation
contract**: its function signatures, formulas, and the conditions under
which a fallible function returns `None` (or, for the three `Money`-based
modules, `Err`). It is deliberately narrow — narrative content (why a
metric matters, its pitfalls, its data sources) belongs in the module's
rustdoc, not here, so that content is written once and doesn't drift.

## Files

85 spec files, one per `src/` module:

| Spec | Module |
| --- | --- |
| [`activation-and-uptake.md`](activation-and-uptake.md) | [`src/activation_and_uptake.rs`](../src/activation_and_uptake.rs) |
| [`adherence-and-persistence.md`](adherence-and-persistence.md) | [`src/adherence_and_persistence.rs`](../src/adherence_and_persistence.rs) |
| [`ai-developer-productivity.md`](ai-developer-productivity.md) | [`src/ai_developer_productivity.rs`](../src/ai_developer_productivity.rs) |
| [`ai-quality-metrics.md`](ai-quality-metrics.md) | [`src/ai_quality_metrics.rs`](../src/ai_quality_metrics.rs) |
| [`ai-regulatory-evaluation.md`](ai-regulatory-evaluation.md) | [`src/ai_regulatory_evaluation.rs`](../src/ai_regulatory_evaluation.rs) |
| [`ai-return-on-investment.md`](ai-return-on-investment.md) | [`src/ai_return_on_investment.rs`](../src/ai_return_on_investment.rs) |
| [`analysis-perspective.md`](analysis-perspective.md) | [`src/analysis_perspective.rs`](../src/analysis_perspective.rs) |
| [`avoidable-outsourcing-costs.md`](avoidable-outsourcing-costs.md) | [`src/avoidable_outsourcing_costs.rs`](../src/avoidable_outsourcing_costs.rs) |
| [`avoided-downstream-costs.md`](avoided-downstream-costs.md) | [`src/avoided_downstream_costs.rs`](../src/avoided_downstream_costs.rs) |
| [`bed-days-saved.md`](bed-days-saved.md) | [`src/bed_days_saved.rs`](../src/bed_days_saved.rs) |
| [`benefits-realization.md`](benefits-realization.md) | [`src/benefits_realization.rs`](../src/benefits_realization.rs) |
| [`budget-impact-analysis.md`](budget-impact-analysis.md) | [`src/budget_impact_analysis.rs`](../src/budget_impact_analysis.rs) |
| [`build-vs-buy.md`](build-vs-buy.md) | [`src/build_vs_buy.rs`](../src/build_vs_buy.rs) |
| [`carbon-footprint-per-qaly.md`](carbon-footprint-per-qaly.md) | [`src/carbon_footprint_per_qaly.rs`](../src/carbon_footprint_per_qaly.rs) |
| [`cash-releasing-vs-non-cash-releasing.md`](cash-releasing-vs-non-cash-releasing.md) | [`src/cash_releasing_vs_non_cash_releasing.rs`](../src/cash_releasing_vs_non_cash_releasing.rs) |
| [`clinical-ai-evaluation.md`](clinical-ai-evaluation.md) | [`src/clinical_ai_evaluation.rs`](../src/clinical_ai_evaluation.rs) |
| [`cloud-unit-economics.md`](cloud-unit-economics.md) | [`src/cloud_unit_economics.rs`](../src/cloud_unit_economics.rs) |
| [`cost-benefit-analysis.md`](cost-benefit-analysis.md) | [`src/cost_benefit_analysis.rs`](../src/cost_benefit_analysis.rs) |
| [`cost-consequence-analysis.md`](cost-consequence-analysis.md) | [`src/cost_consequence_analysis.rs`](../src/cost_consequence_analysis.rs) |
| [`cost-effectiveness-analysis.md`](cost-effectiveness-analysis.md) | [`src/cost_effectiveness_analysis.rs`](../src/cost_effectiveness_analysis.rs) |
| [`cost-minimization-analysis.md`](cost-minimization-analysis.md) | [`src/cost_minimization_analysis.rs`](../src/cost_minimization_analysis.rs) |
| [`cost-of-delay.md`](cost-of-delay.md) | [`src/cost_of_delay.rs`](../src/cost_of_delay.rs) |
| [`cost-utility-analysis.md`](cost-utility-analysis.md) | [`src/cost_utility_analysis.rs`](../src/cost_utility_analysis.rs) |
| [`cross-currency-icer-comparison.md`](cross-currency-icer-comparison.md) | [`src/cross_currency_icer_comparison.rs`](../src/cross_currency_icer_comparison.rs) |
| [`currency-safe-cost-rollup.md`](currency-safe-cost-rollup.md) | [`src/currency_safe_cost_rollup.rs`](../src/currency_safe_cost_rollup.rs) |
| [`did-not-attend-rate.md`](did-not-attend-rate.md) | [`src/did_not_attend_rate.rs`](../src/did_not_attend_rate.rs) |
| [`diga-fast-track.md`](diga-fast-track.md) | [`src/diga_fast_track.rs`](../src/diga_fast_track.rs) |
| [`digital-endpoints-and-biomarkers.md`](digital-endpoints-and-biomarkers.md) | [`src/digital_endpoints_and_biomarkers.rs`](../src/digital_endpoints_and_biomarkers.rs) |
| [`disability-adjusted-life-year.md`](disability-adjusted-life-year.md) | [`src/disability_adjusted_life_year.rs`](../src/disability_adjusted_life_year.rs) |
| [`discounting-and-time-preference.md`](discounting-and-time-preference.md) | [`src/discounting_and_time_preference.rs`](../src/discounting_and_time_preference.rs) |
| [`dominance-and-efficiency-frontier.md`](dominance-and-efficiency-frontier.md) | [`src/dominance_and_efficiency_frontier.rs`](../src/dominance_and_efficiency_frontier.rs) |
| [`dora-metrics.md`](dora-metrics.md) | [`src/dora_metrics.rs`](../src/dora_metrics.rs) |
| [`downstream-resource-optimization.md`](downstream-resource-optimization.md) | [`src/downstream_resource_optimization.rs`](../src/downstream_resource_optimization.rs) |
| [`earlier-intervention.md`](earlier-intervention.md) | [`src/earlier_intervention.rs`](../src/earlier_intervention.rs) |
| [`emergency-attendance-avoidance.md`](emergency-attendance-avoidance.md) | [`src/emergency_attendance_avoidance.rs`](../src/emergency_attendance_avoidance.rs) |
| [`engagement-metrics.md`](engagement-metrics.md) | [`src/engagement_metrics.rs`](../src/engagement_metrics.rs) |
| [`eq-5d.md`](eq-5d.md) | [`src/eq_5d.rs`](../src/eq_5d.rs) |
| [`exact-cents-cost-allocation.md`](exact-cents-cost-allocation.md) | [`src/exact_cents_cost_allocation.rs`](../src/exact_cents_cost_allocation.rs) |
| [`expected-value-of-perfect-information.md`](expected-value-of-perfect-information.md) | [`src/expected_value_of_perfect_information.rs`](../src/expected_value_of_perfect_information.rs) |
| [`expected-value-of-sample-information.md`](expected-value-of-sample-information.md) | [`src/expected_value_of_sample_information.rs`](../src/expected_value_of_sample_information.rs) |
| [`flow-metrics.md`](flow-metrics.md) | [`src/flow_metrics.rs`](../src/flow_metrics.rs) |
| [`gds-service-metrics.md`](gds-service-metrics.md) | [`src/gds_service_metrics.rs`](../src/gds_service_metrics.rs) |
| [`hard-cash-releasing-savings-deficit-defense.md`](hard-cash-releasing-savings-deficit-defense.md) | [`src/hard_cash_releasing_savings_deficit_defense.rs`](../src/hard_cash_releasing_savings_deficit_defense.rs) |
| [`health-adjusted-life-expectancy.md`](health-adjusted-life-expectancy.md) | [`src/health_adjusted_life_expectancy.rs`](../src/health_adjusted_life_expectancy.rs) |
| [`health-app-unit-economics.md`](health-app-unit-economics.md) | [`src/health_app_unit_economics.rs`](../src/health_app_unit_economics.rs) |
| [`health-technology-assessment.md`](health-technology-assessment.md) | [`src/health_technology_assessment.rs`](../src/health_technology_assessment.rs) |
| [`human-capital-and-friction-cost.md`](human-capital-and-friction-cost.md) | [`src/human_capital_and_friction_cost.rs`](../src/human_capital_and_friction_cost.rs) |
| [`incremental-cost-effectiveness-ratio.md`](incremental-cost-effectiveness-ratio.md) | [`src/incremental_cost_effectiveness_ratio.rs`](../src/incremental_cost_effectiveness_ratio.rs) |
| [`inference-unit-economics.md`](inference-unit-economics.md) | [`src/inference_unit_economics.rs`](../src/inference_unit_economics.rs) |
| [`length-of-stay.md`](length-of-stay.md) | [`src/length_of_stay.rs`](../src/length_of_stay.rs) |
| [`life-years-gained.md`](life-years-gained.md) | [`src/life_years_gained.rs`](../src/life_years_gained.rs) |
| [`marginal-vs-average-cost.md`](marginal-vs-average-cost.md) | [`src/marginal_vs_average_cost.rs`](../src/marginal_vs_average_cost.rs) |
| [`multi-criteria-decision-analysis.md`](multi-criteria-decision-analysis.md) | [`src/multi_criteria_decision_analysis.rs`](../src/multi_criteria_decision_analysis.rs) |
| [`national-tariff-and-unit-costs.md`](national-tariff-and-unit-costs.md) | [`src/national_tariff_and_unit_costs.rs`](../src/national_tariff_and_unit_costs.rs) |
| [`net-monetary-benefit.md`](net-monetary-benefit.md) | [`src/net_monetary_benefit.rs`](../src/net_monetary_benefit.rs) |
| [`nice-evidence-standards-framework.md`](nice-evidence-standards-framework.md) | [`src/nice_evidence_standards_framework.rs`](../src/nice_evidence_standards_framework.rs) |
| [`number-needed-to-treat.md`](number-needed-to-treat.md) | [`src/number_needed_to_treat.rs`](../src/number_needed_to_treat.rs) |
| [`opportunity-cost.md`](opportunity-cost.md) | [`src/opportunity_cost.rs`](../src/opportunity_cost.rs) |
| [`patient-reported-outcomes.md`](patient-reported-outcomes.md) | [`src/patient_reported_outcomes.rs`](../src/patient_reported_outcomes.rs) |
| [`practitioner-time.md`](practitioner-time.md) | [`src/practitioner_time.rs`](../src/practitioner_time.rs) |
| [`prevention-economics.md`](prevention-economics.md) | [`src/prevention_economics.rs`](../src/prevention_economics.rs) |
| [`probabilistic-sensitivity-analysis.md`](probabilistic-sensitivity-analysis.md) | [`src/probabilistic_sensitivity_analysis.rs`](../src/probabilistic_sensitivity_analysis.rs) |
| [`qaly-shortfall-and-severity-modifiers.md`](qaly-shortfall-and-severity-modifiers.md) | [`src/qaly_shortfall_and_severity_modifiers.rs`](../src/qaly_shortfall_and_severity_modifiers.rs) |
| [`quality-adjusted-life-year.md`](quality-adjusted-life-year.md) | [`src/quality_adjusted_life_year.rs`](../src/quality_adjusted_life_year.rs) |
| [`reach-and-equity.md`](reach-and-equity.md) | [`src/reach_and_equity.rs`](../src/reach_and_equity.rs) |
| [`readmission-rate.md`](readmission-rate.md) | [`src/readmission_rate.rs`](../src/readmission_rate.rs) |
| [`referral-to-treatment.md`](referral-to-treatment.md) | [`src/referral_to_treatment.rs`](../src/referral_to_treatment.rs) |
| [`remote-patient-monitoring-economics.md`](remote-patient-monitoring-economics.md) | [`src/remote_patient_monitoring_economics.rs`](../src/remote_patient_monitoring_economics.rs) |
| [`retention-and-churn.md`](retention-and-churn.md) | [`src/retention_and_churn.rs`](../src/retention_and_churn.rs) |
| [`return-on-investment.md`](return-on-investment.md) | [`src/return_on_investment.rs`](../src/return_on_investment.rs) |
| [`screening-economics.md`](screening-economics.md) | [`src/screening_economics.rs`](../src/screening_economics.rs) |
| [`sensitivity-analysis.md`](sensitivity-analysis.md) | [`src/sensitivity_analysis.rs`](../src/sensitivity_analysis.rs) |
| [`social-return-on-investment.md`](social-return-on-investment.md) | [`src/social_return_on_investment.rs`](../src/social_return_on_investment.rs) |
| [`space-and-devex.md`](space-and-devex.md) | [`src/space_and_devex.rs`](../src/space_and_devex.rs) |
| [`technical-debt.md`](technical-debt.md) | [`src/technical_debt.rs`](../src/technical_debt.rs) |
| [`time-horizon.md`](time-horizon.md) | [`src/time_horizon.rs`](../src/time_horizon.rs) |
| [`total-cost-of-ownership.md`](total-cost-of-ownership.md) | [`src/total_cost_of_ownership.rs`](../src/total_cost_of_ownership.rs) |
| [`value-generating-capacity-operational-turnaround.md`](value-generating-capacity-operational-turnaround.md) | [`src/value_generating_capacity_operational_turnaround.rs`](../src/value_generating_capacity_operational_turnaround.rs) |
| [`value-of-a-statistical-life.md`](value-of-a-statistical-life.md) | [`src/value_of_a_statistical_life.rs`](../src/value_of_a_statistical_life.rs) |
| [`waiting-list-impact.md`](waiting-list-impact.md) | [`src/waiting_list_impact.rs`](../src/waiting_list_impact.rs) |
| [`wearable-validation.md`](wearable-validation.md) | [`src/wearable_validation.rs`](../src/wearable_validation.rs) |
| [`willingness-to-pay-thresholds.md`](willingness-to-pay-thresholds.md) | [`src/willingness_to_pay_thresholds.rs`](../src/willingness_to_pay_thresholds.rs) |
| [`work-productivity-and-activity-impairment.md`](work-productivity-and-activity-impairment.md) | [`src/work_productivity_and_activity_impairment.rs`](../src/work_productivity_and_activity_impairment.rs) |
| [`workforce-retention.md`](workforce-retention.md) | [`src/workforce_retention.rs`](../src/workforce_retention.rs) |
| [`wsjf-and-cd3.md`](wsjf-and-cd3.md) | [`src/wsjf_and_cd3.rs`](../src/wsjf_and_cd3.rs) |

Each spec file's formula and worked-example numbers are themselves sourced
from the [health-economics-metrics](https://github.com/healthprojects/health-economics-metrics)
project's topic docs — see the `Upstream topic` line in each file.

## Change process

A change to a formula, a function's signature, or the condition under which
it returns `None`/`Err` must update, in the same change:

1. This directory's spec file for that metric.
2. The implementing function in `src/`.
3. That function's rustdoc (`# Arguments` / `# Returns` / `# Examples`, and
   `# Errors` for the three `Money`-based modules).
4. Its unit test(s) and doctest(s) in the same module.

Narrative-only edits (why it matters, pitfalls, sources) touch only the
module's rustdoc and don't require a spec change.

## Conventions used in most spec files

- Almost every public function takes and returns `f64`, or `Option<f64>`
  whenever an argument is a denominator that can legitimately be zero — it
  returns `None` rather than producing `NaN` or `inf`.
- Three modules — [`exact_cents_cost_allocation`](../src/exact_cents_cost_allocation.rs),
  [`currency_safe_cost_rollup`](../src/currency_safe_cost_rollup.rs), and
  [`cross_currency_icer_comparison`](../src/cross_currency_icer_comparison.rs)
  — instead work with the [`rusty_money`](https://docs.rs/rusty-money) crate's
  `Money` type for exact decimal currency arithmetic, and return
  `Result<_, MoneyError>`; their spec files document an `Err` condition in
  place of the usual `None` condition. This is the crate's only dependency
  and its only departure from plain `f64`.
- No function panics. Where a function sorts floating-point values, it uses
  `f64::total_cmp` specifically so that `NaN` input cannot panic.
- No function validates that a numerator is non-negative or `<=` its
  denominator beyond the zero-denominator check — see
  [`AGENTS.md`](../AGENTS.md) for why that's a deliberate scope boundary,
  not an oversight.
