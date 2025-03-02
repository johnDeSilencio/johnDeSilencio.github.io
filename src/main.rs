pub mod biography;
pub mod bulldog;
pub mod command;
pub mod map;

use biography::Biography;
use bulldog::GonzagaLogo;
use command::{CommandInterpreter, Content, TerminalContent};
use leptos::component;
use leptos::html;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::view;
use leptos_meta::{Title, provide_meta_context};
use log::Level;
use map::Map;

fn main() {
    console_log::init_with_level(Level::Debug)
        .unwrap_or_else(|_| panic!("Failed to initialize logging"));
    provide_meta_context();
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! {
            <Title text="Nicholas R. Smith" />
            <App />
        }
    });
}

#[component]
fn App() -> impl IntoView {
    let (terminal_content, set_terminal_content) = signal(TerminalContent::new());

    view! {
        <div>
            <Biography />
        </div>
        <div class="flex flex-wrap gap-4">
            <Terminal terminal_content=terminal_content set_terminal_content=set_terminal_content />
        </div>
        <div>
            <Map />
        </div>
        <div>
            <GonzagaLogo />
        </div>
    }
}

#[component]
fn Terminal(
    terminal_content: ReadSignal<TerminalContent>,
    set_terminal_content: WriteSignal<TerminalContent>,
) -> impl IntoView {
    view! {
        <div class="text-base border-solid border-2 border-white rounded m-1 p-0.5 shadow-2xl shadow-white">
            <PreviousCommands terminal_content=terminal_content />
            <div class="flex flex-wrap">
                <CommandPrompt />
                <CommandInput set_terminal_content=set_terminal_content />
            </div>
        </div>
    }
}

#[component]
fn PreviousCommands(terminal_content: ReadSignal<TerminalContent>) -> impl IntoView {
    view! {
        <div class="text-base" id="previous-commands">
            <For
                each=move || terminal_content.get()
                // A unique key for each item
                key=|content: &Content| {
                    match content {
                        Content::Command(cmd) => cmd.id,
                        Content::CommandOutput(cmd_output) => cmd_output.id,
                    }
                }
                // Renders each item to a view
                children=move |content: Content| {
                    view! {
                        <div class="flex flex-wrap">
                            {match content {
                                Content::Command(cmd) => {
                                    view! {
                                        <CommandPrompt />
                                        <PreviousCommand command=cmd.command />
                                    }
                                        .into_any()
                                }
                                Content::CommandOutput(cmd_output) => {
                                    view! { <PreviousCommand command=cmd_output.output /> }
                                        .into_any()
                                }
                            }}
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
fn PreviousCommand(command: String) -> impl IntoView {
    view! { <span class="bg-black text-white">{command}</span> }
}

#[component]
fn CommandPrompt() -> impl IntoView {
    view! { <span class="ml-2 mr-2">"^ > "</span> }
}

#[component]
fn CommandInput(set_terminal_content: WriteSignal<TerminalContent>) -> impl IntoView {
    let input_element: NodeRef<Input> = NodeRef::new();
    let form_ref: NodeRef<html::Form> = NodeRef::new();

    let interpreter = CommandInterpreter::new(set_terminal_content);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value = input_element
            .read()
            .as_ref()
            .expect("<input> should be mounted")
            .value();

        interpreter.execute(value);

        // clear input field
        form_ref.on_load(|form| form.reset());
    };

    view! {
        <form node_ref=form_ref on:submit=on_submit>
            <input
                node_ref=input_element
                type="text"
                class="bg-black text-white outline-none"
                aria-label="Command input"
                id="command-input"
            />
        </form>
    }
}
