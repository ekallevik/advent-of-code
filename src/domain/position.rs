use std::str::FromStr;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Position2D(pub i16, pub i16);

impl FromStr for Position2D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (pre, suf) = s.split_once(',').unwrap();

        let pre = pre.parse().unwrap();
        let suf = suf.parse().unwrap();

        Ok(Position2D(pre, suf))
    }
}