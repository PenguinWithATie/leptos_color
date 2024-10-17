use crate::{components::color_picker::ColorPicker, theme::Theme};
use csscolorparser::Color;
use floating_ui_leptos::{
    use_floating, Alignment, AutoPlacement, AutoPlacementOptions, AutoUpdateOptions,
    DetectOverflowOptions, Flip, FlipOptions, IntoReference, MiddlewareVec, Offset, OffsetOptions,
    Padding, Placement, Shift, ShiftOptions, UseFloatingOptions, UseFloatingReturn,
};
use html::{Div, Input};
use leptos::*;
#[component]
pub fn ColorInput(
    #[prop(into, default=Theme::default().into())] theme: MaybeSignal<Theme>,
    #[prop(into)] color: Signal<Color>,
    #[prop(into, optional)] hide_alpha: MaybeSignal<bool>,
    #[prop(into, optional)] hide_hex: MaybeSignal<bool>,
    #[prop(into, optional)] hide_rgb: MaybeSignal<bool>,
    #[prop(into)] on_change: Callback<Color>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let reference_ref = NodeRef::<Input>::new();
    let node_ref = NodeRef::<Div>::new();
    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(-1.0))),
        Box::new(Flip::new(FlipOptions::default().cross_axis(false))),
    ];
    let (open, set_open) = create_signal(false);
    let UseFloatingReturn {
        placement,
        floating_styles,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        node_ref,
        UseFloatingOptions::default()
            .open(open.into())
            .placement(Placement::Top.into())
            .middleware(middleware.into())
            .while_elements_mounted_auto_update(),
    );
    view! {
            <input
                class=class
                _ref=reference_ref
                on:mouseover=move |_| set_open.set(true)
                on:mouseleave=move |_| set_open.set(false)
                prop:value=move || {
                    let rgba = color.get().to_rgba8();
                    format!("rgba({},{},{},{})",rgba[0],rgba[1],rgba[2],rgba[3])
                }
                on:blur={move |ev| {
                    match event_target_value(&ev).parse::<Color>() {
                        Ok(new_color) => on_change.call(new_color),
                        Err(_) => {},
                    };
                }}
                on:change={move |ev| {
                    match event_target_value(&ev).parse::<Color>() {
                        Ok(new_color) => on_change.call(new_color),
                        Err(_) => {},
                    };
                }}
            >

            </input>
            <div
                node_ref={node_ref}
                style:display=move || match open.get() {
                            true => "block",
                            false => "none"
                        }
                style:border-width="5px"
                style:border-color="transparent"
                style:border-style="solid"
                style:position=move || floating_styles.get().style_position()
                style:top=move || floating_styles.get().style_top()
                style:left=move || floating_styles.get().style_left()
                style:transform=move || floating_styles.get().style_transform()
                style:will-change=move || floating_styles.get().style_will_change()
                style:z-index="1000"
                on:mouseenter=move |_| set_open.set(true)
                on:mouseover=move |_| set_open.set(true)
                on:mouseleave=move |_| set_open.set(false)
                >


                    <ColorPicker theme=theme color=color hide_hex=hide_hex hide_rgb=hide_rgb hide_alpha=hide_alpha on_change=move |new_color: Color| {
                        on_change.call(new_color);
                    }/>
            </div>
    }
}
