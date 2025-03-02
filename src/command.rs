use leptos::prelude::*;
use leptos::reactive::signal::WriteSignal;
use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
pub struct Command {
    pub command: String,
    pub id: u32,
}

impl Command {
    pub fn new(command: String, id: u32) -> Self {
        Self { command, id }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Commands(VecDeque<Command>);

impl Commands {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn next_id(&self) -> u32 {
        match self.0.back() {
            // First command. Return 0 as the first index
            None => 0,
            Some(command) => command.id + 1,
        }
    }

    pub fn push_back(&mut self, command: Command) {
        self.0.push_back(command);
    }
}

impl Iterator for Commands {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct CommandInterpreter {
    pub set_commands: WriteSignal<Commands>,
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
