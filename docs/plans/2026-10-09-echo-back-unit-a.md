# Contract — Echo-Back Unit A: Boolean Toggles (2026-10-09)

Read `docs/plans/2026-10-09-echo-back.md` first (the pattern + census). This
contract is self-contained for the six P1 files. The shared helper
`src/utils/echo_back.rs` (`echo_signal`, 5 tests) ALREADY EXISTS — use it, do
not modify it.

## Global rules

1. NO git commands. Edit files only; the orchestrator commits.
2. Every edit must keep `cargo +nightly fmt` and
   `cargo +nightly clippy --all-targets --all-features -- -D warnings` clean,
   and `cargo +nightly test --lib --all-features` green (currently 543
   passing).
3. IA standards: no `#[allow]`, no panics on user input, follow existing file
   style (Apache-2.0 header, doc comments on public items).
4. If any anchor below does not match the file verbatim, or anything is
   ambiguous, STOP that file, note it in the report (rule 7), continue others.
5. Demo uses none of these components in controlled mode; do NOT touch `demo/`.

## The conversion pattern (uniform)

BEFORE (in component props):
```rust
    #[prop(optional)] checked: Option<RwSignal<bool>>,
```
AFTER:
```rust
    #[prop(optional)] checked: Option<ReadSignal<bool>>,
```

BEFORE (body):
```rust
    let is_checked = checked.unwrap_or_else(|| RwSignal::new(false));
```
AFTER:
```rust
    let is_checked = echo_signal(checked, false);
```
(`echo_signal` is `pub use`d from `crate::utils`; import as the file's
existing import style dictates — most components use
`use leptos::prelude::*;` and `use crate::utils::*;` — match neighbors.)

All reads (`is_checked.get()` in view closures) work unchanged. Write sites
(`is_checked.set(..)` / `.update(..)`) work unchanged — they now write the
component-owned echo signal. ADD callback firing on paths that lack it (per
file below). The external signal is never written anywhere.

## Per-file work

### 1. `src/components/switch.rs`

- Anchor line 30: `#[prop(optional)] checked: Option<RwSignal<bool>>,` → ReadSignal.
- Anchor line 42: `let is_checked = checked.unwrap_or_else(|| RwSignal::new(false));` → `let is_checked = echo_signal(checked, false);`
- Line ~148 write+callback already fires `on_change` — NO other change.
- Add ONE doc-comment line on the `checked` prop: `/// External controlled value (read-only); echo-back convention — see docs/plans/2026-10-09-echo-back.md.` (match file's doc-comment style; if the component's props carry no doc comments, add it anyway — public API docs.)

### 2. `src/components/checkbox.rs`

- Anchor line 19 prop → ReadSignal.
- Anchor line 34: `let is_checked = checked.unwrap_or_else(|| RwSignal::new(false));` → `echo_signal(checked, false)`.
- Write site ~169 (`is_checked.set(new_value);` inside `handle_change`, followed by existing `on_change` fire) — unchanged.
- Prop doc-comment as above.

### 3. `src/components/radio.rs` (Radio only, NOT RadioGroup at ~237)

- Anchor line 20 prop → ReadSignal.
- Anchor line 36: `let is_checked = checked.unwrap_or_else(|| RwSignal::new(false));` → `echo_signal(checked, false)`.
- Write site ~175 (`is_checked.set(true);` + existing `on_change`) — unchanged.
- Prop doc-comment as above.
- DO NOT touch `_value`/`_on_change` (RadioGroup ~237) — that is Unit B.

### 4. `src/components/popover.rs`

- Anchor line 16: `#[prop(optional)] opened: Option<RwSignal<bool>>,` → `Option<ReadSignal<bool>>`.
- NEW prop directly after it (same style):
```rust
    #[prop(optional)] on_change: Option<Callback<bool>>,
```
  Doc-comment both props (echo-back convention; on_change fires with the new opened state).
- Anchor line 24: `let is_opened = opened.unwrap_or_else(|| RwSignal::new(false));` → `let is_opened = echo_signal(opened, false);`
- Line 27 `provide_context::<RwSignal<bool>>(is_opened);` — UNCHANGED (now carries the internal echo signal; consumers keep writing it).
- `PopoverTarget` (line ~58) consumer: `use_context::<RwSignal<bool>>` stays; its toggle at line ~61 (`is_opened.update(|o| *o = !*o);`) must ALSO fire the callback. The context carries only the signal — to fire `on_change`, provide a second context value: add
```rust
    #[derive(Clone)]
    struct PopoverOnChange(Option<Callback<bool>>);
    provide_context(PopoverOnChange(on_change));
```
  in `Popover` next to the existing provide_context, and in `PopoverTarget`'s toggle:
```rust
        is_opened.update(|o| *o = !*o);
        if let Some(PopoverOnChange(Some(cb))) = use_context::<PopoverOnChange>() {
            cb.run(is_opened.get_untracked());
        }
```
  (Make the struct `pub(crate)` or file-private as visibility requires; no clippy dead_code warnings — it is used.)
- `PopoverDropdown` (~92) reads only — unchanged.
- Line 102 `let is_open = is_opened.get();` — unchanged.

### 5. `src/components/accordion.rs` (AccordionItem)

- Anchor line 78: `#[prop(optional)] opened: Option<RwSignal<bool>>,` → ReadSignal.
- NEW prop after it: `#[prop(optional)] on_change: Option<Callback<bool>>,` (doc-comment: fires with the new opened state on toggle).
- Anchor line 87: `let is_opened = opened.unwrap_or_else(|| RwSignal::new(false));` → `echo_signal(opened, false)`.
- Anchor ~170 `handle_toggle`:
```rust
    let handle_toggle = move |_| {
        is_opened.update(|opened| *opened = !*opened);
    };
```
  becomes
```rust
    let handle_toggle = move |_| {
        is_opened.update(|opened| *opened = !*opened);
        if let Some(callback) = on_change {
            callback.run(is_opened.get_untracked());
        }
    };
```

### 6. `src/components/banner.rs`

- Anchor line 51: `#[prop(optional)] opened: Option<RwSignal<bool>>,` → ReadSignal.
- Anchor line 63: `let is_opened = opened.unwrap_or_else(|| RwSignal::new(true));` → `let is_opened = echo_signal(opened, true);` (note the `true` default).
- Write site ~146 `handle_close` (sets false + existing `on_close`) — unchanged. Banner is close-only; NO on_change(bool) addition (documented decision in the plan).
- Prop doc-comment as above.

## Wasm browser tests — NEW FILE `tests/echo_back_wasm.rs`

Copy the header block + `#![cfg(all(feature = "node-graph", target_arch = "wasm32"))]` shape is WRONG for this file — use `#![cfg(target_arch = "wasm32")]` (toggles are default-feature components; do NOT gate on node-graph). Follow `tests/node_graph_wasm.rs` conventions: `mount_to` + `.forget()`, remove host element cleanup, synthetic `MouseEvent` dispatch by type.

Import `mingot::{Switch, ...}` and `leptos::prelude::*` as the harness file does. Three tests:

1. `switch_external_syncs_and_callback_fires`: parent-style setup —
   `let (ext, set_ext) = signal(false); let (fired, set_fired) = signal(None::<bool>);`
   mount `<Switch checked=Some(ext) on_change=Callback::new(move |v| set_fired.set(Some(v))) />`
   (match the actual Switch prop names/signature in `src/components/switch.rs`;
   the interactive root carries a `mingot-switch*` class — verify the exact
   class string in the file and use it).
   - Click the element (`MouseEvent::new("click")` dispatch as harness does).
   - Assert `fired.get() == Some(true)` (callback fired).
   - Assert the element's `class`/`aria-checked`/inline style now reflects
     the ON state (pick what the view actually renders — verify in file).
   - Assert `ext.get_untracked() == false` (component did NOT write parent state).
   - Then `set_ext.set(true)` and assert the view reflects ON without any
     click (external sync inward via echo signal). Then `set_ext.set(false)`
     and assert OFF.

2. `popover_target_toggles_and_fires_on_change`: mount
   `<Popover opened=Some(ext) on_change=Callback> <PopoverTarget>..."t"</...> <PopoverDropdown>..."d"</...> </Popover>`
   (match actual required-props/children API from `src/components/popover.rs`).
   Click target → assert fired==Some(true) and dropdown visible; assert
   `ext.get_untracked()==false` (not written).

3. `accordion_item_toggle_fires_on_change`: mount an `AccordionItem`
   (match its required props — title/children) with `opened=Some(ext)`,
   `on_change` capturing. Click the header → fired==Some(true);
   `ext.get_untracked()==false`.

If any view detail (class strings, required props) differs from what this
contract assumes, adapt to the real file — those are conveniences, not
anchors. Anchors are the quoted code lines only.

Verify: `cargo +nightly check --target wasm32-unknown-unknown --tests` passes
(wasm tests compile; runtime browser run happens orchestrator-side).

## CHANGELOG

Under `## [Unreleased]` → `### Changed` add a subsection:

```markdown
#### Echo-back component API convention (breaking)

- **Breaking**: Boolean-toggle components (`Switch`, `Checkbox`, `Radio`,
  `Popover`, `AccordionItem`, `Banner`) now take optional `ReadSignal` value
  props instead of `RwSignal` — read-signals in, intent-callbacks out; the
  parent owns state. `Popover` and `AccordionItem` gain `on_change:
  Option<Callback<bool>>`. Controlled usage changes from
  `checked=signal(...)` to `checked=Some(read)` + `on_change=Callback::new(set)`.
  Uncontrolled usage is unchanged. See docs/plans/2026-10-09-echo-back.md.
```

(Merge cleanly into the existing `### Changed` section; keep entries sorted
as the file is.)

## Report format

List per file: anchors matched / edits made / anything adapted. Then gate
results: fmt, clippy (all-features), `cargo +nightly test --lib --all-features`
count, wasm check result. Any rule-7 stops with exact quotes.
