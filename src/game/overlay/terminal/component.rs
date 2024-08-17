use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Reflect)]
pub struct AutoTerminalText {
    pub text: Vec<String>,
}

impl AutoTerminalText {
    pub fn default_text() -> Self {
        Self {
            text: vec![
                "It is not the strength of the body, but the strength of the soul.".to_string(),
                "War is a series of catastrophes punctuated by miracles.".to_string(),
                "The art of warfare is to avoid strength, attack weakness.".to_string(),
                "All warfare is based on deception.".to_string(),
                "War is the continuation of politics by other means.".to_string(),
            ],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default_text();
    }
}
