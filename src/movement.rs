use crate::utils::update_grid;
use crate::Config;
use crate::Coordinates;
use crate::Direction;
use crate::Element;
use crate::Operation;

pub fn move_obj(
    grid: &mut Vec<Element>,
    config: &Config,
    cords: &mut Coordinates,
    ops: &Operation,
) {
    match ops {
        Operation::Increase((magnitude, direction)) => match direction {
            Direction::X => {
                cords.x += magnitude;
                if !is_valid_move(cords, config) {
                    cords.x -= magnitude
                }
            }
            Direction::Y => {
                cords.y += magnitude;
                if !is_valid_move(cords, config) {
                    cords.y -= magnitude
                }
            }
        },
        Operation::Decrease((magnitude, direction)) => match direction {
            Direction::X => {
                cords.x -= magnitude;
                if !is_valid_move(cords, config) {
                    cords.x += magnitude
                }
            }
            Direction::Y => {
                cords.y -= magnitude;
                if !is_valid_move(cords, config) {
                    cords.y += magnitude
                }
            }
        },
    };
    update_grid(grid, cords, config).expect("Failed to update the grid");
}

fn is_valid_move(cords: &Coordinates, config: &Config) -> bool {
    cords.x < config.columns as i32 && cords.y < config.rows as i32 && cords.x >= 0 && cords.y >= 0
}
