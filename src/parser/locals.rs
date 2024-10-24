use super::{Parsable, ValType};

#[derive(Debug)]
#[allow(unused)]
pub struct Locals {
    pub n: u32,
    pub t: ValType,
}
impl Parsable for Locals {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self {
            n: Parsable::parse(data, stack)?,
            t: Parsable::parse(data, stack)?,
        })
    }
}
