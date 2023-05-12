use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

pub const IMAGE_ROW: usize = 28usize;
pub const IMAGE_COLUMN: usize = 28usize;
pub const IMAGE_SIZE: usize = IMAGE_ROW * IMAGE_COLUMN;
pub const HEAD_SIZE: usize = 4usize;
pub const LABEL_SIZE: usize = 1usize;

/// 读取下一个Head
pub fn read_next_head(source: &mut File) -> u32 {
    let mut buffer: [u8; HEAD_SIZE] = [0; HEAD_SIZE];
    match source.read(&mut buffer[..]) {
        Err(_) => panic!("Error, read file head failed!"),
        Ok(_) => {}
    };
    u32::from_be_bytes(buffer)
}

/// 读取下一个Image
pub fn read_next_image(source: &mut File) -> [u8; IMAGE_SIZE] {
    let mut buffer: [u8; IMAGE_SIZE] = [0; IMAGE_SIZE];
    match source.read(&mut buffer[..]) {
        Err(_) => panic!("read file head error!"),
        Ok(_) => {}
    };
    buffer
}

/// 读取下一个Label
pub fn read_next_label(source: &mut File) -> u8 {
    let mut buffer: [u8; LABEL_SIZE] = [0; LABEL_SIZE];
    match source.read(&mut buffer[..]) {
        Err(_) => panic!("Error, read label failed!"),
        Ok(_) => {}
    }
    buffer[0]
}

/// Sigmoid函数
pub fn sigmoid_activation(input: f32, response: f32) -> f32 {
    1.0 / (1.0 + (input * -1.0f32 * response).exp())
}

///　求激活函数导数
pub fn back_propagation(activation: f32) -> f32 {
    activation * (1.0f32 - activation)
}

/// 预处理输入值（无噪音）
pub fn prepare_inputs_no_noise(inputs: &[u8; IMAGE_SIZE]) -> Vec<f32> {
    let mut output = Vec::with_capacity(IMAGE_SIZE);
    for index in 0..IMAGE_SIZE {
        output.push(match inputs[index] >= 128u8 {
            true => 1.0f32,
            false => 0.0f32,
        });
    }
    output
}

/// 预处理输入值（含噪音）
pub fn prepare_inputs_with_noise(inputs: &[u8; IMAGE_SIZE], rng: &mut ThreadRng) -> Vec<f32> {
    let mut output = Vec::with_capacity(IMAGE_SIZE);
    for index in 0..IMAGE_SIZE {
        let rand_f32: f32 = rng.gen();
        output.push(match inputs[index] >= 128u8 {
            true => 1.0f32 + rand_f32 * 0.1f32,
            false => rand_f32 * 0.1f32,
        });
    }
    output
}

/// 返回vec最大值的坐标
pub fn find_max_index_in_vec(target_vec: &Vec<f32>) -> u8 {
    let mut max_index: usize = 0usize;
    let mut max_value: f32 = target_vec[max_index];
    for index in 1..target_vec.len() {
        if target_vec[index] > max_value {
            max_index = index;
            max_value = target_vec[index];
        }
    }
    max_index as u8
}
