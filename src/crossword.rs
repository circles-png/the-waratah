use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use itertools::Itertools;
use lazy_static::lazy_static;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Letter {
    pub character: char,
    pub position: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Across,
    Down,
}

impl Crossword {
    pub fn from_words_and_clues(words: &[(&'static str, &'static str)]) -> Self {
        let seed: [u8; 32] = words
            .iter()
            .flat_map(|(word, clue)| word.bytes().chain(clue.bytes()))
            .collect_vec()[..32]
            .try_into()
            .unwrap();
        let mut words = words.to_vec();
        words.sort_unstable_by_key(|(word, _)| -(word.len() as i128));
        let mut words = words.into_iter();
        let mut crossword = Self { words: Vec::new() };
        let (answer, clue) = words.next().unwrap();
        crossword.words.push(Word {
            answer,
            clue,
            direction: Direction::Across,
            position: Vec2::default(),
        });
        for (answer, clue) in words {
            let mut place_word = None;
            for existing in &crossword.words {
                let mut positions: Vec<_> = existing
                    .answer
                    .char_indices()
                    .filter(|&(_, existing_char)| answer.contains(existing_char))
                    .flat_map(|(existing_index, existing_char)| {
                        answer.char_indices().filter_map(move |(index, char)| {
                            (char == existing_char).then_some(match existing.direction {
                                Direction::Across => (
                                    Direction::Down,
                                    Vec2 {
                                        x: existing.position.x
                                            + (i32::try_from(existing_index).unwrap()),
                                        y: existing.position.y - (i32::try_from(index).unwrap()),
                                    },
                                ),
                                Direction::Down => (
                                    Direction::Down,
                                    Vec2 {
                                        x: existing.position.x - (i32::try_from(index).unwrap()),
                                        y: existing.position.y
                                            + (i32::try_from(existing_index).unwrap()),
                                    },
                                ),
                            })
                        })
                    })
                    .collect();
                positions.shuffle(&mut StdRng::from_seed(seed));
                let position = positions
                    .iter()
                    .find(|(direction, position)| {
                        crossword
                            .words
                            .iter()
                            .filter(|word| *word != existing)
                            .all(|existing| {
                                !intersect_words(
                                    (*position, *direction, answer.len()),
                                    (existing.position, existing.direction, existing.answer.len()),
                                )
                            })
                    })
                    .unwrap();
                place_word = Some(Word {
                    answer,
                    clue,
                    direction: position.0,
                    position: position.1,
                });
            }
            if let Some(word) = place_word {
                crossword.words.push(word);
            }
        }
        crossword
    }

    pub fn bounds(&self) -> (Vec2, Vec2) {
        self.words
            .iter()
            .map(|word| {
                let start = word.position;
                let end = match word.direction {
                    Direction::Across => {
                        start
                            + Vec2 {
                                x: i32::try_from(word.answer.len()).unwrap() - 1,
                                y: 0,
                            }
                    }
                    Direction::Down => {
                        start
                            + Vec2 {
                                x: 0,
                                y: i32::try_from(word.answer.len()).unwrap() - 1,
                            }
                    }
                };
                (start, end)
            })
            .fold(
                (Vec2 { x: 0, y: 0 }, Vec2 { x: 0, y: 0 }),
                |(min, max), (start, end)| {
                    (
                        Vec2 {
                            x: min.x.min(start.x),
                            y: min.y.min(start.y),
                        },
                        Vec2 {
                            x: max.x.max(end.x),
                            y: max.y.max(end.y),
                        },
                    )
                },
            )
    }

    pub fn size(&self) -> Vec2 {
        let bounds = self.bounds();
        bounds.1 - bounds.0 + Vec2 { x: 1, y: 1 }
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
                                x: word.position.x + i32::try_from(index).unwrap(),
                                y: word.position.y,
                            },
                            Direction::Down => Vec2 {
                                x: word.position.x,
                                y: word.position.y + i32::try_from(index).unwrap(),
                            },
                        },
                    })
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
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

fn intersect_words(a: (Vec2, Direction, usize), b: (Vec2, Direction, usize)) -> bool {
    let (a_position, a_direction, a_length) = a;
    let (b_position, b_direction, b_length) = b;
    match (a_direction, b_direction) {
        (Direction::Across, Direction::Down) => {
            a_position.x <= b_position.x
                && b_position.x <= a_position.x + i32::try_from(a_length).unwrap()
                && b_position.y <= a_position.y
                && a_position.y <= b_position.y + i32::try_from(b_length).unwrap()
        }
        (Direction::Down, Direction::Across) => {
            b_position.x <= a_position.x
                && a_position.x <= b_position.x + i32::try_from(b_length).unwrap()
                && a_position.y <= b_position.y
                && b_position.y <= a_position.y + i32::try_from(a_length).unwrap()
        }
        (Direction::Across, Direction::Across) => {
            a_position.y == b_position.y
                && a_position.x <= b_position.x
                && b_position.x <= a_position.x + i32::try_from(a_length).unwrap()
        }
        (Direction::Down, Direction::Down) => {
            a_position.x == b_position.x
                && a_position.y <= b_position.y
                && b_position.y <= a_position.y + i32::try_from(a_length).unwrap()
        }
    }
}

lazy_static! {
    pub static ref CROSSWORDS: &'static [Crossword] = {
        let data = include_str!(concat!(env!("OUT_DIR"), "/crosswords"));
        let crosswords: Vec<_> = data
            .split("\n\n")
            .map(|crossword| {
                Crossword::from_words_and_clues(
                    &crossword
                        .lines()
                        .map(|line| line.split_once(' ').unwrap())
                        .collect_vec(),
                )
            })
            .collect();
        crosswords.leak()
    };
}
