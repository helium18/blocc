use console_error_panic_hook::set_once;
use gloo::{
    events::EventListener,
    utils::{body, document},
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Element;

mod movement;
mod utils;

#[derive(Clone, Copy, Default)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Config {
    pub rows: u32,
    pub columns: u32,
    pub empty_char: char,
    pub filled_char: char,
}

pub enum Operation {
    Increase((i32, Direction)),
    Decrease((i32, Direction)),
}

pub enum Direction {
    X,
    Y,
}

#[wasm_bindgen(start)]
pub fn main_js() {
    set_once();

    let mut cords = Coordinates::default();

    let element = document()
        .get_element_by_id("body")
        .expect("No such element");

    let config = Config {
        rows: 12,
        columns: 37,
        empty_char: '🐷',
        filled_char: '🐸',
    };

    let mut grid = utils::get_grid(&config).unwrap();

    EventListener::new(&element, "keydown", move |event| {
        let ops;

        match event
            .clone()
            .dyn_into::<web_sys::KeyboardEvent>()
            .unwrap()
            .code()
            .as_str()
        {
            "ArrowLeft" => ops = Some(Operation::Decrease((1, Direction::X))),
            "ArrowUp" => ops = Some(Operation::Decrease((1, Direction::Y))),
            "ArrowRight" => ops = Some(Operation::Increase((1, Direction::X))),
            "ArrowDown" => ops = Some(Operation::Increase((1, Direction::Y))),
            _ => ops = None,
        };

        if let Some(ops) = ops {
            movement::move_obj(&mut grid, &config, &mut cords, &ops)
        }
    })
    .forget();
}
