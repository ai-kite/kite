use std::cell::RefCell;

use serde::Deserialize;

use crate::error::*;

use cereal::mood_capnp;
use cereal::utils::CapnpRead;
use cereal::utils::CapnpWrite;

mod events;

thread_local! { static CURRENT_MOOD: RefCell<Mood> =
    RefCell::new(Mood::load().expect("Failed to load mood"));
}

// TODO: Introduce decay_rate or some similiar mechanic
#[derive(Debug, PartialEq)]
pub struct Mood {
    valence: i32,
    arousal: i32,
    dominance: i32,
}

impl Mood {
    pub fn new() -> Self {
        Self {
            valence: 0,
            arousal: 0,
            dominance: 0,
        }
    }

    /// Saves current mood state to storage.
    fn save(&self) -> Result<()> {
        // TODO: capnp serialization
        todo!()
    }

    /// Loads mood from storage, if doesn't exists creates a new one.
    fn load() -> Result<(Self)> {
        // TODO: capnp deserialization
        //
        // Return a new Mood for now
        Ok(Mood::new())
    }

    pub fn process_event(&mut self, event_name: &str) -> Result<()> {
        if let Some(event) = events::get_event(event_name) {
            self.valence = (self.valence + event.valence).clamp(-100, 100);
            self.arousal = (self.arousal + event.arousal).clamp(-100, 100);
            self.dominance = (self.dominance + event.dominance).clamp(-100, 100);
            // self.save()?;
        }
        Ok(())
    }

    /// Converts numerical states into descriptive language.
    fn describe(&self) -> String {
        let mood_desc = match (self.valence, self.arousal, self.dominance) {
            (v, a, d) if v > 70 && a > 70 && d > 50 => "ecstatic and unstoppable",
            (v, a, d) if v > 50 && a > 50 && d < -30 => "excited but hesitant",
            (v, a, d) if v > 50 && a < -50 && d > 50 => "calm yet in control",
            (v, a, d) if v > 50 && a < -50 && d < -30 => "peaceful but vulnerable",
            (v, a, d) if v < -70 && a > 70 => "furious and reckless",
            (v, a, d) if v < -50 && a > 50 && d < -30 => "agitated but insecure",
            (v, a, d) if v < -50 && a < -50 && d > 50 => "depressed but determined",
            (v, a, d) if v < -50 && a < -50 && d < -50 => "completely withdrawn",
            (v, _, d) if v > 50 => "cheerful and optimistic",
            (v, _, d) if v < -50 => "melancholic and distant",
            (_, a, d) if a > 50 => "restless and impulsive",
            (_, a, d) if a < -50 => "relaxed and composed",
            (_, _, d) if d > 50 => "bold and confident",
            (_, _, d) if d < -50 => "timid and uncertain",
            _ => "neutral and observant",
        };
        format!("Currently, Kite feels {mood_desc}.")
    }
}

pub fn describe() -> String {
    // TODO: use `try_borrow` to handle errors.
    CURRENT_MOOD.with(|mood| mood.borrow().describe())
}

impl<'a> CapnpWrite<'a> for Mood {
    type Builder = mood_capnp::mood::Builder<'a>;

    fn write_capnp(&self, builder: &mut Self::Builder) {
        builder.set_valence(self.valence);
        builder.set_arousal(self.arousal);
        builder.set_dominance(self.dominance);
    }
}

impl<'a> CapnpRead<'a> for Mood {
    type Reader = mood_capnp::mood::Reader<'a>;

    fn read_capnp(reader: Self::Reader) -> Self {
        Self {
            valence: reader.get_valence(),
            arousal: reader.get_arousal(),
            dominance: reader.get_dominance(),
        }
    }
}

mod tests {

    use super::*;

    // NOTE: Only works with a newly created `Mood` state.
    #[test]
    fn stimulate_buy() {
        CURRENT_MOOD.with(|mood| {
            mood.borrow_mut().process_event("buy_kite").unwrap();
        });

        let expected_mood = Mood {
            valence: 8,
            arousal: 7,
            dominance: 5,
        };

        CURRENT_MOOD.with(|mood| {
            assert_eq!(*mood.borrow(), expected_mood);
        })
    }
}
