use crate::rangeset::RangeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub enum Command {
    Erase(RangeSet),
    New(RangeSet),
    Zero(RangeSet),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Command::Erase(_) => write!(f, "Erase"),
            Command::New(_) => write!(f, "New"),
            Command::Zero(_) => write!(f, "Zero"),
        }
    }
}

pub struct TransferList {
    version: u8,
    new_blocks: u64,
    commands: Vec<Command>,
}

impl TransferList {
    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn new_blocks(&self) -> u64 {
        self.new_blocks
    }

    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }
}

impl TryFrom<&Path> for TransferList {
    type Error = std::io::Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let mut transfer_list = BufReader::new(File::open(value)?).lines();

        // First line in transfer list is the version number
        let version = transfer_list
            .next()
            .unwrap()?
            .parse::<u8>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Second line in transfer list is the total number of blocks we expect to write
        let new_blocks = transfer_list
            .next()
            .unwrap()?
            .parse::<u64>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if version >= 2 {
            // Third line is how many stash entries are needed simultaneously
            transfer_list.next();
            // Fourth line is the maximum number of blocks that will be stashed simultaneously
            transfer_list.next();
        }

        let mut commands = Vec::new();
        while let Some(line) = transfer_list.next() {
            let line = line?;
            let mut tokens = line.split_whitespace();
            let command = tokens.next().unwrap();

            // Skip lines that start with a number
            if command.chars().nth(0).unwrap().is_digit(10) {
                continue;
            }

            let rangeset = tokens.next().unwrap();
            let rangeset = RangeSet::try_from(rangeset)?;

            let command = match command {
                "erase" => Command::Erase(rangeset),
                "new" => Command::New(rangeset),
                "zero" => Command::Zero(rangeset),
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unknown command: {}", command),
                    ))
                }
            };
            commands.push(command);
        }

        Ok(TransferList {
            version,
            new_blocks,
            commands,
        })
    }
}
