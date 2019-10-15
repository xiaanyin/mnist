#![allow(unused_imports, dead_code, unused_variables)]

mod util;
mod repository;

use repository::*;

const TRAIN_IMAGES: &str = "resources/train-images.idx3-ubyte";
const TRAIN_LABELS: &str = "resources/train-labels.idx1-ubyte";
const TEST_IMAGES: &str = "resources/t10k-images.idx3-ubyte";
const TEST_LABELS: &str = "resources/t10k-labels.idx1-ubyte";

const ACTIVATION_RESPONSE: f32 = 0.7f32;
const BIAS: i32 = 1i32;

// 输出层神经元个数,固定784个(28*28)
const NUM_NET_IN: usize = 784usize;
// 输出层神经元个数, 0~9, 固定10个
const NUM_NET_OUT: usize = 10usize;
// 隐含层神经元个数， TODO 理解有待加深， 为什么是200？
const NUM_HIDDEN: usize = 200usize;
// 学习率 TODO 理解有待加深
const NET_LEARNING_RATE: f32 = 0.4f32;

fn main() {
    println!("Hello, world!");

    let mut train_data_provider = DataProvider::new(TRAIN_IMAGES, TRAIN_LABELS);
    let mut test_data_provider = DataProvider::new(TEST_IMAGES, TEST_LABELS);

    loop {
        match train_data_provider.next() {
            None => break,
            Some(data) => {
                // TODO
                // do train
                println!("label = [{}]", data.1);
            }
        };
    }

    loop {
        match test_data_provider.next() {
            None => break,
            Some(data) => {
                // TODO
                // do test
            }
        };
    }
}
