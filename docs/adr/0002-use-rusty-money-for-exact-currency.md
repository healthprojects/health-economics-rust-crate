# ADR 0002: Use `rusty-money` for exact currency

- Status: accepted

Allocation, rollup, and cross-currency comparison must preserve decimal minor
units and currency identity. Those modules expose `rusty_money::Money` directly
rather than a local wrapper, avoiding binary-float drift and making the caller's
construction type obvious. Other economic estimates remain `f64` under ADR
0001.
