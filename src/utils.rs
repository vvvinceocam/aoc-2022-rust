use std::convert::Infallible;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Point<T: Hash + Clone> {
    pub x: T,
    pub y: T,
}

impl<T, E> FromStr for Point<T>
    where
        T: FromStr<Err=E> + Hash + Clone,
        E: Debug
{
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(
            Self {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
            }
        )
    }
}