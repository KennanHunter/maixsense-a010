mod frame;

use frame::{Frame, FrameHead};
use serialport::SerialPort;
use std::io::Read;
use std::{
    io::{self, Write},
    time::Duration,
};

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut reader = SerialPortWrapper::new(
        serialport::new("COM4", 115200)
            .timeout(Duration::from_secs(5))
            .open()
            .unwrap(),
    );

    println!(
        "Receiving data on {} at {} baud:",
        reader.port.name().unwrap(),
        reader.port.baud_rate().unwrap()
    );

    println!("sending AT");
    // Sanity check to ensure the camera is at this COM port
    reader.write_all(b"AT\r\n").unwrap();

    println!("reading");

    let ok_res = reader.read_byte_slice::<4>();

    let response = String::from_utf8_lossy(&ok_res).to_string();

    println!("Read {:?} bytes: {}", ok_res.len(), response.trim());

    // Starts LCD + USB output
    // https://wiki.sipeed.com/hardware/en/maixsense/maixsense-a010/at_command_en.html#DISP-instruction
    reader.write_all(b"AT+DISP=3\r\n").unwrap();

    reader.reset_count();

    let frame_head = FrameHead {
        frame_begin_flag: reader.read_byte_slice(),
        frame_data_len: reader.read_u16(),
        reserved1: reader.read_u8(),
        output_mode: reader.read_u8(),
        senser_temp: reader.read_u8(),
        driver_temp: reader.read_u8(),
        exposure_time: reader.read_byte_slice(),
        error_code: reader.read_u8(),
        reserved2: reader.read_u8(),
        resolution_rows: reader.read_u8(),
        resolution_cols: reader.read_u8(),
        frame_id: reader.read_u16(),
        isp_version: reader.read_u8(),
        reserved3: reader.read_u8(),
    };

    let frame_begin_flag_size = 2;
    let extra_header_size: usize = size_of::<FrameHead>() - frame_begin_flag_size;

    let mut payload = Vec::with_capacity(frame_head.frame_data_len as usize - extra_header_size);

    reader.read_exact(&mut payload).unwrap();

    let frame = Frame {
        frame_head,
        payload,
    };

    println!("{:?}", frame.frame_head);

    let sum = reader.read_u8();

    println!("Lower eight bits of sum: {:02x}", sum);

    assert_eq!(reader.read_u8(), 0xDD)
}

pub struct SerialPortWrapper {
    pub port: Box<dyn SerialPort>,
    pub internal_buff: Vec<u8>,
    count: u128,
}

impl SerialPortWrapper {
    pub fn new(reader: Box<dyn SerialPort>) -> SerialPortWrapper {
        SerialPortWrapper {
            port: reader,
            internal_buff: vec![0; 4],
            count: 0,
        }
    }

    pub fn read_byte_slice<const N: usize>(&mut self) -> [u8; N] {
        self.internal_buff.resize(N, 0);

        self.port.read_exact(&mut self.internal_buff).unwrap();

        for byte in &self.internal_buff {
            self.count += byte.clone() as u128;
        }

        return TryInto::<[u8; N]>::try_into(self.internal_buff.as_slice()).unwrap();
    }

    pub fn read_u8(&mut self) -> u8 {
        u8::from_le_bytes(self.read_byte_slice())
    }

    pub fn read_u16(&mut self) -> u16 {
        u16::from_le_bytes(self.read_byte_slice())
    }

    pub fn reset_count(&mut self) {
        self.count = 0;
    }

    pub fn count(&self) -> u128 {
        self.count
    }
}

impl Read for SerialPortWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.port.read(buf)
    }
}

impl Write for SerialPortWrapper {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.port.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.port.flush()
    }
}
