use std::io::ErrorKind;

use super::{error::ParseError, Name, Parsable};

#[derive(Debug, Default)]
pub struct CustomSection {
    pub size: u32,
    pub name: Name,
    pub sections: Vec<Vec<u8>>,
}
impl CustomSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.sections.append(&mut other.sections);

        if self.name.is_empty() {
            self.name = other.name;
        } else {
            self.name.0 += "+";
            self.name.0 += &*other.name;
        }
    }
}
impl Parsable for CustomSection {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse(data, stack)?;
        let name = Name::parse(data, stack)?;
        let mut section = Vec::new();
        loop {
            let value = u8::parse(data, stack);
            match value {
                Ok(v) => section.push(v),
                Err(ParseError::IOError(e)) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(e) => Err(e)?,
            }
        }
        Ok(Self {
            size,
            name,
            sections: vec![section],
        })
    }
}
