use teloxide::{
    dispatching::dialogue::{Dialogue, ErasedStorage},
    types::Message,
    Bot,
};

use crate::state::{Command, State};

pub type BotStorage = std::sync::Arc<ErasedStorage<State>>;
pub type BotDialogue = Dialogue<State, ErasedStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
