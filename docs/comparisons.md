# Comparison with adjacent tools

| Need | This crate | Spreadsheet | General statistics package | Full decision-model platform |
| --- | --- | --- | --- | --- |
| Auditable named formulas | Strong; one spec and module per topic | Depends on workbook discipline | Must be assembled | Usually strong |
| Compile-time API checks | Yes | No | Language-dependent | Platform-dependent |
| Exact currency allocation | Built in for three money topics | Possible but easy to drift | Library-dependent | Platform-dependent |
| Statistical fitting | Out of scope | Limited | Strong | Often strong |
| Stateful cohort simulation | Small deterministic Markov helper | Manual | Custom code | Strong |
| Visualization/reporting | Out of scope | Strong | Strong | Strong |
| Dependencies/runtime | Rust library, pure calculations | Office runtime | Language ecosystem | Product-specific |

The crate complements rather than replaces statistical estimation, data
cleaning, visualization, or a validated institutional model. It is strongest
when a Rust system needs transparent, reusable calculations at its domain
boundary.
