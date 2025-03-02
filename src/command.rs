use leptos::prelude::*;
use leptos::reactive::signal::WriteSignal;
use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
pub struct Command {
    pub command: String,
    pub id: u32,
}

#[derive(Default, Debug, Clone)]
pub struct CommandOutput {
    pub output: String,
    pub id: u32,
}

#[derive(Debug, Clone)]
pub enum Content {
    Command(Command),
    CommandOutput(CommandOutput),
}

#[derive(Default, Debug, Clone)]
pub struct TerminalContent {
    content: VecDeque<Content>,
    next_id: u32,
}

impl TerminalContent {
    #[must_use]
    pub fn new() -> Self {
        Self {
            content: VecDeque::new(),
            next_id: 0,
        }
    }

    #[must_use]
    pub fn next_id(&self) -> u32 {
        self.next_id
    }

    pub fn push_back_command(&mut self, command: String) {
        let command = Command {
            command,
            id: self.next_id,
        };

        self.content.push_back(Content::Command(command));
        self.next_id += 1;
    }

    pub fn push_back_command_output(&mut self, output: String) {
        let command_output = CommandOutput {
            output,
            id: self.next_id,
        };

        self.content
            .push_back(Content::CommandOutput(command_output));
        self.next_id += 1;
    }
}

impl Iterator for TerminalContent {
    type Item = Content;

    fn next(&mut self) -> Option<Self::Item> {
        self.content.pop_front()
    }
}

pub struct CommandInterpreter {
    pub set_commands: WriteSignal<TerminalContent>,
}

impl CommandInterpreter {
    pub fn new(set_commands: WriteSignal<TerminalContent>) -> Self {
        Self { set_commands }
    }

    pub fn execute(&self, command: String) {
        match command.as_str() {
            "clear" => (self.set_commands)(TerminalContent::new()),
            _ => self.set_commands.update(|commands| {
                commands.push_back_command(command.clone());
                commands.push_back_command_output(format!("ERROR: No such command \"{command}\""));
            }),
        }
    }
}
