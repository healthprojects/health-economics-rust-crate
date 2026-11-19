# Rust notebook

[`qaly-to-decision.ipynb`](qaly-to-decision.ipynb) is an executable Jupyter
notebook for the `evcxr_jupyter` Rust kernel. Start Jupyter with this repository
as the working tree, select the Rust kernel, and run cells in order. The first
cell uses the repository checkout through a path dependency, so it does not
publish or alter the crate.

The notebook deliberately contains no stored execution output. Its assertions
are the reproducible result checks; the equivalent continuously tested program
is `examples/qaly_to_decision.rs`.
