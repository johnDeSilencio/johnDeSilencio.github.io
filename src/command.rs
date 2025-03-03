use js_sys::Date;
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
    pub table: bool,
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
            table: false,
            id: self.next_id,
        };

        self.content
            .push_back(Content::CommandOutput(command_output));
        self.next_id += 1;
    }

    pub fn push_back_command_output_table(&mut self, output: String) {
        let command_output = CommandOutput {
            output,
            table: true,
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
            "ps" => self.set_commands.update(|commands| {
                let current_date = Date::now();
                let starting_date = Date::parse("13 Jun 2022 16:00:00 GMT");
                let seconds_elapsed = (current_date - starting_date) / 1000.0;
                let mut seconds_elapsed = seconds_elapsed as u32;

                let days_elapsed = seconds_elapsed / (60 * 60 * 24);
                seconds_elapsed -= days_elapsed * 60 * 60 * 24;

                let hours_elapsed = seconds_elapsed / (60 * 60);
                seconds_elapsed -= hours_elapsed * 60 * 60;

                let minutes_elapsed = seconds_elapsed / 60;
                seconds_elapsed -= minutes_elapsed * 60;

                commands.push_back_command(command);
                commands.push_back_command_output_table(format!(
                    "{} days, {:#02}:{:#02}:{:#02}",
                    days_elapsed, hours_elapsed, minutes_elapsed, seconds_elapsed
                ));
            }),
            "pwd" => self.set_commands.update(|commands| {
                commands.push_back_command(command);
                commands.push_back_command_output("Pullman, WA".to_string());
            }),
            "whoami" => self.set_commands.update(|commands| {
                commands.push_back_command(command);
                commands.push_back_command_output("Nicholas R. Smith".to_string());
            }),
            "help" => self.set_commands.update(|commands| {
                commands.push_back_command(command);
                commands.push_back_command_output("clear - Clears the terminal".to_string());
                commands.push_back_command_output("help - Prints this command".to_string());
                commands.push_back_command_output(
                    "ps - Prints this website's author current job".to_string(),
                );

                commands.push_back_command_output(
                    "pwd - Returns the city where this website's author currently lives"
                        .to_string(),
                );
                commands.push_back_command_output(
                    "whoami - Returns the name of this website's author".to_string(),
                );
            }),
            _ => self.set_commands.update(|commands| {
                commands.push_back_command(command.clone());
                commands.push_back_command_output(format!("ERROR: No such command \"{command}\""));
            }),
        }
    }
}
