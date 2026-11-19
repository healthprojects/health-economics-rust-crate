# Migration guide

## From a spreadsheet

Name every input and unit, identify denominator-zero behavior, and reproduce one
known workbook row as a Rust test. Move formulas individually; compare outputs
before composing them. Keep presentation rounding outside the calculation.

## From Python or JavaScript

Rust `Option<f64>` replaces sentinel `NaN`, `null`, or exceptions for specified
zero denominators. Use pattern matching or `?` in a caller that also returns an
`Option`. Slices (`&[f64]`) accept borrowed arrays and vectors without copying.

```rust
use health_economics::number_needed_to_treat::number_needed_to_treat;

let result = number_needed_to_treat(0.008);
match result {
    Some(value) => println!("NNT: {value:.1}"),
    None => println!("NNT undefined"),
}
```

For exact money, construct `rusty_money::Money` in the currency named by the
public signature. Do not convert an existing decimal ledger through `f64`.

## Compatibility expectations

Function contracts live in `spec/`. A formula, signature, or fallibility change
is treated as an API change and updates docs and tests in the same release.
