use regex::Regex;

use crate::{Error, Result};

#[derive(Clone)]
pub struct Search {
    re: Regex,
    matches: Vec<(usize, usize)>, // byte (start, end)
    forward: bool,
}

impl Search {
    pub fn new(patt: &str, text: &str, forward: bool) -> Result<Search> {
        let re = err_at!(BadPattern, Regex::new(patt), format!("{}", patt))?;
        let matches = re.find_iter(text).map(|m| (m.start(), m.end())).collect();
        Ok(Search {
            re,
            matches,
            forward,
        })
    }

    pub fn is_forward(&self) -> bool {
        self.forward
    }

    pub fn iter(&self, byte_off: usize) -> impl Iterator<Item = (usize, usize)> {
        match self.find(byte_off, &self.matches[..]) {
            Some(i) => {
                let mut ms = self.matches[i..].to_vec();
                ms.extend(&self.matches[..i]);
                ms.into_iter()
            }
            None => self.matches.clone().into_iter(),
        }
    }

    pub fn rev(&self, byte_off: usize) -> impl Iterator<Item = (usize, usize)> {
        match self.find(byte_off, &self.matches[..]) {
            Some(i) => {
                let mut ms = self.matches[i..].to_vec();
                ms.extend(&self.matches[..i]);
                ms.into_iter().rev()
            }
            None => self.matches.clone().into_iter().rev(),
        }
    }

    fn find(&self, byte_off: usize, rs: &[(usize, usize)]) -> Option<usize> {
        if rs.len() < 8
        /* TODO: no magic number */
        {
            let mut iter = rs
                .iter()
                .enumerate()
                .skip_while(|(_, (_, e))| *e < byte_off)
                .skip_while(|(_, (s, _))| byte_off >= *s);
            match iter.next() {
                None => None,
                Some((i, _)) => Some(i),
            }
        } else {
            let m = rs.len() / 2;
            match &rs[m] {
                (_, e) if *e < byte_off => match self.find(byte_off, &rs[m..]) {
                    None => None,
                    Some(i) => Some(m + i),
                },
                (s, _) if byte_off >= *s => match self.find(byte_off, &rs[m..]) {
                    None => None,
                    Some(i) => Some(m + i),
                },
                _ => self.find(byte_off, &rs[..m]),
            }
        }
    }
}
