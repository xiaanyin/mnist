use std::io;
use std::io::prelude::*;
use std::fs::File;
use crate::{HEAD_SIZE, IMAGE_SIZE, LABEL_SIZE};


pub fn read_next_head(source: &mut File) -> u32 {
    let mut buffer: [u8; HEAD_SIZE] = [0; HEAD_SIZE];
    match source.read_exact(&mut buffer) {
        Err(_) => panic!("Error, read file head failed!"),
        Ok(_) => {},
    };
    u32::from_be_bytes(buffer)
}

pub fn read_next_image(source: &mut File) -> [u8; IMAGE_SIZE] {
    let mut buffer: [u8; IMAGE_SIZE] = [0; IMAGE_SIZE];
    match source.read_exact(&mut buffer) {
        Err(_) => panic!("read file head error!"),
        Ok(_) => {},
    };
    buffer
}

pub fn read_next_label(source: &mut File) -> u8 {
    let mut buffer: [u8; LABEL_SIZE] = [0; LABEL_SIZE];
    match source.read_exact(&mut buffer) {
        Err(_) => panic!("Error, read label failed!"),
        Ok(_) => {},
    }
    buffer[0]
}
