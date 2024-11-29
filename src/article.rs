use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    combinator::{all_consuming, map, map_res, rest},
    error::Error,
    multi::{length_data, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub id: &'static str,
    pub topic: &'static str,
    pub index: usize,
    pub blurb: &'static str,
    pub title: &'static str,
    pub image: Image,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fragment {
    Text(&'static str),
    Image(Image),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub url: &'static str,
    pub caption: &'static str,
}

impl Fragment {
    pub const fn as_text(&self) -> Option<&&'static str> {
        if let Self::Text(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Article {
    pub fn parse(input: &'static str) -> IResult<&'static str, Self> {
        map(
            tuple((
                length_data(map_res(
                    terminated(take_until1::<_, _, Error<&str>>(" "), tag(" ")),
                    |input: &str| input.parse::<usize>(),
                )),
                preceded(tag(" "), take_until1(" ")),
                preceded(
                    tag(" "),
                    map_res(take_until1("\n"), |input: &str| input.parse::<usize>()),
                ),
                preceded(tag("\n"), take_until1("\n")),
                preceded(tag("\n"), take_until1("\n")),
                map(
                    pair(
                        preceded(tag("\n"), take_until1("\n")),
                        preceded(tag("\n"), take_until1("\n")),
                    ),
                    |(url, caption)| Image { url, caption },
                ),
                preceded(
                    tag("\n"),
                    separated_list1(
                        tag("\n\n"),
                        alt((
                            map(
                                tuple((
                                    preceded(tag("image\n"), take_until1("\n")),
                                    preceded(tag("\n"), take_until1("\n")),
                                )),
                                |(url, caption)| Fragment::Image(Image { url, caption }),
                            ),
                            map(alt((take_until1("\n\n"), rest)), Fragment::Text),
                        )),
                    ),
                ),
            )),
            |(topic, id, index, title, blurb, image, fragments)| Self {
                id,
                topic,
                index,
                blurb,
                title,
                image,
                fragments,
            },
        )(input)
    }

    pub fn words(&self) -> usize {
        self.fragments
            .iter()
            .filter_map(|fragment| {
                fragment
                    .as_text()
                    .map(|text| text.split_ascii_whitespace().count())
            })
            .sum()
    }

    pub fn reading_time(&self) -> usize {
        const AVERAGE_ADULT_READING_SPEED: usize = 238;
        self.words().div_ceil(AVERAGE_ADULT_READING_SPEED)
    }
}

lazy_static! {
    pub static ref ARTICLES: &'static [Article] = (|| -> Result<&'static [Article]> {
        let mut data = include_str!(concat!(env!("OUT_DIR"), "/articles"));
        let mut articles = Vec::new();
        while !data.is_empty() {
            let (length, rest) = data
                .split_once(' ')
                .ok_or_else(|| anyhow!("invalid data"))?;
            let length: usize = length.parse()?;
            let (_, article) = all_consuming(Article::parse)(&rest[..length])?;
            articles.push(article);
            data = rest
                .get(length + 1..)
                .or_else(|| rest.get(length..))
                .unwrap();
        }
        articles.sort_unstable_by_key(|article| -(article.index as i128));
        Ok(articles.leak())
    })()
    .unwrap();
}
