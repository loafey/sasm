use super::{Parsable, Pretty, ValType};

#[derive(Debug)]
#[allow(unused)]
pub struct ResultType {
    pub types: Vec<ValType>,
}
impl Parsable for ResultType {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(ResultType {
            types: Vec::parse(data, stack)?,
        })
    }
}
impl Pretty for ResultType {
    fn pretty_indent(&self, _: usize) -> String {
        let s = self
            .types
            .iter()
            .map(|s| s.pretty())
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{s}]")
    }
}
