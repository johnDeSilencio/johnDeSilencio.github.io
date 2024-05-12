use leptos::*;
use leptos_meta::*;
use log::{debug, error, info, trace, warn, Level};

fn main() {
    console_log::init_with_level(Level::Debug)
        .unwrap_or_else(|_| panic!("Failed to initialize logging"));
    provide_meta_context();
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Title text="Nicholas R. Smith" />
            <App />
        }
    });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <CommandPrompt />
            <CommandInput />
        </div>
    }
}

#[component]
fn CommandPrompt() -> impl IntoView {
    view! {
        <span name="prompt" class="ml-2 mr-2">"^ > "</span>
    }
}

#[component]
fn CommandInput() -> impl IntoView {
    view! {
        <input type="text" class="bg-black text-white outline-none" aria-label="Command input" />
    }
}
