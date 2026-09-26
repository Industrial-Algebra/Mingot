# Contract B — 7C demo: Run button, report rendering, JSON export

Unit B of Phase 7C. Depends on: Contract A (built-in nodes — landed in the
working tree by the previous implementer). This is the 7C demo DoD: "a
working compute graph in the demo (exact decimal arithmetic), with
serialized results."

**You implement this file exactly.** Contracts win over prose. No git
commands. Pipe long outputs through grep/tail; timeouts SECONDS (cargo ≤ 600).

## Files

- MODIFY `demo/src/pages/node_graph.rs` — replace with the full final
  content fenced below (the whole file; nothing else).
- MODIFY `demo/Cargo.toml` — add the two pinned dependencies below its
  `[dependencies]` table's `mingot = ...` line, changing nothing else.

No other file may be created, modified, or deleted.

## Context you need

- `mingot::prelude` (already imported in the page) re-exports from
  `node_graph`: `CanvasPoint, Connection, NodeCanvas, NodeDefinition,
  NodeGraph, NodeId, PendingConnection, PortDef, PortType, Viewport`.
  Contract A additionally re-exports from `mingot::node_graph::nodes`
  (also reachable as `mingot::node_graph::{AddDecimal, Constant, ...}`):
  `Constant`, `AddDecimal` and friends — all are also root re-exports of
  `mingot::node_graph` (including `Engine`, `ExecutionReport`,
  `NodeOutcome`, `NodeOp`, `Value`, `ExecError`), which is how the page
  imports them (see the fence).
- `Value` is `Clone` + `serde::Serialize` (numerics serialize as strings
  preserving scale). `ExecError` implements `Display` (thiserror).
  `ExecutionReport { pub outcomes: BTreeMap<NodeId, NodeOutcome>, pub
  values: BTreeMap<(NodeId, u32), Value>, pub errors: Vec<(NodeId,
  ExecError)> }` and is `Clone`. `NodeOutcome::{Executed, Failed(
  ExecError), Skipped}` is `Clone + PartialEq`. `NodeId(pub u64)`.
- The demo's Mingot `Button` component takes `on_click=Callback::new(...)`
  (see `demo/src/pages/getting_started.rs` for working usage).
- Leptos 0.8: `signal()` returns `(ReadSignal, WriteSignal)`;
  `Callback::new` is `leptos::prelude::Callback`. Inside `view!`, a
  function returning `Option<Vec<_>>` of views renders fine.

## Contracts

### `demo/src/pages/node_graph.rs` — full final content

```rust
// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Node graph editor demo: a small precision pipeline (Constant → Add →
//! Result) wired through every NodeCanvas callback, plus a **Run** button
//! executing the graph on the 7C engine (exact decimal arithmetic,
//! refuse-to-fire numerics contract) with the structured
//! [`ExecutionReport`] rendered and exported as JSON.
//!
//! Pan with background drag, zoom with the wheel, drag from an output
//! port and drop on an input port to connect, click a node to select,
//! `Delete` removes it, arrow keys nudge.

use leptos::prelude::*;
use mingot::node_graph::{AddDecimal, Constant, Engine, ExecutionReport, NodeOp, NodeOutcome, Value};
use mingot::prelude::*;
use rust_decimal::Decimal;
use std::collections::BTreeMap;

/// Demo-local terminal op: consumes one decimal, produces nothing. The
/// value it received is still visible in the report's `values` map
/// because the *upstream* node's output is what gets recorded.
struct RunSink {
    def: mingot::node_graph::NodeDefinition,
}

impl NodeOp for RunSink {
    fn definition(&self) -> &mingot::node_graph::NodeDefinition {
        &self.def
    }
    fn evaluate(&self, _inputs: &[Value]) -> Result<Vec<Value>, mingot::node_graph::ExecError> {
        Ok(vec![])
    }
}

/// Serialize one engine run for export.
#[derive(serde::Serialize)]
struct ExportReport {
    nodes: Vec<ExportNode>,
    errors: Vec<String>,
}

#[derive(serde::Serialize)]
struct ExportNode {
    node: u64,
    outcome: String,
    value: Option<Value>,
}

fn outcome_label(outcome: &NodeOutcome) -> String {
    match outcome {
        NodeOutcome::Executed => "executed".to_string(),
        NodeOutcome::Failed(e) => format!("failed: {e}"),
        NodeOutcome::Skipped => "skipped".to_string(),
    }
}

fn value_label(value: &Value) -> String {
    match value {
        Value::Integer(v) => v.to_string(),
        Value::Decimal(d) => d.to_string(),
        Value::Arbitrary(d) => d.to_string(),
        Value::Text(t) => t.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Custom(c) => c.summary(),
    }
}

#[component]
pub fn NodeGraphPage() -> impl IntoView {
    // -- Model state (all owned here; NodeCanvas reports intent back) ------
    let (graph, set_graph) = signal(NodeGraph::new());
    let (positions, set_positions) = signal(BTreeMap::<NodeId, CanvasPoint>::new());
    let (selected, set_selected) = signal(BTreeMap::<NodeId, bool>::new());
    let (viewport, set_viewport) = signal(Viewport::default());
    let (last_report, set_last_report) = signal("No edits yet.".to_string());
    let (exec_report, set_exec_report) = signal(Option::<ExecutionReport>::None);
    let (exec_json, set_exec_json) = signal(String::new());

    // Seed the demo graph: c0 (out) -> a0 (in0), c1 (out) -> a0 (in1),
    // a0 (out) -> r0 (in0). Decimal(2) everywhere, all connections Ok.
    {
        let mut g = NodeGraph::new();
        g.add_node(
            NodeId(0),
            NodeDefinition::new(
                "Constant 10.00",
                vec![],
                vec![PortDef::new("value", PortType::Decimal(2))],
            ),
        );
        g.add_node(
            NodeId(1),
            NodeDefinition::new(
                "Constant 2.50",
                vec![],
                vec![PortDef::new("value", PortType::Decimal(2))],
            ),
        );
        g.add_node(
            NodeId(2),
            NodeDefinition::new(
                "Add",
                vec![
                    PortDef::new("lhs", PortType::Decimal(2)),
                    PortDef::new("rhs", PortType::Decimal(2)),
                ],
                vec![PortDef::new("sum", PortType::Decimal(2))],
            ),
        );
        g.add_node(
            NodeId(3),
            NodeDefinition::new(
                "Result",
                vec![PortDef::new("in", PortType::Decimal(2))],
                vec![],
            ),
        );
        g.connect(Connection::new(NodeId(0), 0, NodeId(2), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(2), 1));
        g.connect(Connection::new(NodeId(2), 0, NodeId(3), 0));
        set_graph.set(g);

        let mut pos = BTreeMap::new();
        pos.insert(NodeId(0), CanvasPoint::new(40.0, 60.0));
        pos.insert(NodeId(1), CanvasPoint::new(40.0, 220.0));
        pos.insert(NodeId(2), CanvasPoint::new(320.0, 130.0));
        pos.insert(NodeId(3), CanvasPoint::new(600.0, 130.0));
        set_positions.set(pos);
    }

    // -- Echo-back handlers ------------------------------------------------
    let on_node_move = Callback::new(move |(id, pt): (NodeId, CanvasPoint)| {
        set_positions.update(|p| {
            p.insert(id, pt);
        });
    });
    let on_connect = Callback::new(move |conn: Connection| {
        set_last_report.set(format!(
            "Connect {}:{} -> {}:{}",
            conn.from_node.0, conn.from_output, conn.to_node.0, conn.to_input
        ));
        set_graph.update(|g| {
            g.connect(conn);
        });
    });
    let on_node_select = Callback::new(move |(id, _): (NodeId, bool)| {
        // Exclusive selection: clicking a node clears all others.
        set_selected.update(|s| {
            s.retain(|_, v| {
                *v = false;
                true
            });
            s.insert(id, true);
        });
    });
    let on_node_delete = Callback::new(move |id: NodeId| {
        set_last_report.set(format!("Delete node {}", id.0));
        set_graph.update(|g| {
            g.remove_node(id);
        });
        set_positions.update(|p| {
            p.remove(&id);
        });
        set_selected.update(|s| {
            s.remove(&id);
        });
    });
    let on_viewport_change = Callback::new(move |vp: Viewport| {
        set_viewport.set(vp);
    });

    // -- Run: execute the graph on the 7C engine ---------------------------
    let on_run = Callback::new(move |_| {
        let g = graph.get_untracked();
        let mut ops: BTreeMap<NodeId, Box<dyn NodeOp>> = BTreeMap::new();
        let dec = |s: &str| Decimal::from_str_exact(s).expect("demo constants are exact");
        for (id, text) in [(NodeId(0), "10.00"), (NodeId(1), "2.50")] {
            if g.node(id).is_some() {
                let op: Box<dyn NodeOp> =
                    Constant::with_type(Value::Decimal(dec(text)), PortType::Decimal(2))
                        .map(|c| Box::new(c) as Box<dyn NodeOp>)
                        .expect("seeded constant is valid");
                ops.insert(id, op);
            }
        }
        if g.node(NodeId(2)).is_some() {
            ops.insert(NodeId(2), Box::new(AddDecimal::new(2, 2)));
        }
        if g.node(NodeId(3)).is_some() {
            ops.insert(
                NodeId(3),
                Box::new(RunSink {
                    def: NodeDefinition::new(
                        "Result",
                        vec![PortDef::new("in", PortType::Decimal(2))],
                        vec![],
                    ),
                }),
            );
        }
        match Engine::build(g, ops) {
            Ok(engine) => {
                let report = engine.execute();
                let export = ExportReport {
                    nodes: report
                        .outcomes
                        .iter()
                        .map(|(id, o)| ExportNode {
                            node: id.0,
                            outcome: outcome_label(o),
                            value: report.values.get(&(*id, 0)).cloned(),
                        })
                        .collect(),
                    errors: report.errors.iter().map(|(_, e)| e.to_string()).collect(),
                };
                let json = serde_json::to_string_pretty(&export).unwrap_or_default();
                set_exec_json.set(json);
                set_exec_report.set(Some(report));
                set_last_report.set("Run complete.".to_string());
            }
            Err(e) => {
                set_last_report.set(format!("engine build failed: {e}"));
                set_exec_report.set(None);
                set_exec_json.set(String::new());
            }
        }
    });

    let report = move || {
        let g = graph.get();
        format!(
            "{} | {} nodes, {} connections | zoom {:.2}",
            last_report.get(),
            g.node_ids().count(),
            g.connections().len(),
            viewport.get().zoom
        )
    };

    let exec_lines = move || {
        exec_report.get().map(|r| {
            r.outcomes
                .iter()
                .map(|(id, o)| {
                    let value = r
                        .values
                        .get(&(*id, 0))
                        .map(|v| format!(" = {}", value_label(v)))
                        .unwrap_or_default();
                    format!("#{}: {}{}", id.0, outcome_label(o), value)
                })
                .collect::<Vec<_>>()
        })
    };

    view! {
        <div class="demo-section">
            <h2>"Node Graph Editor"</h2>
            <p>
                "Pan with background drag, zoom with the wheel, drag output → input to connect, "
                "click a node to select, Delete to remove, arrow keys to nudge (Shift = fine)."
            </p>
            <p class="demo-hint">{report}</p>
            <div
                class="node-graph-frame"
                style="height: 420px; border: 1px solid #d1d5db; border-radius: 8px; overflow: hidden;"
            >
                <NodeCanvas
                    graph=graph
                    positions=positions
                    selected=selected
                    viewport=viewport
                    on_node_move=on_node_move
                    on_connect=on_connect
                    on_node_select=on_node_select
                    on_node_delete=on_node_delete
                    on_viewport_change=on_viewport_change
                />
            </div>
            <div style="margin-top: 12px; display: flex; gap: 8px; align-items: center;">
                <Button on_click=on_run>"Run graph"</Button>
                <span class="demo-hint">
                    "Executes with the 7C engine: exact decimal arithmetic; lossy or
                     incompatible edges refuse to fire."
                </span>
            </div>
            {move || {
                exec_lines()
                    .map(|lines| {
                        view! {
                            <div style="margin-top: 12px;">
                                <h3>"Execution report"</h3>
                                <ul>
                                    {lines
                                        .into_iter()
                                        .map(|l| view! { <li>{l}</li> })
                                        .collect::<Vec<_>>()}
                                </ul>
                                <h3>"Result JSON"</h3>
                                <pre class="demo-hint" style="text-align: left;">{exec_json.get()}</pre>
                            </div>
                        }
                    })
            }}
        </div>
    }
}
```

### `demo/Cargo.toml` — anchored insert

After the line `mingot = { path = "..", features = ["high-precision", "theme-tokens", "node-graph"] }`, add:

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rust_decimal = { version = "1.42", default-features = false }
```

Change nothing else. (The page calls `Decimal::from_str_exact` directly,
so the demo must declare `rust_decimal`; the pin mirrors the library's.)

## Tests

This unit has no new unit tests (the demo crate has no test harness); the
Completion gates below are the verification. Do not add test files.

## Constraints

- No new crates beyond the two pinned `serde`/`serde_json` entries.
- No changes to `src/` (the library) — library tests must remain green
  untouched.
- License header at the top of the page file exactly as fenced.

## Completion

Run, in the repo root, fixing until all green:

```bash
cargo +nightly test --all-features 2>&1 | grep -E 'test result|FAILED|error' | tail -6
cargo +nightly build -p mingot-demo --target wasm32-unknown-unknown 2>&1 | grep -E '^error|warning: unused' | tail -10; echo "WASM-DEMO-EXIT:$?"
cargo +nightly clippy --all-targets --all-features -- -D warnings 2>&1 | grep -E '^(error|warning)' | tail -8
cargo +nightly fmt
cargo +nightly fmt -- --check && echo FMT-OK
```

Note: the wasm build command's exit code prints as WASM-DEMO-EXIT — it
must be 0. If `grep` finds nothing the build is clean; the exit code line
still prints (it belongs to `echo`, not grep).

## Out of scope

- Any file under `src/` (the library). The `tests/node_graph_wasm.rs`
  browser harness. Routing/sidebar changes (the page is already routed).
- Executing the demo in a browser. Visual styling beyond what's fenced.

## Finish

Report: files touched, gate results (test counts, wasm exit, clippy,
fmt), and any ambiguity you resolved and how.
