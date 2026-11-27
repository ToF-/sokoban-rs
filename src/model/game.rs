use crate::Direction;
use crate::Level;
use crate::Puzzle;
use crate::levels_from_bytes;
use crate::model::move_::Move_;
use crate::model::move_::reverse_move;
use crate::model::path_finder::PathFinder;
use gtk::gio;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

const GAME_MAX: usize = 90;

#[derive(Clone)]
pub struct Game {
    levels: Vec<Level>,
    level_index: usize,
    puzzle: Puzzle,
    moves: Vec<Vec<Move_>>,
    auto_mode: bool,
    directions: Vec<Direction>,
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
        let auto_mode = false;
        let directions = Vec::new();
        Self {
            levels,
            level_index,
            puzzle,
            moves,
            auto_mode,
            directions,
        }
    }

    pub fn moves(&self) -> Vec<Vec<Move_>> {
        self.moves.clone()
    }

    pub fn directions(&self) -> Vec<Direction> {
        let mut result = Vec::new();
        for moves in self.moves().iter() {
            let (coords, destination) = moves[0];
            result.push(Direction::between(coords, destination));
        }
        result
    }

    pub fn puzzle(&self) -> Puzzle {
        self.puzzle.clone()
    }

    pub fn auto_mode(&self) -> bool {
        self.auto_mode
    }

    pub fn direction(&self) -> Vec<Direction> {
        self.directions.clone()
    }

    pub fn goto(&mut self, letter: char) {
        let directions_opt = {
            let origin = self.puzzle.sokoban_coords();
            if let Some(target) = self.puzzle.find_mark_coords(letter) {
                let mut path_finder = PathFinder::new(&self.puzzle);
                path_finder.find_path(origin, target)
            } else {
                None
            }
        };

        if let Some(directions) = directions_opt {
            self.directions = directions.clone();
            self.auto_mode = true
        }
    }

    pub fn auto(&mut self) {
        println!("{}", self.directions.len());
        if self.auto_mode() {
            if !self.directions.is_empty() {
                let direction = self.directions.remove(0);
                println!("{:?}", &direction);
                self.move_towards(direction)
            } else {
                self.auto_mode = false
            }
        }
    }

    pub fn mark(&mut self, letter: char) {
        self.puzzle.set_mark(letter);
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
                .map(|move_| reverse_move(move_))
                .collect();
            self.puzzle.make_moves(backwards)
        }
    }

    pub fn restart(&mut self) {
        while !self.moves.is_empty() {
            self.undo()
        }
    }

    pub fn puzzle_solved(&self) -> bool {
        self.puzzle.is_solved()
    }
}
