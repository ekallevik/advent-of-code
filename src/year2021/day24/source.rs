use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Source { Reg(usize), Val(i64) }

impl FromStr for Source {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let source = match s {
            "w" => Self::Reg(0),
            "x" => Self::Reg(1),
            "y" => Self::Reg(2),
            "z" => Self::Reg(3),
            _   => Self::Val(s.parse().unwrap()),
        };

        Ok(source)
    }
}
