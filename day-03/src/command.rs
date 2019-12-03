use std::fmt::Debug;
use std::str::FromStr;

pub enum Command {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[derive(Debug)]
pub enum CommandParseError {
    InvalidDirection,
    InvalidNumber,
}

impl Debug for Command {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Command::Up(size) => write!(fmt, "U{}", size),
            Command::Down(size) => write!(fmt, "D{}", size),
            Command::Left(size) => write!(fmt, "L{}", size),
            Command::Right(size) => write!(fmt, "R{}", size),
        }
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let command = match input.chars().nth(0) {
            Some('U') => Command::Up(
                input[1..]
                    .parse()
                    .map_err(|_| CommandParseError::InvalidNumber)?,
            ),
            Some('D') => Command::Down(
                input[1..]
                    .parse()
                    .map_err(|_| CommandParseError::InvalidNumber)?,
            ),
            Some('L') => Command::Left(
                input[1..]
                    .parse()
                    .map_err(|_| CommandParseError::InvalidNumber)?,
            ),
            Some('R') => Command::Right(
                input[1..]
                    .parse()
                    .map_err(|_| CommandParseError::InvalidNumber)?,
            ),
            _ => Err(CommandParseError::InvalidDirection)?,
        };

        Ok(command)
    }
}
