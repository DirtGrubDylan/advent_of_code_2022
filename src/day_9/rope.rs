#[derive(Debug, PartialEq)]
enum Motion {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

impl From<&String> for Motion {
    fn from(input: &String) -> Motion {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Rope {
    head: (usize, usize),
    tail: (usize, usize),
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
        }
    }
}
