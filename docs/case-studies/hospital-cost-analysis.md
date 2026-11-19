# Case study: hospital capacity and cost

A virtual ward substitutes 14,600 inpatient bed days. The inpatient day costs
£400 at the margin relevant to the decision and the virtual-ward day costs £250.
The gross substitution value is therefore £2.19 million. This is not
automatically a cash saving: the case must state whether the freed capacity is
refilled, flexed down, or absorbed as slack.

Use `remote_patient_monitoring_economics::nhs_style_net_value` for the pathway
rollup, then `bed_days_saved::freed_capacity_value` to classify the physical use
of capacity. Keep the intervention's service cost in the net-value calculation
and do not also claim the same bed days as avoided admissions.

The `business_case` example shows how the resulting line enters discounted
cost-benefit analysis and how cash-only ROI differs from economic ROI.
