use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use teloxide::macros::BotCommands;

use crate::training::Training;

#[derive(Debug, BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Help,
    Start,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum State {
    #[default]
    Start,
    Training(HashMap<String, Training>),
}
