# ADR 0003: Deny missing docs and pedantic Clippy lints

- Status: accepted

This crate is a public formula library where API clarity and numerical intent
matter more than minimizing lint configuration. Missing public documentation and
pedantic Clippy warnings fail the build. A false positive may be allowed only on
the narrow item or line, with a comment explaining why the lint does not apply.
