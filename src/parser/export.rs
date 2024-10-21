use super::{ExportDesc, Name, Parsable, Pretty};

#[derive(Debug)]
#[allow(unused)]
pub struct Export {
    pub nm: Name,
    pub d: ExportDesc,
}
impl Parsable for Export {
    fn parse(data: &mut std::io::Cursor<&[u8]>) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let nm = Name::parse(data)?;
        let d = ExportDesc::parse(data)?;
        Ok(Self { nm, d })
    }
}
impl Pretty for Export {
    fn pretty_indent(&self, _: usize) -> String {
        format!("(func {} {})", self.nm.pretty(), self.d.pretty())
    }
}
