# Metric cheat sheet

| Decision question | Formula | Module |
| --- | --- | --- |
| How much quality-adjusted health is gained? | duration × utility | `quality_adjusted_life_year` |
| What is the extra cost per extra effect? | Δcost / Δeffect | `incremental_cost_effectiveness_ratio` |
| Is value positive at threshold λ? | effect × λ − cost | `net_monetary_benefit` |
| Is the programme affordable? | eligible population × uptake × incremental cost | `budget_impact_analysis` |
| What is the return on investment? | (benefit − cost) / cost | `cost_benefit_analysis` |
| How many must be treated? | 1 / absolute risk reduction | `number_needed_to_treat` |
| What is perfect information worth? | expected max NMB − max expected NMB | `expected_value_of_perfect_information` |
| How much delivery value is delayed? | value × delay | `cost_of_delay` |

All units and zero-denominator behavior are part of the linked module’s contract
under `spec/`. Do not combine currencies, time bases, or populations without an
explicit conversion.
