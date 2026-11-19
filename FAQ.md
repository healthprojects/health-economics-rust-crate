# Frequently asked questions

## Why does the crate mostly use `f64`?

Most metrics are ratios, probabilities, utilities, rates, or model estimates.
Their source data are already uncertain and floating-point arithmetic makes the
functions easy to compose. Callers remain responsible for rounding displayed
results.

## When should I use `Money`?

Use the three exact-money modules when every minor unit must reconcile:
`exact_cents_cost_allocation`, `currency_safe_cost_rollup`, and
`cross_currency_icer_comparison`. Their public APIs name `rusty_money::Money`
directly. Do not convert their values through `f64`.

## Why do ratio functions return `Option<f64>`?

`None` means the specified denominator is exactly zero. The crate deliberately
does not invent a value, return infinity, or apply unrelated input validation.

## Does the crate validate probabilities and counts?

No. Apart from specified zero-denominator and shape checks, functions trust the
caller. This keeps the calculation contract narrow and avoids silently imposing
domain assumptions that may not fit a model.

## Which function should I use for an adoption decision?

Prefer net monetary benefit when comparing multiple options or uncertain draws.
If using an ICER, classify dominance first: a negative ICER is ambiguous without
the cost-effectiveness quadrant.

## How do I add or change a metric?

Read [`AGENTS.md`](AGENTS.md) and [`spec/README.md`](spec/README.md). Formula,
signature, fallibility, rustdoc, tests, and machine-readable indexes must change
together.

## What quality gates must pass?

Run `bin/ci` and `bin/coverage`. The latter enforces 100% line, function, and
region coverage.
