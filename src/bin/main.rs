use esperto_fit::{
    state::{Command, State},
    storage::{BotDialogue, BotStorage, HandlerResult},
    training::{ContinuousSerie, Exercise, RepetitionSerie, Serie, Training},
};
use log::{debug, info};
use std::{collections::HashMap, error::Error};
use teloxide::{
    dispatching::dialogue::{self, ErasedStorage, GetChatId, InMemStorage, SqliteStorage, Storage},
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Me},
    utils::command::BotCommands,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::load().unwrap();
    pretty_env_logger::init();
    info!("Starting the smartest fit bot ever existed...");

    let bot = Bot::from_env();

    let storage: BotStorage = SqliteStorage::open(
        "db.sqlite",
        teloxide::dispatching::dialogue::serializer::Json,
    )
    .await
    .unwrap()
    .erase();

    let handler = dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_callback_query()
                .branch(dptree::case![State::Training(training)].endpoint(callback_handler)),
        )
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, ErasedStorage<State>, State>()
                .branch(dptree::case![State::Start].endpoint(start)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn invalid_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Invalid command").await?;
    Ok(())
}

async fn start(bot: Bot, msg: Message, me: Me, dialogue: BotDialogue) -> HandlerResult {
    let keyboard = make_workout_keyboard();
    let trainings: Vec<Training> = serde_json::from_str(include_str!("../workout.json")).unwrap();

    let trainings = trainings
        .into_iter()
        .map(|training| (training.identifier.clone(), training))
        .collect::<HashMap<_, _>>();

    dialogue.update(State::Training(trainings)).await?;

    println!("dialogue: {:?}", dialogue.get().await);
    bot.send_message(msg.chat.id, "Choose your training:")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

fn make_workout_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let trainings: Vec<Training> = serde_json::from_str(include_str!("../workout.json")).unwrap();

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

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
    dialogue: BotDialogue,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Start) => {
                let keyboard = make_workout_keyboard();
                let trainings: Vec<Training> =
                    serde_json::from_str(include_str!("../workout.json")).unwrap();

                let trainings = trainings
                    .into_iter()
                    .map(|training| (training.identifier.clone(), training))
                    .collect::<HashMap<_, _>>();

                dialogue.update(State::Training(trainings)).await?;
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

async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    // training: HashMap<String, Training>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("in callback_handler");
    // match &q.data {
    //     Some(data) => {
    //         debug!("Callback handler data: {data}");
    //         let mut data_slipt = data.split(':');
    //         let mode = data_slipt.next();

    //         if let Some("T") = mode {
    //             let training_identifier = data_slipt.next().unwrap();
    //             let keyboard =
    //                 make_training_keyboard(training.get(training_identifier).cloned().unwrap());

    //             bot.send_message(
    //                 q.chat_id().unwrap(),
    //                 format!("Exercises for training {training_identifier}"),
    //             )
    //             .reply_markup(keyboard)
    //             .await?;

    //             bot.answer_callback_query(q.id.clone()).await?;
    //         }

    //         if let Some("E") = mode {
    //             let training = data_slipt.next().unwrap();
    //             let exercise = data_slipt.next().unwrap();

    //             let trainings: Vec<Training> =
    //                 serde_json::from_str(include_str!("../workout.json")).unwrap();
    //             let trainings = trainings
    //                 .into_iter()
    //                 .map(|training| (training.identifier, training.exercises))
    //                 .collect::<HashMap<_, _>>();

    //             let training = trainings.get(training).unwrap();

    //             let exercise = training
    //                 .into_iter()
    //                 .filter(|training| training.name == exercise)
    //                 .collect::<Vec<_>>()
    //                 .first()
    //                 .cloned()
    //                 .unwrap();

    //             bot.send_message(
    //                 q.chat_id().unwrap(),
    //                 format!("{} - {}", exercise.name, exercise.serie),
    //             )
    //             .await?;

    //             bot.answer_callback_query(q.id.clone()).await?;
    //         }
    //     }
    //     _ => (),
    // }

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
