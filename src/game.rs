use crate::Direction;
use crate::Level;
use crate::Puzzle;
use crate::Rc;
use crate::RefCell;
use crate::Sprite;
use crate::State;
use crate::Step;
use crate::UNIT;
use crate::fixed_for_level;
use crate::picture;
use gtk::prelude::FixedExt;
use std::cell::Ref;

#[derive(Clone)]
pub struct Game {
    levels: Vec<Level>,
    level_index: usize,
    pub fixed: gtk::Fixed,
    pub label: gtk::Label,
    pub puzzle_rc: Rc<RefCell<Puzzle>>,
    pub steps_rc: Rc<RefCell<Vec<Step>>>,
    pub sokoban_sprite_rc: Rc<RefCell<Sprite>>,
    pub crate_sprites_rc: Rc<RefCell<Vec<Sprite>>>,
}

const GAME_MAX: usize = 90;

impl Game {
    pub fn new(levels: &Vec<Level>, level_index: usize) -> Self {
        let puzzle = Puzzle::from_level(&levels[level_index]);
        let mut crate_sprites = Vec::new();
        let fixed = fixed_for_level(&levels[level_index]);
        let label = gtk::Label::new(Some(&format!("{}", level_index)));
        fixed.put(&label, 0.0, 0.0);
        for crate_index in 0..puzzle.max_crate() {
            let crate_coords = puzzle.crate_coords(crate_index);
            let mut sprite = Sprite::make_sprite(&State::Crate, puzzle.crate_coords(crate_index));
            sprite.place(&fixed);
            crate_sprites.push(sprite);
        }
        let mut sokoban_sprite = Sprite::make_sprite(&State::Sokoban, puzzle.sokoban_position());
        sokoban_sprite.place(&fixed);
        Self {
            levels: levels.clone(),
            level_index: level_index,
            fixed: fixed,
            label: label,
            puzzle_rc: Rc::new(RefCell::new(puzzle)),
            steps_rc: Rc::new(RefCell::new(Vec::new())),
            sokoban_sprite_rc: Rc::new(RefCell::new(sokoban_sprite)),
            crate_sprites_rc: Rc::new(RefCell::new(crate_sprites.clone())),
        }
    }

    pub fn levels(&self) -> Vec<Level> {
        self.levels.clone()
    }

    pub fn next_level(game: &Game) -> Self {
        let mut level_index = game.level_index() + 1;
        if level_index == GAME_MAX {
            level_index = 1
        };
        Self::new(&game.levels(), level_index)
    }

    pub fn previous_level(game: &Game) -> Self {
        let mut level_index = game.level_index() - 1;
        if level_index == 0 {
            level_index = GAME_MAX - 1
        };
        Self::new(&game.levels(), level_index)
    }

    pub fn level_index(&self) -> usize {
        self.level_index
    }

    pub fn undo(&self) {
        let mut puzzle = self.puzzle_rc.borrow_mut();
        if puzzle.is_solved() {
            return
        };
        let fixed = &self.fixed;
        if !puzzle.is_solved() {
            {
                let mut steps = self.steps_rc.borrow_mut();
                if let Some(step) = steps.pop() {
                    let reversed = step.reverse();
                    let backwards = Direction::between(reversed.from(), reversed.to());
                    puzzle.r#move(backwards);
                    if let Some((crate_index, boxed_crate_step)) = reversed.pushing() {
                        let crate_step = *boxed_crate_step;
                        let crate_origin = crate_step.from();
                        let crate_destination = crate_step.to();
                        puzzle.move_piece_at(crate_origin, backwards);
                        let mut crate_sprites = self.crate_sprites_rc.borrow_mut();
                        let mut sprite = &mut crate_sprites[crate_index];
                        sprite.r#move(&fixed, crate_destination);
                    };
                    let mut sokoban_sprite = self.sokoban_sprite_rc.borrow_mut();
                    let position = puzzle.sokoban_position();
                    sokoban_sprite.r#move(&fixed, position);
                }
            }
            let max_step = {
                let steps = self.steps_rc.borrow();
                steps.len()
            };
            self.label.set_text(&format!("{}", max_step));
        };
    }

    pub fn restart(&self) {
        {
        let puzzle = self.puzzle_rc.borrow();
        let steps = self.steps_rc.borrow();
        if puzzle.is_solved() || steps.is_empty() {
            return
        };
        }
        self.undo();
        self.restart();
    }

    pub fn step(&self, direction: Direction) {
        let mut puzzle = self.puzzle_rc.borrow_mut();
        if puzzle.is_solved() {
            return
        }
        let fixed = &self.fixed;
        if puzzle.can_move(direction) {
            let origin = puzzle.sokoban_position();
            let destination = direction.from(origin);
            let target_opt = puzzle.r#move(direction);
            let mut crate_index = 0;
            if let Some(target) = target_opt {
                let mut crate_sprites = self.crate_sprites_rc.borrow_mut();
                crate_index = crate_sprites
                    .iter()
                    .position(|mut sprite| sprite.coords() == destination)
                    .expect("can't find crate");
                let mut sprite = &mut crate_sprites[crate_index];
                sprite.r#move(&fixed, target);
            };
            let position = puzzle.sokoban_position();
            let mut sokoban_sprite = self.sokoban_sprite_rc.borrow_mut();
            let direction_state = match direction {
                Direction::Right => State::Right,
                Direction::Down => State::Down,
                Direction::Up => State::Up,
                Direction::Left => State::Left,
            };
            sokoban_sprite.remove(&fixed);
            sokoban_sprite.set_picture(&direction_state);
            sokoban_sprite.place(&fixed);
            sokoban_sprite.r#move(&fixed, destination);
            let mut steps = self.steps_rc.borrow_mut();
            let pushing = if let Some(target) = target_opt {
                Some(crate_index)
            } else {
                None
            };
            steps.push(Step::new(origin, direction, pushing));
        };
        if puzzle.is_solved() {
            for crate_index in 0..puzzle.max_crate() {
                let coords = puzzle.crate_coords(crate_index);
                let mut sprite = Sprite::make_sprite(&State::Stored, coords);
                sprite.place(fixed);
            }
        };
        let max_step = {
            let steps = self.steps_rc.borrow();
            steps.len()
        };
        self.label.set_text(&format!("{}", max_step));
    }
}
