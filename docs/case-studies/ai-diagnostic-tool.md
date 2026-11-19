# Case study: AI diagnostic tool

At 1% prevalence, 90% sensitivity, and 93% specificity, the positive rate is
7.83% but PPV is only about 11.5%. For 10,000 screened people, roughly 90 true
positives and 693 false positives receive work-up. At £350 per positive work-up,
cost per true case is about £3,045 before adding the model and workflow costs.

Use `clinical_ai_evaluation` to recompute predictive values at deployment
prevalence, not trial prevalence. Use `ai_quality_metrics` for the expected cost
of errors that survive human review, and `health_technology_assessment` for PSA
and reference-case gating. The synthetic draws in `data/ai_diagnostic_draws.csv`
illustrate the cost/effect input shape.
