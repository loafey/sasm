use super::{
    error::ParseError, BlockType, DataIdx as DataIdX, FuncIdx, FuncType, GlobalIdX, LabelIdX,
    LocalIdX, Locals, MemArg, NumType, Parsable, RefTyp, TableIdX, TypeIdX, ValType,
};
use crate::hex::Hex;
use std::io::Read;
use Instr::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BT {
    Block,
    Loop,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types, unused)]
#[repr(u16)]
pub enum Instr {
    x00_unreachable = 0x00,
    x01_nop = 0x01,
    x02_block(BlockType, Vec<Instr>) = 0x02,
    x03_loop(BlockType, Vec<Instr>) = 0x03,
    x04_if_else(BlockType, Vec<Instr>, Option<Vec<Instr>>) = 0x04,
    x05 = 0x05,
    x06 = 0x06,
    x07 = 0x07,
    x08 = 0x08,
    x09 = 0x09,
    x0a = 0x0a,
    x0b = 0x0b,
    x0c_br(LabelIdX) = 0x0c,
    x0d_br_if(LabelIdX) = 0x0d,
    x0e_br_table(Vec<LabelIdX>, LabelIdX) = 0x0e,
    x0f_return = 0x0f,
    x10_call(FuncIdx) = 0x41,
    x11_call_indirect(TypeIdX, TableIdX) = 0x11,
    x12 = 0x12,
    x13 = 0x13,
    x14 = 0x14,
    x15 = 0x15,
    x16 = 0x16,
    x17 = 0x17,
    x18 = 0x18,
    x19 = 0x19,
    x1a_drop = 0x1a,
    x1b_select = 0x1b,
    x1c = 0x1c,
    x1d = 0x1d,
    x1e = 0x1e,
    x1f = 0x1f,
    x20_local_get(LocalIdX) = 0x20,
    x21_local_set(LocalIdX) = 0x21,
    x22_local_tee(LocalIdX) = 0x22,
    x23_global_get(GlobalIdX) = 0x23,
    x24_global_set(GlobalIdX) = 0x24,
    x25 = 0x25,
    x26_table_set(TableIdX) = 0x26,
    x27 = 0x27,
    x28_i32_load(MemArg) = 0x28,
    x29_i64_load(MemArg) = 0x29,
    x2a_f32_load(MemArg) = 0x2a,
    x2b_f64_load(MemArg) = 0x2b,
    x2c_i32_load8_s(MemArg) = 0x2c,
    x2d_i32_load8_u(MemArg) = 0x2d,
    x2e_i32_load16_s(MemArg) = 0x2e,
    x2f_i32_load16_u(MemArg) = 0x2f,
    x30_i64_load8_s(MemArg) = 0x30,
    x31_i64_load8_u(MemArg) = 0x31,
    x32_i64_load16_s(MemArg) = 0x32,
    x33_i64_load16_u(MemArg) = 0x33,
    x34_i64_load32_s(MemArg) = 0x34,
    x35_i64_load32_u(MemArg) = 0x35,
    x36_i32_store(MemArg) = 0x36,
    x37_i64_store(MemArg) = 0x37,
    x38_f32_store(MemArg) = 0x38,
    x39_f64_store(MemArg) = 0x39,
    x3a_i32_store8(MemArg) = 0x3a,
    x3b_i32_store16(MemArg) = 0x3b,
    x3c_i64_store8(MemArg) = 0x3c,
    x3d_i64_store16(MemArg) = 0x3d,
    x3e_i64_store32(MemArg) = 0x3e,
    x3f = 0x3f,
    x40_grow = 0x40,
    x41_i32_const(i32) = 0x10,
    x42_i64_const(i64) = 0x42,
    x43_f32_const(f32) = 0x43,
    x44_f64_const(f64) = 0x44,
    x45_i32_eqz = 0x45,
    x46_i32_eq = 0x46,
    x47_i32_ne = 0x47,
    x48_i32_lt_s = 0x48,
    x49_i32_lt_u = 0x49,
    x4a_i32_gt_s = 0x4a,
    x4b_i32_gt_u = 0x4b,
    x4c_i32_le_s = 0x4c,
    x4d_i32_le_u = 0x4d,
    x4e_i32_ge_s = 0x4e,
    x4f_i32_ge_u = 0x4f,
    x50_i64_eqz = 0x50,
    x51_i64_eq = 0x51,
    x52_i64_ne = 0x52,
    x53_i64_lt_s = 0x53,
    x54_i64_lt_u = 0x54,
    x55_i64_gt_s = 0x55,
    x56_i64_gt_u = 0x56,
    x57 = 0x57,
    x58 = 0x58,
    x59 = 0x59,
    x5a_i64_ge_u = 0x5a,
    x5b = 0x5b,
    x5c = 0x5c,
    x5d = 0x5d,
    x5e_f32_gt = 0x5e,
    x5f = 0x5f,
    x60 = 0x60,
    x61_f64_eq = 0x61,
    x62_f64_ne = 0x62,
    x63_f64_lt = 0x63,
    x64 = 0x64,
    x65 = 0x65,
    x66_f64_ge = 0x66,
    x67_i32_clz = 0x67,
    x68_i32_ctz = 0x68,
    x69 = 0x69,
    x6a_i32_add = 0x6a,
    x6b_i32_sub = 0x6b,
    x6c_i32_mul = 0x6c,
    x6d_i32_div_s = 0x6d,
    x6e_i32_div_u = 0x6e,
    x6f = 0x6f,
    x70_i32_rem_u = 0x70,
    x71_i32_and = 0x71,
    x72_i32_or = 0x72,
    x73_i32_xor = 0x73,
    x74_i32_shl = 0x74,
    x75_i32_shr_s = 0x75,
    x76_i32_shr_u = 0x76,
    x77_i32_rotl = 0x77,
    x78 = 0x78,
    x79 = 0x79,
    x7a = 0x7a,
    x7c_i64_add = 0x7c,
    x7d_i64_sub = 0x7d,
    x7e_i64_mul = 0x7e,
    x7f_i64_div_s = 0x7f,
    x80_i64_div_u = 0x80,
    x81 = 0x81,
    x82 = 0x82,
    x83_i64_and = 0x83,
    x84_i64_or = 0x84,
    x85_i64_xor = 0x85,
    x86_i64_shl = 0x86,
    x87 = 0x87,
    x88_i64_shr_u = 0x88,
    x89 = 0x89,
    x8a = 0x8a,
    x8b = 0x8b,
    x8c = 0x8c,
    x8d = 0x8d,
    x8e = 0x8e,
    x8f = 0x8f,
    x90 = 0x90,
    x91 = 0x91,
    x92 = 0x92,
    x93 = 0x93,
    x94 = 0x94,
    x95 = 0x95,
    x96 = 0x96,
    x97 = 0x97,
    x98 = 0x98,
    x99_f64_abs = 0x99,
    x9a_f64_neg = 0x9a,
    x9b = 0x9b,
    x9c = 0x9c,
    x9d = 0x9d,
    x9e = 0x9e,
    x9f = 0x9f,
    xa0_f64_add = 0xa0,
    xa1_f64_sub = 0xa1,
    xa2_f64_mul = 0xa2,
    xa3 = 0xa3,
    xa4 = 0xa4,
    xa5 = 0xa5,
    xa6 = 0xa6,
    xa7_i32_wrap_i64 = 0xa7,
    xa8 = 0xa8,
    xa9 = 0xa9,
    xaa_i32_trunc_f64_s = 0xaa,
    xab_i32_trunc_f64_u = 0xab,
    xac_i64_extend_i32_s = 0xac,
    xad_i64_extend_i32_u = 0xad,
    xae = 0xae,
    xaf = 0xaf,
    xb0 = 0xb0,
    xb1 = 0xb1,
    xb2 = 0xb2,
    xb3 = 0xb3,
    xb4 = 0xb4,
    xb5 = 0xb5,
    xb6 = 0xb6,
    xb7_f64_convert_i32_s = 0xb7,
    xb8_f64_convert_i32_u = 0xb8,
    xb9 = 0xb9,
    xba = 0xba,
    xbb = 0xbb,
    xbd_i64_reinterpret_f64 = 0xbd,
    xbe = 0xbe,
    xbf_f64_reinterpret_i64 = 0xbf,
    xc0 = 0xc0,
    xc1 = 0xc1,
    xc2 = 0xc2,
    xc3 = 0xc3,
    xc4 = 0xc4,
    xc5 = 0xc5,
    xc6 = 0xc6,
    xc7 = 0xc7,
    xc8 = 0xc8,
    xc9 = 0xc9,
    xca = 0xca,
    xcb = 0xcb,
    xcc = 0xcc,
    xcd = 0xcd,
    xce = 0xce,
    xcf = 0xcf,
    xd0_ref_null(RefTyp) = 0xd0,
    xd1 = 0xd1,
    xd2_ref_func(FuncIdx) = 0xd2,
    xd3 = 0xd3,
    xd4 = 0xd4,
    xd5 = 0xd5,
    xd6 = 0xd6,
    xd7 = 0xd7,
    xd8 = 0xd8,
    xd9 = 0xd9,
    xda = 0xda,
    xdb = 0xdb,
    xdc = 0xdc,
    xdd = 0xdd,
    xde = 0xde,
    xdf = 0xdf,
    xe0 = 0xe0,
    xe1 = 0xe1,
    xe2 = 0xe2,
    xe3 = 0xe3,
    xe4 = 0xe4,
    xe5 = 0xe5,
    xe6 = 0xe6,
    xe7 = 0xe7,
    xe8 = 0xe8,
    xe9 = 0xe9,
    xea = 0xea,
    xeb = 0xeb,
    xec = 0xec,
    xed = 0xed,
    xee = 0xee,
    xef = 0xef,
    xf0 = 0xf0,
    xf1 = 0xf1,
    xf2 = 0xf2,
    xf3 = 0xf3,
    xf4 = 0xf4,
    xf5 = 0xf5,
    xf6 = 0xf6,
    xf7 = 0xf7,
    xf8 = 0xf8,
    xf9 = 0xf9,
    xfa = 0xfa,
    xfb = 0xfb,
    xfc_0_i32_trunc_sat_f32_s = 0xfc00,
    xfc_1_i32_trunc_sat_f32_u = 0xfc01,
    xfc_2_i32_trunc_sat_f64_u = 0xfc02,
    xfc_3_i32_trunc_sat_f64_s = 0xfc03,
    xfc_4_i64_trunc_sat_f32_s = 0xfc04,
    xfc_5_i64_trunc_sat_f32_u = 0xfc05,
    xfc_6_i64_trunc_sat_f64_s = 0xfc06,
    xfc_7_i64_trunc_sat_f64_u = 0xfc07,
    xfc_8_memory_init(DataIdX) = 0xfc08,
    xfc_9_data_drop(DataIdX) = 0xfc09,
    xfc_10_memory_copy(u8, u8) = 0xfc0a,
    xfc_11_memory_fill(u8) = 0xfc0b,
    xfd = 0xfd,
    xfe = 0xfe,
    xff = 0xff,
    block_start(BT, usize, BlockType),
    block_end(BT, usize, BlockType),
    comment(String, Box<Instr>),
}

#[derive(Debug)]
pub enum TypingRuleError {
    MissingFunction(&'static str),
    MissingLocal,
}

#[derive(Debug, Default)]
pub struct TypingRules {
    pub input: Vec<ValType>,
    pub output: Vec<ValType>,
}
impl TypingRules {
    pub fn single_input(valtype: ValType) -> TypingRules {
        Self {
            input: vec![valtype],
            output: Vec::new(),
        }
    }
    pub fn single_output(valtype: ValType) -> TypingRules {
        Self {
            output: vec![valtype],
            input: Vec::new(),
        }
    }
}
impl Instr {
    pub fn get_types(
        &self,
        locals: &[ValType],
        function_types: &[FuncType],
        raw_types: &[FuncType],
        globals: &[ValType],
    ) -> Result<TypingRules, TypingRuleError> {
        Ok(match self {
            x00_unreachable => todo!(),
            x01_nop => todo!(),
            x02_block(bt, _) | x03_loop(bt, _) => match bt {
                BlockType::Eps => TypingRules::default(),
                BlockType::T(val_type) => TypingRules {
                    input: Vec::new(),
                    output: vec![*val_type],
                },
                BlockType::TypIdx(i) => {
                    let ft = raw_types
                        .get(*i as usize)
                        .ok_or(TypingRuleError::MissingFunction("block type"))?;
                    TypingRules {
                        input: ft.input.types.clone(),
                        output: ft.output.types.clone(),
                    }
                }
            },
            x04_if_else(_, _, _) => TypingRules::default(),
            x05 => todo!(),
            x06 => todo!(),
            x07 => todo!(),
            x08 => todo!(),
            x09 => todo!(),
            x0a => todo!(),
            x0b => todo!(),
            x0c_br(_) => todo!(),
            x0d_br_if(_) => todo!(),
            x0e_br_table(_, _) => todo!(),
            x0f_return => TypingRules::default(),
            x10_call(FuncIdx(i)) => {
                let ft = function_types
                    .get(*i as usize)
                    .ok_or(TypingRuleError::MissingFunction("invalid call"))?;
                TypingRules {
                    input: ft.input.types.clone(),
                    output: ft.output.types.clone(),
                }
            }
            x11_call_indirect(_, _) => todo!(),
            x12 => todo!(),
            x13 => todo!(),
            x14 => todo!(),
            x15 => todo!(),
            x16 => todo!(),
            x17 => todo!(),
            x18 => todo!(),
            x19 => todo!(),
            x1a_drop => TypingRules {
                input: vec![ValType::Nil],
                output: Vec::new(),
            },
            x1b_select => todo!(),
            x1c => todo!(),
            x1d => todo!(),
            x1e => todo!(),
            x1f => todo!(),
            x20_local_get(LocalIdX(i)) => locals
                .get(*i as usize)
                .map(|l| TypingRules::single_output(*l))
                .ok_or(TypingRuleError::MissingLocal)?,
            x21_local_set(LocalIdX(i)) => TypingRules::single_input(locals[*i as usize]),
            x22_local_tee(LocalIdX(i)) => TypingRules {
                input: vec![locals[*i as usize]],
                output: vec![locals[*i as usize]],
            },
            x23_global_get(GlobalIdX(i)) => TypingRules::single_output(globals[*i as usize]),
            x24_global_set(GlobalIdX(i)) => TypingRules::single_input(globals[*i as usize]),
            x25 => todo!(),
            x26_table_set(_) => todo!(),
            x27 => todo!(),
            x28_i32_load(_) => TypingRules::single_output(ValType::Num(NumType::I32)),
            x29_i64_load(_) => TypingRules::single_output(ValType::Num(NumType::I64)),
            x2a_f32_load(_) => TypingRules::single_output(ValType::Num(NumType::F32)),
            x2b_f64_load(_) => TypingRules::single_output(ValType::Num(NumType::F64)),
            x2c_i32_load8_s(_) => TypingRules::single_output(ValType::Num(NumType::I32)),
            x2d_i32_load8_u(_) => TypingRules::single_output(ValType::Num(NumType::I32)),
            x2e_i32_load16_s(_) => TypingRules::single_output(ValType::Num(NumType::I32)),
            x2f_i32_load16_u(_) => TypingRules::single_output(ValType::Num(NumType::I32)),
            x30_i64_load8_s(_) => TypingRules::single_output(ValType::Num(NumType::I64)),
            x31_i64_load8_u(_) => TypingRules::single_output(ValType::Num(NumType::I64)),
            x32_i64_load16_s(_) => TypingRules::single_output(ValType::Num(NumType::I64)),
            x33_i64_load16_u(_) => TypingRules::single_output(ValType::Num(NumType::I64)),
            x34_i64_load32_s(_) => TypingRules::single_output(ValType::Num(NumType::I64)),
            x35_i64_load32_u(_) => TypingRules::single_output(ValType::Num(NumType::I64)),
            x36_i32_store(_) => TypingRules::single_input(ValType::Num(NumType::I32)),
            x37_i64_store(_) => todo!(),
            x38_f32_store(_) => todo!(),
            x39_f64_store(_) => todo!(),
            x3a_i32_store8(_) => todo!(),
            x3b_i32_store16(_) => todo!(),
            x3c_i64_store8(_) => todo!(),
            x3d_i64_store16(_) => todo!(),
            x3e_i64_store32(_) => todo!(),
            x3f => todo!(),
            x40_grow => TypingRules {
                input: vec![ValType::Num(NumType::I32)],
                output: vec![ValType::Num(NumType::I32)],
            },
            x41_i32_const(_) => TypingRules::single_output(ValType::Num(NumType::I32)),
            x42_i64_const(_) => TypingRules::single_output(ValType::Num(NumType::I64)),
            x43_f32_const(_) => TypingRules::single_output(ValType::Num(NumType::F32)),
            x44_f64_const(_) => TypingRules::single_output(ValType::Num(NumType::F64)),
            x45_i32_eqz => TypingRules {
                input: vec![ValType::Num(NumType::I32)],
                output: vec![ValType::Num(NumType::I32)],
            },
            x46_i32_eq => TypingRules {
                input: vec![ValType::Num(NumType::I32), ValType::Num(NumType::I32)],
                output: vec![ValType::Num(NumType::I32)],
            },
            x47_i32_ne => todo!(),
            x48_i32_lt_s => todo!(),
            x49_i32_lt_u => todo!(),
            x4a_i32_gt_s => todo!(),
            x4b_i32_gt_u => todo!(),
            x4c_i32_le_s => todo!(),
            x4d_i32_le_u => todo!(),
            x4e_i32_ge_s => todo!(),
            x4f_i32_ge_u => todo!(),
            x50_i64_eqz => todo!(),
            x51_i64_eq => todo!(),
            x52_i64_ne => todo!(),
            x53_i64_lt_s => todo!(),
            x54_i64_lt_u => todo!(),
            x55_i64_gt_s => todo!(),
            x56_i64_gt_u => todo!(),
            x57 => todo!(),
            x58 => todo!(),
            x59 => todo!(),
            x5a_i64_ge_u => todo!(),
            x5b => todo!(),
            x5c => todo!(),
            x5d => todo!(),
            x5e_f32_gt => TypingRules {
                input: vec![ValType::Num(NumType::F32), ValType::Num(NumType::F32)],
                output: vec![ValType::Num(NumType::I32)],
            },
            x5f => todo!(),
            x60 => todo!(),
            x61_f64_eq => todo!(),
            x62_f64_ne => todo!(),
            x63_f64_lt => todo!(),
            x64 => todo!(),
            x65 => todo!(),
            x66_f64_ge => todo!(),
            x67_i32_clz => todo!(),
            x68_i32_ctz => TypingRules {
                input: vec![ValType::Num(NumType::I32)],
                output: vec![ValType::Num(NumType::I32)],
            },
            x69 => todo!(),
            x6a_i32_add => TypingRules {
                input: vec![ValType::Num(NumType::I32), ValType::Num(NumType::I32)],
                output: vec![ValType::Num(NumType::I32)],
            },
            x6b_i32_sub => todo!(),
            x6c_i32_mul => TypingRules {
                input: vec![ValType::Num(NumType::I32), ValType::Num(NumType::I32)],
                output: vec![ValType::Num(NumType::I32)],
            },
            x6d_i32_div_s => todo!(),
            x6e_i32_div_u => todo!(),
            x6f => todo!(),
            x70_i32_rem_u => todo!(),
            x71_i32_and => todo!(),
            x72_i32_or => todo!(),
            x73_i32_xor => todo!(),
            x74_i32_shl => todo!(),
            x75_i32_shr_s => todo!(),
            x76_i32_shr_u => todo!(),
            x77_i32_rotl => todo!(),
            x78 => todo!(),
            x79 => todo!(),
            x7a => todo!(),
            x7c_i64_add => todo!(),
            x7d_i64_sub => todo!(),
            x7e_i64_mul => todo!(),
            x7f_i64_div_s => todo!(),
            x80_i64_div_u => todo!(),
            x81 => todo!(),
            x82 => todo!(),
            x83_i64_and => todo!(),
            x84_i64_or => todo!(),
            x85_i64_xor => todo!(),
            x86_i64_shl => todo!(),
            x87 => todo!(),
            x88_i64_shr_u => todo!(),
            x89 => todo!(),
            x8a => todo!(),
            x8b => todo!(),
            x8c => todo!(),
            x8d => todo!(),
            x8e => todo!(),
            x8f => todo!(),
            x90 => todo!(),
            x91 => todo!(),
            x92 => todo!(),
            x93 => todo!(),
            x94 => todo!(),
            x95 => todo!(),
            x96 => todo!(),
            x97 => todo!(),
            x98 => todo!(),
            x99_f64_abs => todo!(),
            x9a_f64_neg => todo!(),
            x9b => todo!(),
            x9c => todo!(),
            x9d => todo!(),
            x9e => todo!(),
            x9f => todo!(),
            xa0_f64_add => todo!(),
            xa1_f64_sub => todo!(),
            xa2_f64_mul => todo!(),
            xa3 => todo!(),
            xa4 => todo!(),
            xa5 => todo!(),
            xa6 => todo!(),
            xa7_i32_wrap_i64 => todo!(),
            xa8 => todo!(),
            xa9 => todo!(),
            xaa_i32_trunc_f64_s => todo!(),
            xab_i32_trunc_f64_u => todo!(),
            xac_i64_extend_i32_s => todo!(),
            xad_i64_extend_i32_u => todo!(),
            xae => todo!(),
            xaf => todo!(),
            xb0 => todo!(),
            xb1 => todo!(),
            xb2 => todo!(),
            xb3 => todo!(),
            xb4 => todo!(),
            xb5 => todo!(),
            xb6 => todo!(),
            xb7_f64_convert_i32_s => todo!(),
            xb8_f64_convert_i32_u => todo!(),
            xb9 => todo!(),
            xba => todo!(),
            xbb => todo!(),
            xbd_i64_reinterpret_f64 => todo!(),
            xbe => todo!(),
            xbf_f64_reinterpret_i64 => todo!(),
            xc0 => todo!(),
            xc1 => todo!(),
            xc2 => todo!(),
            xc3 => todo!(),
            xc4 => todo!(),
            xc5 => todo!(),
            xc6 => todo!(),
            xc7 => todo!(),
            xc8 => todo!(),
            xc9 => todo!(),
            xca => todo!(),
            xcb => todo!(),
            xcc => todo!(),
            xcd => todo!(),
            xce => todo!(),
            xcf => todo!(),
            xd0_ref_null(_) => todo!(),
            xd1 => todo!(),
            xd2_ref_func(_) => todo!(),
            xd3 => todo!(),
            xd4 => todo!(),
            xd5 => todo!(),
            xd6 => todo!(),
            xd7 => todo!(),
            xd8 => todo!(),
            xd9 => todo!(),
            xda => todo!(),
            xdb => todo!(),
            xdc => todo!(),
            xdd => todo!(),
            xde => todo!(),
            xdf => todo!(),
            xe0 => todo!(),
            xe1 => todo!(),
            xe2 => todo!(),
            xe3 => todo!(),
            xe4 => todo!(),
            xe5 => todo!(),
            xe6 => todo!(),
            xe7 => todo!(),
            xe8 => todo!(),
            xe9 => todo!(),
            xea => todo!(),
            xeb => todo!(),
            xec => todo!(),
            xed => todo!(),
            xee => todo!(),
            xef => todo!(),
            xf0 => todo!(),
            xf1 => todo!(),
            xf2 => todo!(),
            xf3 => todo!(),
            xf4 => todo!(),
            xf5 => todo!(),
            xf6 => todo!(),
            xf7 => todo!(),
            xf8 => todo!(),
            xf9 => todo!(),
            xfa => todo!(),
            xfb => todo!(),
            xfc_0_i32_trunc_sat_f32_s => todo!(),
            xfc_1_i32_trunc_sat_f32_u => todo!(),
            xfc_2_i32_trunc_sat_f64_u => todo!(),
            xfc_3_i32_trunc_sat_f64_s => todo!(),
            xfc_4_i64_trunc_sat_f32_s => todo!(),
            xfc_5_i64_trunc_sat_f32_u => todo!(),
            xfc_6_i64_trunc_sat_f64_s => todo!(),
            xfc_7_i64_trunc_sat_f64_u => todo!(),
            xfc_8_memory_init(_) => todo!(),
            xfc_9_data_drop(_) => todo!(),
            xfc_10_memory_copy(_, _) => todo!(),
            xfc_11_memory_fill(_) => todo!(),
            xfd => todo!(),
            xfe => todo!(),
            xff => todo!(),
            block_start(_, _, _) => todo!(),
            block_end(_, _, _) => todo!(),
            comment(_, _) => todo!(),
        })
    }
}
impl Parsable for Instr {
    fn parse_inner(
        data: &mut std::io::Cursor<&[u8]>,
        stack: super::DebugStack,
    ) -> Result<Self, super::error::ParseError>
    where
        Self: std::marker::Sized,
    {
        macro_rules! p {
            () => {
                Parsable::parse(data, stack)?
            };
        }
        macro_rules! val {
            ($cons:expr, $size:expr) => {{
                let p = MemArg::parse(data, stack)?;
                if p.align > $size {
                    return Err(ParseError::AlignmentError);
                }
                $cons(p)
            }};
        }
        let mut typ = [0];
        data.read_exact(&mut typ)?;
        Ok(match typ[0] {
            0x00 => x00_unreachable,
            0x01 => x01_nop,
            0x02 => {
                let block_type = p!();
                let mut v = Vec::new();
                loop {
                    match Instr::parse(data, stack) {
                        Ok(i) => v.push(i),
                        Err(ParseError::EndOfInstructions) => {
                            stack.pop();
                            break;
                        }
                        Err(e) => Err(e)?,
                    }
                }
                x02_block(block_type, v)
            }
            0x03 => {
                let block_type = p!();
                let mut v = Vec::new();
                loop {
                    match Instr::parse(data, stack) {
                        Ok(i) => v.push(i),
                        Err(ParseError::EndOfInstructions) => {
                            stack.pop();
                            break;
                        }
                        Err(e) => Err(e)?,
                    }
                }
                x03_loop(block_type, v)
            }
            0x04 => {
                let block_type = p!();
                let mut v = Vec::new();
                loop {
                    match Instr::parse(data, stack) {
                        Ok(i) => v.push(i),
                        Err(ParseError::ElseHit) => {
                            stack.pop();
                            break;
                        }
                        Err(ParseError::EndOfInstructions) => {
                            stack.pop();
                            return Ok(x04_if_else(block_type, v, None));
                        }
                        Err(e) => Err(e)?,
                    }
                }
                let els = {
                    let mut v = Vec::new();
                    loop {
                        match Instr::parse(data, stack) {
                            Ok(i) => v.push(i),
                            Err(ParseError::EndOfInstructions) => {
                                stack.pop();
                                break;
                            }
                            Err(e) => Err(e)?,
                        }
                    }
                    v
                };
                x04_if_else(block_type, v, Some(els))
            }
            0x05 => Err(ParseError::ElseHit)?,
            0x06 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x07 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x08 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x09 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x0a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x0b => Err(ParseError::EndOfInstructions)?,
            0x0c => x0c_br(p!()),
            0x0d => x0d_br_if(p!()),
            0x0e => x0e_br_table(p!(), p!()),
            0x0f => x0f_return,
            0x10 => x10_call(p!()),
            0x11 => x11_call_indirect(p!(), p!()),
            0x12 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x13 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x14 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x15 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x16 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x17 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x18 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x19 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1a => x1a_drop,
            0x1b => x1b_select,
            0x1c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x1f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x20 => x20_local_get(p!()),
            0x21 => x21_local_set(p!()),
            0x22 => x22_local_tee(p!()),
            0x23 => x23_global_get(p!()),
            0x24 => x24_global_set(p!()),
            0x25 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x26 => x26_table_set(p!()),
            0x27 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x28 => val!(x28_i32_load, 4),
            0x29 => val!(x29_i64_load, 8),
            0x2a => val!(x2a_f32_load, 4),
            0x2b => val!(x2b_f64_load, 8),
            0x2c => val!(x2c_i32_load8_s, 1),
            0x2d => val!(x2d_i32_load8_u, 1),
            0x2e => val!(x2e_i32_load16_s, 2),
            0x2f => val!(x2f_i32_load16_u, 2),
            0x30 => val!(x30_i64_load8_s, 1),
            0x31 => val!(x31_i64_load8_u, 1),
            0x32 => val!(x32_i64_load16_s, 2),
            0x33 => val!(x33_i64_load16_u, 2),
            0x34 => val!(x34_i64_load32_s, 4),
            0x35 => val!(x35_i64_load32_u, 4),
            0x36 => val!(x36_i32_store, 4),
            0x37 => val!(x37_i64_store, 8),
            0x38 => val!(x38_f32_store, 4),
            0x39 => val!(x39_f64_store, 8),
            0x3a => val!(x3a_i32_store8, 1),
            0x3b => val!(x3b_i32_store16, 2),
            0x3c => val!(x3c_i64_store8, 1),
            0x3d => val!(x3d_i64_store16, 2),
            0x3e => val!(x3e_i64_store32, 4),
            0x3f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x40 => {
                let parse = u8::parse(data, stack)?;
                if 0x00 != parse {
                    Err(ParseError::Unknown0x40(Hex([parse])))?;
                }
                x40_grow
            }
            0x41 => x41_i32_const(p!()),
            0x42 => x42_i64_const(p!()),
            0x43 => x43_f32_const(p!()),
            0x44 => x44_f64_const(p!()),
            0x45 => x45_i32_eqz,
            0x46 => x46_i32_eq,
            0x47 => x47_i32_ne,
            0x48 => x48_i32_lt_s,
            0x49 => x49_i32_lt_u,
            0x4a => x4a_i32_gt_s,
            0x4b => x4b_i32_gt_u,
            0x4c => x4c_i32_le_s,
            0x4d => x4d_i32_le_u,
            0x4e => x4e_i32_ge_s,
            0x4f => x4f_i32_ge_u,
            0x50 => x50_i64_eqz,
            0x51 => x51_i64_eq,
            0x52 => x52_i64_ne,
            0x53 => x53_i64_lt_s,
            0x54 => x54_i64_lt_u,
            0x55 => x55_i64_gt_s,
            0x56 => x56_i64_gt_u,
            0x57 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x58 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x59 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5a => x5a_i64_ge_u,
            0x5b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x5e => x5e_f32_gt,
            0x5f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x60 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x61 => x61_f64_eq,
            0x62 => x62_f64_ne,
            0x63 => x63_f64_lt,
            0x64 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x65 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x66 => x66_f64_ge,
            0x67 => x67_i32_clz,
            0x68 => x68_i32_ctz,
            0x69 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x6a => x6a_i32_add,
            0x6b => x6b_i32_sub,
            0x6c => x6c_i32_mul,
            0x6d => x6d_i32_div_s,
            0x6e => x6e_i32_div_u,
            0x6f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x70 => x70_i32_rem_u,
            0x71 => x71_i32_and,
            0x72 => x72_i32_or,
            0x73 => x73_i32_xor,
            0x74 => x74_i32_shl,
            0x75 => x75_i32_shr_s,
            0x76 => x76_i32_shr_u,
            0x77 => x77_i32_rotl,
            0x78 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x79 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x7a => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x7b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x7c => x7c_i64_add,
            0x7d => x7d_i64_sub,
            0x7e => x7e_i64_mul,
            0x7f => x7f_i64_div_s,
            0x80 => x80_i64_div_u,
            0x81 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x82 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x83 => x83_i64_and,
            0x84 => x84_i64_or,
            0x85 => x85_i64_xor,
            0x86 => x86_i64_shl,
            0x87 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x88 => x88_i64_shr_u,
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
            0x99 => x99_f64_abs,
            0x9a => x9a_f64_neg,
            0x9b => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9c => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9d => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9e => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0x9f => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa0 => xa0_f64_add,
            0xa1 => xa1_f64_sub,
            0xa2 => xa2_f64_mul,
            0xa3 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa4 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa5 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa6 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa7 => xa7_i32_wrap_i64,
            0xa8 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xa9 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xaa => xaa_i32_trunc_f64_s,
            0xab => xab_i32_trunc_f64_u,
            0xac => xac_i64_extend_i32_s,
            0xad => xad_i64_extend_i32_u,
            0xae => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xaf => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb0 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb1 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb2 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb3 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb4 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb5 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb6 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xb7 => xb7_f64_convert_i32_s,
            0xb8 => xb8_f64_convert_i32_u,
            0xb9 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xba => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xbb => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xbc => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xbd => xbd_i64_reinterpret_f64,
            0xbe => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xbf => xbf_f64_reinterpret_i64,
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
            0xd0 => xd0_ref_null(p!()),
            0xd1 => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xd2 => xd2_ref_func(p!()),
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
            0xfc => match u32::parse(data, stack)? {
                0 => xfc_0_i32_trunc_sat_f32_s,
                1 => xfc_1_i32_trunc_sat_f32_u,
                2 => xfc_2_i32_trunc_sat_f64_u,
                3 => xfc_3_i32_trunc_sat_f64_s,
                4 => xfc_4_i64_trunc_sat_f32_s,
                5 => xfc_5_i64_trunc_sat_f32_u,
                6 => xfc_6_i64_trunc_sat_f64_s,
                7 => xfc_7_i64_trunc_sat_f64_u,
                8 => xfc_8_memory_init(p!()),
                9 => xfc_9_data_drop(p!()),
                10 => xfc_10_memory_copy(p!(), p!()),
                11 => xfc_11_memory_fill(p!()),
                ind => todo!("0xfc {ind}"),
            },
            0xfd => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xfe => Err(ParseError::UnknownInstruction(Hex(typ)))?,
            0xff => Err(ParseError::UnknownInstruction(Hex(typ)))?,
        })
    }
}
