use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Training {
    pub identifier: String,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub serie: Serie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepetitionSerie {
    pub sets: u8,
    pub repetitions: u8,
    pub load: Option<f32>,
}

impl Display for RepetitionSerie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.load {
            Some(load) => write!(
                f,
                "{} sets of {} repetitions with load of {:.2}",
                self.repetitions, self.sets, load
            ),
            None => write!(f, "{} sets of {} repetitions", self.repetitions, self.sets),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousSerie {
    pub time_in_seconds: u16,
    pub sets: Option<u8>,
}

impl Display for ContinuousSerie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = if self.time_in_seconds % 60 == 0 {
            format!("{} minutes", self.time_in_seconds / 60)
        } else {
            format!(
                "{:.0} minutes and {} seconds",
                self.time_in_seconds / 60,
                self.time_in_seconds % 60
            )
        };

        match self.sets {
            Some(sets) if sets != 0 => write!(f, "{} sets of {}", sets, time),
            _ => write!(f, "for {}", time),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Serie {
    Repetitions(RepetitionSerie),
    Continuous(ContinuousSerie),
}

impl Display for Serie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Serie::Repetitions(serie) => write!(f, "{}", serie),
            Serie::Continuous(serie) => write!(f, "{}", serie),
        }
    }
}
