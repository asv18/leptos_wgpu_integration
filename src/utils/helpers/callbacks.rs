use crate::utils::state::State;
use crate::utils::types::size::PhysicalSize;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen_futures::wasm_bindgen::{prelude::Closure, JsCast};

pub fn _mouse_moved_callback(window: &Rc<wgpu::web_sys::Window>, state: Rc<RefCell<State>>) {
    let mouse_moved_closure: Closure<dyn FnMut(leptos::web_sys::MouseEvent)> =
        Closure::wrap(Box::new({
            move |event: leptos::web_sys::MouseEvent| {
                let mouse_pos = PhysicalSize {
                    width: event.client_x() as u32,
                    height: event.client_y() as u32,
                };

                state.borrow_mut().mouse_challenge(mouse_pos);
            }
        }) as Box<dyn FnMut(leptos::web_sys::MouseEvent)>);

    window
        .add_event_listener_with_callback("mousemove", mouse_moved_closure.as_ref().unchecked_ref())
        .unwrap();

    mouse_moved_closure.forget()
}

pub fn keydown_callback(window: &Rc<wgpu::web_sys::Window>, state: Rc<RefCell<State>>) {
    let keydown_closure: Closure<dyn FnMut(leptos::web_sys::KeyboardEvent)> =
        Closure::wrap(Box::new({
            move |event: leptos::web_sys::KeyboardEvent| {
                state.borrow_mut().handle_key(event);
            }
        }) as Box<dyn FnMut(leptos::web_sys::KeyboardEvent)>);

    window
        .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
        .unwrap();

    keydown_closure.forget()
}

pub fn resize_callback(
    window: &Rc<wgpu::web_sys::Window>,
    state: Rc<RefCell<State>>,
    canvas: Arc<wgpu::web_sys::HtmlCanvasElement>,
) {
    let resize_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        move || {
            let width = canvas.client_width() as u32;
            let height = canvas.client_height() as u32;

            canvas.set_width(width);
            canvas.set_height(height);

            state.borrow_mut().resize(PhysicalSize { width, height });
        }
    }) as Box<dyn FnMut()>);

    window
        .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
        .unwrap();

    resize_closure.forget()
}
