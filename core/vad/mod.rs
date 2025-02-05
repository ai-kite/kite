// VAD model:
// valence: pleasantness or unpleasantness of an emotion
// arousal: intensity of the emotional experience
// dominance: degree of control or influence one feels in a situation

use serde::Deserialize;
use std::fs;

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
struct KiteVAD {
    valence: i32,
    arousal: i32,
    dominance: i32,
}

impl KiteVAD {
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
}

fn load_interactions() -> InteractionEffects {
    let toml_str =
        fs::read_to_string("core/vad/interactions.toml").expect("Failed to read TOML file");
    toml::from_str(&toml_str).expect("Failed to parse TOML")
}

pub fn calc_mood() {
    let mut kite_state = KiteVAD {
        valence: 0,
        arousal: 0,
        dominance: 0,
    };

    let interaction_effects = load_interactions();

    println!("Before interaction: {:?}", kite_state);

    kite_state.apply_interaction("buy_kite", &interaction_effects);
    println!("After buying $KITE: {:?}", kite_state);

    kite_state.apply_interaction("ignore", &interaction_effects);
    println!("After ignoring Kite: {:?}", kite_state);
}
