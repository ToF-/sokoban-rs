use crate::model::puzzle_move::reverse_move;
use crate::model::puzzle_move::PuzzleMove;
use crate::Direction;
use crate::Level;
use crate::Puzzle;
use crate::levels_from_bytes;
use crate::model::basic_step::Move;
use gtk::gio;

const GAME_MAX: usize = 90;

#[derive(Clone)]
pub struct Game {
    levels: Vec<Level>,
    level_index: usize,
    puzzle: Puzzle,
    moves: Vec<Vec<PuzzleMove>>,
}

impl Game {
    pub fn new() -> Self {
        let levels_file_name = format!("/com/example/MyApp/files/levels.txt");
        let bytes = gio::resources_lookup_data(&levels_file_name, gio::ResourceLookupFlags::NONE)
            .expect("Failed to find resource");
        let levels = levels_from_bytes(&bytes);
        let level_index = 1;
        let puzzle = Puzzle::from_level(&levels[level_index]);
        let moves = Vec::new();
        Self {
            levels,
            level_index,
            puzzle,
            moves,
        }
    }

    pub fn puzzle(&self) -> Puzzle {
        self.puzzle.clone()
    }

    pub fn goto(&mut self, letter: char) {
        println!("here I should goto the letter {}", letter);
    }

    pub fn mark(&mut self, letter: char) {
        println!("here I should mark the letter {}", letter);
    }

    pub fn move_towards(&mut self, direction: Direction) {
        if let Some(moves) = self.puzzle.walk(direction) {
            self.moves.push(moves)
        }
    }

    pub fn previous_level(&mut self) {
        if self.level_index > 1 {
            self.level_index -= 1
        } else {
            self.level_index = GAME_MAX - 1
        };
        self.puzzle = Puzzle::from_level(&self.levels[self.level_index]);
    }

    pub fn next_level(&mut self) {
        self.level_index += 1;
        if self.level_index == GAME_MAX {
            self.level_index = 1
        };
        self.puzzle = Puzzle::from_level(&self.levels[self.level_index]);
    }

    pub fn undo(&mut self) {
        if let Some(moves) = self.moves.pop() {
            let backwards = moves
                .iter()
                .rev()
                .map(|puzzle_move| reverse_move(puzzle_move))
                .collect();
            self.puzzle.make_moves(backwards)
        }
    }

    pub fn restart(&mut self) {
        while !self.moves.is_empty() {
            self.undo()
        }
    }
}
