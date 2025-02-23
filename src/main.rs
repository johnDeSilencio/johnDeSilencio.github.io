pub mod bulldog;
pub mod map;

use std::collections::VecDeque;

use bulldog::GonzagaLogo;
use leptos::component;
use leptos::html;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::reactive::signal::WriteSignal;
use leptos::view;
use leptos_meta::*;
use log::Level;
use map::Map;

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

struct CommandInterpreter {
    set_commands: WriteSignal<Commands>,
}

impl CommandInterpreter {
    pub fn new(set_commands: WriteSignal<Commands>) -> Self {
        Self { set_commands }
    }

    pub fn execute(&self, command: String) {
        match command.as_str() {
            "clear" => (self.set_commands)(Commands::new()),
            _ => self
                .set_commands
                .update(|commands| commands.push_back(Command::new(command, commands.next_id()))),
        }
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

    mount_to_body(move || {
        view! {
            <Title text="Nicholas R. Smith" />
            <App />
        }
    });
}

#[component]
fn App() -> impl IntoView {
    let (commands, set_commands) = signal(Commands::new());

    view! {
        <div class="flex flex-wrap gap-4">
            <Terminal commands=commands set_commands=set_commands />
            <Foo />
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
fn Terminal(commands: ReadSignal<Commands>, set_commands: WriteSignal<Commands>) -> impl IntoView {
    view! {
        <div class="text-base border-solid border-2 border-white rounded m-1 p-0.5 shadow-2xl shadow-white">
            <PreviousCommands commands=commands />
            <div class="flex flex-wrap">
                <CommandPrompt />
                <CommandInput set_commands=set_commands />
            </div>
        </div>
    }
}

#[component]
fn Foo() -> impl IntoView {
    view! {
        <div>
            <h1>This is another thing</h1>
        </div>
    }
}

#[component]
fn PreviousCommands(commands: ReadSignal<Commands>) -> impl IntoView {
    view! {
        <div class="text-base">
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
    view! { <span class="bg-black text-white">{command}</span> }
}

#[component]
fn CommandPrompt() -> impl IntoView {
    view! { <span class="ml-2 mr-2">"^ > "</span> }
}

#[component]
fn CommandInput(set_commands: WriteSignal<Commands>) -> impl IntoView {
    let input_element: NodeRef<Input> = NodeRef::new();
    let form_ref: NodeRef<html::Form> = NodeRef::new();

    let interpreter = CommandInterpreter::new(set_commands);

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
            />
        </form>
    }
}
