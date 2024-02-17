use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Me},
    utils::command::BotCommands,
};

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    Help,
    Start,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Training {
    identifier: String,
    exercises: Vec<Exercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Exercise {
    name: String,
    serie: Serie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RepetitionSerie {
    sets: u8,
    repetitions: u8,
    load: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContinuousSerie {
    time_in_seconds: u16,
    sets: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Serie {
    Repetitions(RepetitionSerie),
    Continuous(ContinuousSerie),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::load().unwrap();
    pretty_env_logger::init();
    info!("Starting the smartest fit bot ever existed...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

fn make_workout_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let trainings: Vec<Training> = serde_json::from_str(include_str!("workout.json")).unwrap();

    for training in trainings {
        let identifier = training.identifier;
        let row = vec![InlineKeyboardButton::callback(
            identifier.clone(),
            format!("training:{identifier}"),
        )];

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

fn make_training_keyboard(training_identifier: String) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let trainings: Vec<Training> = serde_json::from_str(include_str!("workout.json")).unwrap();
    let trainings = trainings
        .into_iter()
        .map(|training| (training.identifier, training.exercises))
        .collect::<HashMap<_, _>>();

    let exercises = trainings.get(&training_identifier).unwrap();

    for exercise in exercises {
        let row: Vec<InlineKeyboardButton> = vec![InlineKeyboardButton::callback(
            exercise.name.clone(),
            exercise.name.clone(),
        )];

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Start) => {
                let keyboard = make_workout_keyboard();
                bot.send_message(msg.chat.id, "Choose your training:")
                    .reply_markup(keyboard)
                    .await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    match &q.data {
        Some(data) => {
            debug!("Callback handler data: {data}");
            let mut data_slipt = data.split(':');
            let mode = data_slipt.next();

            if let Some("training") = mode {
                let training = &data_slipt.next().unwrap();
                let keyboard = make_training_keyboard(training.to_string());
                bot.send_message(
                    q.chat_id().unwrap(),
                    format!("Exercises for training {training}"),
                )
                .reply_markup(keyboard)
                .await?;

                bot.answer_callback_query(q.id.clone()).await?;
            }
        }
        _ => (),
    }

    Ok(())
}

fn _make_json() {
    let trainings = vec![
        Training {
            identifier: "A".to_owned(),
            exercises: vec![
                Exercise {
                    name: "Smith Machine Squat".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 4,
                        repetitions: 10,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Leg Press Horizontal".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Leg Curl Machine".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Glute Machine".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Glute Leg Raise".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Plank".to_owned(),
                    serie: Serie::Continuous(ContinuousSerie {
                        time_in_seconds: 30,
                        sets: Some(3),
                    }),
                },
            ],
        },
        Training {
            identifier: "B".to_owned(),
            exercises: vec![
                Exercise {
                    name: "Free Weight Squat".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 4,
                        repetitions: 10,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Leg Press 45 degrees".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Hip Abductor Machine".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Hip Adductor Machine".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Glute Machine".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Glute Leg Raise".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Side Plank".to_owned(),
                    serie: Serie::Continuous(ContinuousSerie {
                        time_in_seconds: 30,
                        sets: Some(3),
                    }),
                },
            ],
        },
        Training {
            identifier: "C".to_owned(),
            exercises: vec![
                Exercise {
                    name: "Side Plank".to_owned(),
                    serie: Serie::Continuous(ContinuousSerie {
                        time_in_seconds: 30,
                        sets: Some(3),
                    }),
                },
                Exercise {
                    name: "Plank".to_owned(),
                    serie: Serie::Continuous(ContinuousSerie {
                        time_in_seconds: 30,
                        sets: Some(3),
                    }),
                },
                Exercise {
                    name: "Ab Crunch Machine".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 15,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Leg Raise".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Cardio".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Plank".to_owned(),
                    serie: Serie::Continuous(ContinuousSerie {
                        time_in_seconds: 1800,
                        sets: None,
                    }),
                },
            ],
        },
        Training {
            identifier: "D".to_owned(),
            exercises: vec![
                Exercise {
                    name: "Sumo Squat".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 4,
                        repetitions: 10,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Leg Press Horizontal".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Leg Curl Machine".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Glute Machine".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Glute Leg Raise".to_owned(),
                    serie: Serie::Repetitions(RepetitionSerie {
                        sets: 3,
                        repetitions: 12,
                        load: None,
                    }),
                },
                Exercise {
                    name: "Plank".to_owned(),
                    serie: Serie::Continuous(ContinuousSerie {
                        time_in_seconds: 30,
                        sets: Some(3),
                    }),
                },
            ],
        },
    ];

    println!("{}", serde_json::to_string_pretty(&trainings).unwrap());
}
