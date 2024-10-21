use super::error::ParseError;
use std::{
    fmt::Debug,
    io::{Cursor, Read},
};

pub type DebugStack<'t> = &'t mut Vec<&'static str>;

pub trait Parsable: Debug {
    const STACK_NAME: &'static str = std::any::type_name::<Self>();
    fn parse(data: &mut Cursor<&[u8]>, stack: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        stack.push(Self::STACK_NAME);
        #[allow(deprecated)]
        let res = Self::parse_inner(data, stack)?;
        stack.pop();
        Ok(res)
    }
    #[deprecated]
    fn parse_inner(
        data: &mut Cursor<&[u8]>,
        stack: &mut Vec<&'static str>,
    ) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized;
}

impl<T: Parsable> Parsable for Vec<T> {
    fn parse_inner(data: &mut Cursor<&[u8]>, stack: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let len = u32::parse(data, stack)?;
        let mut v = Vec::new();
        for _ in 0..len {
            v.push(T::parse(data, stack)?);
        }
        Ok(v)
    }
}
impl Parsable for u8 {
    fn parse_inner(data: &mut Cursor<&[u8]>, _: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut b = [0];
        data.read_exact(&mut b)?;
        Ok(b[0])
    }
}

impl Parsable for u32 {
    fn parse_inner(data: &mut Cursor<&[u8]>, _: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(leb128::read::unsigned(data)? as u32)
    }
}

impl Parsable for i32 {
    fn parse_inner(data: &mut Cursor<&[u8]>, _: DebugStack) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized,
    {
        Ok(leb128::read::signed(data)? as i32)
    }
}
