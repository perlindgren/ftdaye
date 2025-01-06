use ftdaye::ftdaye::jtag::FtdiMpsse;
use ftdaye::ftdaye::mpsse::cmd_write_imm;
use ftdaye::ftdaye::{BitMode, Device, Interface};
use log::*;
use nusb::DeviceInfo;
use std::{
    io::{Read, Write},
    time::Duration,
};

fn main() {
    pretty_env_logger::init();

    let device_info = nusb::list_devices()
        .unwrap()
        .find(|dev| dev.vendor_id() == 0x0403 && dev.product_id() == 0x6010)
        .expect("device not connected");

    debug!("device_info {:?}", device_info);

    let mut device = ftdaye::ftdaye::Builder::new()
        .with_interface(Interface::A)
        .with_read_timeout(Duration::from_secs(5))
        .with_write_timeout(Duration::from_secs(5))
        .usb_open(device_info)
        .unwrap();

    let mut ft = FtdiMpsse::new(device, 1000);

    println!("-- reset --");
    ft.reset_to_rti();

    let mut data = [0u8; 4];
    ft.read_write_register(0b00_1001, &mut data);

    println!("read data   {:#04x?}", data);
    let idcode = u32::from_le_bytes(data);
    println!("read idcode {:#010x?}", idcode);
    assert_eq!(idcode, 0x0362d093);

    println!("write user3 through setting ir");
    let mut data = [0x1, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    ft.read_write_register(0b10_0010, &mut data);
    println!("first read {:x?}", data);

    let mut data = [0xde, 0xad, 0xbe, 0xef, 1, 3, 3, 7];
    ft.read_write_register(0b10_0010, &mut data);
    println!("second read {:x?}", data);

    let mut data = [0, 0, 0, 0, 0, 0, 0, 0];
    ft.read_write_register(0b10_0010, &mut data);
    println!("third read {:x?}", data);
}
