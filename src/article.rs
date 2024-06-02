
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub id: &'static str,
    pub title: &'static str,
    pub image_url: &'static str,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fragment {
    Text(&'static str),
    Image(&'static str),
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
        let (id, title) = lines.next().ok_or_else(|| anyhow!("no data"))?.split_once(' ').ok_or_else(|| anyhow!("no title"))?;
        let image_url = lines.next().ok_or_else(|| anyhow!("no image"))?;
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
                        "image" => Fragment::Image(fragment[1]),
                        _ => unreachable!(),
                    },
                )
            })
            .collect();
        let fragments = fragments?;

        Ok(Self {
            id,
            title,
            image_url,
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
