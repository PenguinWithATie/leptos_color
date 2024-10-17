use crate::components::alpha::Alpha;
use crate::components::hue::Hue;
use crate::theme::Theme;
use crate::{components::saturation::Saturation, mount_style::mount_style};
use csscolorparser::Color;
use html::Div;
use leptos::logging::warn;
use leptos::*;
use leptos_use::{use_css_var_with_options, UseCssVarOptions};
#[component]
pub fn ColorPicker(
    #[prop(into, default=Theme::default().into())] theme: MaybeSignal<Theme>,
    #[prop(into)] color: Signal<Color>,
    #[prop(into, optional)] hide_alpha: MaybeSignal<bool>,
    #[prop(into, optional)] hide_hex: MaybeSignal<bool>,
    #[prop(into, optional)] hide_rgb: MaybeSignal<bool>,
    #[prop(into)] on_change: Callback<Color>,
) -> impl IntoView {
    mount_style("ColorPicker", include_str!("./color_picker.css"));
    let el = create_node_ref::<Div>();
    let (hue, set_hue) = use_css_var_with_options(
        "--lpc-hue",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0")
            .observe(false),
    );

    // Define the other CSS variables
    let (red, set_red) = use_css_var_with_options(
        "--lpc-red",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0")
            .observe(false),
    );

    let (green, set_green) = use_css_var_with_options(
        "--lpc-green",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0")
            .observe(false),
    );

    let (blue, set_blue) = use_css_var_with_options(
        "--lpc-blue",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0")
            .observe(false),
    );

    let (hex, set_hex) = use_css_var_with_options(
        "--lpc-hex",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("#000000")
            .observe(false),
    );

    let (alpha, set_alpha) = use_css_var_with_options(
        "--lpc-alpha",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("1")
            .observe(false),
    );

    let (rgba, set_rgba) = use_css_var_with_options(
        "--lpc-rgba",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("rgba(0, 0, 0, 1)")
            .observe(false),
    );

    let (hue_pointer, set_hue_pointer) = use_css_var_with_options(
        "--lpc-hue-pointer",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0%")
            .observe(false),
    );

    let (alpha_pointer, set_alpha_pointer) = use_css_var_with_options(
        "--lpc-alpha-pointer",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("100%")
            .observe(false),
    );

    let (saturation_pointer_top, set_saturation_pointer_top) = use_css_var_with_options(
        "--lpc-saturation-pointer-top",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("calc(100% - 6px)")
            .observe(false),
    );

    let (saturation_pointer_left, set_saturation_pointer_left) = use_css_var_with_options(
        "--lpc-saturation-pointer-left",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("calc(0% - 6px)")
            .observe(false),
    );

    // React to color changes and update CSS variables
    Effect::new(move |_| {
        color.track();
        let hsla = color.get().to_hsla();
        let rgba = color.get().to_rgba8();
        let alpha = rgba[3];
        let hex = color.get().to_hex_string();
        let hsva = color.get().to_hsva();

        set_hue.set((hsla[0] as u16).to_string());
        set_red.set(rgba[0].to_string());
        set_green.set(rgba[1].to_string());
        set_blue.set(rgba[2].to_string());
        set_hex.set(hex);
        set_alpha.set(alpha.to_string());
        set_rgba.set(format!(
            "rgba({}, {}, {}, {})",
            rgba[0],
            rgba[1],
            rgba[2],
            (alpha as f32 / 255.0)
        ));
        set_hue_pointer.set(format!("{}%", (hsla[0] * 100.0 / 360.0).round()));
        set_alpha_pointer.set(format!("{}%", (alpha as f32 / 255.0 * 100.0).round()));
        set_saturation_pointer_top.set(format!("calc({}% - 6px)", -(hsva[2] * 100.0) + 100.0));
        set_saturation_pointer_left.set(format!("calc({}% - 6px)", (hsva[1] * 100.0).round()));
    });

    view! {
        <div node_ref={el} class="leptos-color-container" style=move || theme.with(|value| value.to_style())>
            <Saturation on_change=move |(left,top): (f64,f64)| {
                let mut hsva = color.get().to_hsva();
                hsva[2] = (1.0 - top) as f32;
                hsva[1] = left as f32;
                if hsva[2] <= 0.0 {
                    hsva[2] = 0.001;
                }
                if hsva[1] <= 0.0 {
                    hsva[1] = 0.001;
                }
                on_change.call(Color::from_hsva(hsva[0], hsva[1], hsva[2], hsva[3]));
            }/>
            <div class="leptos-color-flex">
                <div class="leptos-color-value-wrapper">
                    <div class="leptos-color-checkboard">
                        <div class="leptos-color-value" />
                    </div>
                </div>
                <div class="leptos-color-ranges">
                    <Hue on_change=move |(left,_)| {
                        let hsla = color.get().to_hsla();
                        on_change.call(Color::from_hsla((left*360.0) as f32, hsla[1], hsla[2], hsla[3]));
                    } />
                    <Show
                        when=move || { !hide_alpha.get()}
                      >
                      <Alpha on_change=move |(left,_)| {
                          let mut color = color.get();
                          color.a = left as f32;
                          on_change.call(color);
                      }/>
                    </Show>
                </div>
            </div>

            <div class="leptos-color-inputs">
                <Show
                    when=move || { !hide_hex.get()}
                >
                <label class="leptos-color-label">
                    <div class="leptos-color-wrapper">
                        <span class="leptos-color-prefix">"#"</span>

                        <input
                        class="leptos-color-input"
                        type="text"
                        name="hex"
                        style:width="54px"
                        on:blur={move |ev| {
                            match event_target_value(&ev).parse::<Color>() {
                                Ok(new_color) => on_change.call(new_color),
                                Err(_) => {},
                            }
                        }}
                        on:change={move |ev| {
                            match event_target_value(&ev).parse::<Color>() {
                                Ok(new_color) => on_change.call(new_color),
                                Err(_) => {},
                            }
                        }}
                        maxLength={6}
                        prop:value={move || hex.get().replace("#", "")}
                        />
                        </div>
                        <span>"Hex"</span>
                    </label>
                    <div style="display: flex">
                    </div>

                </Show>
                <Show
                    when=move || { !hide_rgb.get()}
                >
                <label class="leptos-color-label">
                    <div class="leptos-color-wrapper">
                        <input
                            class="leptos-color-input"
                            prop:value=red
                            name="red"
                            type="number"
                            style:width="42px"
                            min={0}
                            max={255}
                            step={1}
                            auto_complete="off"
                            on:change={move |ev| {
                                match event_target_value(&ev).parse::<u8>() {
                                    Ok(value) => {
                                        let mut color = color.get();
                                        color.r = value as f32 / 255.0;
                                        on_change.call(color);
                                    },
                                    Err(_) => todo!(),
                                }
                            }}
                        />

                            </div>
                        <span>"R"</span>
                    </label>
                <label class="leptos-color-label">
                    <div class="leptos-color-wrapper">
                        <input
                            class="leptos-color-input"
                            prop:value=green
                            name="green"
                            type="number"
                            style:width="42px"
                            min={0}
                            max={255}
                            step={1}
                            auto_complete="off"
                            on:change={move |ev| {
                                match event_target_value(&ev).parse::<u8>() {
                                    Ok(value) => {
                                        let mut color = color.get();
                                        color.g = value as f32 / 255.0;
                                        on_change.call(color);
                                    },
                                    Err(_) => todo!(),
                                }
                            }}
                        />
                    </div>
                    <span>"G"</span>
                </label>
                <label class="leptos-color-label">
                    <div class="leptos-color-wrapper">
                        <input
                            class="leptos-color-input"
                            prop:value=blue
                            name="blue"
                            type="number"
                            style:width="42px"
                            min={0}
                            max={255}
                            step={1}
                            auto_complete="off"
                            on:change={move |ev| {
                                match event_target_value(&ev).parse::<u8>() {
                                    Ok(value) => {
                                        let mut color = color.get();
                                        color.b = value as f32 / 255.0;
                                        on_change.call(color);
                                    },
                                    Err(_) => {},
                                }
                            }}
                        />
                    </div>
                    <span>"B"</span>
                </label>
                </Show>
                <Show
                    when=move || { !hide_alpha.get()}
                >
                <label class="leptos-color-label">
                    <div class="leptos-color-wrapper">
                    <input
                        class="leptos-color-input"
                        prop:value=alpha
                        name="alpha"
                        type="number"
                        style:width="42px"
                        min={0}
                        max={255}
                        step={1}
                        auto_complete="off"
                        on:change={move |ev| {
                            match event_target_value(&ev).parse::<u8>() {
                                Ok(value) => {
                                    let mut color = color.get();
                                    color.a = value as f32 / 255.0;
                                    on_change.call(color);
                                },
                                Err(_) => {},
                            }
                        }}/>
                    </div>
                    <span>"Alpha"</span>
                </label>
                </Show>
            </div>
        </div>
    }
}
