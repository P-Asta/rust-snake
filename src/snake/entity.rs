
use super::pos::Pos;
#[derive(Clone, PartialEq)]
pub struct Snake{
    pub pos: Pos
}

impl std::fmt::Debug for Snake{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pos)
    }
}



pub struct Apple{
    pub pos: Pos
}