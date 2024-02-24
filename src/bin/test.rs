use std::collections::HashMap;

use esperto_fit::training::Training;
use teloxide::{
    dispatching::{
        dialogue::{self, GetChatId, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Default)]
pub enum State {
    #[default]
    Start,
    SelectTraining {
        training: HashMap<String, Training>,
    },
}

#[derive(Debug, BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the purchase procedure.")]
    Start,
    #[command(description = "cancel the purchase procedure.")]
    Cancel,
}

#[tokio::main]
async fn main() {
    dotenv::load().unwrap();
    pretty_env_logger::init();
    log::info!("Starting purchase bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Start].endpoint(start)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::SelectTraining { training }].endpoint(select_training));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}

async fn select_training(
    bot: Bot,
    _dialogue: MyDialogue,
    q: CallbackQuery,
    training: HashMap<String, Training>,
) -> HandlerResult {
    println!("in callback_handler");

    if let Some(data) = &q.data {
        let mut data_slipt = data.split(':');
        let mode = data_slipt.next();

        if let Some("T") = mode {
            let training_identifier = data_slipt.next().unwrap();
            let keyboard =
                make_training_keyboard(training.get(training_identifier).cloned().unwrap());

            bot.send_message(
                q.chat_id().unwrap(),
                format!("Exercises for training {training_identifier}"),
            )
            .reply_markup(keyboard)
            .await?;

            bot.answer_callback_query(q.id.clone()).await?;
        }

        if let Some("E") = mode {
            let training = data_slipt.next().unwrap();
            let exercise = data_slipt.next().unwrap();

            let trainings: Vec<Training> =
                serde_json::from_str(include_str!("../workout.json")).unwrap();
            let trainings = trainings
                .into_iter()
                .map(|training| (training.identifier, training.exercises))
                .collect::<HashMap<_, _>>();

            let training = trainings.get(training).unwrap();

            let exercise = training
                .into_iter()
                .filter(|training| training.name == exercise)
                .collect::<Vec<_>>()
                .first()
                .cloned()
                .unwrap();

            bot.send_message(
                q.chat_id().unwrap(),
                format!("{} - {}", exercise.name, exercise.serie),
            )
            .await?;

            bot.answer_callback_query(q.id.clone()).await?;
        }
    }

    Ok(())
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let trainings: Vec<Training> = serde_json::from_str(include_str!("../workout.json")).unwrap();
    let keyboard = make_workout_keyboard(trainings.clone());

    let trainings = trainings
        .into_iter()
        .map(|training| (training.identifier.clone(), training))
        .collect::<HashMap<_, _>>();

    dialogue
        .update(State::SelectTraining {
            training: trainings,
        })
        .await?;

    bot.send_message(msg.chat.id, "Choose your training:")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

fn make_workout_keyboard(trainings: Vec<Training>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for training in trainings {
        let identifier = training.identifier;
        let row = vec![InlineKeyboardButton::callback(
            identifier.clone(),
            format!("T:{identifier}"),
        )];

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

fn make_training_keyboard(training: Training) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let training_name = training.identifier;

    for exercise in training.exercises {
        let exercise_name = exercise.name.clone();
        let row: Vec<InlineKeyboardButton> = vec![
            InlineKeyboardButton::callback(
                exercise_name.clone(),
                format!("E:{training_name}:{exercise_name}"),
            ),
            InlineKeyboardButton::callback(
                "Change Load",
                format!("CL:{training_name}:{exercise_name}"),
            ),
        ];

        keyboard.push(row);
    }

    keyboard.push(vec![InlineKeyboardButton::callback(
        "Completed!",
        format!("FE:{training_name}"),
    )]);

    InlineKeyboardMarkup::new(keyboard)
}
