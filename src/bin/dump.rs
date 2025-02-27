use maixsense_a010::SerialPortWrapper;
use std::{
    fs::{remove_file, OpenOptions}, io::{BufWriter, Write}, path::PathBuf, time::Duration
};

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

    println!("sending AT");
    // Sanity check to ensure the camera is at this COM port
    reader.write_all(b"AT\r\n").unwrap();

    println!("reading");

    let ok_res = reader.read_byte_slice::<4>();

    let response = String::from_utf8_lossy(&ok_res).to_string();

    println!("Read {:?} bytes: {}", ok_res.len(), response.trim());

    assert_eq!("OK", response.trim());

    // Starts LCD + USB output
    // https://wiki.sipeed.com/hardware/en/maixsense/maixsense-a010/at_command_en.html#DISP-instruction
    reader.write_all(b"AT+DISP=3\r\n").unwrap();

    reader.reset_count();


    let path = PathBuf::from("./dump.bin");
    
    let _ = remove_file(&path);
    
    let output = OpenOptions::new().write(true).create_new(true).open(path).unwrap();
    let mut writer = BufWriter::new(output);

    std::io::copy(&mut reader, &mut writer).unwrap();

}