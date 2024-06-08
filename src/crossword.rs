use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

use lazy_static::lazy_static;
use strum::{Display, VariantArray};

#[derive(Debug, Clone)]
pub struct Crossword {
    pub words: Vec<Word>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    pub answer: &'static str,
    pub clue: &'static str,
    pub direction: Direction,
    pub position: Vec2,
}

impl Word {
    pub const fn contains(&self, position: Vec2) -> bool {
        let end = match self.direction {
            Direction::Across => Vec2 {
                x: self.position.x + self.answer.len() - 1,
                y: self.position.y,
            },
            Direction::Down => Vec2 {
                x: self.position.x,
                y: self.position.y + self.answer.len() - 1,
            },
        };
        self.position.x <= position.x
            && position.x <= end.x
            && self.position.y <= position.y
            && position.y <= end.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Letter {
    pub character: char,
    pub position: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, VariantArray, Display)]
pub enum Direction {
    Across,
    Down,
}

impl Direction {
    pub const ALL: &'static [Self] = Self::VARIANTS;
}

impl Crossword {
    fn from_str(s: &'static str) -> Self {
        Self {
            words: s
                .trim()
                .lines()
                .map(|line| {
                    let mut parts = line.splitn(5, |char: char| char.is_whitespace());
                    Word {
                        answer: parts.next().unwrap().to_ascii_uppercase().leak(),
                        position: Vec2 {
                            x: usize::from_str(parts.next().unwrap()).unwrap(),
                            y: usize::from_str(parts.next().unwrap()).unwrap(),
                        },
                        direction: match parts.next().unwrap() {
                            "across" => Direction::Across,
                            "down" => Direction::Down,
                            _ => unreachable!(),
                        },
                        clue: parts.next().unwrap(),
                    }
                })
                .collect(),
        }
    }

    pub fn to_letters(&self) -> HashSet<Letter> {
        self.words
            .iter()
            .flat_map(|word| {
                word.answer
                    .chars()
                    .enumerate()
                    .map(move |(index, character)| Letter {
                        character,
                        position: match word.direction {
                            Direction::Across => Vec2 {
                                x: word.position.x + index,
                                y: word.position.y,
                            },
                            Direction::Down => Vec2 {
                                x: word.position.x,
                                y: word.position.y + index,
                            },
                        },
                    })
            })
            .collect()
    }

    pub fn size(&self) -> Vec2 {
        let mut size = Vec2::default();
        for word in &self.words {
            let end = match word.direction {
                Direction::Across => Vec2 {
                    x: word.position.x + word.answer.len(),
                    y: word.position.y,
                },
                Direction::Down => Vec2 {
                    x: word.position.x,
                    y: word.position.y + word.answer.len(),
                },
            };
            size.x = size.x.max(end.x);
            size.y = size.y.max(end.y);
        }
        size
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

lazy_static! {
    pub static ref CROSSWORDS: &'static [Crossword] = {
        let data = include_str!(concat!(env!("OUT_DIR"), "/crosswords"));
        let crosswords: Vec<_> = data.split("\n\n").map(Crossword::from_str).collect();
        crosswords.leak()
    };
}
