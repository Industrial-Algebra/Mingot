// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! In-browser tests for the echo-back component API convention on the
//! boolean-toggle components (`Switch`, `Popover`, `AccordionItem`).
//!
//! These run only on wasm32 (empty no-op on native builds), so the standard
//! `cargo test` suite is unaffected. Run them locally with a browser:
//!
//! ```text
//! rustup target add wasm32-unknown-unknown
//! cargo install wasm-bindgen-cli --version 0.2.125
//! cargo test --target wasm32-unknown-unknown --test echo_back_wasm --no-run
//! wasm-bindgen-test-runner $(ls -t target/wasm32-unknown-unknown/debug/deps/echo_back_wasm-*.wasm | head -1)
//! ```
//!
//! What is under test: the child owns an internal echo signal seeded from an
//! optional external `ReadSignal`; user actions move the internal state and
//! fire the intent callback, and external changes propagate inward — the
//! external signal is never written by the component.

#![cfg(target_arch = "wasm32")]

use leptos::mount::mount_to;
use leptos::prelude::*;
use mingot::{
    AccordionItem, Matrix, MatrixInput, MingotProvider, Popover, PopoverDropdown, PopoverTarget,
    Select, SelectOption, Switch, TableColumn, TableWithPagination, Tabs, TabsList, TabsTab,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Pump queued effects so assertions observe settled reactive state.
async fn settle() {
    for _ in 0..4 {
        any_spawner::Executor::tick().await;
    }
}

/// Create a host element attached to the document body.
fn host_element() -> web_sys::HtmlElement {
    let doc = document();
    let host = doc.create_element("div").unwrap();
    doc.body().unwrap().append_child(&host).unwrap();
    host.unchecked_into()
}

/// Dispatch a synthetic (bubbling) `click` on `target`.
fn click(target: &web_sys::Element) {
    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    let ev = web_sys::MouseEvent::new_with_mouse_event_init_dict("click", &init)
        .expect("synthetic click event");
    let ev: web_sys::Event = ev.into();
    let _ = target.dispatch_event(&ev);
}

/// The inline `left` offset of the switch thumb ("2px" off, "24px" on for the
/// default `Md` size).
fn thumb_left(host: &web_sys::HtmlElement) -> String {
    host.query_selector(".mingot-switch-thumb")
        .expect("query failed")
        .expect("switch thumb rendered")
        .get_attribute("style")
        .unwrap_or_default()
}

/// `display` value of the popover dropdown's inline style.
fn dropdown_style(host: &web_sys::HtmlElement) -> String {
    host.query_selector(".mingot-popover-dropdown")
        .expect("query failed")
        .expect("popover dropdown rendered")
        .get_attribute("style")
        .unwrap_or_default()
}

/// Current `value` of the Nth matrix cell input (row-major order).
fn cell_value(host: &web_sys::HtmlElement, index: usize) -> String {
    cell_input(host, index).value()
}

/// The Nth matrix cell input element (row-major order).
fn cell_input(host: &web_sys::HtmlElement, index: usize) -> web_sys::HtmlInputElement {
    host.query_selector(&format!(
        ".mingot-matrix-input input:nth-child({})",
        index + 1
    ))
    .expect("query failed")
    .expect("matrix cell rendered")
    .unchecked_into::<web_sys::HtmlInputElement>()
}

/// Set the Nth matrix cell input to `text` and dispatch an `input` event, as a
/// user typing would.
fn type_into_cell(host: &web_sys::HtmlElement, index: usize, text: &str) {
    let cell = cell_input(host, index);
    cell.set_value(text);
    let ev = web_sys::Event::new("input").expect("synthetic input event");
    let _ = cell.dispatch_event(&ev);
}

/// Switch: a click fires `on_change` with the new value and moves the *internal*
/// echo state (the view reflects ON); the external signal is not written.
/// Setting the external signal propagates inward without a click.
#[wasm_bindgen_test]
async fn switch_external_syncs_and_callback_fires() {
    let (ext, set_ext) = signal(false);
    let (fired, set_fired) = signal(None::<bool>);

    let host = host_element();
    mount_to(host.clone(), move || {
        view! {
            <MingotProvider>
                <Switch
                    checked=ext
                    on_change=Callback::new(move |v: bool| set_fired.set(Some(v)))
                />
            </MingotProvider>
        }
    })
    .forget();
    settle().await;

    // OFF to start (seeded from the external signal).
    assert!(
        thumb_left(&host).contains("left: 2px"),
        "initial state should be OFF; style={}",
        thumb_left(&host)
    );

    let switch = host
        .query_selector(".mingot-switch")
        .expect("query failed")
        .expect("switch root rendered");
    click(&switch);
    settle().await;

    assert_eq!(
        fired.get_untracked(),
        Some(true),
        "click must fire on_change(true)"
    );
    assert!(
        thumb_left(&host).contains("left: 24px"),
        "view must reflect ON after the click; style={}",
        thumb_left(&host)
    );
    assert!(
        !ext.get_untracked(),
        "component must not write the parent's external signal"
    );

    // External change propagates inward without any click.
    set_ext.set(true);
    settle().await;
    assert!(
        thumb_left(&host).contains("left: 24px"),
        "external true must sync inward (ON); style={}",
        thumb_left(&host)
    );

    set_ext.set(false);
    settle().await;
    assert!(
        thumb_left(&host).contains("left: 2px"),
        "external false must sync inward (OFF); style={}",
        thumb_left(&host)
    );

    host.remove();
}

/// Popover: clicking `PopoverTarget` toggles the internal state and fires the
/// new `on_change`; the dropdown becomes visible and the external signal is
/// not written.
#[wasm_bindgen_test]
async fn popover_target_toggles_and_fires_on_change() {
    let (ext, set_ext) = signal(false);
    let (fired, set_fired) = signal(None::<bool>);
    let (fired_count, set_fired_count) = signal(0u32);

    let host = host_element();
    mount_to(host.clone(), move || {
        view! {
            <MingotProvider>
                <Popover
                    opened=ext
                    on_change=Callback::new(move |v: bool| {
                        set_fired.set(Some(v));
                        set_fired_count.update(|c| *c += 1);
                    })
                >
                    <PopoverTarget>"toggle"</PopoverTarget>
                    <PopoverDropdown>"content"</PopoverDropdown>
                </Popover>
            </MingotProvider>
        }
    })
    .forget();
    settle().await;

    assert!(
        dropdown_style(&host).contains("display: none"),
        "dropdown starts hidden; style={}",
        dropdown_style(&host)
    );

    let target = host
        .query_selector(".mingot-popover-target")
        .expect("query failed")
        .expect("popover target rendered");
    click(&target);
    settle().await;

    assert_eq!(
        fired.get_untracked(),
        Some(true),
        "target click must fire on_change(true)"
    );
    assert!(
        dropdown_style(&host).contains("display: block"),
        "dropdown must be visible after toggle; style={}",
        dropdown_style(&host)
    );
    assert!(
        !ext.get_untracked(),
        "component must not write the parent's external signal"
    );

    // Inward sync: an external change must reach the rendered state without
    // emitting a new intent callback (the mirrors-external guard).
    let fired_before = fired_count.get_untracked();
    set_ext.set(false);
    settle().await;
    assert!(
        !dropdown_style(&host).contains("display: block"),
        "external set(false) must close the dropdown; style={}",
        dropdown_style(&host)
    );
    assert_eq!(
        fired_count.get_untracked(),
        fired_before,
        "external sync must not re-fire on_change"
    );

    host.remove();
}

/// AccordionItem: clicking the control fires `on_change` with the new value and
/// the external signal is not written.
#[wasm_bindgen_test]
async fn accordion_item_toggle_fires_on_change() {
    let (ext, set_ext) = signal(false);
    let (fired, set_fired) = signal(None::<bool>);

    let host = host_element();
    mount_to(host.clone(), move || {
        view! {
            <MingotProvider>
                <AccordionItem
                    _value="a"
                    label="Header"
                    opened=ext
                    on_change=Callback::new(move |v: bool| set_fired.set(Some(v)))
                >
                    "body"
                </AccordionItem>
            </MingotProvider>
        }
    })
    .forget();
    settle().await;

    let control = host
        .query_selector(".mingot-accordion-control")
        .expect("query failed")
        .expect("accordion control rendered");
    click(&control);
    settle().await;

    assert_eq!(
        fired.get_untracked(),
        Some(true),
        "toggle must fire on_change(true)"
    );
    assert!(
        !ext.get_untracked(),
        "component must not write the parent's external signal"
    );
    assert!(
        item_body_style(&host).contains("1000px"),
        "item body must render open after toggle; style={}",
        item_body_style(&host)
    );

    // Inward sync: external close must reach the rendered state without a
    // new intent callback (fired would flip to Some(false) on a refire).
    set_ext.set(false);
    settle().await;
    assert!(
        !item_body_style(&host).contains("1000px"),
        "external set(false) must close the item body; style={}",
        item_body_style(&host)
    );
    assert_eq!(
        fired.get_untracked(),
        Some(true),
        "external sync must not re-fire on_change"
    );

    host.remove();
}

/// The inline `style` attribute of the tab header matching `selector`.
fn tab_style(host: &web_sys::HtmlElement, selector: &str) -> String {
    host.query_selector(selector)
        .expect("query failed")
        .expect("tab rendered")
        .get_attribute("style")
        .unwrap_or_default()
}

/// Tabs: an external active value seeds and syncs inward; clicking a tab fires
/// `on_change` and does not write the external signal.
#[wasm_bindgen_test]
async fn tabs_external_active_syncs_and_on_change_fires() {
    let (ext, set_ext) = signal("a".to_string());
    let (fired, set_fired) = signal(None::<String>);

    let host = host_element();
    mount_to(host.clone(), move || {
        view! {
            <MingotProvider>
                <Tabs
                    active=ext
                    on_change=Callback::new(move |v: String| set_fired.set(Some(v)))
                >
                    <TabsList>
                        <TabsTab value="a">"A"</TabsTab>
                        <TabsTab value="b">"B"</TabsTab>
                    </TabsList>
                </Tabs>
            </MingotProvider>
        }
    })
    .forget();
    settle().await;

    // "a" is active: its style carries the active underline; "b" is transparent.
    assert!(
        tab_style(&host, ".mingot-tabs-tab:nth-child(1)").contains("margin-bottom: -2px"),
        "tab a should be active initially; style={}",
        tab_style(&host, ".mingot-tabs-tab:nth-child(1)")
    );
    assert!(
        tab_style(&host, ".mingot-tabs-tab:nth-child(2)").contains("transparent"),
        "tab b should be inactive initially; style={}",
        tab_style(&host, ".mingot-tabs-tab:nth-child(2)")
    );

    // External change propagates inward without a click.
    set_ext.set("b".to_string());
    settle().await;
    assert!(
        tab_style(&host, ".mingot-tabs-tab:nth-child(1)").contains("transparent"),
        "external b must sync inward (a inactive); style={}",
        tab_style(&host, ".mingot-tabs-tab:nth-child(1)")
    );
    assert!(
        tab_style(&host, ".mingot-tabs-tab:nth-child(2)").contains("margin-bottom: -2px"),
        "external b must sync inward (b active); style={}",
        tab_style(&host, ".mingot-tabs-tab:nth-child(2)")
    );

    // Clicking tab "a" fires on_change("a") and moves the internal state.
    let tab_a = host
        .query_selector(".mingot-tabs-tab:nth-child(1)")
        .expect("query failed")
        .expect("tab a rendered");
    click(&tab_a);
    settle().await;

    assert_eq!(
        fired.get_untracked(),
        Some("a".to_string()),
        "clicking tab a must fire on_change(\"a\")"
    );
    assert!(
        tab_style(&host, ".mingot-tabs-tab:nth-child(1)").contains("margin-bottom: -2px"),
        "internal state must reflect the clicked tab; style={}",
        tab_style(&host, ".mingot-tabs-tab:nth-child(1)")
    );
    assert_eq!(
        ext.get_untracked(),
        "b".to_string(),
        "component must not write the parent's external signal"
    );

    host.remove();
}

/// Select: an external value seeds and syncs inward; a change on the native
/// select fires `on_change` and does not write the external signal.
#[wasm_bindgen_test]
async fn select_on_change_fires_and_external_syncs() {
    let (ext, set_ext) = signal("a".to_string());
    let (fired, set_fired) = signal(None::<String>);

    let host = host_element();
    mount_to(host.clone(), move || {
        view! {
            <MingotProvider>
                <Select
                    value=ext
                    options=vec![
                        SelectOption::new("a", "A"),
                        SelectOption::new("b", "B"),
                    ]
                    on_change=Callback::new(move |v: String| set_fired.set(Some(v)))
                />
            </MingotProvider>
        }
    })
    .forget();
    settle().await;

    let select = host
        .query_selector(".mingot-select")
        .expect("query failed")
        .expect("select rendered")
        .unchecked_into::<web_sys::HtmlSelectElement>();

    // External value seeds the rendered selection.
    assert_eq!(
        select.value(),
        "a",
        "external value must seed the rendered selection"
    );

    // External change propagates inward without a user event.
    set_ext.set("b".to_string());
    settle().await;
    assert_eq!(select.value(), "b", "external value must sync inward");

    // A native change fires on_change but does not write the external signal.
    select.set_value("a");
    let ev = web_sys::Event::new("change").expect("synthetic change event");
    let _ = select.dispatch_event(&ev);
    settle().await;

    assert_eq!(
        fired.get_untracked(),
        Some("a".to_string()),
        "change must fire on_change(\"a\")"
    );
    assert_eq!(
        ext.get_untracked(),
        "b".to_string(),
        "component must not write the parent's external signal"
    );

    host.remove();
}

/// MatrixInput: an external `ReadSignal<Matrix>` seeds the rendered cells and
/// syncs inward; typing in a cell fires `on_change` with the updated matrix and
/// does not write the external signal.
#[wasm_bindgen_test]
async fn matrix_input_external_syncs_and_on_change_fires() {
    let (ext, set_ext) = signal(Matrix::identity(2));
    let (fired, set_fired) = signal(None::<Matrix>);

    let host = host_element();
    mount_to(host.clone(), move || {
        view! {
            <MingotProvider>
                <MatrixInput
                    value=ext
                    rows=2
                    cols=2
                    on_change=Callback::new(move |m: Matrix| set_fired.set(Some(m)))
                />
            </MingotProvider>
        }
    })
    .forget();
    settle().await;

    // External matrix seeds the rendered cells (row-major: 1,0,0,1).
    assert_eq!(cell_value(&host, 0), "1", "cell (0,0) seeds from external");
    assert_eq!(cell_value(&host, 1), "0", "cell (0,1) seeds from external");
    assert_eq!(cell_value(&host, 2), "0", "cell (1,0) seeds from external");
    assert_eq!(cell_value(&host, 3), "1", "cell (1,1) seeds from external");

    // External change propagates inward without a user event.
    let mut updated = Matrix::zeros(2, 2);
    updated.set(0, 0, 5.0);
    set_ext.set(updated);
    settle().await;
    assert_eq!(
        cell_value(&host, 0),
        "5",
        "external change must sync inward"
    );

    // Typing into a cell fires on_change with the updated matrix but does not
    // write the external signal.
    type_into_cell(&host, 1, "7");
    settle().await;

    let fired_matrix = fired.get_untracked().expect("on_change must fire");
    assert_eq!(
        fired_matrix.get(0, 1),
        Some(7.0),
        "on_change must carry the edited matrix"
    );
    assert_eq!(
        fired_matrix.get(0, 0),
        Some(5.0),
        "on_change matrix keeps prior synced state"
    );
    assert_eq!(
        ext.get_untracked().get(0, 1),
        Some(0.0),
        "component must not write the parent's external signal"
    );

    host.remove();
}

/// The inline `style` attribute of the accordion item's panel.
fn item_body_style(host: &web_sys::HtmlElement) -> String {
    host.query_selector(".mingot-accordion-panel")
        .expect("query failed")
        .expect("accordion panel rendered")
        .get_attribute("style")
        .unwrap_or_default()
}

/// TableWithPagination without `current_page` must default to page 1
/// (pagination is 1-based); page 0 would underflow the slice offset.
/// Regression for the review finding on the uncontrolled default.
#[wasm_bindgen_test]
async fn table_with_pagination_uncontrolled_defaults_to_page_one() {
    let host = host_element();
    mount_to(host.clone(), || {
        view! {
            <MingotProvider>
                <TableWithPagination
                    columns=vec![TableColumn::new("v", "V", |n: &u32| n.to_string())]
                    data=Signal::derive(|| vec![42u32])
                    page_size=Signal::derive(|| 10usize)
                />
            </MingotProvider>
        }
    })
    .forget();
    settle().await;

    assert!(
        host.text_content().unwrap().contains("42"),
        "uncontrolled table must render the first page's rows; text={}",
        host.text_content().unwrap_or_default()
    );

    host.remove();
}

/// Resize intents must reach the parent: clicking + Row fires `on_change`
/// with the resized matrix while the external signal stays unwritten.
/// Regression for the review finding on resize handlers.
#[wasm_bindgen_test]
async fn matrix_resize_reports_intent() {
    let (ext, _set_ext) = signal(Matrix::identity(2));
    let (fired, set_fired) = signal(None::<Matrix>);

    let host = host_element();
    mount_to(host.clone(), move || {
        view! {
            <MingotProvider>
                <MatrixInput
                    value=ext
                    allow_resize=true
                    on_change=Callback::new(move |m| set_fired.set(Some(m)))
                />
            </MingotProvider>
        }
    })
    .forget();
    settle().await;

    let button = host
        .query_selector(".mingot-matrix-input button")
        .expect("query failed")
        .expect("resize button rendered");
    assert!(
        button.text_content().unwrap().contains("Row"),
        "first resize button should be add-row; text={}",
        button.text_content().unwrap_or_default()
    );
    click(&button);
    settle().await;

    assert!(
        host.query_selector(".mingot-matrix-input input:nth-child(6)")
            .expect("query failed")
            .is_some(),
        "a sixth cell input must render for the 3-row matrix"
    );
    assert_eq!(
        ext.get_untracked().rows(),
        2,
        "component must not write the parent's external signal"
    );
    assert_eq!(
        fired.get_untracked().map(|m| m.rows()),
        Some(3),
        "resize must emit the new 3-row matrix via on_change"
    );

    host.remove();
}
