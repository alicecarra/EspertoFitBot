use std::collections::HashMap;

use teloxide::{
    dispatching::dialogue::{Dialogue, InMemStorage},
    macros::BotCommands,
};

use crate::training::Training;

pub type BotDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Debug, BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start.")]
    Start,
}

#[derive(Debug, Clone, Default)]
pub enum State {
    #[default]
    Start,
    SelectTraining {
        training: HashMap<String, Training>,
    },
}
