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
    AccordionItem, MingotProvider, Popover, PopoverDropdown, PopoverTarget, Select, SelectOption,
    Switch, Tabs, TabsList, TabsTab,
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
    let (ext, _set_ext) = signal(false);
    let (fired, set_fired) = signal(None::<bool>);

    let host = host_element();
    mount_to(host.clone(), move || {
        view! {
            <MingotProvider>
                <Popover
                    opened=ext
                    on_change=Callback::new(move |v: bool| set_fired.set(Some(v)))
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

    host.remove();
}

/// AccordionItem: clicking the control fires `on_change` with the new value and
/// the external signal is not written.
#[wasm_bindgen_test]
async fn accordion_item_toggle_fires_on_change() {
    let (ext, _set_ext) = signal(false);
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
