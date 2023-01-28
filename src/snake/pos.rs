use std::{fmt, ops};

#[derive(PartialEq, Clone)]
pub struct Pos{
    pub x: isize,
    pub y: isize,
}

impl fmt::Display for Pos{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add::<(isize, isize)> for Pos{
    type Output = Self;
    fn add(self, rhs: (isize, isize)) -> Self {
        Self { x: rhs.0 + self.x, y: rhs.1 + self.y }
    }
}

impl ops::Add::<Self> for Pos{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: rhs.x, y: rhs.y }
    }
}

impl ops::AddAssign::<Self> for Pos{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}