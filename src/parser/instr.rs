use super::{error::ParseError, FuncIdx, Parsable};
use crate::hex::Hex;
use std::io::Read;
use Instr::*;

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, unused)]
pub enum Instr {
    i32_const(i32),
    call(FuncIdx),
}
impl Parsable for Instr {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        let mut typ = [0];
        data.read_exact(&mut typ)?;
        Ok(match typ[0] {
            0x00 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x01 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x02 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x03 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x04 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x05 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x06 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x07 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x08 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x09 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x0a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x0b => Err(ParseError::EndOfInstructions)?,
            0x0c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x0d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x0e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x0f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x10 => call(FuncIdx::parse(data, stack)?),
            0x11 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x12 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x13 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x14 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x15 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x16 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x17 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x18 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x19 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x20 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x21 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x22 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x23 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x24 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x25 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x26 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x27 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x28 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x29 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x2a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x2b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x2c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x2d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x2e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x2f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x30 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x31 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x32 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x33 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x34 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x35 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x36 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x37 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x38 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x39 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x3a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x3b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x3c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x3d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x3e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x3f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x40 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x41 => i32_const(i32::parse(data, stack)?),
            0x42 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x43 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x44 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x45 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x46 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x47 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x48 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x49 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x4a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x4b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x4c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x4d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x4e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x4f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x50 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x51 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x52 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x53 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x54 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x55 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x56 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x57 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x58 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x59 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x60 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x61 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x62 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x63 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x64 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x65 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x66 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x67 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x68 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x69 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x6a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x6b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x6c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x6d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x6e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x6f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x70 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x71 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x72 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x73 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x74 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x75 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x76 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x77 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x78 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x79 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x7a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x7b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x7c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x7d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x7e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x7f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x80 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x81 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x82 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x83 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x84 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x85 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x86 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x87 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x88 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x89 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x8a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x8b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x8c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x8d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x8e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x8f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x90 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x91 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x92 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x93 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x94 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x95 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x96 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x97 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x98 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x99 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa0 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa1 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa2 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa3 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa4 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa5 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa6 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa7 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa8 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa9 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xaa => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xab => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xac => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xad => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xae => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xaf => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb0 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb1 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb2 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb3 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb4 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb5 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb6 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb7 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb8 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb9 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xba => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xbb => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xbc => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xbd => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xbe => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xbf => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc0 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc1 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc2 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc3 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc4 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc5 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc6 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc7 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc8 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xc9 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xca => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xcb => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xcc => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xcd => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xce => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xcf => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd0 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd1 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd2 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd3 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd4 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd5 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd6 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd7 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd8 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd9 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xda => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xdb => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xdc => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xdd => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xde => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xdf => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe0 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe1 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe2 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe3 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe4 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe5 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe6 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe7 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe8 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xe9 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xea => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xeb => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xec => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xed => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xee => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xef => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf0 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf1 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf2 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf3 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf4 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf5 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf6 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf7 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf8 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xf9 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xfa => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xfb => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xfc => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xfd => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xfe => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xff => Err(ParseError::UnknownInstruction(Hex(typ)))?,
        })
    }
}
