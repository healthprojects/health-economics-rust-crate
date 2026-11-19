# Security policy

## Supported versions

Security fixes are made on the latest released version. Users should upgrade
to that version before reporting a problem that may already be fixed.

## Reporting a vulnerability

Please use GitHub's private vulnerability reporting for this repository:

<https://github.com/healthprojects/health-economics-rust-crate/security/advisories/new>

Do not open a public issue for a suspected vulnerability. Include the affected
version, a minimal reproduction, impact, and any suggested mitigation. You can
expect acknowledgement within seven days. We will coordinate disclosure after
the issue is understood and a fix is available.

This crate contains pure calculation functions and no network, file-system,
authentication, or unsafe-code surface. Numerical correctness, denial of
service through pathological input, dependency vulnerabilities, and accidental
panics are still in scope for security reports.
