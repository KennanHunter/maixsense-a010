pub mod frame;

use std::io;
use serialport::SerialPort;

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

impl std::io::Read for SerialPortWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.port.read(buf)
    }
}

impl std::io::Write for SerialPortWrapper {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.port.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.port.flush()
    }
}
