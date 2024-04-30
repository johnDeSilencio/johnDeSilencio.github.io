use leptos::*;
use leptos_meta::*;

fn main() {
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
        <h1 name="title">{ "Hello world!" }</h1>
        <div>
            <CommandInput />
        </div>
    }
}

#[component]
fn CommandInput() -> impl IntoView {
    view! {
        <input />
    }
}
