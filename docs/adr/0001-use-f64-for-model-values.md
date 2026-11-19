# ADR 0001: Use `f64` for model values

- Status: accepted

Health-economic inputs are generally estimates, rates, utilities, and model
outputs. The crate uses `f64` for these values to keep formulas direct and
composable. Undefined division returns `Option`; presentation rounding remains a
caller concern. Exact ledger money is the exception described in ADR 0002.
