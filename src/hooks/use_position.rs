use ev::{mousemove, mouseup, touchend, touchmove, Event, UiEvent};
use html::Div;
use leptos::*;
use leptos_use::{use_document, use_event_listener};
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent, TouchEvent};
#[derive(Clone)]
pub struct UsePositionProps {
    pub on_move: Callback<(f64, f64), ()>,
}

enum MoveType {
    Mouse,
    Touch,
}

pub fn use_position(props: UsePositionProps) -> (NodeRef<Div>, Callback<UiEvent>) {
    let (dragging, set_dragging) = create_signal(false);
    let ref_div = create_node_ref::<Div>();

    let limit = |value: f64| -> f64 { value.min(1.0).max(0.0) };

    let get_position = move |e: &Event| -> Option<(f64, f64)> {
        if let Some(div) = ref_div.get_untracked() {
            let rect = Element::from(div.deref().clone()).get_bounding_client_rect();
            let (width, height) = (rect.width(), rect.height());

            let (client_x, client_y) = if let Some(mouse_event) = e.dyn_ref::<MouseEvent>() {
                (mouse_event.client_x() as f64, mouse_event.client_y() as f64)
            } else if let Some(touch_event) = e.dyn_ref::<TouchEvent>() {
                if let Some(touch) = touch_event.touches().item(0) {
                    (touch.client_x() as f64, touch.client_y() as f64)
                } else {
                    return None;
                }
            } else {
                return None;
            };
            Some((
                limit((client_x - rect.left()) / width),
                limit((client_y - rect.top()) / height),
            ))
        } else {
            None
        }
    };

    let handle_move = {
        let on_move = props.on_move.clone();
        move |move_type: MoveType, e: Event| {
            if matches!(move_type, MoveType::Mouse) {
                e.prevent_default();
            }
            if let Some(pos) = get_position(&e) {
                on_move.call(pos);
            }
        }
    };

    let handle_start = move |e: UiEvent| {
        set_dragging.set(true);
        if let Some(pos) = get_position(&e) {
            props.on_move.call(pos);
        }
    };

    let handle_end = move || {
        set_dragging.set(false);
    };

    create_effect(move |_| {
        let is_dragging = dragging.get();
        if is_dragging {
            let _ = use_event_listener(use_document(), mousemove, move |evt| {
                handle_move(MoveType::Mouse, evt.into());
            });
            let _ = use_event_listener(use_document(), mouseup, move |_| {
                handle_end();
            });
            let _ = use_event_listener(use_document(), touchmove, move |evt| {
                handle_move(MoveType::Touch, evt.into());
            });
            let _ = use_event_listener(use_document(), touchend, move |_| {
                handle_end();
            });
        };
    });

    (ref_div, Callback::new(handle_start))
}
