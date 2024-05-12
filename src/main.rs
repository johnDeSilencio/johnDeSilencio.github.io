use std::collections::VecDeque;

use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_meta::*;
use log::{debug, error, info, trace, warn, Level};

#[derive(Default, Debug, Clone)]
struct Command {
    command: String,
    id: u32,
}

impl Command {
    fn new(command: String, id: u32) -> Self {
        Self { command, id }
    }
}

#[derive(Default, Debug, Clone)]
struct Commands(VecDeque<Command>);

impl Commands {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn next_id(&self) -> u32 {
        match self.0.back() {
            // First command. Return 0 as the first index
            None => 0,
            Some(command) => command.id + 1,
        }
    }

    fn push_back(&mut self, command: Command) {
        self.0.push_back(command);
    }
}

impl Iterator for Commands {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

struct AppRuntime {
    commands: Commands,
}

impl AppRuntime {
    fn new() -> Self {
        Self {
            commands: Commands::new(),
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Debug)
        .unwrap_or_else(|_| panic!("Failed to initialize logging"));
    provide_meta_context();
    console_error_panic_hook::set_once();

    let (commands, set_commands) = create_signal(Commands::new());

    mount_to_body(move || {
        view! {
            <Title text="Nicholas R. Smith" />
            <App commands=commands set_commands=set_commands />
        }
    });
}

#[component]
fn App(commands: ReadSignal<Commands>, set_commands: WriteSignal<Commands>) -> impl IntoView {
    view! {
        <div>
            <PreviousCommands commands=commands />
            <div class="flex flex-wrap">
                <CommandPrompt />
                <CommandInput set_commands=set_commands />
            </div>
        </div>
    }
}

#[component]
fn PreviousCommands(commands: ReadSignal<Commands>) -> impl IntoView {
    view! {
        <div>
            <For
            each=move || commands.get()
            // a unique key for each item
            key=|command| command.id
            // renders each item to a view
            children=move |command: Command| {
                view! {
                    <div class="flex flex-wrap">
                        <CommandPrompt />
                        <PreviousCommand command=command.command />
                    </div>
                }
            }
            />
        </div>
    }
}

#[component]
fn PreviousCommand(command: String) -> impl IntoView {
    view! {
        <span name="command" type="text" class="bg-black text-white">{command}</span>
    }
}

#[component]
fn CommandPrompt() -> impl IntoView {
    view! {
        <span name="prompt" class="ml-2 mr-2">"^ > "</span>
    }
}

#[component]
fn CommandInput(set_commands: WriteSignal<Commands>) -> impl IntoView {
    let input_element: NodeRef<Input> = create_node_ref();
    let form_ref: NodeRef<html::Form> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value = input_element().expect("<input> should be mounted").value();
        set_commands.update(|commands| commands.push_back(Command::new(value, commands.next_id())));

        // clear input field
        form_ref().expect("<form> should be mounted").reset();
    };

    view! {
        <form node_ref=form_ref on:submit=on_submit>
            <input node_ref=input_element type="text" class="bg-black text-white outline-none" aria-label="Command input" />
        </form>
    }
}
