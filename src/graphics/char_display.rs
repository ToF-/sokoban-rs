use crate::Puzzle;
use crate::UNIT;
use crate::model::coords::MAX_COL;
use crate::model::coords::MAX_ROW;
use crate::model::element::Element;
use crate::model::puzzle;
use glib::clone;
use gtk::gdk::prelude::GdkCairoContextExt;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::DrawingAreaExtManual;
use gtk::{Application, DrawingArea};
use std::cell::OnceCell;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
pub static CHAR_TILES: OnceCell<CharTiles> = const { OnceCell::new() };
}

pub const CHAR_ELEMENT_MAX: usize = 10;

pub struct CharTiles {
    tiles: Vec<Pixbuf>,
}

impl CharTiles {
    pub fn from_picture_files() -> Self {
        let pixbufs = (0usize..CHAR_ELEMENT_MAX)
            .map(|index| {
                let element = Self::char_element(index);
                make_char_tile(&element)
            })
            .collect();
        Self { tiles: pixbufs }
    }

    pub fn char_element(index: usize) -> Element {
        match index {
            0 => Element::Outside,
            1 => Element::Grass_1,
            2 => Element::Grass_2,
            3 => Element::Crate,
            4 => Element::Floor,
            5 => Element::Goal,
            6 => Element::Sokoban,
            7 => Element::Stored,
            8 => Element::Wall,
            9 => Element::OnGoal,
            _ => Element::Outside,
        }
    }

    pub fn pixbuf(&self, index: usize) -> Pixbuf {
        self.tiles[index].clone()
    }
}

pub fn char_tile_pixbuf(element: &Element) -> Pixbuf {
    CHAR_TILES.with(|cell| {
        let char_tiles = cell.get_or_init(CharTiles::from_picture_files);
        let index = match element {
            Element::Outside => 0,
            Element::Grass_1 => 1,
            Element::Grass_2 => 2,
            Element::Crate => 3,
            Element::Floor => 4,
            Element::Goal => 5,
            Element::Sokoban => 6,
            Element::Stored => 7,
            Element::Wall => 8,
            Element::OnGoal => 9,
            _ => 0,
        };
        char_tiles.pixbuf(index).clone()
    })
}

pub fn make_char_tile(element: &Element) -> Pixbuf {
    let tile_name = match element {
        Element::Outside => "outside_char",
        Element::Grass_1 => "grass_1_char",
        Element::Grass_2 => "grass_2_char",
        Element::Floor => "floor_char",
        Element::Goal => "goal_char",
        Element::Sokoban => "sokoban_char",
        Element::Crate => "crate_char",
        Element::Stored => "stored_char",
        Element::Wall => "wall_char",
        Element::OnGoal => "on_goal_char",
        _ => "floor_char",
    };
    let file_name = format!("/com/example/MyApp/images/{}.png", tile_name);
    let pixbuf = Pixbuf::from_resource(&file_name).expect("can't create pixpuf from resource");
    pixbuf
}
