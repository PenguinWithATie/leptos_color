use crate::{components::color_picker::ColorPicker, theme::Theme};
use csscolorparser::Color;
use floating_ui_leptos::{
    use_floating, Alignment, AutoPlacement, AutoPlacementOptions, AutoUpdateOptions,
    DetectOverflowOptions, Flip, FlipOptions, IntoReference, MiddlewareVec, Offset, OffsetOptions,
    Padding, Placement, Shift, ShiftOptions, UseFloatingOptions, UseFloatingReturn,
};
use html::{Div, Input};
use leptos::*;
use wasm_bindgen::JsCast as _;
/// A color input component with a clickable color picker popover.
///
/// This component provides an input field for color values and a floating color picker
/// that appears when the user clicks on the input. The picker can be dismissed by clicking
/// outside or clicking the input again.
///
/// # Props
///
/// * `theme`: A `MaybeSignal<Theme>` representing the theme for the component. Defaults to `Theme::default()`.
/// * `color`: A `Signal<Color>` representing the current color value.
/// * `hide_alpha`: An optional `MaybeSignal<bool>` to hide the alpha channel in the color picker.
/// * `hide_hex`: An optional `MaybeSignal<bool>` to hide the hexadecimal color input in the color picker.
/// * `hide_rgb`: An optional `MaybeSignal<bool>` to hide the RGB color input in the color picker.
/// * `on_change`: A `Callback<Color>` that is called when the color value changes.
/// * `class`: An optional `MaybeProp<String>` for additional CSS classes to apply to the input element.
///
/// # Behavior
///
/// - The input field displays the current color value in RGBA format.
/// - Clicking the input field toggles the color picker popover.
/// - The color picker closes when clicking outside or clicking the input again.
/// - The color picker floats relative to the input using the `floating_ui_leptos` crate.
/// - Changes to the color can be made either by editing the input field directly or using the color picker.
/// - The `on_change` callback is triggered when a valid color value is entered or selected.
///
/// # Example
///
/// ```rust
/// use leptos::*;
/// use csscolorparser::Color;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let (color, set_color) = create_signal(Color::from_rgba8(255, 0, 0, 255));
///
///     view! {
///         <ColorInput
///             color=color
///             on_change=move |new_color| set_color.set(new_color)
///         />
///     }
/// }
/// ```
///
/// # Styling
///
/// The component comes with basic styling for the popover including:
/// - Box shadow for elevation
/// - Border radius for rounded corners
/// - Smooth opacity transition for showing/hiding
/// - Backdrop blur effect (when supported by the browser)
///
/// Additional styling can be applied through the `class` prop for the input element
/// or by targeting the `.color-input-container` and `.color-picker-popover` classes.
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
    let floating_ref = NodeRef::<Div>::new();
    let (open, set_open) = create_signal(false);

    // Click outside detection
    let click_outside = window_event_listener(ev::click, move |ev| {
        if !open.get() {
            return;
        }

        let target = ev.target();
        let target_node = target.and_then(|t| t.dyn_into::<web_sys::Node>().ok());

        if let Some(target_node) = target_node {
            if !reference_ref
                .get()
                .map(|r| r.contains(Some(&target_node)))
                .unwrap_or(false)
                && !floating_ref
                    .get()
                    .map(|f| f.contains(Some(&target_node)))
                    .unwrap_or(false)
            {
                set_open.set(false);
            }
        }
    });

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(8.0))), // Increased offset
        Box::new(Flip::new(FlipOptions::default().cross_axis(false))),
    ];

    on_cleanup(move || {
        set_open.set(false);
        click_outside.remove();
    });
    let UseFloatingReturn {
        floating_styles, ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .open(open.into())
            .placement(Placement::Bottom.into())
            .middleware(middleware.into())
            .while_elements_mounted_auto_update(),
    );

    view! {
        <div class="color-input-container" style="position: relative;">
            <input
                class=class
                _ref=reference_ref
                on:click=move |_| set_open.update(|open| *open = !*open)
                prop:value=move || {
                    let rgba = color.get().to_rgba8();
                    format!("rgba({},{},{},{})", rgba[0], rgba[1], rgba[2], rgba[3])
                }
                on:change=move |ev| {
                    if let Ok(new_color) = event_target_value(&ev).parse::<Color>() {
                        on_change.call(new_color);
                    }
                }
            />
            <div
                node_ref=floating_ref
                class="color-picker-popover"
                style:position="absolute"
                style:display=move || if open.get() { "block" } else { "none" }
                style:top=move || floating_styles.get().style_top()
                style:left=move || floating_styles.get().style_left()
                style:transform=move || floating_styles.get().style_transform()
                style:background-color="#fff"
                style:box-shadow="0 2px 10px rgba(0, 0, 0, 0.1)"
                style:border-radius="4px"
                style:z-index="1000"
                style:opacity=move || if open.get() { "1" } else { "0" }
                style:transition="opacity 0.2s ease-in-out"
            >
                <ColorPicker
                    theme=theme
                    color=color
                    hide_hex=hide_hex
                    hide_rgb=hide_rgb
                    hide_alpha=hide_alpha
                    on_change=move |new_color| {
                        on_change.call(new_color);
                    }
                />
            </div>
        </div>
    }
}
