use crate::model::state::State;
use crate::Direction;
use crate::HashMap;
use crate::Level;
use crate::Puzzle;
use crate::Rc;
use crate::RefCell;
use crate::Sprite;
use crate::Step;
use crate::UNIT;
use crate::model::coords::Coords;
use crate::fixed_for_level;
use crate::grid::Grid;
use crate::model::path_finder::PathFinder;
use crate::picture;
use crate::tile::fill_tiles_for_level;
use gtk::glib::clone;
use gtk::prelude::FixedExt;
use gtk::prelude::WidgetExt;
use std::cell::Cell;
use std::cell::Ref;
use std::ops::ControlFlow;
use std::time::Duration;

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
    pub mark_sprites_rc: Rc<RefCell<HashMap<char, Sprite>>>,
}

const GAME_MAX: usize = 90;

impl Game {
    pub fn new(levels: &Vec<Level>, level_index: usize, level_ready_rc: Rc<Cell<bool>>) -> Self {
        let puzzle = Puzzle::from_level(&levels[level_index]);
        let mut crate_sprites = Vec::new();
        let fixed = fixed_for_level(&levels[level_index]);
        let label = gtk::Label::new(Some(&format!("{}", level_index)));
        let level = &levels[level_index];
        for crate_index in 0..puzzle.max_crate() {
            let crate_coords = puzzle.crate_coords(crate_index);
            let mut sprite = Sprite::make_sprite(&State::Crate, puzzle.crate_coords(crate_index));
            crate_sprites.push(sprite);
        }
        let mut sokoban_sprite = Sprite::make_sprite(&State::Sokoban, puzzle.sokoban_position());
        let crate_sprites_rc = Rc::new(RefCell::new(crate_sprites));
        let sokoban_sprite_rc = Rc::new(RefCell::new(sokoban_sprite));
        let fixed_rc = Rc::new(RefCell::new(fixed.clone()));

        fill_tiles_for_level(
            &levels[level_index],
            fixed_rc,
            crate_sprites_rc.clone(),
            sokoban_sprite_rc.clone(),
            level_ready_rc,
        );
        Self {
            levels: levels.clone(),
            level_index: level_index,
            fixed: fixed,
            label: label,
            puzzle_rc: Rc::new(RefCell::new(puzzle)),
            steps_rc: Rc::new(RefCell::new(Vec::new())),
            sokoban_sprite_rc: sokoban_sprite_rc.clone(),
            crate_sprites_rc: crate_sprites_rc.clone(),
            mark_sprites_rc: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn levels(&self) -> Vec<Level> {
        self.levels.clone()
    }

    pub fn next_level(game: &Game, level_ready_rc: Rc<Cell<bool>>) -> Self {
        let mut level_index = game.level_index() + 1;
        if level_index == GAME_MAX {
            level_index = 1
        };
        level_ready_rc.set(false);
        Self::new(&game.levels(), level_index, level_ready_rc)
    }

    pub fn previous_level(game: &Game, level_ready_rc: Rc<Cell<bool>>) -> Self {
        let mut level_index = game.level_index() - 1;
        if level_index == 0 {
            level_index = GAME_MAX - 1
        };
        level_ready_rc.set(false);
        Self::new(&game.levels(), level_index, level_ready_rc)
    }

    pub fn level_index(&self) -> usize {
        self.level_index
    }

    pub fn undo(&self) {
        let mut puzzle = self.puzzle_rc.borrow_mut();
        if puzzle.is_solved() {
            return;
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
                return;
            };
        }
        self.undo();
        self.restart();
    }

    pub fn move_towards(&self, direction: Direction) {
        let mut puzzle = self.puzzle_rc.borrow_mut();
        if puzzle.is_solved() {
            return;
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
                // sprite.place(&fixed);
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

    pub fn mark(&self, letter: char) {
        let already_present = {
            let puzzle = self.puzzle_rc.borrow();
            puzzle.find_mark_coords(letter).is_some()
        };
        if already_present {
            self.unmark(letter);
        };
        let letter_opt = {
            let puzzle = self.puzzle_rc.borrow();
            let coords = puzzle.sokoban_position();
            puzzle.find_mark_at_coords(coords)
        };
        if let Some(letter) = letter_opt {
            self.unmark(letter);
        };
        let mark_state = match letter {
            'a' => State::Mark_A,
            'b' => State::Mark_B,
            'c' => State::Mark_C,
            _ => return,
        };
        let mut puzzle = self.puzzle_rc.borrow_mut();
        let coords = puzzle.sokoban_position();
        let mut sprite = Sprite::make_sprite(&mark_state, coords);
        sprite.place(&self.fixed);
        let mut mark_sprites = self.mark_sprites_rc.borrow_mut();
        let _ = mark_sprites.insert(letter, sprite);
        puzzle.insert_mark(letter);
    }

    pub fn unmark(&self, letter: char) {
        let mut puzzle = self.puzzle_rc.borrow_mut();
        let mut mark_sprites = self.mark_sprites_rc.borrow_mut();
        if let Some(coords) = puzzle.find_mark_coords(letter) {
            let mut sprite: Sprite = mark_sprites
                .get(&letter)
                .expect("can't access to mark sprite")
                .clone();
            sprite.remove(&self.fixed);
            mark_sprites.remove(&letter);
            puzzle.remove_mark(letter);
        } else {
            println!("mark not present in puzzle");
        }
    }

    pub fn goto(&mut self, letter: char) {
        let directions_opt = {
            let puzzle = self.puzzle_rc.borrow();
            let origin = puzzle.sokoban_position();
            if let Some(target) = puzzle.find_mark_coords(letter) {
                let mut path_finder = PathFinder::new(&puzzle);
                path_finder.find_path(origin, target)
            } else {
                None
            }
        };
        if let Some(directions) = directions_opt {
            self.follow_path(&directions);
        }
    }

    pub fn follow_path(&mut self, directions: &Vec<Direction>) {
        let step_index_rc = Rc::new(RefCell::new(0));
        let fixed = self.fixed.clone();

        glib::timeout_add_local(
            Duration::from_millis(50),
            clone!(
                #[strong (rename_to=this)]
                self,
                #[strong]
                directions,
                move || {
                    let mut step_index = step_index_rc.borrow_mut();
                    if *step_index < directions.len() {
                        let direction = directions[*step_index];
                        this.move_towards(direction);
                        fixed.queue_draw();
                        *step_index += 1;
                        glib::ControlFlow::Continue
                    } else {
                        glib::ControlFlow::Break
                    }
                }
            ),
        );
    }
}
