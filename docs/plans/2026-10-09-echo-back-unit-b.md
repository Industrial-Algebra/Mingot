# Contract — Echo-Back Unit B: Text + Index Components (2026-10-10)

Read `docs/plans/2026-10-09-echo-back.md` (pattern + census) and
`docs/plans/2026-10-09-echo-back-unit-a.md` §"The conversion pattern" (the
uniform before/after shapes; identical here, types differ). The helper
`echo_signal` exists in `src/utils/echo_back.rs` — use it, do not modify.

Prerequisite state: Unit A is already merged into this branch (toggles
converted; `tests/echo_back_wasm.rs` exists as the wasm-test convention
reference).

## Global rules

1. NO git commands. 2. `cargo +nightly fmt` and `cargo +nightly clippy
   --all-targets --all-features -- -D warnings` clean; `cargo +nightly test
   --lib --all-features` green. 3. IA standards (no `#[allow]`, no panics,
   Apache headers, doc comments). 4. Anchor mismatch or ambiguity → stop that
   item, note it (rule 7), continue others. 5. `demo/` DOES use some of these
   components in controlled mode — update every demo call site that passes an
   `RwSignal` to these props (grep-verify; see per-file notes).

## Per-file work

### 1. `src/components/select.rs` — Select

- Anchor (props, ~line 51): `#[prop(optional)] value: Option<RwSignal<String>>,`
  → `Option<ReadSignal<String>>`.
- Anchor (~line 66): `let select_value = value.unwrap_or_else(|| RwSignal::new(String::new()));`
  → `let select_value = echo_signal(value, String::new());`
- Write site (~173 `handle_change`: `select_value.set(value.clone());` followed
  by existing `on_change` fire) — unchanged (already echo-back).
- Prop doc-comment: `/// External controlled value (read-only); echo-back convention.`

### 2. `src/components/textarea.rs` — Textarea

- Anchor (~29): value prop → `Option<ReadSignal<String>>`.
- Body: `let textarea_value = value.unwrap_or_else(...)` → `echo_signal(value, String::new())`.
- Write sites (~160 `handle_input` fires `on_input`; `handle_change` fires
  `on_change`) — unchanged.
- Prop doc-comment as above.

### 3. `src/components/formula_input.rs` — FormulaInput

- Anchor (~599):
```rust
    /// Current formula value
    #[prop(optional, into)]
    value: Option<RwSignal<String>>,
```
  → `Option<ReadSignal<String>>` (keep the `#[prop(optional, into)]` and doc
  comment shape).
- Anchor (~644): `let internal_value = value.unwrap_or_else(|| RwSignal::new(String::new()));`
  → `let internal_value = echo_signal(value, String::new());`
- Write site (~855 `internal_value.set(val.clone());` in the input handler) —
  unchanged; the parse/`on_change: Callback<FormulaResult>` flow is untouched
  (payload stays FormulaResult — no new callbacks).
- Doc-comment line on `value` noting read-only echo-back.

### 4. `src/components/number_input.rs` — NumberInput (dual-pattern completes)

- Anchors (~983/984):
```rust
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] on_change: Option<Callback<String>>,
```
  value → `Option<ReadSignal<String>>` (on_change unchanged).
- Anchor (~1113): `let number_value = value.unwrap_or_else(|| RwSignal::new(String::new()));`
  → `let number_value = echo_signal(value, String::new());`
- ALL ~9 `number_value.set(..)` write sites (undo/redo ~1156/1182, commit
  ~1266, filter ~1341, ~1403/1437/1506, format ~1551, clean ~1585) — write the
  internal echo signal now; where `on_change`/`on_valid_change` already fire
  nearby, unchanged. Verify each site: if any site writes `number_value`
  WITHOUT firing an existing callback, leave semantics as-is (do not invent
  new callback fires in NumberInput — its dual-pattern callbacks are already
  complete per the 7A-era design).
- Doc-comment on `value`: read-only external, echo-back convention.

### 5. `src/components/radio.rs` — RadioGroup ONLY (Radio is done, Unit A)

- Anchors (~237–242, underscored placeholders):
```rust
    #[prop(optional)] _value: Option<RwSignal<String>>,
    ...
    #[prop(optional)] _on_change: Option<Callback<String>>,
```
  Promote to real, echo-back-shaped props:
```rust
    #[prop(optional)] value: Option<ReadSignal<String>>,
    #[prop(optional)] on_change: Option<Callback<String>>,
```
  (also promote `_name: Option<String>` → `name: Option<String>` if it is
  genuinely unused-dead; if `name` has render use already, just un-underscore.
  Report which.)
- Body: `let group_value = echo_signal(value, String::new());` and
  `provide_context::<RwSignal<String>>(group_value);` — group-managed selection
  state, consumable by future coordinated Radio children. `on_change` is
  provided alongside via a new file-private context wrapper struct
  `GroupOnChange(Option<Callback<String>>)` (mirror the PopoverOnChange shape
  from popover.rs) — do NOT install an auto-fire effect; child coordination is
  future work, out of scope. Doc-comment both promoted props accordingly.

### 6. `src/components/tabs.rs` — Tabs (+ Tab child)

- Anchor (~22): `#[prop(into)] active: RwSignal<String>,`
  → `#[prop(optional)] active: Option<ReadSignal<String>>,` (BREAKING:
  required → optional).
- NEW prop: `#[prop(optional)] on_change: Option<Callback<String>>,` doc: fires
  with the newly-activated tab value.
- Anchor (~34): `provide_context::<RwSignal<String>>(active);` — the context
  must carry the internal echo signal. Body becomes:
```rust
    let active = echo_signal(active, String::new());
    provide_context::<RwSignal<String>>(active);
```
- Tab child write site (~246 `active.set(value.clone());`) — now writes the
  internal signal (via context, unchanged code). ALSO fire the group callback:
  provide `TabsOnChange(Option<Callback<String>>)` context from Tabs (same
  wrapper-struct idiom), and in Tab's handler after the set:
```rust
        if let Some(TabsOnChange(Some(cb))) = use_context::<TabsOnChange>() {
            cb.run(value.clone());
        }
```
- Tab reads (~149/268 `use_context::<RwSignal<String>>().unwrap()`) unchanged.

### 7. `src/components/table.rs` — Table + TableWithPagination

Table (generic, props at ~90):
- Anchors (~96/97):
```rust
    #[prop(optional)] sort_column: Option<RwSignal<Option<String>>>,
    #[prop(optional)] sort_direction: Option<RwSignal<SortDirection>>,
```
  → `Option<ReadSignal<...>>` each.
- Anchors (~106/108): `let current_sort_column = sort_column.unwrap_or_else(|| RwSignal::new(None));`
  and `let current_sort_direction = sort_direction.unwrap_or_else(|| RwSignal::new(SortDirection::None));`
  → `echo_signal(sort_column, None)` / `echo_signal(sort_direction, SortDirection::None)`.
- Header-click writes (~121/122): unchanged writes to internal; VERIFY `on_sort`
  fires on that path — if it does not fire today, fire it with
  `(column_key.clone(), new_direction)` after the writes.

TableWithPagination (props at ~513):
- Anchor (~515): `#[prop(into)] current_page: RwSignal<usize>,`
  → `#[prop(optional)] current_page: Option<ReadSignal<usize>>,` (BREAKING:
  required → optional) + NEW
  `#[prop(optional)] on_page_change: Option<Callback<usize>>,`.
- Body: `let current_page = echo_signal(current_page, 0);` then the existing
  `Signal::from(current_page)` pass-through and the `on_page_change=Callback`
  wiring at ~625 become:
```rust
                            on_page_change=Callback::new(move |page: usize| {
                                current_page.set(page);
                                if let Some(cb) = on_page {
                                    cb.run(page);
                                }
                            })
```
  (name the new prop `on_page_change` to match the inner TablePagination
  callback name; adjust closure capture/clone as borrow rules require).
- Sort props (~521/522): same ReadSignal conversion as Table; same
  echo_signal wiring; on_sort identical.
- Doc-comments on all converted props (echo-back convention, read-only).

## Demo call sites

`rg -n "active=|current_page=|sort_column=|sort_direction=" demo/src` and any
`value=`/`checked=` usages of THESE components (Select/Textarea/FormulaInput/
NumberInput/Tabs/TableWithPagination). Convert each controlled call site:
```rust
// before
let (active, set_active) = signal("first".to_string());
<Tabs active=active ...>
// after
<Tabs active=Some(active) on_change=Callback::new(set_active) ...>
```
Uncontrolled call sites (no signal prop passed) need NO change — verify Tabs
call sites especially: previously REQUIRED prop means every demo Tabs usage
passes a signal and MUST be updated. Same for TableWithPagination.
`demo` must compile: `cargo +nightly check -p mingot-demo` (package name may
differ — check demo/Cargo.toml name field).

## Wasm tests — APPEND to `tests/echo_back_wasm.rs` (created by Unit A)

Follow its conventions. Two tests:
1. `tabs_external_active_syncs_and_on_change_fires`: mount `<Tabs
   active=Some(ext) on_change=...>` with two `<Tab value="a">`/`"b"` children
   (match real prop names from tabs.rs). Assert initial active styles/classes
   reflect "a"; `set_ext("b")` → active reflects "b" without clicks; click tab
   "a" header → on_change fired with "a"; `ext.get_untracked()` unchanged.
2. `select_on_change_fires_and_external_syncs`: mount `<Select value=Some(ext)
   on_change=...>` with options (match real API incl. how options are
   declared); set ext to an option value → rendered selection updates; fire
   change on the select element (set its value + dispatch "change" event as
   the harness's synthetic-event notes allow) → on_change fired; ext
   unchanged.
Adapt view-detail assumptions to the real files; quoted anchors only are
binding. Verify: `cargo +nightly check --target wasm32-unknown-unknown --tests`.

## CHANGELOG

Under `## [Unreleased]` → `### Changed`, extend the echo-back subsection (or
add a sibling block if Unit A's was merged already):

```markdown
- **Breaking**: Text and index components (`Select`, `Textarea`,
  `FormulaInput`, `NumberInput`, `RadioGroup`, `Tabs`, `Table`,
  `TableWithPagination`) take optional `ReadSignal` value props instead of
  `RwSignal` (read-signals in, intent-callbacks out). `Tabs.active` and
  `TableWithPagination.current_page` become optional and gain callbacks
  (`on_change`, `on_page_change`); sort props on both table variants are
  read-only. `RadioGroup`'s underscored `_value`/`_on_change`/`_name`
  placeholders are promoted to real `value`/`on_change`/`name` props with
  group-managed context state. Controlled usage:
  `value=Some(read)` + `on_change=Callback::new(set)`.
```

## Report format

Per file: anchors matched / edits / adaptations. Demo call sites changed.
Gates: fmt, clippy, `--lib --all-features` count, wasm check, demo check.
Rule-7 stops with exact quotes.
