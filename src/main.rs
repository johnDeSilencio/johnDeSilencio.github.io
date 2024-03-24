use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>{ "Hello world!" }</h1>
    }
}
