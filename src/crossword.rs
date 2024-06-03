use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Crossword {
    pub words: Vec<Word>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    answer: &'static str,
    clue: &'static str,
    direction: Direction,
    position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Across,
    Down,
}

impl Crossword {
    pub fn from_words_and_clues(words: &[(&'static str, &'static str)]) -> Self {
        let mut words = words.to_vec();
        words.sort_unstable_by_key(|(word, _)| -(word.len() as i128));
        let mut words = words.into_iter();
        let mut crossword = Self { words: Vec::new() };
        let (answer, clue) = words.next().unwrap();
        crossword.words.push(Word {
            answer,
            clue,
            direction: Direction::Across,
            position: Position::default(),
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
                                    Position {
                                        x: existing.position.x
                                            + (i32::try_from(existing_index).unwrap()),
                                        y: existing.position.y - (i32::try_from(index).unwrap()),
                                    },
                                ),
                                Direction::Down => (
                                    Direction::Down,
                                    Position {
                                        x: existing.position.x - (i32::try_from(index).unwrap()),
                                        y: existing.position.y
                                            + (i32::try_from(existing_index).unwrap()),
                                    },
                                ),
                            })
                        })
                    })
                    .collect();
                positions.shuffle(&mut thread_rng());
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

fn intersect_words(a: (Position, Direction, usize), b: (Position, Direction, usize)) -> bool {
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
