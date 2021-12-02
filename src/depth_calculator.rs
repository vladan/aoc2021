pub enum Direction {
    Up,
    Down,
    Forward,
}

pub struct Movement {
    pub direction: Direction,
    pub step: u64,
}

pub struct Position {
    pub aim: u64,
    pub depth: u64,
    pub horizontal: u64,
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for Movement {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.split(" ").collect::<Vec<&str>>()[..] {
            [direction_str, step_str] => Ok(Movement {
                direction: Direction::try_from(direction_str)?,
                step: step_str.parse::<u64>().map_err(|_| ())?,
            }),
            _ => Err(())
        }
    }
}

