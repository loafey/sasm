use std::ops::{Deref, DerefMut};

use super::{Parsable, Pretty};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct MemIdX(u32);
impl Deref for MemIdX {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MemIdX {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Parsable for MemIdX {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(Self(u32::parse(data, stack)?))
    }
}
impl Pretty for MemIdX {
    fn pretty_indent(&self, _: usize) -> String {
        todo!()
    }
}
