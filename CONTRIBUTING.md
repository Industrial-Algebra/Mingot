# Contributing to Mingot

Thank you for your interest in contributing to Mingot! Industrial Algebra
welcomes contributions from anyone building precision-critical applications.

## Contributor License Agreement

Before your first contribution is merged, please sign the
[Industrial Algebra Contributor License Agreement (CLA)](https://github.com/Industrial-Algebra/.github/blob/main/CLA.md).

The CLA is signed once and covers **all** Industrial Algebra projects — there
is no per-project signing. It grants IA a non-exclusive, worldwide,
royalty-free, irrevocable copyright license, and an express patent grant,
including the right to relicense contributions. This protects the project and
its users and lets IA ship Mingot under a permissive license
([Apache-2.0](LICENSE-APACHE)).

## Development workflow

Mingot follows a gitflow pattern:

1. Branch from `develop` (`feature/...`, `bugfix/...`, `chore/...`, `docs/...`).
2. Implement using **TDD** (test first, watch it fail, then implement).
3. Ensure all CI gates pass locally:
   - `cargo fmt -- --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test --all-features`
4. Open a pull request against `develop`.
5. A maintainer reviews and merges. **Never auto-merge** — all PRs require
   human review.

## Coding standards

Mingot follows the Industrial Algebra coding conventions: additive feature
flags, `Result` over panics, exhaustive matches, `thiserror` for error types,
documented public APIs, and `StyleBuilder` + `--mingot-*` CSS variables for
theming. See the source for established patterns.

## License

By contributing, you agree that your contributions are licensed under the
[Apache-2.0](LICENSE-APACHE) license and covered by the CLA above.
