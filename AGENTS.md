# AGENTS.md — Mingot

Guidance for AI agents (and humans) working in this repository.

**Mingot** is the Industrial Algebra precision UI component library: Leptos 0.8
components for applications that cannot afford to lose numeric precision,
compiling to WebAssembly.

## Where truth lives

Do **not** maintain state snapshots in-repo (no CONTEXT.md-style files —
they rot by construction). Derive state from its authoritative source:

| Question | Source of truth |
|---|---|
| What changed, when | `CHANGELOG.md` |
| Direction / phases / 1.0 criteria | `ROADMAP.md` |
| Why things are the way they are | `docs/DECISIONS.md` |
| Phase 7 node-graph doctrine | `docs/plans/2026-09-25-node-graph-7c.md` |
| Audit findings & status | `docs/audits/` |
| API surface, features, error convention | `src/lib.rs` crate docs |
| Versions, deps, feature flags | `Cargo.toml` / `Cargo.lock` |
| CI, publish, hooks | `.github/workflows/`, `.githooks/` |
| Live working state | IA memory service (Ijima) — if you cannot access it, derive from the above; never re-create a snapshot file |

## Working conventions

- **IA coding standards** apply: TDD (failing test before implementation),
  `Result`-not-panic on user-controllable input, `thiserror` typed errors,
  additive feature gates, `Cow<'static, str>` props, no clippy `#[allow]`
  without documented rationale.
- **Node-graph kernel purity**: `src/node_graph/{value,exec}.rs` and
  `src/node_graph/nodes/` must stay free of Leptos/wasm-bindgen — pure Rust,
  testable and consumable without WASM.
- **Refuse-to-fire numerics**: precision change happens only via explicit
  conversion nodes; lossy execution is a structured refusal, never a silent
  narrowing.
- **Component API convention**: read-signals in, intent-callbacks out
  (`Option<Callback<T>>`, never generic closures).
- **Error homes**: errors live in their owning module (see `src/lib.rs`).

## Commands

```bash
cargo test --lib                                # kernel tests
cargo clippy --all-targets --all-features -- -D warnings
cargo build --target wasm32-unknown-unknown --lib   # WASM build
cd demo && trunk serve                          # demo site (dogfoods the library)
```

The 51 `wasm-bindgen-test` browser tests run under `cargo +nightly test
--target wasm32-unknown-unknown` (wasm browser harness), ignored on native.

## License

Apache-2.0. Contributions require the IA CLA — see
`https://github.com/Industrial-Algebra/.github/blob/main/CLA.md` and
`CONTRIBUTING.md`.
