use thiserror::Error;

#[derive(Error, Debug)]
pub enum AoCError {
    #[error("an IO error occurred")]
    Io(#[from] std::io::Error),
    #[error("an HTTP error occurred")]
    Http(#[from] reqwest::Error),
    #[error("could not find .cookie file containing session like session=<value>")]
    CookieFile(std::io::Error),
}

pub trait Day {
    fn day(&self) -> u32;
    fn part1(&self, _lines: &str) -> Option<i64> {
        None
    }
    fn part2(&self, _lines: &str) -> Option<i64> {
        None
    }
}
