use serde::Deserialize;
use std::fs;
use std::io::{BufReader, BufWriter};

use cereal::mood_capnp;
use cereal::utils::CapnpRead;
use cereal::utils::CapnpWrite;


#[derive(Debug, Deserialize)]
struct Interaction {
    name: String,
    valence: i32,
    arousal: i32,
    dominance: i32,
}

#[derive(Debug, Deserialize)]
struct InteractionEffects {
    interaction: Vec<Interaction>,
}

#[derive(Debug)]
struct Mood {
    valence: i32,
    arousal: i32,
    dominance: i32,
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

impl Mood {
    fn new() -> Self {
        Self {
            valence: 0,
            arousal: 0,
            dominance: 0,
        }
    }

    fn apply_interaction(&mut self, interaction_name: &str, effects: &InteractionEffects) {
        if let Some(interaction) = effects
            .interaction
            .iter()
            .find(|i| i.name == interaction_name)
        {
            self.valence = (self.valence + interaction.valence).clamp(-100, 100);
            self.arousal = (self.arousal + interaction.arousal).clamp(-100, 100);
            self.dominance = (self.dominance + interaction.dominance).clamp(-100, 100);
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut message = capnp::message::Builder::new_default();
        let mut mood_builder = message.init_root::<mood_capnp::mood::Builder>();

        self.write_capnp(&mut mood_builder);

        let file = fs::File::create(filename).expect("Failed to create file");
        let mut writer = BufWriter::new(file);
        capnp::serialize::write_message(&mut writer, &message)
            .expect("Failed to write Cap'n Proto data");
    }

    fn load_from_file(filename: &str) -> Self {
        let file = fs::File::open(filename).expect("Failed to open file");
        let mut reader = BufReader::new(file);
        let message_reader = capnp::serialize::read_message(&mut reader, Default::default())
            .expect("Failed to read Cap'n Proto data");

        let mood_reader = message_reader
            .get_root::<mood_capnp::mood::Reader>()
            .expect("Failed to get root");

        Self::read_capnp(mood_reader)
    }

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
