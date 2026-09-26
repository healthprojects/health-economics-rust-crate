---
name: health-economics-metrics
description: Compute health-economics metrics -- QALYs, ICER/NMB, cost-effectiveness/cost-benefit/budget-impact analysis, healthcare operations metrics, digital health product metrics, AI evaluation metrics, and their software-engineering analogues (cost of delay, DORA, technical debt) -- using the `health-economics` Rust crate's pure calculation functions. Use when asked to calculate one of these metrics from raw counts, to explain what one means, or to write Rust code against this crate.
---

# Health Economics Metrics

`health-economics` (this repository) is a Rust crate: one module per
metric, each a small set of pure functions, almost all over `f64`. Three
modules — `exact_cents_cost_allocation`, `currency_safe_cost_rollup`, and
`cross_currency_icer_comparison` — work with `rusty_money::Money` instead,
for exact-decimal currency arithmetic. Full contracts live in
[`spec/`](../spec/README.md); the machine-readable form is
[`llms.json`](../llms.json). This file is the quick-reference for picking
the right module and interpreting its result, organized by the same 8
themes `src/lib.rs`'s module index uses.

## Picking a module

### Health outcome measures

| What it covers | Module |
| --- | --- |
| A QALY is one year of life lived in perfect health. | [`quality_adjusted_life_year`](../src/quality_adjusted_life_year.rs) |
| EQ-5D is the `EuroQol` group's standardized questionnaire for measuring health-related quality of life. | [`eq_5d`](../src/eq_5d.rs) |
| A DALY is one lost year of healthy life — the burden-side mirror of the QALY. | [`disability_adjusted_life_year`](../src/disability_adjusted_life_year.rs) |
| Life-years gained is the additional survival attributable to an intervention, with no quality adjustment: the area between the survival curves with and without it. | [`life_years_gained`](../src/life_years_gained.rs) |
| HALE is a population-level summary: the number of years a person can expect to live *in full health*, discounting years spent in illness or disability. | [`health_adjusted_life_expectancy`](../src/health_adjusted_life_expectancy.rs) |
| PROMs are standardized instruments where patients report their own health status (symptoms, function, quality of life); PREMs capture the care *experience*. | [`patient_reported_outcomes`](../src/patient_reported_outcomes.rs) |
| NNT is the number of patients who must receive an intervention for **one** additional patient to benefit, over a stated time frame. | [`number_needed_to_treat`](../src/number_needed_to_treat.rs) |
| WPAI is a validated self-report questionnaire (Reilly, Zbrozek, Dasbach, 1993) measuring how much a health problem affects paid work and daily activities, usually over the past 7 days. | [`work_productivity_and_activity_impairment`](../src/work_productivity_and_activity_impairment.rs) |

### Economic evaluation frameworks

| What it covers | Module |
| --- | --- |
| CEA compares the costs of alternative interventions against a single outcome measured in **natural units** — life-years, cases detected, admissions avoided, mmHg of blood pressure reduced. | [`cost_effectiveness_analysis`](../src/cost_effectiveness_analysis.rs) |
| CUA is cost-effectiveness analysis with a **generic, preference-weighted outcome** — almost always the QALY (or DALY averted). | [`cost_utility_analysis`](../src/cost_utility_analysis.rs) |
| CBA values both costs *and* outcomes in money. | [`cost_benefit_analysis`](../src/cost_benefit_analysis.rs) |
| CMA compares only costs, and picks the cheapest option — legitimate *only* when the outcomes of the alternatives have been demonstrated to be equivalent. | [`cost_minimization_analysis`](../src/cost_minimization_analysis.rs) |
| CCA presents costs alongside a **disaggregated table of all outcomes** — clinical, operational, experiential — without collapsing them into a single ratio or score. | [`cost_consequence_analysis`](../src/cost_consequence_analysis.rs) |
| BIA estimates what adopting an intervention does to a specific payer's **budget** over the next 1–5 years. | [`budget_impact_analysis`](../src/budget_impact_analysis.rs) |
| SROI extends ROI to outcomes that markets don't price — wellbeing, social connection, carer relief, environmental impact — by monetizing them with financial proxies, for *all* stakeholders affected. | [`social_return_on_investment`](../src/social_return_on_investment.rs) |
| HTA is the formal, institutionalized process by which health systems decide whether a technology — drug, device, or software — is worth paying for. | [`health_technology_assessment`](../src/health_technology_assessment.rs) |
| These are the two competing methods for valuing lost productivity — from illness, disability, or death — in cost-of-illness and cost-benefit studies. | [`human_capital_and_friction_cost`](../src/human_capital_and_friction_cost.rs) |

### Decision rules and thresholds

| What it covers | Module |
| --- | --- |
| The ICER is the extra cost per extra unit of health effect when you choose one option over the next-best alternative. | [`incremental_cost_effectiveness_ratio`](../src/incremental_cost_effectiveness_ratio.rs) |
| NMB converts a cost-effectiveness result into a single money value: health gain priced at the willingness-to-pay threshold, minus cost. | [`net_monetary_benefit`](../src/net_monetary_benefit.rs) |
| A willingness-to-pay (WTP) threshold is the maximum a decision-maker will pay per unit of health gain — the line λ that turns an ICER (incremental cost-effectiveness ratio) into an adopt/reject decision. | [`willingness_to_pay_thresholds`](../src/willingness_to_pay_thresholds.rs) |
| An option is **dominated** if another option costs less *and* delivers more. | [`dominance_and_efficiency_frontier`](../src/dominance_and_efficiency_frontier.rs) |
| QALY shortfall measures how much future health a disease takes from patients compared to the general population. | [`qaly_shortfall_and_severity_modifiers`](../src/qaly_shortfall_and_severity_modifiers.rs) |
| Opportunity cost is the value of the best alternative you give up when you commit a resource. | [`opportunity_cost`](../src/opportunity_cost.rs) |
| Perspective defines *whose* costs and benefits count in an economic analysis: the payer's, the provider's, or society's as a whole. | [`analysis_perspective`](../src/analysis_perspective.rs) |
| The time horizon is the period over which an analysis counts costs and effects. | [`time_horizon`](../src/time_horizon.rs) |
| Discounting converts future costs and benefits into present values, because a benefit today is worth more than the same benefit in five years. | [`discounting_and_time_preference`](../src/discounting_and_time_preference.rs) |
| Average cost is total cost divided by units produced. | [`marginal_vs_average_cost`](../src/marginal_vs_average_cost.rs) |
| The value of a statistical life (VSL) — called the "value of a prevented fatality" (VPF) in UK usage — is the amount a *population* is collectively willing to pay to reduce the risk of one statistical death, derived from wage-risk trade-off studies (how much extra pay workers demand for riskier jobs) and stated-preference surveys. | [`value_of_a_statistical_life`](../src/value_of_a_statistical_life.rs) |
| Comparing an [`crate::incremental_cost_effectiveness_ratio`] computed in one country's currency against another country's [`crate::willingness_to_pay_thresholds`] — or pooling cost data collected across a multinational trial — requires an explicit, auditable currency-conversion step. | [`cross_currency_icer_comparison`](../src/cross_currency_icer_comparison.rs) |
| Multi-criteria decision analysis (MCDA) is a weighted-sum scoring model used in health technology assessment when a single ICER/willingness-to-pay threshold doesn't capture everything a decision-maker cares about: equity, unmet need, innovation, budget impact, disease severity. | [`multi_criteria_decision_analysis`](../src/multi_criteria_decision_analysis.rs) |
| Carbon per QALY is an efficiency ratio — an intervention's carbon emissions (or emissions avoided) divided by the QALYs it delivers — directly analogous to cost per QALY, letting an intervention's carbon efficiency be assessed alongside its cost efficiency. | [`carbon_footprint_per_qaly`](../src/carbon_footprint_per_qaly.rs) |

### Uncertainty and evidence

| What it covers | Module |
| --- | --- |
| Deterministic sensitivity analysis (DSA) varies one assumption at a time across a plausible range to see whether the conclusion survives. | [`sensitivity_analysis`](../src/sensitivity_analysis.rs) |
| PSA assigns a probability distribution to every uncertain parameter, samples them all simultaneously thousands of times (Monte Carlo), and reports the *probability* that an option is the best choice — instead of a single point estimate. | [`probabilistic_sensitivity_analysis`](../src/probabilistic_sensitivity_analysis.rs) |
| EVPI is the maximum amount a decision-maker should pay to eliminate uncertainty before deciding — the formal price of "let's run a study first". | [`expected_value_of_perfect_information`](../src/expected_value_of_perfect_information.rs) |
| EVSI is the value of a *specific proposed study* — a given design, a given sample size — before it is run, as opposed to expected value of perfect information (EVPI), which prices eliminating all uncertainty outright. | [`expected_value_of_sample_information`](../src/expected_value_of_sample_information.rs) |
| Benefits realization management (BRM) is the discipline of identifying, baselining, tracking, and *evidencing* that the benefits promised in a business case actually materialized after delivery. | [`benefits_realization`](../src/benefits_realization.rs) |

### Healthcare operations

| What it covers | Module |
| --- | --- |
| A bed day is one patient occupying one hospital bed for one day. | [`bed_days_saved`](../src/bed_days_saved.rs) |
| Length of stay is the number of days from hospital admission to discharge — the core flow-efficiency metric of inpatient care. | [`length_of_stay`](../src/length_of_stay.rs) |
| The 30-day readmission rate is the percentage of discharged patients who return as an emergency within 30 days. | [`readmission_rate`](../src/readmission_rate.rs) |
| Referral to treatment is the elapsed time from a GP's referral to the start of consultant-led treatment. | [`referral_to_treatment`](../src/referral_to_treatment.rs) |
| Waiting list impact converts saved clinical capacity into patients removed from (or moved faster through) the waiting list — extra appointment slots, patients actually seen after DNA (did-not-attend) losses, net list reduction after induced demand, and the queueing pull-forward of waits. | [`waiting_list_impact`](../src/waiting_list_impact.rs) |
| The DNA rate is the percentage of booked appointments where the patient neither attends nor cancels. | [`did_not_attend_rate`](../src/did_not_attend_rate.rs) |
| Counts ED (A&E) visits and emergency admissions prevented by upstream intervention — triage apps, remote monitoring, virtual wards, urgent-care redirection — and converts "we caught it earlier" into a costed claim. | [`emergency_attendance_avoidance`](../src/emergency_attendance_avoidance.rs) |
| Practitioner time is the scarcest resource in most health systems. | [`practitioner_time`](../src/practitioner_time.rs) |
| The NHS pays providers for activity under a rules-based national price list — historically the National Tariff / Payment by Results, replaced by the **NHS Payment Scheme (NHSPS)** on 1 April 2023. | [`national_tariff_and_unit_costs`](../src/national_tariff_and_unit_costs.rs) |
| When a trust cannot meet targets with internal capacity, it buys capacity at premium rates: weekend overtime for its own staff, or outsourcing procedures to private providers. | [`avoidable_outsourcing_costs`](../src/avoidable_outsourcing_costs.rs) |
| Avoided downstream costs (cost offsets) are future treatment expenses prevented by earlier or better action, netted against the intervention's own cost. | [`avoided_downstream_costs`](../src/avoided_downstream_costs.rs) |
| Saving an hour for a senior practitioner — a GP, a senior registrar, a consultant — often prevents bottleneck delays for an entire multi-disciplinary team (MDT) of nurses, administrative clerks, and therapists who are waiting on clinical sign-offs. | [`downstream_resource_optimization`](../src/downstream_resource_optimization.rs) |
| Values the double dividend of moving patients from waiting list to active treatment sooner. | [`earlier_intervention`](../src/earlier_intervention.rs) |
| The economics of intervening before disease occurs or progresses. | [`prevention_economics`](../src/prevention_economics.rs) |
| Screening economics govern the value of testing asymptomatic populations. | [`screening_economics`](../src/screening_economics.rs) |
| Workforce retention economics quantify what staff turnover costs a health system — recruitment, onboarding/productivity ramp, and vacancy cover at agency premium — and therefore what software that reduces administrative burnout is worth. | [`workforce_retention`](../src/workforce_retention.rs) |
| Cash-releasing savings reduce actual expenditure — a budget line gets smaller. | [`cash_releasing_vs_non_cash_releasing`](../src/cash_releasing_vs_non_cash_releasing.rs) |
| Hard cash-releasing savings are line items a hospital can actively **delete from next month's budget** because of your software. | [`hard_cash_releasing_savings_deficit_defense`](../src/hard_cash_releasing_savings_deficit_defense.rs) |
| Value-generating capacity is the "opportunity benefit" of freed clinical time: what the hospital can now *achieve* with the hours software releases, expressed as the extra value-generating activity (clinics, assessments, monitoring reviews) the released hours enable, valued at national tariff / NHS Payment Scheme prices. | [`value_generating_capacity_operational_turnaround`](../src/value_generating_capacity_operational_turnaround.rs) |

### Digital health products

| What it covers | Module |
| --- | --- |
| Activation rate is the share of sign-ups who reach first meaningful value (the "aha" action — first reading logged, first lesson done). | [`activation_and_uptake`](../src/activation_and_uptake.rs) |
| Adherence is how closely actual use matches prescribed use (intensity); persistence is how long use continues before discontinuation (duration). | [`adherence_and_persistence`](../src/adherence_and_persistence.rs) |
| Measures how much users actually use a health app: DAU/MAU stickiness, session frequency and duration, feature usage. | [`engagement_metrics`](../src/engagement_metrics.rs) |
| Retention measures what fraction of a user cohort is still active N days after starting (D1/D7/D30 curves); churn is its complement. | [`retention_and_churn`](../src/retention_and_churn.rs) |
| RE-AIM — Reach, Effectiveness, Adoption, Implementation, Maintenance — is the standard framework for judging the *population* impact of an intervention. | [`reach_and_equity`](../src/reach_and_equity.rs) |
| The commercial arithmetic of consumer health products: customer acquisition cost (CAC), lifetime value (LTV), average revenue per user (ARPU), per-member-per-month (PMPM) pricing, and the employer-market distinction between ROI and VOI (value on investment). | [`health_app_unit_economics`](../src/health_app_unit_economics.rs) |
| The reimbursement and cost-offset economics of monitoring patients at home. | [`remote_patient_monitoring_economics`](../src/remote_patient_monitoring_economics.rs) |
| A digital biomarker is an objective physiological or behavioral measure collected via sensors (gait speed from a phone, sleep from a wearable, tremor from accelerometry). | [`digital_endpoints_and_biomarkers`](../src/digital_endpoints_and_biomarkers.rs) |
| Validation metrics quantify how well a wearable's measurements agree with a clinical gold standard (ECG for heart rate, polysomnography for sleep): MAPE, Lin's concordance correlation coefficient (CCC), and Bland–Altman limits of agreement — plus the operational metrics that gate real-world data quality: wear-time compliance and data completeness. | [`wearable_validation`](../src/wearable_validation.rs) |
| `DiGA` (Digitale Gesundheitsanwendungen) is Germany's statutory "apps on prescription" pathway — the world's first national system where doctors prescribe approved health apps and statutory insurance must reimburse them. | [`diga_fast_track`](../src/diga_fast_track.rs) |
| The ESF is NICE's framework specifying **how much evidence a digital health technology needs, proportionate to its risk**. | [`nice_evidence_standards_framework`](../src/nice_evidence_standards_framework.rs) |
| The UK Government Digital Service (GDS) Service Manual mandates four KPIs for every government digital service: **cost per transaction, user satisfaction, completion rate, and digital take-up**. | [`gds_service_metrics`](../src/gds_service_metrics.rs) |

### AI evaluation and economics

| What it covers | Module |
| --- | --- |
| The core statistics for evaluating a clinical AI or diagnostic model: sensitivity, specificity, AUROC, predictive values, and number needed to screen. | [`clinical_ai_evaluation`](../src/clinical_ai_evaluation.rs) |
| Metrics for the correctness of AI-generated output: accuracy against ground truth, faithfulness/groundedness (is every claim supported by the provided context?), and hallucination rate (what fraction of outputs contain unsupported or false content?). | [`ai_quality_metrics`](../src/ai_quality_metrics.rs) |
| The regulatory frameworks that govern AI in health care — FDA's Software as a Medical Device (`SaMD`) regime with Predetermined Change Control Plans (PCCPs), and real-world evaluation programs like the NHS AI in Health and Care Award — and what they cost and enable economically. | [`ai_regulatory_evaluation`](../src/ai_regulatory_evaluation.rs) |
| AI ROI is the measurable P&L return attributable to AI initiatives. | [`ai_return_on_investment`](../src/ai_return_on_investment.rs) |
| Metrics for what AI coding assistance actually does to engineering output: suggestion acceptance rates, controlled-study speedups, PR throughput, and code retention — plus the value model that turns measured time saved into a capacity benefit line. | [`ai_developer_productivity`](../src/ai_developer_productivity.rs) |
| Inference unit economics price AI features by their marginal compute: **cost per token**, rolled up to cost per call, per business unit (triage episode, drafted letter, consultation summary), per year. | [`inference_unit_economics`](../src/inference_unit_economics.rs) |

### Software engineering economics

| What it covers | Module |
| --- | --- |
| The DORA (DevOps Research and Assessment) metrics are four measures of software delivery performance — deployment frequency, lead time for changes, change failure rate, and failed-deployment recovery time — plus reliability as a fifth. | [`dora_metrics`](../src/dora_metrics.rs) |
| Measures how work moves through a delivery system: cycle time, lead time, throughput, work in progress (WIP), and flow efficiency. | [`flow_metrics`](../src/flow_metrics.rs) |
| SPACE (Satisfaction & well-being, Performance, Activity, Communication & collaboration, Efficiency & flow) and `DevEx` (feedback loops, cognitive load, flow state) are frameworks for measuring developer productivity multi-dimensionally — the field's answer to the discovery that no single metric survives contact with reality. | [`space_and_devex`](../src/space_and_devex.rs) |
| Technical debt is the implied future cost of expedient past decisions in a codebase: the remediation work owed (**principal**) and the ongoing drag it exerts on delivery (**interest**). | [`technical_debt`](../src/technical_debt.rs) |
| Cost of Delay is the economic value lost per unit time that a feature, product, or service is *not* delivered. | [`cost_of_delay`](../src/cost_of_delay.rs) |
| CD3 (Cost of Delay Divided by Duration) and WSJF (Weighted Shortest Job First) are prioritization rules that schedule work by **value density**: how much delay cost is removed per unit of scarce capacity consumed. | [`wsjf_and_cd3`](../src/wsjf_and_cd3.rs) |
| ROI is the ratio of net gain to money invested. | [`return_on_investment`](../src/return_on_investment.rs) |
| TCO is the full cost of a system over its life: acquisition or build, integration, operation, maintenance, support, training, and decommissioning. | [`total_cost_of_ownership`](../src/total_cost_of_ownership.rs) |
| Build-vs-buy is a structured comparison of custom development against commercial acquisition, on risk-adjusted total cost of ownership (TCO), time-to-value, and cost of delay. | [`build_vs_buy`](../src/build_vs_buy.rs) |
| Cloud unit economics translate raw cloud spend into **cost per unit of output** — per customer, per transaction, per case resolved, per token. | [`cloud_unit_economics`](../src/cloud_unit_economics.rs) |
| Splitting a total amount of money — a shared grant, an infrastructure bill, a budget-impact total — across several recipients by naive percentage arithmetic routinely produces parts that don't sum back to the original total. | [`exact_cents_cost_allocation`](../src/exact_cents_cost_allocation.rs) |
| Summing many money line items — monthly invoices, per-site costs, multi-year budget-impact figures — with ordinary binary floating-point (`f64`) numbers accumulates small representation errors, because most decimal fractions ($1,234.56, for instance) aren't exactly representable in binary floating point (Goldberg, "What Every Computer Scientist Should Know About Floating-Point Arithmetic," 1991). | [`currency_safe_cost_rollup`](../src/currency_safe_cost_rollup.rs) |

## Reading the result

Every rate/ratio function that can have an undefined denominator returns
`Option<f64>` — `None` means exactly one thing in this crate: the
denominator argument was `0.0`. It is never an error condition to surface
as a bug; treat it as "undefined for this input" and handle it accordingly
(e.g. by omitting that row from a report rather than showing `0%` or
`NaN`).

```rust
use health_economics::number_needed_to_treat::number_needed_to_treat;

match number_needed_to_treat(0.25) {
    Some(nnt) => println!("NNT: {nnt:.1}"),
    None => println!("NNT: undefined (zero absolute risk reduction)"),
}
```

The three `Money`-based modules instead return `Result<_, rusty_money::MoneyError>` —
an error there means a real fallible condition (mismatched currencies, an
empty input, arithmetic overflow), not "zero denominator." Don't conflate
the two: an `Option::None` from most of this crate is a normal, expected
outcome; an `Err` from the three `Money` modules is a genuine problem the
caller should handle or propagate.

## Before computing a metric

- Check which count is the denominator — several modules have functions
  that look similar but divide by different totals (e.g. in
  `patient_reported_outcomes`, some functions divide by a cohort size and
  others by a completer count; check the function's own `spec/` entry
  rather than assuming).
- This crate does not validate that a numerator is non-negative or `<=` its
  denominator; it trusts the caller's counts. Validate upstream if the data
  source might not guarantee that.
- For a money-flavored question (splitting a total exactly, summing many
  line items without drift, or comparing figures across currencies), use
  one of the three `Money`-based modules rather than plain `f64` — see
  their entries in the "Software engineering economics" and "Decision
  rules and thresholds" tables above.

## Further reading

- [`spec/`](../spec/README.md) — exact formula and `None`/`Err` condition
  per function
- [`AGENTS.md`](../AGENTS.md) — conventions for modifying this crate
- [`docs/tutorials/`](../docs/tutorials/) — four long-form worked
  walkthroughs
