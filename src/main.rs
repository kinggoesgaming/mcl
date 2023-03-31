use std::{error, fmt};

mod advancement;

#[derive(Clone, Copy, Debug)]
enum Gamemode {
    Adventure,
    Creative,
    Hardcore,
    Spectator,
    Survival,
}

impl Gamemode {
    const fn name(&self) -> &str {
        match self {
            Gamemode::Adventure => "adventure",
            Gamemode::Creative => "creative",
            Gamemode::Hardcore => "hardcore",
            Gamemode::Spectator => "spectator",
            Gamemode::Survival => "survival",
        }
    }
}

impl fmt::Display for Gamemode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

trait Identifier<IdentifierType> {
    fn identifier(&self) -> IdentifierType;
}

fn main() -> Result<(), Box<dyn error::Error>> {
    Ok(())
}
