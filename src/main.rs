use maixsense_a010::frame::{Frame, FrameHead};
use maixsense_a010::SerialPortWrapper;
use std::io::Read;
use std::{io::Write, time::Duration};

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut reader = SerialPortWrapper::new(
        serialport::new("COM9", 115200)
            .timeout(Duration::from_secs(5))
            .open()
            .unwrap(),
    );

    println!(
        "Receiving data on {} at {} baud:",
        reader.port.name().unwrap(),
        reader.port.baud_rate().unwrap()
    );

    // println!("sending AT");
    // // Sanity check to ensure the camera is at this COM port
    // reader.write_all(b"AT\r\n").unwrap();

    // println!("reading");

    // let ok_res = reader.read_byte_slice::<4>();

    // let response = String::from_utf8_lossy(&ok_res).to_string();

    // println!("Read {:?} bytes: {}", ok_res.len(), response.trim());

    // assert_eq!("OK", response.trim());

    // Starts LCD + USB output
    // https://wiki.sipeed.com/hardware/en/maixsense/maixsense-a010/at_command_en.html#DISP-instruction
    reader.write_all(b"AT+DISP=3\r\n").unwrap();

        // read to next frame start
    loop {
        reader.reset_count();
        if reader.read_byte_slice::<2>() == [0x00, 0xFF] {
            break;
        }
    }

    let frame_head = FrameHead {
        frame_begin_flag: [0x00, 0xFF],
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

    let mut payload = Vec::with_capacity(frame_head.frame_data_len as usize - 2);

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
