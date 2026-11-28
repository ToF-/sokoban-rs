use crate::Direction;
use crate::model::game::LEVEL_MAX;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    level: usize,
    moves: Vec<String>,
}

const HISTORY_FILE: &str = "sokoban_rs.json";
impl History {
    pub fn new() -> Self {
        let moves = vec![String::new(); LEVEL_MAX];
        let level = 0;
        let new = Self { level, moves };
        if let Ok(history) = Self::load() {
            history
        } else {
            new
        }
    }

    pub fn directions_for_level(&self, level: usize) -> Vec<Direction> {
        let mut result: Vec<Direction> = Vec::new();
        for c in self.moves[level].chars() {
            let direction = Direction::from(c);
            result.push(direction)
        }
        result
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn record_level(&mut self, level: usize, directions: &Vec<Direction>) {
        self.level = level;
        let mut s = String::new();
        for direction in directions.iter() {
            s.push(char::from(*direction));
        }
        self.moves[level] = s.clone()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        match fs::write(&HISTORY_FILE, json) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("error with file:{}", HISTORY_FILE);
                Err(e)
            }
        }
    }

    pub fn load() -> std::io::Result<Self> {
        let data = fs::read_to_string(&HISTORY_FILE)?;
        let history: History = serde_json::from_str(&data)?;
        Ok(history)
    }
}
