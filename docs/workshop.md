# Hands-on workshop

This 90-minute workshop takes a Rust-capable learner from raw service data to
an auditable funding recommendation. It uses the synthetic
`data/waiting_list_pathway.csv` file and requires only Rust and this checkout.

## Learning outcomes

Participants will distinguish health value, cost effectiveness, affordability,
and uncertainty; find a metric contract; compose pure functions without hiding
units; and challenge a result with scenario analysis.

## Agenda

1. **10 min — Contracts and units:** locate a metric in `spec/` and its module.
2. **20 min — Health effect:** calculate QALYs recovered from wait reduction.
3. **20 min — Decision rules:** calculate ICER and NMB and reconcile them.
4. **15 min — Budget:** separate economic benefit from cash impact.
5. **15 min — Uncertainty:** vary uptake, cost, and effect assumptions.
6. **10 min — Review:** state perspective, horizon, limitations, and decision.

## Exercises

Run `cargo run --example qaly_to_decision`, then change one input at a time.
Find the intervention cost that makes NMB zero. Next, use the waiting-list CSV
to explain which columns are inputs, intermediate values, and outcomes. Finally,
write a downside case and state whether the recommendation changes.

## Answer checks

The unmodified example produces 50 QALYs, a £10,000 ICER, and £500,000 NMB at
the £20,000 threshold. The break-even incremental cost is £1,000,000. A good
review explicitly says that positive NMB does not prove the payer can afford
the intervention or that released capacity becomes cash.

## Facilitator checklist

- Ask learners to name every unit before accepting an answer.
- Treat `None` as a meaningful undefined ratio, not an error to unwrap blindly.
- Use the relevant spec to resolve formula disputes.
- Do not present synthetic inputs as observed clinical evidence.
- End with `cargo test` so the exercise remains reproducible.
