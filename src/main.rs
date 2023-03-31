use std::error;

trait Identifier {
    fn identifier(&self) -> uuid::Uuid;
}

fn main() -> Result<(), Box<dyn error::Error>> {
    Ok(())
}
