use std::fs;
use std::io::Error;

pub struct Rom {
    pub content: Vec<u8>,
}

impl Rom {
    pub fn load(path: String) -> Result<Rom, Error> {
        let bytes = fs::read(path)?;

        Ok(
            Rom {
                content: bytes,
            }
        )
    }
}
