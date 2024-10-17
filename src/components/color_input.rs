use crate::{components::color_picker::ColorPicker, theme::Theme};
use csscolorparser::Color;
use floating_ui_leptos::{
    use_floating, Alignment, AutoPlacement, AutoPlacementOptions, AutoUpdateOptions,
    DetectOverflowOptions, Flip, FlipOptions, IntoReference, MiddlewareVec, Offset, OffsetOptions,
    Padding, Placement, Shift, ShiftOptions, UseFloatingOptions, UseFloatingReturn,
};
use html::{Div, Input};
use leptos::*;
/// A color input component with a floating color picker.
///
/// This component provides an input field for color values and a floating color picker
/// that appears when the user hovers over the input.
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
/// - Hovering over the input field shows the color picker.
/// - The color picker floats above the input field using the `floating_ui_leptos` crate.
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
/// This example creates a `ColorInput` component with a default red color and updates the color
/// when changed.
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
