use crate::error::*;
use common::interning::InternedString;

use serde::Deserialize;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;

use toml::from_str;

// FIXME: lazy_static?
thread_local! {
    static EVENTS: RefCell<BTreeMap<InternedString, Event>> =
        RefCell::new(Events::load().expect("Failed to load events").into_map());
}

#[derive(Debug, Deserialize, Clone)]
pub struct Event {
    pub valence: i32,
    pub arousal: i32,
    pub dominance: i32,
}

#[derive(Debug, Deserialize)]
pub struct Events {
    pub event: Vec<EventWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct EventWrapper {
    name: String,
    valence: i32,
    arousal: i32,
    dominance: i32,
}

impl Events {
    /// Loads events from `events.toml`.
    pub fn load() -> Result<Self> {
        let path = "core/mood/events.toml";
        let toml_content =
            fs::read_to_string(path).map_err(|e| Error::FileNotFound(format!("{path}: {e}")))?;
        let events: Events = from_str(&toml_content).map_err(|e| Error::TomlParse(e))?;
        Ok(events)
    }

    /// Converts events into a BTree for faster "look-ups".
    pub fn into_map(self) -> BTreeMap<InternedString, Event> {
        let mut map = BTreeMap::new();
        for event in self.event {
            let name = InternedString::new(&event.name);
            map.insert(name, Event {
                valence: event.valence,
                arousal: event.arousal,
                dominance: event.dominance,
            });
        }
        map
    }
}

// FIXME: No need for cloning the name.
//
/// Gets events.
pub fn get_event(name: &str) -> Option<Event> {
    EVENTS.with(|events| events.borrow().get(&InternedString::new(name)).cloned())
}
