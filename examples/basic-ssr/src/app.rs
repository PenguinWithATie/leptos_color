use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_color::components::color_input::ColorInput;
use leptos_color::{theme::Theme, Color};
use leptos_meta::*;
use leptos_router::*;
use log::info;
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/basic-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos Color"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let color = RwSignal::new(Color::new(1.0, 1.0, 1.0, 1.0));
    view! {
        <h1>"Welcome to Leptos Color!"</h1>
        <div style="height: 500px; width: 500px; display: flex; align-items: center; justify-content: center;">
            <ColorInput color=color on_change=move |x: Color| {info!("{:?}", x); color.set(x)}></ColorInput>
        </div>
    }
}
