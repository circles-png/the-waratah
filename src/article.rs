use lazy_static::lazy_static;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub id: &'static str,
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
        let (id, title) = lines
            .next()
            .ok_or_else(|| anyhow!("no data"))?
            .split_once(' ')
            .ok_or_else(|| anyhow!("no title"))?;
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
            title,
            image,
            fragments,
        })
    }

    pub fn text_len(&self) -> usize {
        self.fragments
            .iter()
            .filter_map(|fragment| fragment.as_text().map(|text| text.len()))
            .sum()
    }

    pub fn reading_time(&self) -> usize {
        self.text_len().div_ceil(200)
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
        Ok(articles.leak())
    })()
    .unwrap();
}
