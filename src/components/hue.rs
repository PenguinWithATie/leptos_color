use csscolorparser::Color;
use leptos::logging::warn;
use leptos::*;

use crate::{
    hooks::use_position::{use_position, UsePositionProps},
    mount_style::mount_style,
};
#[component]
pub fn Hue(#[prop(into)] on_change: Callback<(f64, f64)>) -> impl IntoView {
    mount_style("Hue", include_str!("./hue.css"));
    let handle_move =
        Callback::new(move |(left, top): (f64, f64)| Callable::call(&on_change, (left, top)));

    // Use the `use_position` hook to get the ref and handle_start function
    let (ref_div, handle_start) = use_position(UsePositionProps {
        on_move: handle_move.clone(),
    });
    view! {
        <div class="leptos-color-hue-container" node_ref={ref_div} on:touchstart=move |ev| {
            Callable::call(&handle_start, ev.into())} on:mousedown=move |ev| {
            Callable::call(&handle_start, ev.into())}>
            <div class="leptos-color-hue-pointer">
                <div class="leptos-color-hue-slider" />
            </div>
        </div>
    }
}
