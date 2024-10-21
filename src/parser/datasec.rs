use super::{Data, Parsable, Pretty};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct DataSection {
    pub size: u32,
    pub data: Vec<Data>,
}
impl DataSection {
    pub fn concat(&mut self, mut other: Self) {
        self.size += other.size;
        self.data.append(&mut other.data);
    }
}
impl Parsable for DataSection {
    fn parse_inner(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let size = u32::parse_inner(data)?;
        let data = Vec::parse_inner(data)?;
        Ok(Self { size, data })
    }
}
impl Pretty for DataSection {
    fn pretty_indent(&self, _: usize) -> String {
        todo!()
    }
}
