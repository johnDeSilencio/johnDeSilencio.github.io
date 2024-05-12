use leptos::*;
use leptos_meta::*;

fn main() {
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
        <h1 name="title" class="text-7xl">{ "Hello world!" }</h1>
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
        <input type="text" aria-label="Command input" />
    }
}
