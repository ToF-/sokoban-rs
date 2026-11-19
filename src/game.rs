use std::cell::Ref;
use crate::RefCell;
use crate::Rc;
use crate::State;
use crate::fixed_for_level;
use crate::Sprite;
use crate::Step;
use crate::Puzzle;
use crate::Level;

#[derive(Clone)]
pub struct Game {
    levels: Vec<Level>,
    level_index: usize,
    pub fixed: gtk::Fixed,
    pub puzzle_rc: Rc<RefCell<Puzzle>>,
    pub steps_rc: Rc<RefCell<Vec<Step>>>,
    pub sokoban_sprite_rc: Rc<RefCell<Sprite>>,
    pub crate_sprites_rc: Rc<RefCell<Vec<Sprite>>>,
}

impl Game {
    pub fn new(levels: &Vec<Level>, level_index: usize) -> Self {

        let puzzle = Puzzle::from_level(&levels[level_index]);
        let mut crate_sprites = Vec::new();
        let fixed = fixed_for_level(&levels[level_index]);
        for crate_index in 0..puzzle.max_crate() {
            let crate_coords = puzzle.crate_coords(crate_index);
            let mut sprite = Sprite::make_sprite(
                    &State::Crate, puzzle.crate_coords(crate_index));
            sprite.place(&fixed);
            crate_sprites.push(sprite);
        };
        let mut sokoban_sprite = Sprite::make_sprite(
            &State::Sokoban, puzzle.sokoban_position());
        sokoban_sprite.place(&fixed);
        Self {
            levels: levels.clone(),
            level_index: level_index,
            fixed: fixed,
            puzzle_rc: Rc::new(RefCell::new(puzzle)),
            steps_rc: Rc::new(RefCell::new(Vec::new())),
            sokoban_sprite_rc: Rc::new(RefCell::new(sokoban_sprite)),
            crate_sprites_rc:  Rc::new(RefCell::new(crate_sprites.clone())),
        }
    }

    pub fn level_index(&self) -> usize {
        self.level_index
    }

}
