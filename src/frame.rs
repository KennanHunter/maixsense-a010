#![allow(unused)]

const FRAME_BEGIN_FLAG: u16 = 0xFF00;
const FRAME_END_FLAG: u8 = 0xDD;

const FRAME_HEAD_SIZE: usize = 20;
const FRAME_HEAD_DATA_SIZE: usize = 16;
const FRAME_CHECKSUM_SIZE: usize = 1;
const FRAME_END_SIZE: usize = 1;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FrameHead {
    pub frame_begin_flag: [u8; 2],
    pub frame_data_len: u16,
    pub reserved1: u8,   // fixed to 0xFF
    pub output_mode: u8, // 0: depth only, 1: depth + IR
    pub senser_temp: u8,
    pub driver_temp: u8,
    pub exposure_time: [u8; 4],
    pub error_code: u8,
    pub reserved2: u8, // fixed to 0x00
    pub resolution_rows: u8,
    pub resolution_cols: u8,
    pub frame_id: u16, // 12-bit, 0~4095
    pub isp_version: u8,
    pub reserved3: u8, // fixed to 0xFF
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Frame {
    pub frame_head: FrameHead,
    pub payload: Vec<u8>, // Variable-length payload
}
