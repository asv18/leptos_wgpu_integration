use leptos::prelude::*;

use crate::components::window::Window;

/// Default Home Page
#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <div class="container">
                <Window />

            // <select
            // on:change:target=move |ev| {
            // set_value.set(ev.target().value().parse().unwrap());
            // }
            // prop:value=move || value.get().to_string()
            // >
            // <option value="0">"Repeat"</option>
            // <option value="1">"Clamp-to-Edge"</option>
            // </select>
            </div>
        </ErrorBoundary>
    }
}
