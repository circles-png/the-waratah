use anyhow::{anyhow, Result};
use lazy_static::lazy_static;

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
    pub fn from_str(s: &'static str) -> Result<Self> {
        let mut lines = s.lines();
        let first = lines.next().ok_or_else(|| anyhow!("no data"))?;
        let (topic_len, rest) = first
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid data"))?;
        let topic_len: usize = topic_len.parse()?;
        let topic = &rest[..topic_len];
        let rest = &rest[topic_len + 1..];
        let (id, index) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid data"))?;
        let index = index.parse().unwrap();
        let title = lines.next().ok_or_else(|| anyhow!("no title"))?;
        let blurb = lines.next().ok_or_else(|| anyhow!("no blurb"))?;
        let image = Image {
            url: lines.next().ok_or_else(|| anyhow!("no image"))?,
            caption: lines.next().ok_or_else(|| anyhow!("no image title"))?,
        };
        let fragments: Vec<_> = lines.collect();
        let fragments: Result<Vec<_>> = fragments
            .split(|line| line.is_empty())
            .map(|fragment| {
                Ok(
                    match *fragment
                        .first()
                        .ok_or_else(|| anyhow!("no fragment data"))?
                    {
                        "text" => Fragment::Text(fragment[1]),
                        "image" => Fragment::Image(Image {
                            url: fragment[1],
                            caption: fragment[2],
                        }),
                        _ => unreachable!(),
                    },
                )
            })
            .collect();
        let fragments = fragments?;

        Ok(Self {
            id,
            topic,
            index,
            blurb,
            title,
            image,
            fragments,
        })
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
            let article = Article::from_str(&rest[..length])?;
            articles.push(article);
            data = rest
                .get(length + 1..)
                .or_else(|| rest.get(length..))
                .unwrap();
        }
        articles.sort_unstable_by_key(|article| article.index);
        Ok(articles.leak())
    })()
    .unwrap();
}
