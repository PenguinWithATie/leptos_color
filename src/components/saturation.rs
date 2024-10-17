use csscolorparser::Color;
use leptos::logging::warn;
use leptos::*;

use crate::{
    hooks::use_position::{use_position, UsePositionProps},
    mount_style::mount_style,
};

#[component]
pub fn Saturation(#[prop(into)] on_change: Callback<(f64, f64)>) -> impl IntoView {
    mount_style("Saturation", include_str!("./saturation.css"));
    // Callback for position changes, updates the color based on left and top
    // let on_change = move |new_hsl: HSL| {
    //     set_hsl.set(new_hsl);
    //     // You can add additional logic if needed
    //     log::info!("HSL updated: {:?}", new_hsl);
    // };

    // Closure that handles the position move
    let handle_move = Callback::new(move |(left, top): (f64, f64)| on_change.call((left, top)));

    // Use the `use_position` hook to get the ref and handle_start function
    let (ref_div, handle_start) = use_position(UsePositionProps {
        on_move: handle_move.clone(),
    });
    view! {
        <div node_ref={ref_div} class="leptos-color-color" on:touchstart=move |ev| {
            Callable::call(&handle_start, ev.into());} on:mousedown=move |ev| {
            Callable::call(&handle_start, ev.into());}>
            <style>r"
            .saturation-white {
                background: -webkit-linear-gradient(to right, #fff, rgba(255,255,255,0));
                background: linear-gradient(to right, #fff, rgba(255,255,255,0));
            }
            .saturation-black {
                background: -webkit-linear-gradient(to top, #000, rgba(0,0,0,0));
                background: linear-gradient(to top, #000, rgba(0,0,0,0));
            }
            "</style>
            <div class="saturation-white leptos-color-gradient">
            <div class="saturation-black leptos-color-gradient" />
            <div class="leptos-color-pointer">
                <div class="leptos-color-circle" />
            </div>
            </div>
        </div>
    }
}
