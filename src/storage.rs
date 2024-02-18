use teloxide::{
    dispatching::dialogue::{Dialogue, ErasedStorage},
    types::Message,
    Bot,
};

use crate::state::{Command, State};

pub type MyDialogue = Dialogue<State, ErasedStorage<State>>;
pub type MyStorage = std::sync::Arc<ErasedStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
