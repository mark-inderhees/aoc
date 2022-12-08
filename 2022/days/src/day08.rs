use anyhow::Result;
use grid::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::puzzle::Puzzle;

pub struct Day08 {
    board: Board<u32>,
    visible: Board<char>,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Board<T> {
    grid: Grid<T>,
    location: Point,
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<T> Board<T> {
    pub fn new() -> Board<T> {
        Board {
            grid: grid![],
            location: Point { x: 0, y: 0 },
        }
    }

    pub fn set_location(&mut self, x: i32, y: i32) {
        self.location.x = x;
        self.location.y = y;
    }

    pub fn get_current_value(&self) -> &T {
        let x: usize = self.location.x.try_into().unwrap();
        let y: usize = self.location.y.try_into().unwrap();
        self.grid.get(y, x).unwrap()
    }

    pub fn step(&mut self, direction: Direction) -> Option<&T> {
        let (step_x, step_y) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_location = Point {
            x: self.location.x + step_x,
            y: self.location.y + step_y,
        };

        let x_max: i32 = self.grid.size().0.try_into().unwrap();
        let y_max: i32 = self.grid.size().1.try_into().unwrap();
        match new_location {
            _ if new_location.x == -1 => None,
            _ if new_location.y == -1 => None,
            _ if new_location.x == x_max => None,
            _ if new_location.y == y_max => None,
            _ => {
                self.location = new_location;
                let x: usize = new_location.x.try_into().unwrap();
                let y: usize = new_location.y.try_into().unwrap();
                Some(self.grid.get(y, x).unwrap())
            }
        }
    }
}

impl Puzzle for Day08 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day08 {
            board: Board::new(),
            visible: Board::new(),
        };

        for line in input.lines() {
            let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let len = row.len();
            day.board.grid.push_row(row);
            day.visible.grid.push_row(vec!['.'; len]);
        }

        log::debug!("Input Grid: {:#?}", day.board);

        let x_max: i32 = day.board.grid.size().0.try_into().unwrap();
        let y_max: i32 = day.board.grid.size().1.try_into().unwrap();
        let mut count = x_max * 2 + y_max * 2 - 4;

        for y in 1..(y_max - 1) {
            for x in 1..(x_max - 1) {
                for direction in Direction::iter() {
                    day.board.set_location(x, y);
                    let height = day.board.get_current_value().clone();
                    let mut heights = vec![];
                    while let Some(height2) = day.board.step(direction) {
                        heights.push(height2.clone());
                    }
                    let height_max = heights.iter().max().unwrap();
                    let visible = height > *height_max;
                    log::debug!("At {x},{y} going {direction:?}: {height} vs {height_max} = {visible}, {heights:?}");
                    if visible {
                        count += 1;
                        let x_: usize = x.try_into().unwrap();
                        let y_: usize = y.try_into().unwrap();
                        day.visible.grid[y_][x_] = 'v';
                        break;
                    }
                }
            }
        }

        log::debug!("Input Grid: {:#?}", day.board.grid);
        log::debug!("{:#?}", day.visible.grid);
        log::info!("{count}");

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
            false => None,
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
            false => None,
        }
    }
}
