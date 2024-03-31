use leptos::*;
use leptos_meta::*;

fn main() {
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
        <h1>{ "Hello world!" }</h1>
    }
}
