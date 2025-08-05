use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use leptos::{html::Canvas, prelude::*};
use leptos::wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::wasm_bindgen::prelude::Closure;

use crate::utils::graphics::types::buffers::TriangleUniform;
use crate::utils::helpers::callbacks::*;
use crate::utils::graphics::state::State;

#[allow(non_snake_case)]
#[component]
pub fn Window() -> impl IntoView {
    let canvas_node_ref = NodeRef::<Canvas>::new();

    canvas_node_ref.on_load(move |canvas| {
        let canvas = Arc::new(canvas.clone());
        
        let width = canvas.client_width() as u32;
        let height = canvas.client_height() as u32;
        
        canvas.set_width(width);
        canvas.set_height(height);
        
        spawn_local(async move {
            leptos::logging::log!("Spawning local thread to handle state");
            
            let state: Rc<RefCell<State<'static>>> = Rc::new(RefCell::new(State::new(canvas.clone()).await.unwrap()));

            state.borrow_mut().resize(canvas.clone());

            let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
            let f_clone: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = f.clone();

            let window = Rc::new(leptos::web_sys::window().expect("no global window"));

            let state_for_render = state.clone();

            *f_clone.borrow_mut() = Some(Closure::wrap(Box::new({
                let window = window.clone();
                move || {
                    // Call the render function
                    // leptos::logging::log!("redraw");
                    state_for_render.borrow_mut().update();
                    let _ = state_for_render.borrow_mut().render();

                    // Schedule next frame
                    window
                        .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                        .expect("Failed to request animation frame");
                }
            }) as Box<dyn FnMut()>));

            window
                .request_animation_frame(f_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("Failed to start render loop");

            resize_callback(&window, state.clone(), canvas.clone());
            keydown_callback(&window, state.clone());
            // mouse_moved_callback(&window, state.clone());
        });
    });

    view! {
        <canvas
            node_ref=canvas_node_ref
            style="width: 100vw; height: 100vh; display: block"
        ></canvas>
    }
}